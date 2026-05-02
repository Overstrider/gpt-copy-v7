# TASK-001: Prepare root hygiene files

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 1
- Parallel Group: foundation
- Owner Role: docs
- Depends On: none

## Objective

Create or update root project hygiene for placeholders, ignored secrets, ignored generated outputs, and high-level development commands.

## Context

- PROJECT_RULES_STATUS: approved
- PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
- PROJECT_RULES_READ: yes
- .env must remain ignored and .env.example must contain placeholders only
- Generated databases, build outputs, node_modules, and test reports must not be tracked

## Write Scope

- .gitignore
- .env.example
- README.md

## Acceptance Criteria

- .env.example documents OPENROUTER_API_KEY, OPENROUTER_MODEL, backend settings, and NEXT_PUBLIC_API_BASE_URL using placeholder values only
- .gitignore excludes .env, backend build/database outputs, frontend node_modules/.next, and Playwright reports
- README lists backend and frontend setup and verification commands without claiming final completion

## Verification Commands

- git diff -- .gitignore .env.example README.md
- if (git grep -n -I -E "sk-or-v1-[A-Za-z0-9_-]+|OPENROUTER_API_KEY=(sk-|sk-or-v1-)" -- . ':!.env.example' ':!.env') { exit 1 } else { exit 0 }

## Risk Notes

- Do not copy local values from .env into tracked files

