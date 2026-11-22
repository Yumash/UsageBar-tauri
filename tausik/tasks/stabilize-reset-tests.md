---
slug: stabilize-reset-tests
title: "Stabilize clock-dependent reset tests"
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
scope_exclude: "All runtime provider behavior, UI, preferences, packaging, branding, and unrelated formatting."
relevant_files:
  - "src-tauri/src/providers.rs"
  - "tausik/gates.json"
  - "tausik/README.md"
scope_paths:
  - "src-tauri/src/providers.rs"
  - ".github/workflows/**"
  - "tausik/gates.json"
  - "tausik/README.md"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on:
  - adopt-usagebar-baseline
completed_at: "2026-10-24T21:59:39Z"
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

Make reset-card parser tests independent of calendar time without changing production expiry filtering.

## Acceptance Criteria

1. The three reset-credit tests use a deterministic or runtime-relative clock and pass after calendar time advances. 2. cargo test passes for the full Rust test suite. 3. Negative scenario: expired and consumed reset cards remain filtered exactly as in upstream production behavior. 4. Every provider and runtime parsing path remains unchanged; the patch contains no unrelated formatting cleanup.

## Plan

## Rollback

Revert the isolated test and clock-injection changes.

## Journal

- 2026-10-24T21:58:40Z [implementation] — Introduced internal parser variants with an injected current-time value while keeping production wrappers on now_ms(). Updated the three expired fixed-date tests to use a shared deterministic instant. Targeted reset and ZCode tests pass; full cargo test passes 87/87.
- 2026-10-24T21:59:38Z [implementation] — AC verified: 1. ✓ providers::tests::codex_reset_credits_keep_available_and_drop_used and providers::tests::parse_zcode_reset_status_keeps_usable_and_drops_expired pass with RESET_FIXTURE_NOW_MS. 2. ✓ cargo test: 87 passed, 0 failed. 3. ✓ providers::tests::reset_credits_keep_usable_and_drop_used_or_expired proves consumed and expired cards are still filtered. 4. ✓ git diff is limited to injected-clock parser wrappers, their tests, and TAUSIK exact-path legacy exemptions; production callers still use now_ms(). Domain: runtime parsing receives the same current timestamp as before, while tests can control time. Verification: TAUSIK verify run #10 passed.
