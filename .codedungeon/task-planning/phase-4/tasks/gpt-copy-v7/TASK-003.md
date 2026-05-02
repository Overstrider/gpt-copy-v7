# TASK-003: Scaffold frontend app, styling, validation, query, and test tooling

PROJECT_RULES_STATUS: approved
PROJECT_RULES_DIGEST: 62b79626958677deaa3fd0d932f147920ee8a93f3de3d5f5fa283023aaf20d6b
PROJECT_RULES_READ: yes

- Repo: gpt-copy-v7
- Kind: dev
- Wave: 1
- Parallel Group: foundation
- Owner Role: frontend
- Depends On: none

## Objective

Create the Next.js App Router TypeScript Tailwind frontend foundation with test runners and dependencies needed for the chat UI.

## Context

- Frontend must live in frontend/
- Use Next.js App Router, TypeScript, Tailwind, zod, TanStack Query, react-markdown, remark-gfm, component tests, and Playwright
- Browser code talks only to the backend API base URL

## Write Scope

- frontend/package.json
- frontend/package-lock.json
- frontend/next.config.*
- frontend/tsconfig.json
- frontend/tailwind.config.*
- frontend/postcss.config.*
- frontend/playwright.config.*
- frontend/src/app/**
- frontend/src/test/**

## Acceptance Criteria

- Next.js app renders a minimal App Router page
- Tailwind and global styles are wired
- TanStack Query provider is available to client components
- Component test and Playwright scripts exist in package.json
- No OpenRouter URL or key is exposed in frontend code

## Verification Commands

- npm --prefix frontend run lint
- npm --prefix frontend test -- --run

## Risk Notes

- Do not introduce NEXT_PUBLIC OpenRouter configuration

