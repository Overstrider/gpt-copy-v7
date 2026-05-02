# TASK-002: Scaffold backend crate, config, database, health, tracing, and CORS

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 1
- Parallel Group: foundation
- Owner Role: backend
- Depends On: none

## Objective

Create the Rust 2024 Axum backend foundation with runtime config defaults, SQLite pool/migrations, router assembly, health endpoint, tracing, and CORS setup.

## Context

- Backend must live in backend/
- Use Rust 2024, Tokio, Axum, SQLite/sqlx, dotenvy, tracing, and tower-http CORS
- OPENROUTER_MODEL defaults to nvidia/nemotron-3-super-120b-a12b:free
- Health must not require a real OpenRouter key

## Write Scope

- backend/Cargo.toml
- backend/migrations/**
- backend/src/main.rs
- backend/src/lib.rs
- backend/src/config.rs
- backend/src/db.rs
- backend/src/routes/mod.rs
- backend/src/routes/health.rs
- backend/tests/health_config.rs

## Acceptance Criteria

- Backend crate builds as Rust edition 2024
- SQLite migrations create conversations and messages tables with UUID text IDs, UTC text timestamps, role checks, cascade delete, and message lookup index
- Config loads local .env at runtime and defaults OPENROUTER_MODEL when unset
- GET /health returns service status without provider access
- CORS is configurable for the frontend origin

## Verification Commands

- cargo fmt --check --manifest-path backend/Cargo.toml
- cargo test --manifest-path backend/Cargo.toml health config

## Risk Notes

- Use runtime sqlx queries rather than compile-time macros that require a prepared database

