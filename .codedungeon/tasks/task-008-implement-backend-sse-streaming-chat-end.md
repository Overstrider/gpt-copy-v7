# TASK-008: Implement backend SSE streaming chat endpoint

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 6
- Parallel Group: streaming
- Owner Role: backend
- Depends On: TASK-006, TASK-007

## Objective

Add POST /api/conversations/:conversation_id/messages/stream that forwards provider deltas as SSE and persists assistant output only after a complete stream.

## Context

- Streaming endpoint uses native SSE events
- Interrupted streams emit SSE error and do not persist partial assistant output
- OpenRouter stream errors must not crash the server
- Tests must mock provider streams

## Write Scope

- backend/src/provider.rs
- backend/src/routes/messages.rs
- backend/src/routes/mod.rs
- backend/tests/chat_stream.rs

## Acceptance Criteria

- Streaming success emits assistant delta SSE events and a completion event
- Assistant message is persisted only after provider stream completion
- Interrupted provider stream emits an error event and leaves no partial assistant message
- Rate limit, timeout, and invalid stream payload failures are handled explicitly
- Streaming tests run without external provider calls

## Verification Commands

- cargo test --manifest-path backend/Cargo.toml chat_stream

## Risk Notes

- Take care that client disconnects and provider stream errors do not leave partial assistant messages

