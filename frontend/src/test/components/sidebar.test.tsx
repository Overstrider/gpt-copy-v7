import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, test, vi } from "vitest";

import { Sidebar } from "@/components/Sidebar";

const conversations = [
  {
    id: "c1",
    title: "Planning",
    created_at: "2026-05-02T12:00:00Z",
    updated_at: "2026-05-02T12:00:00Z"
  },
  {
    id: "c2",
    title: "Implementation",
    created_at: "2026-05-02T12:03:00Z",
    updated_at: "2026-05-02T12:03:00Z"
  }
];

describe("Sidebar", () => {
  test("selects conversations and starts a new chat", async () => {
    const onSelectConversation = vi.fn();
    const onCreateConversation = vi.fn();
    const user = userEvent.setup();

    render(
      <Sidebar
        activeConversationId="c1"
        conversations={conversations}
        isLoading={false}
        mobileOpen={false}
        onClose={vi.fn()}
        onCreateConversation={onCreateConversation}
        onSelectConversation={onSelectConversation}
      />
    );

    await user.click(screen.getByRole("button", { name: "New chat" }));
    await user.click(screen.getByRole("button", { name: "Implementation" }));

    expect(onCreateConversation).toHaveBeenCalledTimes(1);
    expect(onSelectConversation).toHaveBeenCalledWith("c2");
    expect(screen.getByRole("button", { name: "Planning" })).toHaveAttribute(
      "aria-current",
      "page"
    );
  });

  test("exposes mobile drawer close behavior", async () => {
    const onClose = vi.fn();
    const user = userEvent.setup();

    render(
      <Sidebar
        activeConversationId="c1"
        conversations={conversations}
        isLoading={false}
        mobileOpen
        onClose={onClose}
        onCreateConversation={vi.fn()}
        onSelectConversation={vi.fn()}
      />
    );

    expect(screen.getByTestId("mobile-sidebar")).toHaveAttribute("data-open", "true");

    await user.click(screen.getByRole("button", { name: "Close conversations" }));

    expect(onClose).toHaveBeenCalledTimes(1);
  });
});
