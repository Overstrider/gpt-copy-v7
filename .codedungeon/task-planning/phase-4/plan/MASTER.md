# CodeDungeon Task Plan: gpt-copy-v7

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

## Tasks

| Task | Wave | Owner | Title | Depends On |
| --- | ---: | --- | --- | --- |
| [TASK-001](../tasks/gpt-copy-v7/TASK-001.md) | 1 | docs | Prepare root hygiene files | none |
| [TASK-002](../tasks/gpt-copy-v7/TASK-002.md) | 1 | backend | Scaffold backend crate, config, database, health, tracing, and CORS | none |
| [TASK-003](../tasks/gpt-copy-v7/TASK-003.md) | 1 | frontend | Scaffold frontend app, styling, validation, query, and test tooling | none |
| [TASK-004](../tasks/gpt-copy-v7/TASK-004.md) | 2 | backend | Implement backend models, validation, errors, and repository functions | TASK-002 |
| [TASK-009](../tasks/gpt-copy-v7/TASK-009.md) | 2 | frontend | Implement frontend zod API client and TanStack Query hooks | TASK-003 |
| [TASK-010](../tasks/gpt-copy-v7/TASK-010.md) | 2 | frontend | Implement frontend chat UI components | TASK-003 |
| [TASK-006](../tasks/gpt-copy-v7/TASK-006.md) | 3 | backend | Implement OpenRouter provider abstraction and non-streaming client | TASK-002 |
| [TASK-011](../tasks/gpt-copy-v7/TASK-011.md) | 3 | frontend | Wire frontend page state to API hooks | TASK-009, TASK-010 |
| [TASK-005](../tasks/gpt-copy-v7/TASK-005.md) | 4 | backend | Implement backend conversation and message JSON routes | TASK-004 |
| [TASK-007](../tasks/gpt-copy-v7/TASK-007.md) | 5 | backend | Implement backend non-streaming chat send endpoint | TASK-004, TASK-006 |
| [TASK-008](../tasks/gpt-copy-v7/TASK-008.md) | 6 | backend | Implement backend SSE streaming chat endpoint | TASK-006, TASK-007 |
| [TASK-012](../tasks/gpt-copy-v7/TASK-012.md) | 7 | frontend | Implement frontend native streaming send and Playwright smoke | TASK-008, TASK-011 |
| [TASK-013](../tasks/gpt-copy-v7/TASK-013.md) | 8 | docs | Update README and environment docs against implemented commands | TASK-001, TASK-005, TASK-007, TASK-008, TASK-012 |
| [TASK-014](../tasks/gpt-copy-v7/TASK-014.md) | 9 | qa | Run final QA verification and hygiene scans | TASK-002, TASK-003, TASK-004, TASK-005, TASK-006, TASK-007, TASK-008, TASK-009, TASK-010, TASK-011, TASK-012, TASK-013 |

## Verification Gate

Final completion requires backend fmt/clippy/tests, frontend lint/tests/build/e2e, secret scan, generated-artifact scan, standalone CodeDungeon review evidence, and CodeDungeon finalization.
