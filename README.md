# gpt-copy-v7

ChatGPT-style monorepo with a Rust 2024 Axum backend and a Next.js App Router frontend.

## Structure

- `backend/`: Axum API, SQLite persistence through `sqlx`, server-side OpenRouter proxy, tests.
- `frontend/`: Next.js, TypeScript, Tailwind, TanStack Query, zod API validation, Playwright smoke test.

## Environment

Create a local `.env` from `.env.example` and set your local OpenRouter key:

```dotenv
OPENROUTER_API_KEY=<your-openrouter-api-key>
OPENROUTER_MODEL=nvidia/nemotron-3-super-120b-a12b:free
DATABASE_URL=sqlite://backend/gpt-copy-v7.sqlite
BACKEND_HOST=127.0.0.1
BACKEND_PORT=8080
FRONTEND_ORIGIN=http://localhost:3000
NEXT_PUBLIC_API_BASE_URL=http://localhost:8080
```

Never commit real provider keys. Automated tests use mocks and do not call OpenRouter.

## Run Locally

Install frontend dependencies:

```powershell
npm --prefix frontend install
```

Run the backend:

```powershell
cargo run --manifest-path backend/Cargo.toml
```

Run the frontend in another terminal:

```powershell
npm --prefix frontend run dev
```

Open `http://localhost:3000`.

## Backend API

- `GET /health`
- `GET /api/conversations`
- `POST /api/conversations`
- `GET /api/conversations/{conversation_id}/messages`
- `POST /api/conversations/{conversation_id}/messages`
- `POST /api/conversations/{conversation_id}/messages/stream`

OpenRouter calls are made only by the backend through `OPENROUTER_API_KEY` and `OPENROUTER_MODEL`.

## Verification Commands

Backend:

```powershell
cargo fmt --check --manifest-path backend/Cargo.toml
cargo clippy --manifest-path backend/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path backend/Cargo.toml
cargo build --manifest-path backend/Cargo.toml
```

Frontend:

```powershell
npm --prefix frontend run lint
npm --prefix frontend test -- --run
npm --prefix frontend run build
npm --prefix frontend run test:e2e
```

Hygiene:

```powershell
git grep -n -I -E "sk-or-v1-[A-Za-z0-9_-]+|OPENROUTER_API_KEY=(sk-|sk-or-v1-)" -- . ':!.env.example' ':!.env'
git ls-files backend/target frontend/.next frontend/node_modules frontend/playwright-report frontend/test-results
```

## Troubleshooting

- Backend cannot open SQLite: make sure the `backend/` directory exists and `DATABASE_URL` points to a writable local path.
- Frontend cannot reach API: confirm `NEXT_PUBLIC_API_BASE_URL` matches the backend URL.
- Chat returns `provider_not_configured`: set `OPENROUTER_API_KEY` in local `.env`.
- Chat returns `provider_rate_limited` or `provider_timeout`: retry later or choose a different `OPENROUTER_MODEL`.
- Playwright browser missing: run `npx --prefix frontend playwright install chromium`.
