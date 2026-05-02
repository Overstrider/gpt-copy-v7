# TASK-009: Implement frontend zod API client and TanStack Query hooks

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 2
- Parallel Group: frontend-core
- Owner Role: frontend
- Depends On: TASK-003

## Objective

Create a typed browser API layer that validates backend JSON responses and provides conversation/message query and mutation hooks.

## Context

- Frontend talks only to backend API base URL
- zod validation failures become user-visible app errors
- TanStack Query owns conversation/message cache and invalidation

## Write Scope

- frontend/src/lib/api.ts
- frontend/src/lib/schemas.ts
- frontend/src/lib/errors.ts
- frontend/src/hooks/useConversations.ts
- frontend/src/hooks/useMessages.ts
- frontend/src/test/api-client.test.ts

## Acceptance Criteria

- API client validates conversations, messages, and structured backend errors with zod
- Malformed backend responses are rejected and surfaced as app errors
- Conversation create/list and message list hooks invalidate or refresh expected caches
- No frontend code references OpenRouter domains, keys, or models

## Verification Commands

- npm --prefix frontend test -- --run api-client

## Risk Notes

- Keep DTO names aligned with backend route contracts before shell integration

