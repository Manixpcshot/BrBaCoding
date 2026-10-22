# Br Ba Coding Agent

A dark, desktop-first AI workspace inspired by the reference screenshot, built with **Rust + Tauri 2** and React/TypeScript. English is the default UI language; Persian (فارسی) is available from the language button.

## Features in this starter

- Multiple named API providers with a Base URL and API key, plus model discovery through the provider's `/models` endpoint.
- Model selection, streamed chat completions, configurable reasoning-effort hint, and a visible request-status indicator. The app intentionally does **not** expose hidden chain-of-thought returned by a model.
- Local chat history, project grouping, image attachments, drag-and-drop, and text/code-file context for common text formats.
- Memory control (send the full conversation, or only the latest six messages), a selectable permission-mode preference, and English/Persian labels.
- In the native app, workspace data is stored at Tauri's per-user app-data location (on Windows typically `%APPDATA%\com.brbacoding.agent\workspace.json`). In web preview it falls back to browser local storage.

## Run the desktop app

Install Node.js (18+) and Rust, then run:

```sh
npm install
npm run tauri dev
```

Build Windows installers (`.msi` and NSIS `.exe`) from Windows:

```sh
npm run tauri build
```

The Tauri bundle configuration is in `src-tauri/tauri.conf.json`. A Windows host is recommended for producing and testing Windows installers. Pushing an update to this Arena branch runs `.github/workflows/windows-release.yml`; it builds the MSI/EXE and publishes a GitHub pre-release.

## Provider compatibility

The current network adapter uses OpenAI-compatible `GET /models` and streaming `POST /chat/completions` endpoints (for example OpenAI-compatible gateways such as OpenRouter, or a local service). Enter the provider's API root as the Base URL, usually ending in `/v1`. Native requests are proxied from Rust to avoid WebView CORS restrictions. The browser preview uses `fetch`, so the provider must allow CORS. Native model requests and chat streaming are handled in Rust; API keys are included in local workspace state and protected only by the current user's operating-system file permissions. **Do not share `workspace.json` or commit it.**

## Scope and safety

This version provides an AI chat workspace; it does not execute shell commands, modify arbitrary files, or invoke model tool/function calls. The “Full access” control is a saved permission-mode preference only and does not grant the model access to the computer. “Memory” controls how much chat history is included in requests; it is not a separate vector database. Text/code files are included as text where possible, images are passed as image inputs, and other binary formats are shown as attachments but are not parsed. Validate provider support for image inputs and `reasoning_effort`—some compatible APIs may ignore or reject these options.
