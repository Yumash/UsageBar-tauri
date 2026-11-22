---
slug: verify-cross-platform-release
title: "Verify cross-platform release"
status: done
epic: desktop-widget
story: usagebar-adoption
complexity: medium
role: qa
stack: rust
tier: null
call_budget: null
defect_of: null
scope: null
scope_exclude: null
relevant_files:
  - ".github/workflows/ci.yml"
  - "docs/BUILDING.md"
scope_paths:
  - ".github/workflows/**"
  - "src-tauri/tauri.conf.json"
  - "docs/**"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on:
  - secure-glm-key-entry
completed_at: "2026-10-24T22:13:49Z"
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

Verify Windows deliverables and preserve Linux and macOS release readiness for the complete provider catalog.

## Acceptance Criteria

Windows MSI and NSIS build and smoke tests pass; a native CI workflow is configured to build macOS DMG and app plus Linux AppImage and DEB; all providers remain in the catalog; primary four providers receive live or credential-aware local checks; negative scenario: verification and documentation do not create a Git tag, GitHub Release, or alter updater endpoints.

## Plan

## Rollback

Revert release-only workflow or documentation changes; do not alter product behavior to satisfy packaging.

## Journal

- 2026-10-24T22:11:00Z [implementation] — Added a non-publishing native build workflow for Windows MSI/NSIS, Linux DEB/AppImage, and macOS Intel/Apple Silicon DMG/app bundles, plus local build and credential-aware smoke-test documentation.
- 2026-10-24T22:13:42Z [implementation] — AC-1: ✓ npm build produced MSI and NSIS; MSI administrative extraction exited 0; NSIS silent install exited 0; installed usagebar.exe remained responsive; NSIS uninstall exited 0.
- 2026-10-24T22:13:43Z [implementation] — AC-2: ✓ .github/workflows/ci.yml parses as YAML and defines native Windows MSI/NSIS, Ubuntu DEB/AppImage, and macOS Intel/Apple Silicon DMG/app jobs without release permissions.
- 2026-10-24T22:13:43Z [implementation] — AC-3: ✓ src-tauri/src/prefs.rs catalog test passed within the 90-test Rust suite; catalog remains codex, cursor, grok, glm, zcode, claude, copilot, gemini, antigravity.
- 2026-10-24T22:13:43Z [implementation] — AC-4: ✓ local credential-aware check found Codex, Claude, and Cursor sources; no GLM API-key source was present, so its expected result is the localized configuration state. Local prefs select Codex, Claude, Cursor, and GLM with remaining-value display.
- 2026-10-24T22:13:44Z [implementation] — AC-5: ✓ no tag or release command was run; CI triggers only on pull_request or manual dispatch; upstream updater endpoint and product identity are unchanged. Domain: generated packages install and the packaged process responds on Windows; Linux and macOS remain native CI build targets.
- 2026-10-24T22:13:48Z [implementation] — AC-1: ✓ Windows MSI and NSIS packages built and passed extraction/install/process smoke tests. AC-2: ✓ Native CI matrix covers Windows, Ubuntu, macOS Apple Silicon, and macOS Intel packages. AC-3: ✓ 90 Rust tests retain all nine catalog providers. AC-4: ✓ Credential-aware local checks covered Codex, Claude, Cursor, and the missing-key GLM state. AC-5: ✓ No tag or release was created and the upstream updater endpoint is unchanged. Domain: installed Windows binary remained responsive; cross-platform jobs use native runners.
