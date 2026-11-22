---
slug: fix-tooltip-edge-position
title: "Fix tooltip positioning after edge changes"
status: done
epic: desktop-widget
story: quota-visibility
complexity: medium
role: developer
stack: javascript
tier: null
call_budget: null
defect_of: adopt-usagebar-baseline
scope: null
scope_exclude: "Do not change provider fetch/auth behavior, settings selection logic, tray behavior, branding, release tags, or publish artifacts."
relevant_files:
  - "src/main.js"
  - "src/geometry.js"
  - "tests/geometry.test.js"
  - package.json
scope_paths:
  - "src/main.js"
  - "src/styles.css"
  - "src-tauri/src/overlay.rs"
  - "src-tauri/src/prefs.rs"
  - "src/geometry.js"
  - "test/geometry.test.js"
  - package.json
  - "tests/geometry.test.js"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on:
  - repair-zai-identity
completed_at: "2026-10-25T07:05:14Z"
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

Keep usage, settings, provider, and update popovers attached to the correct side of the bar after moving or snapping it to another screen edge.

## Acceptance Criteria

1. A bar snapped to bottom opens usage, settings, provider, and update cards above the bar with a downward pointer; top opens below, left opens right, and right opens left. 2. Changing edge while a popover is open invalidates cached stage and card geometry and repositions only after the bar reports its new frame. 3. Placement is clamped to the active monitor work area and remains correct with negative monitor coordinates and non-100-percent DPI. 4. Negative scenario: repeated right-to-bottom-to-left moves cannot leave a stale right-side card, off-screen window, incorrect arrow, or unclickable overlay. 5. Table-driven geometry tests cover all four edges, floating mode, different card sizes, and multi-monitor boundaries.

## Plan

## Rollback

Revert the placement-cache invalidation and geometry helper changes to restore the current positioning behavior.

## Journal

- 2026-10-25T07:04:17Z [implementation] — Extracted edge-aware popover geometry, invalidated cached stages/cards on edge changes, clamped usage/menu overlays to monitor work areas, and added table-driven Node tests for every edge plus negative-coordinate monitors.
- 2026-10-25T07:05:05Z [implementation] — AC verified: 1. ✓ Edge-to-side mapping and arrows use shared geometry. 2. ✓ Edge changes invalidate usage/menu/update caches before placement. 3. ✓ Work-area clamping handles negative coordinates and logical frames. 4. ✓ Repeated moves cannot reuse menuCardFrame or tipStageFrame. 5. ✓ Node table tests cover four edges, floating, card sizes, and monitor boundaries.
- 2026-10-25T07:05:10Z [implementation] — Root cause (regression): popover windows cached geometry independently from the bar edge, so snapping the bar reused a stage/card computed for the previous edge. Prevention: centralize edge geometry, invalidate every popup cache on edge changes, and retain all-edge placement tests.
- 2026-10-25T07:05:14Z [implementation] — NO-DEAD-END: the first verification used no relevant_files and a singular test/ directory, so TAUSIK could not discover the already passing Node tests; declaring the files and using tests/ resolved the gate configuration.
