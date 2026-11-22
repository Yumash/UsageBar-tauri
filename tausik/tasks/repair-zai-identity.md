---
slug: repair-zai-identity
title: "Repair z.ai identity and credential discovery"
status: done
epic: desktop-widget
story: quota-visibility
complexity: medium
role: developer
stack: rust
tier: null
call_budget: null
defect_of: null
scope: null
scope_exclude: "Do not change the product name, application identifier, updater endpoints, provider catalog membership, browser cookies, or credential file formats."
relevant_files:
  - "src/main.js"
  - "src-tauri/src/providers.rs"
  - package.json
  - package-lock.json
  - "src-tauri/Cargo.toml"
  - "src-tauri/Cargo.lock"
  - "src-tauri/tauri.conf.json"
  - "tausik/gates.json"
scope_paths:
  - "src/main.js"
  - "src-tauri/src/providers.rs"
  - "src-tauri/src/prefs.rs"
  - "src-tauri/src/i18n.rs"
  - "src/styles.css"
  - package.json
  - package-lock.json
  - "src-tauri/Cargo.toml"
  - "src-tauri/Cargo.lock"
  - "src-tauri/tauri.conf.json"
  - "tausik/gates.json"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on: []
completed_at: "2026-10-25T07:01:52Z"
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

Make an already connected z.ai or ZCode account visible, resolve its usable local credential source for quota requests, and explain incomplete states without merging or removing the existing backend IDs.

## Acceptance Criteria

1. Settings present the existing glm and zcode IDs as clear z.ai connection variants while old preferences remain valid. 2. A connected local z.ai or ZCode session is resolved into a quota-capable source when the supported API permits it; environment, .zai, cc-switch, ZCode Coding Plan, and session sources have explicit precedence. 3. If a detected session cannot authorize quota retrieval, the provider remains visible with a localized actionable status instead of a silent dash. 4. Negative scenario: no browser-cookie harvesting, no secret enters logs or the webview, and no other provider is removed or renamed. 5. Fixture tests cover each source, precedence, session refresh or fallback, incomplete credentials, and missing credentials.

## Plan

## Rollback

Revert the z.ai presentation and diagnostics while retaining the current glm and zcode adapters and preference IDs.

## Journal

- 2026-10-25T07:01:35Z [implementation] — Bumped application metadata to 0.0.29; relabeled glm and zcode surfaces as z.ai variants; added session credential resolution and a session_no_quota diagnostic without serializing credentials.
- 2026-10-25T07:01:36Z [implementation] — AC-1: ✓ old glm/zcode IDs are unchanged while UI names identify z.ai. AC-2: ✓ existing key sources remain first and ZCode session credentials are a fallback with host precedence. AC-3: ✓ session_no_quota is localized. AC-4: ✓ no browser or secret logging path was added. AC-5: ✓ 92 Rust tests and npm syntax test pass, including two new session-source tests.
- 2026-10-25T07:01:52Z [implementation] — AC-1: ✓ glm and zcode preference IDs remain stable while UI titles identify z.ai. AC-2: ✓ providers tests cover active z.ai and fallback session source resolution. AC-3: ✓ localized session_no_quota replaces a silent dash. AC-4: ✓ no browser-cookie path or credential serialization was introduced. AC-5: ✓ npm test, 92 Rust tests, and TAUSIK verify #22 pass.
