"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { apiClient } from "@/lib/api";
import type { Conversation } from "@/lib/schemas";

export const conversationsQueryKey = ["conversations"] as const;

export function useConversations() {
  const queryClient = useQueryClient();
  const query = useQuery({
    queryKey: conversationsQueryKey,
    queryFn: apiClient.listConversations
  });

  const createMutation = useMutation({
    mutationFn: (title?: string) => apiClient.createConversation(title),
    onSuccess: (created) => {
      queryClient.setQueryData<Conversation[]>(conversationsQueryKey, (current = []) => [
        created,
        ...current.filter((conversation) => conversation.id !== created.id)
      ]);
      void queryClient.invalidateQueries({ queryKey: conversationsQueryKey });
    }
  });

  return {
    conversations: query.data,
    error: query.error,
    isLoading: query.isLoading,
    isFetching: query.isFetching,
    createConversation: createMutation.mutateAsync,
    createError: createMutation.error,
    isCreating: createMutation.isPending
  };
}
