import { getApiBaseUrl, parseApiErrorResponse } from "@/lib/api";
import { AppError } from "@/lib/errors";
import { backendErrorResponseSchema } from "@/lib/schemas";

type SendStreamingMessageOptions = {
  conversationId: string;
  content: string;
  signal?: AbortSignal;
  onDelta: (content: string) => void;
};

const deltaEvents = new Set(["assistant_delta", "delta", "message"]);
const doneEvents = new Set(["done", "complete", "completion"]);

function parseJson(data: string) {
  try {
    return JSON.parse(data) as unknown;
  } catch {
    return data;
  }
}

function streamErrorFromData(data: string) {
  const parsedData = parseJson(data);
  const backendError = backendErrorResponseSchema.safeParse(parsedData);
  if (backendError.success) {
    return new AppError(backendError.data.error.code, backendError.data.error.message, {
      details: backendError.data.error.details
    });
  }

  if (
    parsedData &&
    typeof parsedData === "object" &&
    "code" in parsedData &&
    "message" in parsedData &&
    typeof parsedData.code === "string" &&
    typeof parsedData.message === "string"
  ) {
    return new AppError(parsedData.code, parsedData.message);
  }

  return new AppError("stream_error", typeof parsedData === "string" ? parsedData : "Stream failed.");
}

function deltaFromData(data: string) {
  const parsedData = parseJson(data);
  if (typeof parsedData === "string") {
    return parsedData;
  }

  if (
    parsedData &&
    typeof parsedData === "object" &&
    "content" in parsedData &&
    typeof parsedData.content === "string"
  ) {
    return parsedData.content;
  }

  if (
    parsedData &&
    typeof parsedData === "object" &&
    "delta" in parsedData &&
    typeof parsedData.delta === "string"
  ) {
    return parsedData.delta;
  }

  return "";
}

function parseEventBlock(block: string) {
  let eventName = "message";
  const dataLines: string[] = [];

  for (const line of block.split("\n")) {
    if (!line || line.startsWith(":")) {
      continue;
    }

    if (line.startsWith("event:")) {
      eventName = line.slice("event:".length).trim();
      continue;
    }

    if (line.startsWith("data:")) {
      dataLines.push(line.slice("data:".length).replace(/^ /, ""));
    }
  }

  return {
    eventName,
    data: dataLines.join("\n")
  };
}

export async function sendStreamingMessage({
  conversationId,
  content,
  signal,
  onDelta
}: SendStreamingMessageOptions) {
  const trimmed = content.trim();
  if (!trimmed) {
    throw new AppError("validation_error", "Message content is required.");
  }

  const response = await fetch(
    `${getApiBaseUrl()}/api/conversations/${encodeURIComponent(conversationId)}/messages/stream`,
    {
      method: "POST",
      headers: {
        accept: "text/event-stream",
        "content-type": "application/json"
      },
      body: JSON.stringify({ content: trimmed }),
      signal
    }
  );

  if (!response.ok) {
    throw await parseApiErrorResponse(response);
  }

  if (!response.body) {
    throw new AppError("stream_error", "The server did not return a stream.");
  }

  const reader = response.body.getReader();
  const decoder = new TextDecoder();
  let buffer = "";
  let assistantContent = "";
  let completed = false;

  const processBuffer = (final = false) => {
    buffer = buffer.replace(/\r\n/g, "\n");
    const blocks = buffer.split("\n\n");
    buffer = final ? "" : (blocks.pop() ?? "");

    for (const block of blocks) {
      if (!block.trim()) {
        continue;
      }

      const event = parseEventBlock(block);

      if (event.data === "[DONE]" || doneEvents.has(event.eventName)) {
        completed = true;
        continue;
      }

      if (event.eventName === "error") {
        throw streamErrorFromData(event.data);
      }

      if (deltaEvents.has(event.eventName)) {
        const delta = deltaFromData(event.data);
        assistantContent += delta;
        onDelta(assistantContent);
      }
    }
  };

  while (true) {
    const result = await reader.read();
    if (result.done) {
      break;
    }

    buffer += decoder.decode(result.value, { stream: true });
    processBuffer(false);
  }

  buffer += decoder.decode();
  if (buffer.trim()) {
    processBuffer(true);
  }

  if (!completed) {
    throw new AppError("stream_interrupted", "The server ended the stream before completion.");
  }

  return assistantContent;
}
