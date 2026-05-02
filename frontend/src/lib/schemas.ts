import { z } from "zod";

export const conversationSchema = z.object({
  id: z.string().min(1),
  title: z.string().nullable(),
  created_at: z.string().min(1),
  updated_at: z.string().min(1)
});

export const messageSchema = z.object({
  id: z.string().min(1),
  conversation_id: z.string().min(1),
  role: z.enum(["user", "assistant"]),
  content: z.string(),
  created_at: z.string().min(1)
});

export const backendErrorSchema = z.object({
  code: z.string().min(1),
  message: z.string().min(1),
  details: z.unknown().optional()
});

export const backendErrorResponseSchema = z.object({
  error: backendErrorSchema
});

export const conversationListSchema = z
  .union([
    z.array(conversationSchema),
    z.object({
      conversations: z.array(conversationSchema)
    })
  ])
  .transform((value) => (Array.isArray(value) ? value : value.conversations));

export const messageListSchema = z
  .union([
    z.array(messageSchema),
    z.object({
      messages: z.array(messageSchema)
    })
  ])
  .transform((value) => (Array.isArray(value) ? value : value.messages));

export type Conversation = z.infer<typeof conversationSchema>;
export type Message = z.infer<typeof messageSchema>;
export type BackendError = z.infer<typeof backendErrorSchema>;
