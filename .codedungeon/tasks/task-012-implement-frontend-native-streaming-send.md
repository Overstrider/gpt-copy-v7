# TASK-012: Implement frontend native streaming send and Playwright smoke

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 7
- Parallel Group: streaming
- Owner Role: frontend
- Depends On: TASK-008, TASK-011

## Objective

Add native fetch SSE message sending, optimistic transcript updates, cache reconciliation, and a mocked Playwright smoke flow.

## Context

- Streaming endpoint is POST /api/conversations/:conversation_id/messages/stream
- Use native browser fetch for streaming
- Playwright must mock backend network calls and assert no OpenRouter request is made

## Write Scope

- frontend/src/lib/streaming.ts
- frontend/src/components/ChatShell.tsx
- frontend/src/app/page.tsx
- frontend/e2e/chat-smoke.spec.ts
- frontend/playwright.config.*
- frontend/src/test/streaming.test.ts

## Acceptance Criteria

- Sending a message consumes mocked SSE deltas and displays assistant output incrementally or after completion
- Composer disables during streaming and re-enables after success or error
- Stream errors surface an app error without losing existing conversation history
- Playwright smoke mocks conversations, messages, and streaming send
- Playwright asserts the browser does not request OpenRouter

## Verification Commands

- npm --prefix frontend test -- --run streaming
- npm --prefix frontend run test:e2e

## Risk Notes

- SSE parser must tolerate chunk boundaries and preserve user-visible errors

