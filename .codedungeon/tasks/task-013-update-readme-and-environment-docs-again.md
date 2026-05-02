# TASK-013: Update README and environment docs against implemented commands

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: test
- Wave: 8
- Parallel Group: docs-final
- Owner Role: docs
- Depends On: TASK-001, TASK-005, TASK-007, TASK-008, TASK-012

## Objective

Reconcile README and .env.example with the final backend/frontend command set and runtime configuration after implementation.

## Context

- Docs must remain placeholder-only for secrets
- README should document local backend/frontend startup, tests, and streaming behavior at a practical level
- Do not claim CodeDungeon final completion in README

## Write Scope

- README.md
- .env.example

## Acceptance Criteria

- README commands match actual package and cargo scripts
- .env.example remains placeholder-only and includes OPENROUTER_MODEL default guidance
- Docs explain that automated tests use mocks and do not call OpenRouter
- Docs note backend-only OpenRouter proxying

## Verification Commands

- git diff -- README.md .env.example
- if (git grep -n -I -E "sk-or-v1-[A-Za-z0-9_-]+|OPENROUTER_API_KEY=(sk-|sk-or-v1-)" -- . ':!.env.example' ':!.env') { exit 1 } else { exit 0 }

## Risk Notes

- Documentation drift can cause final verification commands to be misleading

