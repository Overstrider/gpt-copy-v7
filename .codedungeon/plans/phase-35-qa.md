# Phase 3.5 QA Trap Plan

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

## Focused Verification Strategy

Use tests to prove contracts and side effects, not live provider behavior. Automated checks must not call OpenRouter.

## Backend Tests

- `GET /health` succeeds without a real `OPENROUTER_API_KEY`.
- Config defaults `OPENROUTER_MODEL` to `nvidia/nemotron-3-super-120b-a12b:free`.
- SQLite migrations create usable conversation/message tables.
- Conversation create/list/load persists data.
- Message validation rejects:
  - empty content.
  - whitespace-only content.
  - oversized content.
  - malformed conversation IDs.
  - missing conversation IDs.
- JSON errors follow `{ error: { code, message, details } }`.
- Non-streaming send with mocked provider success persists exactly one user and one assistant message.
- Mocked provider failures map to:
  - `503 provider_not_configured`.
  - `429 provider_rate_limited`.
  - `504 provider_timeout`.
  - `502 provider_invalid_response`.
- Streaming success emits SSE deltas and persists assistant only after completion.
- Interrupted stream emits SSE error and does not persist a partial assistant.
- CORS permits the configured frontend origin.

## Frontend Tests

- Composer blocks empty input, disables during send, and re-enables after success/error.
- Sidebar creates/selects conversations and works in mobile drawer mode.
- Transcript renders assistant markdown with GFM tables/code.
- Unsafe raw HTML/script content is not rendered as executable markup.
- API client rejects malformed backend responses with `zod` and surfaces app error state.
- Playwright smoke mocks backend network calls, sends one message, consumes mocked SSE, and displays assistant output.
- Playwright asserts no browser request is made to OpenRouter.

## Commands

Backend:

```powershell
cargo fmt --check --manifest-path backend/Cargo.toml
cargo clippy --manifest-path backend/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path backend/Cargo.toml
```

Frontend:

```powershell
npm --prefix frontend run lint
npm --prefix frontend test
npm --prefix frontend run build
npm --prefix frontend run test:e2e
```

Secret/generated-artifact checks:

```powershell
git grep -n -I -E "sk-or-v1-[A-Za-z0-9_-]+|OPENROUTER_API_KEY=(sk-|sk-or-v1-)" -- . ':!.env.example' ':!.env'
git ls-files backend/target frontend/.next frontend/node_modules frontend/playwright-report frontend/test-results .codedungeon/codedungeon.db
```

Acceptance: no real secrets and no generated build/test/db artifacts are tracked.

## Environment Assumptions

- Rust stable supports edition 2024.
- Node/npm can install and build the generated Next.js project.
- Playwright browsers can be installed or are already available locally.
- `.env` may contain local secrets but is ignored.
- `.env.example` must use placeholders only.

## Residual Manual Checks

- Live OpenRouter compatibility is not automated because quota, keys, and provider availability are external.
- Final UI pass should inspect desktop and mobile layout for overlap, responsive sidebar behavior, and loading/error clarity.
