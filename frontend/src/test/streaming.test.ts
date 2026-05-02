import { describe, expect, test, vi } from "vitest";

import { sendStreamingMessage } from "@/lib/streaming";

function streamFrom(chunks: string[]) {
  return new ReadableStream<Uint8Array>({
    start(controller) {
      const encoder = new TextEncoder();
      for (const chunk of chunks) {
        controller.enqueue(encoder.encode(chunk));
      }
      controller.close();
    }
  });
}

describe("sendStreamingMessage", () => {
  test("consumes assistant deltas across chunk boundaries", async () => {
    const deltas: string[] = [];
    vi.stubGlobal(
      "fetch",
      vi.fn(async () =>
        new Response(
          streamFrom([
            'event: assistant_delta\ndata: {"content":"Hel',
            'lo"}\n\n',
            'event: assistant_delta\ndata: {"content":" from stream"}\n\n',
            "event: done\ndata: {}\n\n"
          ]),
          { status: 200, headers: { "content-type": "text/event-stream" } }
        )
      )
    );

    const result = await sendStreamingMessage({
      conversationId: "c1",
      content: "Hello",
      onDelta: (delta) => deltas.push(delta)
    });

    expect(result).toBe("Hello from stream");
    expect(deltas).toEqual(["Hello", "Hello from stream"]);
    expect(fetch).toHaveBeenCalledWith(
      "http://localhost:8080/api/conversations/c1/messages/stream",
      expect.objectContaining({
        method: "POST",
        body: JSON.stringify({ content: "Hello" })
      })
    );
  });

  test("turns stream error events into app errors", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(async () =>
        new Response(
          streamFrom([
            "event: error\n",
            'data: {"error":{"code":"provider_timeout","message":"The provider timed out"}}\n\n'
          ]),
          { status: 200, headers: { "content-type": "text/event-stream" } }
        )
      )
    );

    await expect(
      sendStreamingMessage({
        conversationId: "c1",
        content: "Hello",
        onDelta: vi.fn()
      })
    ).rejects.toMatchObject({
      code: "provider_timeout",
      message: "The provider timed out"
    });
  });
});
