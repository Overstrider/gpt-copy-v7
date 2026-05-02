import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { act, renderHook, waitFor } from "@testing-library/react";
import type { ReactNode } from "react";
import { describe, expect, test, vi } from "vitest";

import { useConversations } from "@/hooks/useConversations";
import { apiClient } from "@/lib/api";
import { AppError } from "@/lib/errors";

const conversation = {
  id: "c1",
  title: "Planning",
  created_at: "2026-05-02T12:00:00Z",
  updated_at: "2026-05-02T12:00:00Z"
};

function jsonResponse(body: unknown, init: ResponseInit = {}) {
  return new Response(JSON.stringify(body), {
    status: init.status ?? 200,
    headers: { "content-type": "application/json", ...init.headers }
  });
}

function queryWrapper(client: QueryClient) {
  return function Wrapper({ children }: { children: ReactNode }) {
    return <QueryClientProvider client={client}>{children}</QueryClientProvider>;
  };
}

describe("apiClient", () => {
  test("validates conversation list responses", async () => {
    vi.stubGlobal("fetch", vi.fn(async () => jsonResponse([conversation])));

    await expect(apiClient.listConversations()).resolves.toEqual([conversation]);
  });

  test("rejects malformed backend JSON as an app error", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(async () =>
        jsonResponse([{ id: "m1", role: "assistant", content: "Missing required fields" }])
      )
    );

    await expect(apiClient.listMessages("c1")).rejects.toMatchObject({
      code: "invalid_response",
      message: "The server returned an unexpected response."
    });
  });

  test("maps structured backend errors", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(async () =>
        jsonResponse(
          { error: { code: "conversation_not_found", message: "Conversation not found" } },
          { status: 404 }
        )
      )
    );

    await expect(apiClient.listMessages("missing")).rejects.toMatchObject({
      code: "conversation_not_found",
      status: 404
    });
  });

  test("create conversation hook refreshes the conversation cache", async () => {
    const created = {
      ...conversation,
      id: "c2",
      title: "Fresh chat",
      created_at: "2026-05-02T12:05:00Z",
      updated_at: "2026-05-02T12:05:00Z"
    };
    let listCalls = 0;

    vi.stubGlobal(
      "fetch",
      vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
        const url = input.toString();
        if (url.endsWith("/api/conversations") && init?.method === "POST") {
          return jsonResponse(created, { status: 201 });
        }
        if (url.endsWith("/api/conversations")) {
          listCalls += 1;
          return jsonResponse(listCalls === 1 ? [conversation] : [created, conversation]);
        }
        throw new AppError("unexpected_request", `Unexpected request: ${url}`);
      })
    );

    const client = new QueryClient({
      defaultOptions: { queries: { retry: false }, mutations: { retry: false } }
    });
    const { result } = renderHook(() => useConversations(), {
      wrapper: queryWrapper(client)
    });

    await waitFor(() => expect(result.current.conversations).toHaveLength(1));

    await act(async () => {
      await result.current.createConversation("Fresh chat");
    });

    await waitFor(() =>
      expect(result.current.conversations?.map((item) => item.id)).toEqual(["c2", "c1"])
    );
  });
});
