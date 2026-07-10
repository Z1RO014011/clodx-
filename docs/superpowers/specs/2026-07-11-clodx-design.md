# Clodx Design

## Goal

Build a local-first Codex quota monitor for macOS and Windows. It shows the 5-hour and weekly quota remaining, each reset time, and currently available rate-limit reset credits and their expiration dates.

## Product behavior

The app lives in the menu bar/system tray and opens a compact, always-on-top widget. It refreshes on demand and periodically, supports hiding, launch at login, and a click-through lock. It does not redeem credits or alter account settings.

## Architecture

Tauri 2 provides a Rust backend and native tray/window behavior on macOS and Windows. A React UI receives a normalized quota snapshot through Tauri commands. A Codex provider reads the existing local Codex Desktop login state and makes bounded, authenticated requests to the quota service.

The provider is isolated behind a shared snapshot model. A future Claude provider can implement the same model without changing the widget.

## Data and safety

The access token is read only from `CODEX_HOME/auth.json` or `~/.codex/auth.json`, used in memory only, and never persisted, logged, or sent to non-OpenAI hosts. Requests have a timeout, response-size cap, 30-second cache, and duplicate-refresh protection. Failures return an explicit signed-out, stale, or unavailable state rather than guessed values.

The provider uses non-public quota-service responses; their format can change. The app treats missing or unparseable values as unavailable.

## Platform delivery

One codebase produces a macOS Universal application and a Windows executable/installer. Local development may use unsigned builds. Public distribution requires Apple signing/notarization and Windows code signing.

## Scope boundary

The first release supports Codex only. “Reset count” means the number of currently available rate-limit reset credits, not a lifetime count of reset actions. Claude is deferred until an approved, reliable source is identified.
