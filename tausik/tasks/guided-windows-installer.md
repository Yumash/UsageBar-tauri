---
slug: guided-windows-installer
title: "Build guided Windows installer"
status: done
epic: null
story: null
complexity: medium
role: developer
stack: rust
tier: null
call_budget: null
defect_of: null
scope: null
scope_exclude: null
relevant_files:
  - package.json
  - package-lock.json
  - "src-tauri/Cargo.toml"
  - "src-tauri/Cargo.lock"
  - "src-tauri/tauri.conf.json"
  - "src-tauri/src/lib.rs"
  - "src-tauri/windows/installer-hooks.nsh"
  - README.md
  - README_RU.md
  - README_CN.md
  - "docs/BUILDING.md"
  - CHANGELOG.md
  - ".github/workflows/release.yml"
scope_paths:
  - package.json
  - package-lock.json
  - "src-tauri/Cargo.toml"
  - "src-tauri/Cargo.lock"
  - "src-tauri/tauri.conf.json"
  - "src-tauri/windows"
  - "src/main.js"
  - "src/styles.css"
  - tests
  - README.md
  - README_RU.md
  - README_CN.md
  - "docs/BUILDING.md"
  - CHANGELOG.md
  - ".github/workflows"
  - "src-tauri/src/lib.rs"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on: []
completed_at: "2026-10-25T08:44:23Z"
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

Ship version 0.0.30 with a visible Windows installation wizard and install-time choices for startup behavior while preserving the in-app autostart setting.

## Acceptance Criteria

1. Version is 0.0.30 across package, Cargo, and Tauri metadata. 2. The NSIS installer shows a normal guided installation flow instead of appearing to run the app directly. 3. The installer offers explicit choices for launching after installation and starting UsageBar with Windows; the latter is reflected by the existing in-app setting. 4. Uninstall removes installer-created startup registration. 5. Negative scenario: declining autostart leaves no Run entry and the application remains manually launchable. 6. Windows NSIS builds successfully; Rust and JavaScript tests pass; release documentation describes the choices.

## Plan

## Rollback

Revert the installer configuration/hooks and version/documentation commits; existing 0.0.29 packages remain available.

## Journal

- 2026-10-25T08:38:57Z [implementation] — Researched official Tauri 2 installer and autostart behavior. Decision: keep current-user installation to avoid unnecessary elevation; enable the NSIS language selector and Start Menu page; add a localized installer options page for Windows autostart; leave launch-after-install unchecked by default; keep desktop shortcut and launch controls on the finish page; use the same HKCU Run value name as the in-app autostart plugin.
- 2026-10-25T08:39:38Z [implementation] — Implemented the localized NSIS options page, explicit unchecked post-install launch, Start Menu/language configuration, stable UsageBar autostart registry identity, and the 0.0.30 metadata bump.
- 2026-10-25T08:40:06Z [implementation] — JavaScript tests passed 13/13 and Rust tests passed 102/102 after the installer and autostart identity changes. Starting a real unsigned NSIS package build.
- 2026-10-25T08:43:51Z [implementation] — Domain: UsageBar_0.0.30_x64-setup.exe opened a real UsageBar Setup window, then silently installed version 0.0.30 into an isolated workspace directory. The installed executable and uninstaller existed; silent install left autostart disabled; uninstaller removed its registry key, Run value, and install directory. Native screenshot automation was unavailable in this session, so visual text/layout inspection remains a manual limitation; makensis compiled the three localized strings without warnings.
- 2026-10-25T08:44:22Z [implementation] — AC-1: ✓ package.json, package-lock.json, Cargo.toml, Cargo.lock, tauri.conf.json, binary ProductVersion, and binary FileVersion all report 0.0.30. AC-2: ✓ makensis produced a 4,711,495-byte NSIS package and launching it exposed a real UsageBar Setup window. AC-3: ✓ compiled installer page offers localized Windows autostart; the finish controls remain desktop shortcut and launch, both default unchecked; app startup reads the same UsageBar HKCU Run identity. AC-4: ✓ POSTUNINSTALL deletes UsageBar and legacy usagebar Run/StartupApproved values; isolated uninstall removed all observed registration. AC-5: ✓ silent installation left the UsageBar Run value absent and installed a manually launchable usagebar.exe. AC-6: ✓ 13 JavaScript tests, 102 Rust tests, rustfmt on the changed Rust root, JSON parsing, git diff check, NSIS build, install, and uninstall passed; EN/RU/ZH docs and release notes are updated.
