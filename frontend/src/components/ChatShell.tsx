"use client";

import { useQueryClient } from "@tanstack/react-query";
import { Menu } from "lucide-react";
import { useMemo, useState } from "react";

import { Composer } from "@/components/Composer";
import { ErrorBanner } from "@/components/ErrorBanner";
import { Sidebar } from "@/components/Sidebar";
import { Transcript } from "@/components/Transcript";
import { useConversations } from "@/hooks/useConversations";
import { messagesQueryKey, useMessages } from "@/hooks/useMessages";
import { toAppError } from "@/lib/errors";
import type { Conversation, Message } from "@/lib/schemas";
import { sendStreamingMessage } from "@/lib/streaming";

function nowIso() {
  return new Date().toISOString();
}

function mergeById(messages: Message[]) {
  return messages.filter(
    (message, index, all) => all.findIndex((candidate) => candidate.id === message.id) === index
  );
}

function tempMessage(
  conversationId: string,
  role: Message["role"],
  content: string,
  suffix: string
): Message {
  return {
    id: `temp-${role}-${Date.now()}-${suffix}`,
    conversation_id: conversationId,
    role,
    content,
    created_at: nowIso()
  };
}

export function ChatShell() {
  const queryClient = useQueryClient();
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(null);
  const [mobileOpen, setMobileOpen] = useState(false);
  const [localError, setLocalError] = useState<unknown>(null);
  const [isSending, setIsSending] = useState(false);
  const [optimisticMessages, setOptimisticMessages] = useState<Message[]>([]);

  const {
    conversations = [],
    error: conversationsError,
    createError,
    createConversation,
    isCreating,
    isLoading: conversationsLoading
  } = useConversations();

  const activeConversationId = useMemo(() => {
    if (
      selectedConversationId &&
      conversations.some((conversation) => conversation.id === selectedConversationId)
    ) {
      return selectedConversationId;
    }

    return conversations[0]?.id ?? null;
  }, [selectedConversationId, conversations]);

  const activeConversation = useMemo(
    () => conversations.find((conversation) => conversation.id === activeConversationId) ?? null,
    [activeConversationId, conversations]
  );

  const {
    messages = [],
    error: messagesError,
    isLoading: messagesLoading
  } = useMessages(activeConversationId);

  const visibleMessages = useMemo(
    () =>
      mergeById([
        ...messages,
        ...optimisticMessages.filter((message) => message.conversation_id === activeConversationId)
      ]),
    [activeConversationId, messages, optimisticMessages]
  );

  const visibleError = localError ?? conversationsError ?? createError ?? messagesError;

  const handleCreateConversation = async () => {
    setLocalError(null);
    try {
      const created = await createConversation("New chat");
      setOptimisticMessages([]);
      setSelectedConversationId(created.id);
    } catch (error) {
      setLocalError(error);
    }
  };

  const ensureConversation = async (): Promise<Conversation> => {
    if (activeConversation) {
      return activeConversation;
    }

    const created = await createConversation("New chat");
    setSelectedConversationId(created.id);
    return created;
  };

  const handleSend = async (content: string) => {
    setLocalError(null);
    setIsSending(true);

    let userMessage: Message | null = null;
    let assistantMessage: Message | null = null;

    try {
      const conversation = await ensureConversation();
      userMessage = tempMessage(conversation.id, "user", content, "user");
      assistantMessage = tempMessage(conversation.id, "assistant", "", "assistant");

      setOptimisticMessages((current) => [...current, userMessage as Message, assistantMessage as Message]);

      const assistantContent = await sendStreamingMessage({
        conversationId: conversation.id,
        content,
        onDelta: (partialContent) => {
          setOptimisticMessages((current) =>
            current.map((message) =>
              message.id === assistantMessage?.id ? { ...message, content: partialContent } : message
            )
          );
        }
      });

      const finalAssistantMessage = { ...assistantMessage, content: assistantContent };
      queryClient.setQueryData<Message[]>(messagesQueryKey(conversation.id), (current = []) =>
        mergeById([...current, userMessage as Message, finalAssistantMessage])
      );
      setOptimisticMessages((current) =>
        current.filter((message) => message.id !== userMessage?.id && message.id !== assistantMessage?.id)
      );
    } catch (error) {
      const appError = toAppError(error);
      setLocalError(appError);
      if (assistantMessage) {
        setOptimisticMessages((current) =>
          current.filter((message) => message.id !== assistantMessage?.id)
        );
      }
    } finally {
      setIsSending(false);
    }
  };

  return (
    <div className="flex h-dvh overflow-hidden bg-canvas text-ink">
      <Sidebar
        activeConversationId={activeConversationId}
        conversations={conversations}
        isLoading={conversationsLoading}
        mobileOpen={mobileOpen}
        onClose={() => setMobileOpen(false)}
        onCreateConversation={() => void handleCreateConversation()}
        onSelectConversation={(conversationId) => {
          setOptimisticMessages([]);
          setSelectedConversationId(conversationId);
        }}
      />

      <main className="flex min-w-0 flex-1 flex-col">
        <header className="flex h-14 flex-none items-center gap-3 border-b border-line bg-panel px-3 sm:px-5">
          <button
            aria-label="Open conversations"
            className="grid h-10 w-10 place-items-center rounded-md border border-line text-ink hover:bg-neutral-100 focus:outline-none focus:ring-2 focus:ring-accent md:hidden"
            type="button"
            onClick={() => setMobileOpen(true)}
          >
            <Menu aria-hidden="true" className="h-5 w-5" />
          </button>
          <h1 className="truncate text-base font-semibold" title={activeConversation?.title ?? "gpt-copy-v7"}>
            {activeConversation?.title ?? "gpt-copy-v7"}
          </h1>
          {isCreating ? <span className="ml-auto text-xs text-muted">Creating</span> : null}
        </header>

        <ErrorBanner error={visibleError} onDismiss={() => setLocalError(null)} />

        <section className="min-h-0 flex-1 overflow-y-auto">
          <Transcript
            isLoading={Boolean(activeConversationId) && messagesLoading && messages.length === 0}
            messages={visibleMessages}
          />
        </section>

        <Composer
          disabled={conversationsLoading || isCreating}
          isSending={isSending}
          onSend={handleSend}
        />
      </main>
    </div>
  );
}
