# TASK-006: Implement OpenRouter provider abstraction and non-streaming client

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 3
- Parallel Group: backend-core
- Owner Role: backend
- Depends On: TASK-002

## Objective

Add a mockable ChatProvider trait, OpenRouter request/response client, provider error taxonomy, and tests for server-side failure mapping.

## Context

- OpenRouter calls must remain server-side
- Provider failures include missing key, rate limit, timeout, and invalid response
- Automated tests must use mocks or local fake HTTP and never real OpenRouter quota

## Write Scope

- backend/src/provider.rs
- backend/src/config.rs
- backend/src/errors.rs
- backend/src/lib.rs
- backend/tests/provider.rs

## Acceptance Criteria

- ChatProvider trait supports non-streaming completion calls
- OpenRouter implementation reads OPENROUTER_API_KEY and OPENROUTER_MODEL only from backend runtime config
- Missing key maps to 503 provider_not_configured
- 429 maps to provider_rate_limited, timeout maps to provider_timeout, invalid payload maps to provider_invalid_response
- Provider tests do not require external network or provider credentials

## Verification Commands

- cargo test --manifest-path backend/Cargo.toml provider

## Risk Notes

- Do not add provider secrets to logs or frontend-facing responses

