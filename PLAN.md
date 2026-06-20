# Speakeasy Windows OSS Plan

## Intent

This repository is the clean open-source home for the Windows version of Speakeasy.
It is separate from the macOS product repo and from any other OSS experiments or variants.

The product idea is narrow and practical: hold to talk, transcribe, optionally clean up the text, and paste into the active Windows app.

## Repository Boundary

- Fresh GitHub repository and fresh Git history.
- Default branch: main.
- No fork relationship with the macOS app.
- No copied private docs, build artifacts, packaged apps, local settings, environment files, or secrets.
- No macOS-specific implementation, packaging assumptions, or permission behavior should be carried over by default.
- Shared ideas are allowed; shared code should be introduced only after deliberate review.

## OSS Hygiene

- Add an explicit open-source license before meaningful implementation work.
- Keep all credentials and provider keys out of the repo.
- Document configuration with placeholder names only.
- Keep generated binaries, installers, logs, caches, and local test recordings ignored.
- Run a secret scan before public release milestones.

## Product Scope

Initial scope:

- Windows global hold-to-talk hotkey.
- Microphone capture.
- Speech-to-text provider integration.
- Optional cleanup pass.
- Immediate paste into the active application.
- Minimal settings needed to configure local use.

Out of scope until explicitly chosen:

- Accounts.
- Cloud sync.
- Dictation history.
- Multi-step editing workflows.
- Team or admin features.

## Implementation Milestones

1. Define Windows-specific product and platform constraints.
2. Choose the app stack based on Windows capture, hotkey, tray, paste, installer, and OSS contributor ergonomics.
3. Prototype the critical native loop: hotkey, recording, transcription, cleanup, paste.
4. Add a small settings surface with safe local configuration.
5. Package a signed or clearly documented unsigned developer build.
6. Add contributor docs, issue templates, and release hygiene.

## Local Root Target

Target local working tree: `/DEV/speakeasy-windows`.

Do not place the project inside the macOS/private Speakeasy checkout. If `/DEV` resolves to the system device directory or is not writable, fix or choose a separate non-project path before cloning locally.
