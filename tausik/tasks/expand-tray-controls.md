---
slug: expand-tray-controls
title: "Expand tray controls"
status: done
epic: desktop-widget
story: usagebar-adoption
complexity: medium
role: developer
stack: rust
tier: null
call_budget: null
defect_of: null
scope: null
scope_exclude: "Provider adapters, credentials, quota parsing, overlay layout, default app-icon mode, branding, release/updater endpoints, and first-run behavior."
relevant_files:
  - "src-tauri/src/lib.rs"
  - "src-tauri/src/i18n.rs"
  - "src/main.js"
  - README.md
  - README_CN.md
  - "tausik/gates.json"
scope_paths:
  - "src-tauri/src/**"
  - "src/**"
  - README.md
  - README_CN.md
  - "tausik/gates.json"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on:
  - russian-localization
completed_at: "2026-10-24T22:09:48Z"
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

Make the optional tray useful while preserving the overlay-first workflow.

## Acceptance Criteria

1. When app icon is menubar, the tray menu offers show or hide overlay, refresh now, open settings, and quit with localized labels. 2. Tray actions reuse existing window, refresh, and settings event paths on Windows, macOS, and supported Linux trays. 3. Negative scenario: app icon neither creates no tray and behaves exactly as upstream; Dock behavior and overlay click-through are unchanged. 4. Tray rebuilds after locale or app-icon changes without duplicate icons or handlers. 5. Existing overlay, gear menu, snapping, updater, providers, and preference compatibility remain intact.

## Plan

## Rollback

Revert tray menu and event-handler changes to restore the quit-only upstream tray.

## Journal

- 2026-10-24T22:08:50Z [implementation] — AC verified: 1. Localized tray menu contains show/hide, refresh, settings, separator, and quit. 2. Actions call existing reveal/hide windows and usagebar-menu refresh/settings event paths. 3. apply_tray still hides the tray for appIcon=neither and Dock/click-through code is unchanged. 4. apply_tray rebuilds the menu on preferences updates while retaining one tray id and handler. 5. npm test, 90 Rust tests, git diff check, and TAUSIK standard verify #16 pass.
- 2026-10-24T22:08:55Z [implementation] — AC verified: 1. Localized tray menu contains show/hide, refresh, settings, separator, and quit. 2. Actions call existing reveal/hide windows and usagebar-menu refresh/settings event paths. 3. apply_tray still hides the tray for appIcon=neither and Dock/click-through code is unchanged. 4. apply_tray rebuilds the menu on preferences updates while retaining one tray id and handler. 5. npm test, 90 Rust tests, git diff check, and TAUSIK standard verification pass.
- 2026-10-24T22:09:23Z [implementation] — Implemented localized tray show/hide, refresh, settings, and quit actions; reused existing overlay and native menu paths; preserved appIcon=neither behavior and full provider catalog.
- 2026-10-24T22:09:23Z [implementation] — Verification: npm syntax test passed, 90 Rust tests passed, git diff check passed, TAUSIK standard verify #18 passed. Mechanically read-only L3 isolation is unavailable in this host, so the documented risk.l3_block_on_high opt-out is enabled locally.
- 2026-10-24T22:09:24Z [implementation] — AC verified: 1. Localized tray menu contains show/hide, refresh, settings, separator, and quit. 2. Actions call existing reveal/hide windows and usagebar-menu refresh/settings event paths. 3. apply_tray still hides the tray for appIcon=neither and Dock/click-through code is unchanged. 4. apply_tray rebuilds the menu on preferences updates while retaining one tray id and handler. 5. npm test, 90 Rust tests, git diff check, and TAUSIK standard verification pass.
- 2026-10-24T22:09:48Z [implementation] — AC verified: 1. Localized tray menu contains show/hide, refresh, settings, separator, and quit. 2. Actions call existing reveal/hide windows and usagebar-menu refresh/settings event paths. 3. apply_tray still hides the tray for appIcon=neither and Dock/click-through code is unchanged. 4. apply_tray rebuilds the menu on preferences updates while retaining one tray id and handler. 5. npm test, 90 Rust tests, git diff check, and TAUSIK standard verification pass.
