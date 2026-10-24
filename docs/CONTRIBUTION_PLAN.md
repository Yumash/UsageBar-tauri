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

## Proposed pull-request sequence

Each phase is intentionally independent so upstream can review and merge it
without accepting the rest of the roadmap.

### 1. Stabilize the test baseline

- Replace fixed reset-card expiry fixtures with times relative to the test run,
  or inject a clock into the parser helpers.
- Preserve the production rule that expired cards are filtered out.
- Add a normal CI workflow for tests on pull requests; keep release packaging
  separate.
- Avoid mixing broad rustfmt/clippy cleanup into the behavioral fix.

Acceptance: tests remain valid after calendar time advances and pass on all
three operating-system runners.

### 2. Add Russian localization

- Add Russian alongside English and Chinese in the frontend and native tray
  strings.
- Centralize locale normalization so an unsupported locale still falls back to
  English and an existing `en`/`zh` preference is unchanged.
- Cover settings, provider cards, reset notices, updater UI, errors, and tray
  actions. Vendor and model names remain untranslated.
- Check narrow cards and the settings popover at common Windows scale factors.

Acceptance: `ru` can be selected and persisted, every current provider remains
selectable, and English/Chinese behavior is unchanged.

### 3. Make the tray useful without replacing the overlay

- Retain the current `App icon: Dock / menu bar / neither` setting.
- Expand the tray menu with show/hide overlay, refresh now, open settings, and
  quit.
- Where platform APIs allow it reliably, show a concise freshness/status
  summary without attempting to put all provider metrics in the menu.
- Preserve the existing overlay, gear menu, edge snapping, and click-through
  workflow.

Acceptance: all tray actions work on Windows, macOS, and supported Linux tray
implementations; choosing `neither` behaves exactly as before.

### 4. Improve first-run setup without narrowing the product

- Offer a first-run provider selection using the full upstream catalog.
- A convenience selection for Codex, Claude, Cursor, and GLM/z.ai may be
  offered, but it is not a new global default and never hides other providers.
- Let a new user choose used versus remaining display during setup.
- Never invoke onboarding for an existing preferences file.

Acceptance: all upstream providers are visible in setup and settings, existing
installs retain their provider order, and skipping setup preserves upstream
defaults.

### 5. Improve optional GLM/z.ai configuration

- Keep every existing source (`GLM_API_KEY`, `ZAI_API_KEY`, `Z_AI_API_KEY`,
  `~/.zai/config.json`, and cc-switch) and its precedence behavior.
- Investigate an optional in-app key entry backed by Windows Credential
  Manager, macOS Keychain, and Linux Secret Service.
- Do not copy an existing environment/file credential into storage or return a
  stored secret to the webview.
- Ship this only if the Linux fallback and uninstall/clear behavior are
  explicit and testable; otherwise improve setup diagnostics instead.

Acceptance: existing GLM and ZCode users see no behavior change, and optional
storage neither logs nor exposes the key.

### 6. Release verification

- Smoke-test the Windows NSIS installer, start-at-login, updater, overlay,
  settings, and tray against a clean user profile.
- Run the existing native GitHub Actions matrix for macOS DMG/app and Linux
  AppImage/DEB readiness.
- Verify no provider disappeared by exercising the complete catalog with parser
  fixtures, while running live/manual checks for the owner's four configured
  providers when credentials are available.

## Explicit non-goals

- No rebranding or package/application identifier fork.
- No provider deletion, hidden provider tier, or four-provider-only edition.
- No rewrite in another desktop framework.
- No invented quota periods when a provider does not expose them.
- No credential upload, chat inspection, telemetry, or cloud account service.
- No unrelated mass formatting in feature pull requests.

## Contribution discipline

Work should be split into one branch and one upstream-focused pull request per
phase. Every PR must state the preserved behaviors, include tests for the
changed surface, and be rebased on upstream `main` before submission. Fork-only
governance files stay out of upstream pull requests.
