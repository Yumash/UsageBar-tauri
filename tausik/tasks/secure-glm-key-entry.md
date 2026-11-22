---
slug: secure-glm-key-entry
title: "Evaluate secure GLM key entry"
status: done
epic: desktop-widget
story: usagebar-adoption
complexity: complex
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
  - "src-tauri/**"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on:
  - first-run-setup
completed_at: "2026-10-24T22:10:23Z"
resolution: obsolete
resolution_reason: "Existing GLM and z.ai discovery already covers environment, .zai, cc-switch, and ZCode credentials without sending secrets into the webview; adding cross-platform secret storage is unnecessary scope and increases credential risk."
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

Evaluate and, only if portable, implement optional native GLM or z.ai key storage without replacing existing credential sources.

## Acceptance Criteria

Environment, config-file, cc-switch, and ZCode paths remain unchanged; secrets never reach logs or the webview after storage; Windows, macOS, and Linux clear and fallback behavior is documented and tested; otherwise deliver diagnostics only.

## Plan

## Rollback

Remove optional keyring integration and UI, leaving every upstream credential source untouched.

## Journal
