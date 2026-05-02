# TASK-005: Implement backend conversation and message JSON routes

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 4
- Parallel Group: backend-http
- Owner Role: backend
- Depends On: TASK-004

## Objective

Expose conversation list/create, message list, and request validation routes over Axum using the repository and structured errors.

## Context

- GET /api/conversations
- POST /api/conversations with optional title
- GET /api/conversations/:conversation_id/messages
- JSON endpoints return structured errors

## Write Scope

- backend/src/routes/conversations.rs
- backend/src/routes/messages.rs
- backend/src/routes/mod.rs
- backend/src/lib.rs
- backend/tests/conversation_routes.rs

## Acceptance Criteria

- Conversation create/list endpoints persist and return typed DTOs
- Message list returns messages for an existing conversation
- Malformed and missing conversation IDs return documented status codes and error codes
- Route tests run with in-memory or temporary SQLite and no provider

## Verification Commands

- cargo test --manifest-path backend/Cargo.toml conversation_routes

## Risk Notes

- Avoid changing provider or streaming code in this task

