---
slug: select-visible-quota-metrics
title: "Add per-provider quota selection"
status: done
epic: desktop-widget
story: quota-visibility
complexity: complex
role: developer
stack: rust
tier: null
call_budget: null
defect_of: null
scope: null
scope_exclude: "Do not remove providers, alter provider fetch/auth logic, hide metrics from detail cards, change bar geometry, implement tray hover, rebrand, tag, release, or publish artifacts."
relevant_files:
  - "src/main.js"
  - "src/styles.css"
  - "src-tauri/src/prefs.rs"
  - "src-tauri/src/lib.rs"
scope_paths:
  - "src/main.js"
  - "src/styles.css"
  - "src-tauri/src/prefs.rs"
  - "src-tauri/src/lib.rs"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on:
  - repair-zai-identity
completed_at: "2026-10-25T07:09:36Z"
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

Let users choose exactly which stable quota metrics appear on the compact bar for each selected provider.

## Acceptance Criteria

1. Provider settings retain provider enable/order controls and add Auto or explicit per-provider metric selection. 2. Choices use stable metric IDs, persist across restarts, and migrate old preferences to Auto without changing current provider visibility. 3. Up to two compact metrics can be selected per provider; all returned metrics remain available in the detail card. 4. Negative scenario: stale, duplicate, or provider-mismatched metric IDs are sanitized and cannot create an empty or broken slot. 5. English, Chinese, and Russian copy and preference tests cover migration and validation.

## Plan

## Rollback

Remove the visibleMetrics preference and nested selector; old visibleProviders behavior remains the fallback.

## Journal

- 2026-10-25T07:09:26Z [implementation] — Implemented per-provider Auto/explicit quota choices (maximum two), localized EN/ZH/RU picker copy, persisted visibleMetrics through a dedicated Tauri command, retained provider ordering and complete detail metrics, and added Rust migration/validation tests.
- 2026-10-25T07:09:35Z [implementation] — AC verified: 1. ✓ providersPanelHtml retains enable/order controls and adds Auto/explicit chips. 2. ✓ prefs::tests::metric_selection_defaults_to_auto_for_legacy_preferences and round-trip test. 3. ✓ UI and Rust normalization cap explicit selections at two; paintUsageTip still consumes every returned metric. 4. ✓ prefs::tests::metric_selection_dedupes_caps_and_drops_invalid_entries plus runtime available-option filtering. 5. ✓ metricSelectionText provides EN/ZH/RU copy and 26 preference tests pass. Domain: unavailable or stale choices visibly fall back to Auto, so no provider slot becomes empty or unusable.
