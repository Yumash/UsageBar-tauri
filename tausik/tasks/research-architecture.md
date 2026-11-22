---
slug: research-architecture
title: "Research providers and define product architecture"
status: done
epic: desktop-widget
story: core-product
complexity: simple
role: architect
stack: rust
tier: light
call_budget: null
defect_of: null
scope: "Read-only provider/API research plus architecture and UX documentation only."
scope_exclude: ".tausik/**,.tausik-lib/**,.git/**"
relevant_files:
  - "docs/architecture.md"
scope_paths:
  - "docs/**"
  - README.md
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on: []
completed_at: "2026-10-24T21:37:31Z"
resolution: null
resolution_reason: null
tracker_refs: []
started_model_id: gpt-5.6-sol
started_model_version: null
done_model_id: gpt-5.6-sol
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

Establish evidence-backed provider integration contracts, cross-platform architecture, UX model, security constraints, and an executable delivery plan.

## Acceptance Criteria

1. z.ai, Claude, Cursor, and Codex usage acquisition paths are documented with fallback behavior. 2. Architecture covers Windows, Linux, and macOS with a lightweight always-on-top widget and tray behavior. 3. Negative scenarios include missing credentials, changed upstream APIs, offline state, and partial provider failure. 4. Security rules prohibit logging or persisting raw secrets. 5. Follow-up implementation tasks are explicitly scoped.

## Plan

## Rollback

Remove the generated architecture documents and mark the task obsolete.

## Journal

- 2026-10-24T21:35:27Z [implementation] — TAUSIK initialized for Codex; session #1 active; research task passed QG-0. Frontend-design mode: compact data/dashboard widget, optimized for glanceability and complete provider states.
- 2026-10-24T21:37:11Z [implementation] — Architecture documented: normalized provider model, four credential/data paths, cross-platform tray/widget lifecycle, complete UI states, security rules, packaging matrix, and scoped delivery tasks.
- 2026-10-24T21:37:25Z [implementation] — AC verified: 1. ✓ docs/architecture.md Provider contracts documents z.ai, Claude, Cursor, Codex acquisition and fallbacks. 2. ✓ Technology, runtime, UX, and packaging sections cover Windows, Linux, macOS, tray and always-on-top widget behavior. 3. ✓ Honest-data rules and complete states cover missing credentials, changed/malformed APIs, offline/stale data, and partial provider failure. 4. ✓ Security section prohibits secret logging/persistence and makes Codex/Claude/Cursor sources read-only. 5. ✓ Delivery tasks section scopes five implementation phases.
- 2026-10-24T21:37:31Z [implementation] — Verification run #2 is green for the declared documentation scope. This task changes documentation only, so no executable behavior test applies.
