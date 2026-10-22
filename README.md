# Br Ba Coding Agent

A polished, animated desktop AI workspace inspired by the reference screenshot, built with **Rust + Tauri 2** and React/TypeScript. It bundles Manrope and Vazirmatn locally for an offline-ready English/Persian UI.

## Features in this starter

- Multiple named API providers with a Base URL and API key, plus model discovery through the provider's `/models` endpoint.
- Model selection, streamed chat completions, configurable reasoning-effort hint, and a visible request-status indicator. The app intentionally does **not** expose hidden chain-of-thought returned by a model.
- Local chat history, project grouping, image attachments, drag-and-drop, and text/code-file context for common text formats.
- Memory control (send the full conversation, or only the latest six messages), a selectable permission-mode preference, and English/Persian labels.
- Expanded settings for appearance themes/accent colors, reduced motion, app language, update preferences, workspace backups/import, storage location, and chat-history cleanup.
- In the native app, workspace data is stored at Tauri's per-user app-data location (on Windows typically `%APPDATA%\com.brbacoding.agent\workspace.json`). In web preview it falls back to browser local storage.
- A portable `BrBaCodingAgent.exe` is published as the primary Windows download. In-app update checking, signed update installation, and automatic relaunch are integrated.

## Run the desktop app

Install Node.js (18+) and Rust, then run:

```sh
npm install
npm run tauri dev
```

Build the native Windows app from Windows:

```sh
npm run tauri build
```

The Tauri bundle configuration is in `src-tauri/tauri.conf.json`. A Windows host is recommended for producing and testing Windows builds. Pushing an update to this Arena branch runs `.github/workflows/windows-release.yml`; it versions the build and publishes a portable EXE release. When the signing secret is configured, the workflow also publishes signed updater metadata and its NSIS update bundle.

### Enable signed automatic updates

Tauri verifies update signatures before installation. The corresponding public key is included in the app configuration. Add the matching private key as the GitHub Actions repository secret `TAURI_SIGNING_PRIVATE_KEY` (and set `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` only if the key is password-protected). Do not commit or share the private key. Without this secret, releases still contain the portable EXE, but signed update bundles are omitted and in-app update installation cannot work.

## Provider compatibility

The current network adapter uses OpenAI-compatible `GET /models` and streaming `POST /chat/completions` endpoints (for example OpenAI-compatible gateways such as OpenRouter, or a local service). Enter the provider's API root as the Base URL, usually ending in `/v1`. Native requests are proxied from Rust to avoid WebView CORS restrictions. The browser preview uses `fetch`, so the provider must allow CORS. Native model requests and chat streaming are handled in Rust; API keys are included in local workspace state and protected only by the current user's operating-system file permissions. **Do not share `workspace.json` or commit it.**

## Scope and safety

This version provides an AI chat workspace; it does not execute shell commands, modify arbitrary files, or invoke model tool/function calls. The “Full access” control is a saved permission-mode preference only and does not grant the model access to the computer. “Memory” controls how much chat history is included in requests; it is not a separate vector database. Text/code files are included as text where possible, images are passed as image inputs, and other binary formats are shown as attachments but are not parsed. Validate provider support for image inputs and `reasoning_effort`—some compatible APIs may ignore or reject these options.
