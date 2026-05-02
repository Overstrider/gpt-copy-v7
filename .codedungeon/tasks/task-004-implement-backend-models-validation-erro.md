# TASK-004: Implement backend models, validation, errors, and repository functions

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 2
- Parallel Group: backend-core
- Owner Role: backend
- Depends On: TASK-002

## Objective

Add typed conversation/message models, shared validation/error response mapping, and SQLite repository operations with focused tests.

## Context

- Error JSON shape is { error: { code, message, details } }
- Reject empty, whitespace-only, oversized content, malformed IDs, and missing conversations
- Use DB-only operations for create/list/load

## Write Scope

- backend/src/models.rs
- backend/src/errors.rs
- backend/src/validation.rs
- backend/src/repository.rs
- backend/src/lib.rs
- backend/tests/repository_validation.rs

## Acceptance Criteria

- Repository can create, list, and load conversations
- Repository can insert and list messages in conversation order
- Validation produces stable error codes for malformed IDs and invalid content
- Missing conversation maps to conversation_not_found
- Tests use temporary SQLite databases and run without OpenRouter

## Verification Commands

- cargo test --manifest-path backend/Cargo.toml repository validation

## Risk Notes

- Keep database operations independent from provider calls

