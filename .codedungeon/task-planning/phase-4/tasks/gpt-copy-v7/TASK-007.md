# TASK-007: Implement backend non-streaming chat send endpoint

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 5
- Parallel Group: backend-http
- Owner Role: backend
- Depends On: TASK-004, TASK-006

## Objective

Add POST /api/conversations/:conversation_id/messages that persists user input, calls the provider outside a database transaction, persists assistant output on success, and maps failures.

## Context

- Non-streaming send inserts user message before provider call
- Provider call must not hold a SQLite transaction
- Assistant message persists only after provider success
- Provider tests use injected mocks

## Write Scope

- backend/src/routes/messages.rs
- backend/src/routes/mod.rs
- backend/src/lib.rs
- backend/tests/chat_send.rs

## Acceptance Criteria

- Successful mocked send persists exactly one user message and one assistant message
- Provider failures return documented status and error codes
- Validation failures do not call the provider
- Missing conversations return 404 conversation_not_found
- Tests prove provider is mocked

## Verification Commands

- cargo test --manifest-path backend/Cargo.toml chat_send

## Risk Notes

- User message persistence before provider failure is acceptable; incomplete assistant persistence is not

