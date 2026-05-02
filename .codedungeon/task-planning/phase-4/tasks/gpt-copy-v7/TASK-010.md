# TASK-010: Implement frontend chat UI components

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 2
- Parallel Group: frontend-core
- Owner Role: frontend
- Depends On: TASK-003

## Objective

Build reusable ChatGPT-style sidebar, transcript, composer, and error/loading UI components with focused component tests.

## Context

- UI should feel like a working chat app, not a landing page
- Transcript renders assistant markdown with react-markdown and remark-gfm
- Composer blocks empty input and disables while sending
- Sidebar supports selected state and mobile drawer behavior

## Write Scope

- frontend/src/components/ChatShell.tsx
- frontend/src/components/Sidebar.tsx
- frontend/src/components/Transcript.tsx
- frontend/src/components/Composer.tsx
- frontend/src/components/ErrorBanner.tsx
- frontend/src/test/components/**

## Acceptance Criteria

- Composer rejects empty and whitespace-only messages
- Composer exposes disabled/loading states
- Transcript renders GFM tables/code and does not execute raw HTML/script markup
- Sidebar can create/select conversations and supports mobile drawer state in tests
- Text and controls do not overlap in common desktop and mobile layouts

## Verification Commands

- npm --prefix frontend test -- --run components

## Risk Notes

- Avoid decorative one-note layouts; prioritize dense, usable chat workflow

