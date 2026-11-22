---
slug: tray-hover-quota-summary
title: "Show cached limits on tray hover"
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
scope_exclude: "Do not fetch on pointer movement, expose raw provider errors or credentials, remove tray commands, change provider authentication, rebrand, tag, release, or publish artifacts."
relevant_files:
  - "src/main.js"
  - "src/styles.css"
  - "src-tauri/src/lib.rs"
scope_paths:
  - "src/main.js"
  - "src/styles.css"
  - "src-tauri/src/lib.rs"
  - "src-tauri/src/overlay.rs"
  - "src-tauri/src/prefs.rs"
scope_tools: []
assurance_profiles: []
assurance_impact: null
depends_on:
  - render-dual-quota-cells
completed_at: "2026-10-25T07:17:55Z"
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

Expose the selected provider limits when the pointer rests on the tray icon while retaining the existing tray menu.

## Acceptance Criteria

1. On Windows and macOS, tray Enter shows a summary anchored to the tray rect and Leave closes it after a short grace period unless the summary is hovered. 2. The summary uses cached snapshots, selected providers, selected metrics, locale, and used/remaining mode; pointer Move does not trigger repeated network requests. 3. Linux updates the native tray tooltip and keeps the context-menu summary because Tauri does not emit tray hover events there. 4. Negative scenario: appIcon=neither creates no visible summary; tray clicks, menu commands, overlay tooltip, and quit remain unchanged; handlers do not duplicate after preference changes. 5. z.ai status and errors are represented without exposing credentials.

## Plan

## Rollback

Remove tray icon hover handling and summary payload; retain the existing show/hide, refresh, settings, and quit tray menu.

## Journal

- 2026-10-25T07:17:46Z [implementation] — Added one-time Tauri tray Enter/Leave/Click handlers, cached JS-built quota summaries, native tooltip and disabled context-menu summary, a tray-anchored hover card with grace-period handoff, and appIcon=neither suppression. No hover or Move path invokes fetch_usage; unknown errors are reduced to localized safe status text.
- 2026-10-25T07:17:54Z [implementation] — AC verified: 1. ✓ Tauri 2.11 TrayIconEvent Enter/Leave compiles on Windows; JS grace timer retains the card while usagebar-tip-hover is true. 2. ✓ event handlers only render snaps and visibleMetrics; fetch_usage occurs only in refresh(). 3. ✓ set_tray_summary always updates native tooltip and build_tray_menu adds a cached disabled summary, including Linux where hover events are absent. 4. ✓ appIcon check suppresses Enter, apply_app_icon emits close, handlers register only in setup, and existing menu routing is unchanged. 5. ✓ safeTrayStatus emits only localized known statuses and sanitize_tray_summary strips control text. Negative: Click closes only the hover card and remains handled by the existing native tray menu. Domain: debug usagebar.exe starts cleanly with the new tray event handlers.
