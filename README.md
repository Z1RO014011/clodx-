# Clodx

Clodx is a local-first desktop monitor for Codex usage limits. It runs on macOS and Windows, keeping the current quota visible in a compact blue-and-white terminal-style status widget.

## What it shows

- 5-hour quota remaining and reset time
- Weekly quota remaining and reset time
- Currently available rate-limit reset credits
- A compact `5H / WK` desktop status widget; click it to open the detailed panel

## Privacy and safety

Clodx reads the existing Codex Desktop sign-in state from your machine and uses it only in memory to request quota data from OpenAI endpoints. It never stores access tokens, account IDs, prompts, chat history, or raw quota responses. It does not redeem reset credits or change account settings.

> The quota endpoint is not a published public API. If it changes, Clodx reports unavailable data rather than guessing a value.

## Run locally

Requirements: Node.js 20+, Rust stable, and the [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/).

```sh
npm install
npm run dev -- --host 127.0.0.1
# in a second terminal
npm run tauri dev
```

## Checks

```sh
npm test
npm run build
cd src-tauri && cargo test && cargo check
```

## Distribution

Unsigned local builds are suitable for development. Public macOS releases require signing and notarization; public Windows releases require code signing to avoid SmartScreen warnings.

### Build installers on GitHub

Push a version tag such as `v0.1.0`, or run **Build installers** manually from the repository's Actions page. The workflow produces unsigned artifacts for download:

- `clodx-macos-x64`: macOS DMG for Intel Macs (also runs on Apple Silicon through Rosetta)
- `clodx-windows`: Windows MSI and NSIS installers

## License

This project is currently provided without a license. Add a license before accepting outside contributions or distributing source code for reuse.
