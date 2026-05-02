# TASK-014: Run final QA verification and hygiene scans

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: test
- Wave: 9
- Parallel Group: final-qa
- Owner Role: qa
- Depends On: TASK-002, TASK-003, TASK-004, TASK-005, TASK-006, TASK-007, TASK-008, TASK-009, TASK-010, TASK-011, TASK-012, TASK-013

## Objective

Execute the required backend/frontend verification, Playwright smoke, secret scan, generated-artifact scan, and capture failures for follow-up fixes.

## Context

- Final completion requires verification PASS and approved standalone review evidence
- This task verifies but does not merge
- No automated check should call external OpenRouter

## Write Scope

- .codedungeon/**

## Acceptance Criteria

- cargo fmt check passes for backend
- cargo clippy passes for backend with -D warnings
- cargo test passes for backend
- frontend lint, component tests, build, and Playwright smoke pass
- Tracked files contain no real OpenRouter keys
- Generated databases, build outputs, node_modules, and test reports are not tracked

## Verification Commands

- cargo fmt --check --manifest-path backend/Cargo.toml
- cargo clippy --manifest-path backend/Cargo.toml --all-targets -- -D warnings
- cargo test --manifest-path backend/Cargo.toml
- npm --prefix frontend run lint
- npm --prefix frontend test
- npm --prefix frontend run build
- npm --prefix frontend run test:e2e
- if (git grep -n -I -E "sk-or-v1-[A-Za-z0-9_-]+|OPENROUTER_API_KEY=(sk-|sk-or-v1-)" -- . ':!.env.example' ':!.env') { exit 1 } else { exit 0 }
- if (git ls-files backend/target frontend/.next frontend/node_modules frontend/playwright-report frontend/test-results .codedungeon/codedungeon.db) { exit 1 } else { exit 0 }

## Risk Notes

- If verification fails, create targeted fix tasks instead of weakening checks

