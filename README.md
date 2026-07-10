# Clodx

Clodx is a local-first macOS/Windows tray widget for displaying Codex quota windows, reset times, and currently available reset credits.

It reads the existing local Codex Desktop login state only in memory. It does not store tokens, log raw quota responses, redeem reset credits, or modify account settings. The usage endpoint is not a published public API and may change; Clodx reports unavailable data rather than estimating values.

## Development

```sh
npm install
npm test
npm run build
npm run tauri dev
```

Production distribution requires macOS signing/notarization and Windows code signing. Local bundles are unsigned.
