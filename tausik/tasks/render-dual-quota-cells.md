---
slug: render-dual-quota-cells
title: "Render session and weekly limits together"
status: done
epic: desktop-widget
story: quota-visibility
complexity: medium
role: developer
stack: javascript
tier: null
call_budget: null
defect_of: null
scope: null
scope_exclude: "Do not remove providers, alter provider authentication/fetching, truncate hover details, change tray behavior, rebrand, tag, release, or publish artifacts."
relevant_files:
  - "src/main.js"
  - "src/styles.css"
  - "src/metrics.js"
  - "tests/metrics.test.js"
  - "src-tauri/src/dock_shape.rs"
  - "src-tauri/src/overlay.rs"
scope_paths:
  - "src/main.js"
  - "src/styles.css"
  - "src-tauri/src/dock_shape.rs"
  - "src-tauri/src/overlay.rs"
  - "src-tauri/src/prefs.rs"
  - "src/metrics.js"
  - "tests/metrics.test.js"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on:
  - fix-tooltip-edge-position
  - select-visible-quota-metrics
completed_at: "2026-10-25T07:13:16Z"
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

Show the selected 5-hour and weekly limits simultaneously on the overlay without requiring hover.

## Acceptance Criteria

1. Providers that expose both a session window and weekly window show two labeled values at once in used or remaining mode. 2. Providers with one or no usable metric retain a compact, unambiguous fallback. 3. Vertical, horizontal, floating, scaled-display, snap, click-through, and gear behavior remain usable with the expanded geometry. 4. Negative scenario: missing or newly renamed metrics show a dash or Auto fallback and never reuse another provider's value. 5. Hover detail continues to show the complete metric list and localized reset times.

## Plan

## Rollback

Restore one headline ring per provider; the metric selection preference can remain dormant without data loss.

## Journal

- 2026-10-25T07:13:06Z [implementation] — Rendered up to two selected quota metrics beside each provider icon, with Auto preferring session plus weekly windows, localized compact labels, used/remaining values, one/no-metric fallbacks, and synchronized expanded geometry across JS and Rust. Detail cards remain unchanged.
- 2026-10-25T07:13:15Z [implementation] — AC verified: 1. ✓ tests/metrics.test.js Auto prefers session and weekly metrics together. 2. ✓ tests/metrics.test.js single and empty providers remain unambiguous. 3. ✓ dock_shape::tests and overlay::tests pass with synchronized vertical/horizontal geometry. 4. ✓ tests/metrics.test.js stale and provider-mismatched IDs fall back to this provider's Auto metrics. 5. ✓ renderBar changed only compactRows; paintUsageTip retains snap.metrics and localized resets. Negative: stale IDs never index a different snapshot and result in Auto for the same provider. Domain: 5h/wk labels and percentages fit beside the existing icon in both orientations.
