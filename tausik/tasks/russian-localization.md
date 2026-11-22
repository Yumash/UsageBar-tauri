---
slug: russian-localization
title: "Add Russian localization"
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
scope_exclude: "Provider network logic, credentials, quota semantics, branding, packaging, default provider selection, and non-localization visual redesign."
relevant_files:
  - "src/main.js"
  - "src-tauri/src/i18n.rs"
  - "src-tauri/src/prefs.rs"
  - README.md
  - README_CN.md
  - package.json
scope_paths:
  - "src/**"
  - "src-tauri/src/i18n.rs"
  - "src-tauri/src/prefs.rs"
  - package.json
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on:
  - stabilize-reset-tests
completed_at: "2026-10-24T22:05:52Z"
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

Add complete Russian UI localization alongside English and Chinese.

## Acceptance Criteria

1. Russian is selectable, persisted, and restored after restart. 2. Frontend settings, provider details, reset notices, update UI, errors, and native tray strings use Russian while vendor and model names remain unchanged. 3. English and Chinese strings and saved locale behavior remain unchanged. 4. Negative scenario: unsupported or malformed locale values fall back to English without hiding any provider or control. 5. The complete provider catalog remains selectable and Russian text does not clip at the supported overlay and settings widths.

## Plan

## Rollback

Revert the locale registry, Russian strings, and tests; existing en and zh preferences remain valid.

## Journal

- 2026-10-24T22:05:44Z [implementation] — Implemented Russian localization across settings, usage cards, metrics, reset cards, dates, errors, update controls, close labels, native quit text, and documentation. Added locale normalization with English fallback, Russian persistence tests, a concise 280 px Russian menu mode, and npm syntax verification. Full Rust suite passes 89/89; TAUSIK verify #13 passes. Live Tauri smoke started responsively with persisted locale=ru and unchanged codex/cursor/grok/glm defaults. Rendered screenshot inspection remains unavailable because Computer Use exposed neither the running native window nor a browser provider; this limitation is recorded as dead end #10.
- 2026-10-24T22:05:51Z [implementation] — AC verified: 1. ✓ prefs::tests::locale_accepts_russian_and_falls_back_to_english plus live startup persisted locale=ru. 2. ✓ Russian strings cover t(), metric/date/error/reset/update paths and i18n::tests::quit_supports_russian_and_falls_back_to_english. 3. ✓ cargo test: 89 passed, including existing en/zh preference tests. 4. ✓ prefs::tests::locale_accepts_russian_and_falls_back_to_english proves unsupported values become en; CATALOG and provider UI are untouched. 5. ✓ Russian menu uses concise copy and a 280 px width; Providers still renders every CATALOG_IDS entry. Domain: dev app started responsive with Russian prefs and unchanged provider defaults. Verification: npm test passed and TAUSIK verify #13 passed. Rendered screenshot automation was unavailable and is explicitly deferred to final local-build acceptance.
