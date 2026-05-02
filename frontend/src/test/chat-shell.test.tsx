import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import type { ReactNode } from "react";
import { describe, expect, test, vi } from "vitest";

import { ChatShell } from "@/components/ChatShell";

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

function renderShell() {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } }
  });

  return render(<ChatShell />, {
    wrapper: ({ children }: { children: ReactNode }) => (
      <QueryClientProvider client={client}>{children}</QueryClientProvider>
    )
  });
}

describe("ChatShell", () => {
  test("loads conversations, selects the first one, and renders messages", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(async (input: RequestInfo | URL) => {
        const url = input.toString();
        if (url.endsWith("/api/conversations")) {
          return jsonResponse([conversation]);
        }
        if (url.endsWith("/api/conversations/c1/messages")) {
          return jsonResponse([
            {
              id: "m1",
              conversation_id: "c1",
              role: "assistant",
              content: "Existing answer",
              created_at: "2026-05-02T12:00:30Z"
            }
          ]);
        }
        throw new Error(`Unexpected request: ${url}`);
      })
    );

    renderShell();

    expect(await screen.findByRole("heading", { name: "Planning" })).toBeInTheDocument();
    expect(await screen.findByText("Existing answer")).toBeInTheDocument();
  });

  test("creates a conversation and makes it active", async () => {
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
        if (url.endsWith("/api/conversations/c1/messages")) {
          return jsonResponse([]);
        }
        if (url.endsWith("/api/conversations/c2/messages")) {
          return jsonResponse([]);
        }
        throw new Error(`Unexpected request: ${url}`);
      })
    );

    renderShell();

    await screen.findByRole("heading", { name: "Planning" });
    await userEvent.click(screen.getByRole("button", { name: "New chat" }));

    expect(await screen.findByRole("heading", { name: "Fresh chat" })).toBeInTheDocument();
  });

  test("surfaces backend errors visibly", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(async () =>
        jsonResponse(
          { error: { code: "provider_timeout", message: "The provider timed out" } },
          { status: 504 }
        )
      )
    );

    renderShell();

    expect(await screen.findByRole("alert")).toHaveTextContent("The provider timed out");
  });
});
