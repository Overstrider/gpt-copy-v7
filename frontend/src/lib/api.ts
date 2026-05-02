import type { z } from "zod";

import { AppError } from "@/lib/errors";
import {
  backendErrorResponseSchema,
  conversationListSchema,
  conversationSchema,
  messageListSchema,
  type Conversation,
  type Message
} from "@/lib/schemas";

const defaultApiBaseUrl = "http://localhost:8080";

export function getApiBaseUrl() {
  return (process.env.NEXT_PUBLIC_API_BASE_URL || defaultApiBaseUrl).replace(/\/+$/, "");
}

function apiUrl(path: string) {
  return `${getApiBaseUrl()}${path}`;
}

async function readJson(response: Response) {
  const text = await response.text();
  if (!text.trim()) {
    return null;
  }

  try {
    return JSON.parse(text) as unknown;
  } catch (error) {
    throw new AppError("invalid_response", "The server returned invalid JSON.", {
      status: response.status,
      cause: error
    });
  }
}

export async function parseApiErrorResponse(response: Response): Promise<AppError> {
  const data = await readJson(response).catch(() => null);
  const parsed = backendErrorResponseSchema.safeParse(data);

  if (parsed.success) {
    return new AppError(parsed.data.error.code, parsed.data.error.message, {
      status: response.status,
      details: parsed.data.error.details
    });
  }

  return new AppError("request_failed", `Request failed with status ${response.status}.`, {
    status: response.status,
    details: data
  });
}

async function requestJson<T>(path: string, schema: z.ZodType<T>, init: RequestInit = {}) {
  const response = await fetch(apiUrl(path), {
    ...init,
    headers: {
      accept: "application/json",
      ...(init.body ? { "content-type": "application/json" } : {}),
      ...init.headers
    }
  });

  if (!response.ok) {
    throw await parseApiErrorResponse(response);
  }

  const data = await readJson(response);
  const parsed = schema.safeParse(data);

  if (!parsed.success) {
    throw new AppError("invalid_response", "The server returned an unexpected response.", {
      status: response.status,
      details: parsed.error.flatten()
    });
  }

  return parsed.data;
}

export const apiClient = {
  listConversations(): Promise<Conversation[]> {
    return requestJson("/api/conversations", conversationListSchema);
  },

  createConversation(title?: string): Promise<Conversation> {
    return requestJson("/api/conversations", conversationSchema, {
      method: "POST",
      body: JSON.stringify(title ? { title } : {})
    });
  },

  listMessages(conversationId: string): Promise<Message[]> {
    return requestJson(
      `/api/conversations/${encodeURIComponent(conversationId)}/messages`,
      messageListSchema
    );
  }
};
