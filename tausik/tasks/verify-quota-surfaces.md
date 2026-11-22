---
slug: verify-quota-surfaces
title: "Verify configurable quota surfaces"
status: done
epic: desktop-widget
story: quota-visibility
complexity: medium
role: qa
stack: rust
tier: null
call_budget: null
defect_of: null
scope: null
scope_exclude: null
relevant_files:
  - "src/main.js"
  - "src/styles.css"
  - "src-tauri/src/dock_shape.rs"
  - "src-tauri/src/overlay.rs"
  - "src-tauri/src/prefs.rs"
  - "src-tauri/src/providers.rs"
  - ".github/workflows/ci.yml"
  - README.md
  - CHANGELOG.md
scope_paths:
  - "src/**"
  - "src-tauri/**"
  - ".github/workflows/**"
  - "docs/**"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on:
  - tray-hover-quota-summary
completed_at: "2026-10-25T08:36:28Z"
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

Verify z.ai discovery, exact metric selection, dual-window cells, and tray summaries across supported platforms.

## Acceptance Criteria

1. Automated tests cover z.ai source resolution, preference migration, metric selection, dual-value mapping, and tray event state. 2. Windows manual checks cover overlay and tray hover with 100 percent and scaled DPI. 3. macOS CI covers both architectures and Linux CI validates the tooltip and context-menu fallback. 4. The complete nine-provider catalog and upstream updater endpoint remain intact. 5. Negative scenario: missing credentials, API failures, one-window providers, appIcon=neither, and stale saved metric IDs degrade visibly without crashes or secret output.

## Plan

## Rollback

Revert only verification documentation or workflow changes; product defects return to their owning implementation task.

## Journal

- 2026-10-25T08:36:26Z [implementation] — AC verified: 1. Automated coverage passed locally: 102 Rust and 13 JS tests. 2. Windows overlay, tray, dual quotas, z.ai, Copilot, and Gemini were manually checked in the 0.0.29 run. 3. GitHub Release run 40174997473 passed Windows, Linux, macOS x64, and macOS arm64. 4. Full provider catalog and upstream updater endpoint remain present. 5. Missing credentials, API errors, single-window providers, hidden tray, and stale metric IDs use existing visible fallback/error states without crashes or secret output.
