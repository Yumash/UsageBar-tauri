# UsageBar contribution plan

This fork remains **UsageBar**. Its purpose is to develop small, reviewable
improvements that can be contributed back to
[`rayelzz/UsageBar-tauri`](https://github.com/rayelzz/UsageBar-tauri), not to
create a renamed product or a reduced edition.

## Compatibility contract

- Preserve every upstream provider: Codex, Cursor, Grok, GLM/z.ai, ZCode,
  Claude, Copilot, Gemini, and Antigravity.
- Preserve the provider picker, ordering, all current usage windows, reset
  cards, display styles, edge positions, click-through behavior, updater, and
  existing settings.
- Keep the current product name, package identifiers, preference location, and
  updater compatibility unless an upstream maintainer requests otherwise.
- Do not change existing users' saved providers or preferences during an
  upgrade.
- Treat Codex, Claude, Cursor, and GLM/z.ai as the owner's primary acceptance
  scenario, not as the complete catalog and not as a replacement default.
- Keep provider requests local and direct to official APIs. Add no account
  proxy, telemetry, or hosted credential service.

## What upstream already satisfies

The v0.0.28 baseline already implements most of the original request:

- a cross-platform Tauri application with Windows, Linux, Apple Silicon macOS,
  and Intel macOS release jobs;
- a tiny always-on-top edge overlay with 26 px provider rings, multi-edge
  snapping, drag positioning, and optional click-through;
- selectable and reorderable providers, including all four primary providers;
- both used and remaining quota display modes;
- Windows NSIS, Linux AppImage/DEB, and macOS DMG/app build targets;
- an optional system-tray icon, although its menu currently only quits the app.

Consequently, the plan does not duplicate the overlay or replace the existing
architecture. The approximately 30 x 30 px per-provider target is already met
by the 26 px ring design.

## Baseline audit

The fork starts at upstream commit `2605fd5f782c5044743e1d2fad681935844bebb3`
(v0.0.28 source).

- `npm install`: passes with no reported vulnerabilities.
- `cargo test`: 84 tests pass and 3 fail. All three failures use fixed expiry
  timestamps that are now in the past, so the assertions became dependent on
  the wall clock.
- `cargo fmt --check`: fails on inherited formatting differences. A repository-
  wide formatting-only change would create a noisy contribution, so formatting
  should be limited to files or regions changed by each PR unless upstream
  agrees to a dedicated formatting PR.
- `cargo clippy --all-targets -- -D warnings`: fails on inherited warnings,
  including dead code, argument-count/type-complexity, needless borrow, and
  test-style lints. These are baseline findings, not regressions introduced by
  the fork.
- `npm run build -- --no-sign`: passes on Windows and produces both MSI and
  NSIS installers. A normal signed build requires the upstream updater private
  key, as expected; without that secret it packages both installers and then
  reports the missing `TAURI_SIGNING_PRIVATE_KEY`.

## Current contribution scope

The upstream-focused branch `feature/configurable-quota-surfaces` now contains
the completed work planned for the first contribution:

- time-independent reset-card tests and pull-request CI for Windows x64, Linux
  x64, macOS Apple Silicon, and macOS Intel;
- complete Russian UI localization and a Russian README, while retaining the
  English and Chinese interfaces and documentation;
- full tray controls plus a cached quota summary in the native tooltip/menu;
- one or two compact values per visible provider, with Auto or explicit metric
  selection, without narrowing the existing provider catalog;
- edge-aware detail/settings cards for left, right, top, and bottom placement;
- current ZCode encrypted Coding Plan credential discovery and improved z.ai
  session fallback;
- GitHub CLI/keychain fallback for Copilot and Windows Gemini Desktop consumer
  quotas, including 5-hour and weekly windows;
- documentation for local and CI builds, credential sources, privacy, and
  troubleshooting in English, Chinese, and Russian.

Existing preference files are migrated without resetting provider visibility,
order, geometry, or other settings. Hiding a provider remains presentation
only; every upstream provider stays available in the picker.

## Deferred follow-ups

These ideas are intentionally not part of the current pull request:

- first-run onboarding or a new-user provider preset;
- in-app secret entry backed by Windows Credential Manager, macOS Keychain,
  and Linux Secret Service;
- rebranding, provider removal, or a separate release channel.

They require separate product and security review and should not expand this
contribution.

## Verification status

- Windows live checks cover Codex, Cursor, GLM/z.ai, ZCode, Copilot, and Gemini
  Desktop. Gemini was verified both with its cookie database available and
  while the running desktop client held it locked.
- The current local suite passes 102 Rust tests and 13 JavaScript tests.
- A local Windows 0.0.29 release build produces the application binary, NSIS,
  and MSI packages. The final signing step is expected to require the upstream
  `TAURI_SIGNING_PRIVATE_KEY`; no release is published from the fork.
- The pull-request workflow is responsible for unsigned Linux and macOS build
  verification on their native runners.

## Explicit non-goals

- No rebranding or package/application identifier fork.
- No provider deletion, hidden provider tier, or four-provider-only edition.
- No rewrite in another desktop framework.
- No invented quota periods when a provider does not expose them.
- No credential upload, chat inspection, telemetry, or cloud account service.
- No unrelated mass formatting in feature pull requests.

## Contribution discipline

The upstream pull request must state the preserved behaviors, include the test
and build evidence, and remain based on upstream `main`. Fork-only governance
files such as this plan stay out of the upstream pull request.
