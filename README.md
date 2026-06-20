# Speakeasy Windows OSS

Speakeasy Windows OSS is the open-source Windows version of Speakeasy.
It is a fresh public repository and is separate from the macOS/private product line and any other OSS variants.

## Scope

Speakeasy is intentionally narrow:

- Hold to talk
- Transcribe speech
- Optionally clean up the transcript
- Copy the result for paste

The initial Windows work should stay focused on that loop before adding broader product surface area.

## Status

This repository is at the early Windows test stage. The current app stack is Tauri v2, React, TypeScript, and Vite. The app can record microphone audio, send it to Groq with a user-supplied API key, optionally clean up the transcript, and copy the result to the clipboard. Windows-native active-app paste still needs follow-up testing and integration.

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

## Windows Packaging

On Windows, build the installer with one command:

```sh
npm run package:windows
```

This uses Tauri's NSIS bundle target. The generated installer still needs human Windows testing before release.

## Three-Step Test

On a Windows machine:

1. Run `npm run package:windows`.
2. Install and open the generated Speakeasy installer.
3. Enter a Groq API key in the app, press Start, speak, then press Stop.

The app sends the captured audio directly to Groq from the local machine. The key must never be committed or shared in issues, logs, screenshots, or test notes.

## Configuration

Do not commit secrets, local environment files, credentials, recordings, logs, packaged builds, or installer outputs.

For the current tester flow, enter the Groq API key in the app. Do not put real keys in `.env.example`, docs, screenshots, logs, issues, or commits.

## Repository Boundary

This repo should not copy private product files, private docs, secrets, local configs, packaged app artifacts, or macOS-specific implementation assumptions. Shared product ideas are fine; shared code should be introduced only after deliberate review.

## License

MIT. See [LICENSE](LICENSE).
