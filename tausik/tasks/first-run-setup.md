---
slug: first-run-setup
title: "Add non-destructive first-run setup"
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
scope_exclude: null
relevant_files: []
scope_paths:
  - "src/**"
  - "src-tauri/src/prefs.rs"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on:
  - expand-tray-controls
completed_at: "2026-10-24T22:10:23Z"
resolution: obsolete
resolution_reason: "After adopting upstream v0.0.28, the existing Providers panel already exposes the complete catalog and preserves upgrades; a second first-run flow would duplicate that UI and alter upstream defaults."
tracker_refs: []
started_model_id: null
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

Help new users choose providers and display value without narrowing the full UsageBar catalog or altering upgrades.

## Acceptance Criteria

Every upstream provider appears in setup and settings; a four-provider convenience selection is optional only; existing preferences and upstream defaults are preserved; onboarding runs only without prior preferences.

## Plan

## Rollback

Revert onboarding UI and first-run marker; the existing provider menu remains the fallback.

## Journal
