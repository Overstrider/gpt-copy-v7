# Phase 1 Architecture Plan

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

## Scope

Build `gpt-copy-v7` as a tracked monorepo with:

- `backend/`: Rust 2024 Axum API, SQLite persistence through `sqlx`, OpenRouter proxying, validation, tracing, CORS, backend tests.
- `frontend/`: Next.js App Router, TypeScript, Tailwind, ChatGPT-style chat UI, safe markdown rendering, response validation, component tests, Playwright smoke test.
- root docs and environment examples with placeholders only.

## Backend Design

- Runtime: Tokio + Axum.
- Config: load local `.env` only at runtime through `dotenvy`; never track secrets. `OPENROUTER_MODEL` defaults to `nvidia/nemotron-3-super-120b-a12b:free`.
- Persistence: SQLite pool via `sqlx`, runtime migrations in `backend/migrations`.
- Tables:
  - `conversations(id, title, created_at, updated_at)`.
  - `messages(id, conversation_id, role, content, created_at)`.
- App state:
  - `SqlitePool`.
  - CORS/config.
  - `ChatProvider` trait object so tests can inject mocked OpenRouter behavior.
- API:
  - `GET /health`.
  - `GET /api/conversations`.
  - `POST /api/conversations` with optional title.
  - `GET /api/conversations/:conversation_id/messages`.
  - `POST /api/conversations/:conversation_id/messages` for non-streaming send.
  - `POST /api/conversations/:conversation_id/messages/stream` for SSE streaming.
- Error shape: structured JSON `{ "error": { "code", "message", "details" } }` for normal JSON endpoints.
- Validation: reject empty or oversized chat content and malformed IDs with 400; return 404 for missing conversations.
- OpenRouter:
  - Server-side `reqwest` client only.
  - Non-streaming endpoint calls OpenRouter chat completions and persists assistant output.
  - Streaming endpoint forwards model deltas as SSE, accumulates assistant text, then persists the completed assistant message.
  - Handle timeout, 429/rate limit, invalid provider payload, and interrupted stream as explicit provider errors.

## Frontend Design

- App Router routes under `frontend/src/app`.
- API client validates backend responses with `zod`.
- Conversation sidebar:
  - conversation list, create conversation action, selected state.
  - mobile drawer behavior.
- Main chat:
  - transcript bubbles for user and assistant.
  - assistant markdown rendered with `react-markdown` + `remark-gfm`.
  - composer with loading and disabled states.
  - error banner/toast area for validation/provider/backend errors.
- Data:
  - TanStack Query for conversations and messages.
  - Native streaming `fetch` for SSE send path.
- Tests:
  - component tests for composer, transcript/sidebar behavior, and error/loading state.
  - Playwright smoke test mocking backend network calls for sending a message.

## Verification

- Backend:
  - `cargo fmt --check --manifest-path backend/Cargo.toml`
  - `cargo clippy --manifest-path backend/Cargo.toml --all-targets -- -D warnings`
  - `cargo test --manifest-path backend/Cargo.toml`
- Frontend:
  - `npm --prefix frontend run lint`
  - `npm --prefix frontend test`
  - `npm --prefix frontend run build`
  - `npm --prefix frontend run test:e2e`
- Secret hygiene:
  - inspect tracked files for `OPENROUTER_API_KEY=` values that are not placeholders.

## Risks

- `sqlx` compile-time macros would require a prepared database; use runtime queries for this generated repo.
- Provider streaming can fail after the user message is persisted; surface an SSE error and do not persist an incomplete assistant message.
- Playwright should mock API calls to avoid requiring the Rust server or OpenRouter quota in CI/local smoke.

## Success Criteria

- Monorepo builds from clean checkout with documented commands.
- Backend tests prove health, validation, persistence, and mocked provider behavior.
- Frontend tests prove core chat UI behavior and one send-message smoke flow.
- OpenRouter key remains server-side and only placeholder env values are tracked.
- CodeDungeon finalization reports COMPLETE only after verification passes and standalone review is posted.
