# Speakeasy Windows OSS

Speakeasy Windows OSS is the open-source Windows version of Speakeasy.
It is a fresh public repository and is separate from the macOS/private product line and any other OSS variants.

## Scope

Speakeasy is intentionally narrow:

- Hold to talk
- Transcribe speech
- Optionally clean up the transcript
- Paste the result into the active Windows app

The initial Windows work should stay focused on that loop before adding broader product surface area.

## Status

This repository is at the early scaffold stage. The current app stack is Tauri v2, React, TypeScript, and Vite. Windows-native behavior still needs real Windows testing before release.

## Local Setup

```sh
git clone https://github.com/cobibean/speakeasy-windows.git
cd speakeasy-windows
npm install

npm run dev

npm run test
npm run build
```

For the Tauri shell, use `npm run tauri -- dev` during development. Before human Windows testing, use `npm run tauri -- build --no-bundle` to verify the native shell without treating installer packaging as release-ready.

## Configuration

Do not commit secrets, local environment files, credentials, recordings, logs, packaged builds, or installer outputs.

If local configuration is needed, copy `.env.example` to a local ignored env file and fill values on your machine only.

## Repository Boundary

This repo should not copy private product files, private docs, secrets, local configs, packaged app artifacts, or macOS-specific implementation assumptions. Shared product ideas are fine; shared code should be introduced only after deliberate review.

## License

MIT. See [LICENSE](LICENSE).
