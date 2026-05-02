# Phase 2' Domain Plan

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

## Repository Map

- Root owns README, `.env.example`, `.gitignore`, CodeDungeon artifacts, and development commands.
- `backend/` owns all server-side API, persistence, OpenRouter proxying, migrations, and backend tests.
- `frontend/` owns all browser UI, API response validation, component tests, and Playwright smoke tests.

## Backend Boundaries

- Backend is the only OpenRouter caller. The browser never receives `OPENROUTER_API_KEY`.
- SQLite is single-user local application state for v1. There is no auth or tenant boundary in scope.
- Backend modules:
  - `config`: runtime env parsing and defaults.
  - `db`: SQLite pool and migrations.
  - `models`: conversation/message DTOs and row mapping.
  - `provider`: `ChatProvider` trait, OpenRouter implementation, and mockable stream/non-stream responses.
  - `errors`: structured JSON error model and Axum response mapping.
  - `routes`: health, conversation, message, and streaming handlers.
  - `main`/`lib`: tracing, CORS, router assembly, server startup.
- Persistence:
  - `conversations(id, title, created_at, updated_at)`.
  - `messages(id, conversation_id, role, content, created_at)`.
  - UUID text IDs, UTC timestamp text, role checks, conversation cascade delete, message lookup index.
- Transactions:
  - Create/list/load operations are DB-only.
  - Chat send inserts user message before provider call, calls provider outside a DB transaction, then inserts assistant response after success.
  - Streaming send inserts user message, forwards SSE deltas, accumulates assistant text, and persists assistant only after complete provider stream.
  - Do not hold a SQLite transaction while waiting on OpenRouter.
- Failure contracts:
  - Missing key: `503 provider_not_configured`.
  - Rate limit: `429 provider_rate_limited`.
  - Timeout: `504 provider_timeout`.
  - Invalid provider payload: `502 provider_invalid_response`.
  - Interrupted stream/client disconnect: SSE `error`, no partial assistant persistence.
  - Missing conversation: `404 conversation_not_found`.
  - Validation errors: `400 validation_error`.

## Frontend Boundaries

- Frontend treats backend as the only API.
- `NEXT_PUBLIC_API_BASE_URL` points to backend dev server and is safe to expose.
- API client validates JSON with `zod`; failures become user-visible app errors.
- Native streaming fetch handles SSE events for chat streaming.
- TanStack Query owns conversation/message cache and invalidation.
- Components:
  - sidebar: conversation list, selected state, create action, mobile drawer.
  - transcript: message bubbles and assistant markdown.
  - composer: submit, disabled/loading state, validation.
  - shell/page: state orchestration, responsive layout, error banner.
- Tests mock backend responses. No test calls OpenRouter.

## Specialist Roles

- Backend implementation specialist: API, migrations, provider abstraction, backend tests.
- Frontend implementation specialist: Next.js UI, API client, component tests, Playwright smoke.
- QA specialist: command execution, failure triage, secret scan, final reproducibility.
- Review specialists: specification coverage, security/secrets, maintainability, test quality.

## Test Matrix

- Backend health: `GET /health` returns service status without real OpenRouter call.
- Backend validation: empty/oversized content, malformed IDs, missing conversations.
- Backend persistence: create/list/load conversations and messages.
- Backend provider: mocked success, 429, timeout, invalid response, streaming success, interrupted stream.
- Frontend components: composer, transcript markdown, sidebar selection/mobile behavior, loading/error states.
- Frontend smoke: mocked send-message flow through Playwright.

## Blockers

None. v1 accepts single-user local scope and non-idempotent chat POST semantics.
