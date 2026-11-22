---
slug: adopt-usagebar-baseline
title: "Adopt UsageBar upstream and plan changes"
status: done
epic: desktop-widget
story: usagebar-adoption
complexity: medium
role: maintainer
stack: rust
tier: moderate
call_budget: null
defect_of: null
scope: "Repository remote migration, exact upstream baseline adoption, baseline verification, and planning documentation only; no product customization."
scope_exclude: ".tausik/**,.tausik-lib/**,.git/**"
relevant_files:
  - "**"
scope_paths:
  - "**"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on: []
completed_at: "2026-10-24T21:56:49Z"
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

Make the repository track the GitHub fork and upstream UsageBar baseline while preserving TAUSIK governance, then produce an evidence-backed implementation plan with no rebranding.

## Acceptance Criteria

1. GitHub fork Yumash/UsageBar-tauri exists and origin points to it while upstream points to rayelzz/UsageBar-tauri. 2. Upstream main source is imported without changing UsageBar branding, identifiers, defaults, or updater compatibility. 3. TAUSIK governance remains available and the superseded custom-app research is reconciled into an upstream contribution plan. 4. The delta plan covers baseline stabilization, Russian localization, useful tray controls, optional first-run convenience, secure GLM/z.ai configuration research, Windows verification, and Linux/macOS build readiness. 5. Negative scenario: every upstream provider and feature remains available; Codex, Claude, Cursor, and GLM/z.ai are only a priority acceptance scenario, never a reduced catalog or replacement global default. 6. The imported upstream baseline builds or every baseline failure is documented before customization.

## Plan

## Rollback

Restore the prior working tree from git and remove the added GitHub remotes; the GitHub fork can be deleted separately if the owner requests it.

## Journal

- 2026-10-24T21:51:51Z [implementation] — User clarified the preservation contract: retain the complete upstream provider catalog and all functionality. The four personally used providers are a priority acceptance scenario only; no provider deletion, hidden tier, rebranding, global-default replacement, or existing-preference migration.
- 2026-10-24T21:54:57Z [implementation] — Baseline verification: npm install passed with 0 reported vulnerabilities. cargo test: 84 passed, 3 failed because fixed reset-card expiry fixtures are now in the past. cargo fmt --check and clippy -D warnings fail on inherited upstream formatting/lints. Windows tauri build with --no-sign passed and produced MSI plus NSIS; signed updater output correctly requires the unavailable upstream private key.
- 2026-10-24T21:56:21Z [implementation] — Refreshed the Codex TAUSIK profile after upstream import. Project metadata now names UsageBar and stack detection enables the JavaScript gates; the existing Rust task stack remains explicit for backend work.
- 2026-10-24T21:56:47Z [implementation] — AC verified: 1. ✓ Fork https://github.com/Yumash/UsageBar-tauri exists; origin and upstream remotes resolve correctly. 2. ✓ HEAD matches upstream v0.0.28 commit 2605fd5 with UsageBar name, identifiers, defaults, and updater settings unchanged. 3. ✓ TAUSIK Codex profile is refreshed and prior provider research is reconciled into docs/CONTRIBUTION_PLAN.md. 4. ✓ The contribution plan and six ordered TAUSIK tasks cover stabilization, Russian, tray, onboarding, optional secure GLM setup, and release readiness. 5. ✓ Plan compatibility contract and every task explicitly preserve all nine upstream providers and features; the owner four are acceptance priorities only. 6. ✓ npm install passed; cargo test/fmt/clippy baseline failures are documented; npm run build -- --no-sign passed and produced MSI and NSIS.
