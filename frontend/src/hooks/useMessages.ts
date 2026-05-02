"use client";

import { useQuery } from "@tanstack/react-query";

import { apiClient } from "@/lib/api";

export const messagesQueryKey = (conversationId: string) => ["messages", conversationId] as const;

export function useMessages(conversationId: string | null) {
  const query = useQuery({
    queryKey: conversationId ? messagesQueryKey(conversationId) : ["messages", "none"],
    queryFn: () => apiClient.listMessages(conversationId ?? ""),
    enabled: Boolean(conversationId)
  });

  return {
    messages: query.data,
    error: query.error,
    isLoading: query.isLoading,
    isFetching: query.isFetching
  };
}
