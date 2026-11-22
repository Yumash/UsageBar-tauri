---
slug: scaffold-foundation
title: "Scaffold AIRemains desktop foundation"
status: done
epic: desktop-widget
story: implementation
complexity: medium
role: developer
stack: rust
tier: moderate
call_budget: null
defect_of: null
scope: "New application scaffold and normalized model only; no live credential or provider network implementation yet."
scope_exclude: ".tausik/**,.tausik-lib/**,.git/**,docs/architecture.md"
relevant_files: []
scope_paths:
  - package.json
  - package-lock.json
  - tsconfig.json
  - tsconfig.node.json
  - vite.config.ts
  - vitest.config.ts
  - index.html
  - "src/**"
  - "src-tauri/**"
  - "public/**"
  - ".gitignore"
  - README.md
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on: []
completed_at: "2026-10-24T21:45:43Z"
resolution: obsolete
resolution_reason: "Superseded by the owner-approved UsageBar fork strategy; decision is recorded in the task journal and conversation."
tracker_refs: []
started_model_id: gpt-5.6-sol
started_model_version: null
done_model_id: null
done_model_version: null
model_mismatch: 0
no_file_changes_declared: 0
token_budget: null
cost_budget_usd: null
blocked_question: null
unblock_criteria: null
unblocked_by: null
unblocked_at: null
---

## Goal

Create a runnable Tauri 2, React, and TypeScript desktop foundation with normalized usage types, deterministic fixtures, linting, tests, and project branding.

## Acceptance Criteria

1. Development and production frontend builds succeed from documented npm scripts. 2. Rust crate builds and exposes a typed command returning normalized provider snapshots. 3. The initial UI renders loading, ready, empty, and provider-error fixtures without crashing. 4. Negative scenario: malformed or absent fixture fields remain unavailable instead of displaying fabricated zero usage. 5. No source file exceeds 500 lines and generated/build output is ignored.

## Plan

## Rollback

Remove the scaffolded application files while preserving TAUSIK and research documentation.

## Journal

- 2026-10-24T21:45:43Z [implementation] — Owner changed the implementation basis after existing-solution research: use the MIT-licensed UsageBar upstream and preserve its branding. The custom scaffold must not continue.
