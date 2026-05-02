# TASK-011: Wire frontend page state to API hooks

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 3
- Parallel Group: frontend-integration
- Owner Role: frontend
- Depends On: TASK-009, TASK-010

## Objective

Connect the App Router page to query hooks and UI components so users can list/create/select conversations and view messages through mocked backend responses.

## Context

- This task wires non-streaming state only
- Streaming send integration is a later task
- API calls are mocked in tests

## Write Scope

- frontend/src/app/page.tsx
- frontend/src/components/ChatShell.tsx
- frontend/src/hooks/useConversations.ts
- frontend/src/hooks/useMessages.ts
- frontend/src/test/chat-shell.test.tsx

## Acceptance Criteria

- Initial page loads conversations and selects an active conversation when available
- Create conversation action updates sidebar and active state
- Message transcript updates from query data
- Backend and validation errors show a visible error state
- Tests mock backend responses and do not require the Rust server

## Verification Commands

- npm --prefix frontend test -- --run chat-shell

## Risk Notes

- Avoid duplicating server state outside TanStack Query except transient UI input state

