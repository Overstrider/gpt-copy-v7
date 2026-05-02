"use client";

import clsx from "clsx";
import { MessageSquareText, Plus, X } from "lucide-react";

import type { Conversation } from "@/lib/schemas";

type SidebarProps = {
  activeConversationId: string | null;
  conversations: Conversation[];
  isLoading: boolean;
  mobileOpen: boolean;
  onClose: () => void;
  onCreateConversation: () => void;
  onSelectConversation: (conversationId: string) => void;
};

export function Sidebar({
  activeConversationId,
  conversations,
  isLoading,
  mobileOpen,
  onClose,
  onCreateConversation,
  onSelectConversation
}: SidebarProps) {
  const content = (
    <aside className="flex h-full w-72 flex-col border-r border-line bg-neutral-950 text-white">
      <div className="flex items-center justify-between gap-2 border-b border-white/10 p-3">
        <button
          className="flex min-h-10 flex-1 items-center justify-center gap-2 rounded-md border border-white/15 px-3 text-sm font-medium hover:bg-white/10 focus:outline-none focus:ring-2 focus:ring-white"
          type="button"
          onClick={onCreateConversation}
        >
          <Plus aria-hidden="true" className="h-4 w-4" />
          <span>New chat</span>
        </button>
        <button
          aria-label="Close conversations"
          className="grid h-10 w-10 place-items-center rounded-md text-white/80 hover:bg-white/10 focus:outline-none focus:ring-2 focus:ring-white md:hidden"
          type="button"
          onClick={onClose}
        >
          <X aria-hidden="true" className="h-5 w-5" />
        </button>
      </div>

      <div className="min-h-0 flex-1 overflow-y-auto p-2">
        {isLoading ? (
          <div className="space-y-2 p-2" aria-label="Loading conversations">
            <div className="h-10 animate-pulse rounded-md bg-white/10" />
            <div className="h-10 animate-pulse rounded-md bg-white/10" />
          </div>
        ) : null}

        {!isLoading && conversations.length === 0 ? (
          <p className="px-3 py-4 text-sm text-white/60">No conversations yet</p>
        ) : null}

        <nav aria-label="Conversations" className="space-y-1">
          {conversations.map((conversation) => {
            const active = conversation.id === activeConversationId;
            return (
              <button
                key={conversation.id}
                aria-current={active ? "page" : undefined}
                className={clsx(
                  "flex min-h-10 w-full items-center gap-2 rounded-md px-3 py-2 text-left text-sm transition focus:outline-none focus:ring-2 focus:ring-white",
                  active ? "bg-white text-neutral-950" : "text-white/82 hover:bg-white/10"
                )}
                type="button"
                onClick={() => {
                  onSelectConversation(conversation.id);
                  onClose();
                }}
              >
                <MessageSquareText aria-hidden="true" className="h-4 w-4 flex-none" />
                <span className="min-w-0 flex-1 truncate">{conversation.title ?? "Untitled chat"}</span>
              </button>
            );
          })}
        </nav>
      </div>
    </aside>
  );

  return (
    <>
      <div aria-hidden={mobileOpen ? "true" : undefined} className="hidden h-full md:block">
        {content}
      </div>
      <div
        aria-hidden={mobileOpen ? undefined : "true"}
        className={clsx(
          "fixed inset-0 z-40 bg-black/45 transition md:hidden",
          mobileOpen ? "pointer-events-auto opacity-100" : "pointer-events-none opacity-0"
        )}
        data-open={mobileOpen ? "true" : "false"}
        data-testid="mobile-sidebar"
      >
        <div
          className={clsx(
            "h-full max-w-[85vw] transition-transform duration-200",
            mobileOpen ? "translate-x-0" : "-translate-x-full"
          )}
        >
          {content}
        </div>
      </div>
    </>
  );
}
