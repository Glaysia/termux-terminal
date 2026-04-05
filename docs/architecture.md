# Architecture

## Goal

`obsidian-termux` provides an integrated terminal experience inside Android Obsidian by attaching the plugin to a small local bridge process running in `Termux`.

The project is intentionally split across two runtimes:

- `packages/obsidian-plugin`
  - renders UI inside Obsidian
  - owns view lifecycle, settings, connection state, and terminal embedding
- `crates/termux-bridge`
  - runs as a standalone process in `Termux`
  - owns local transport, session lifecycle, shell spawning, and stream forwarding
  - is intended to ship primarily as one `aarch64-unknown-linux-musl` binary that can run in both native Termux and Debian `proot`

## Why The Bridge Exists

Android Obsidian does not expose a native terminal environment to a plugin in the same way a desktop Electron app might. The plugin therefore cannot rely on direct PTY access and must connect to a local process that actually owns the shell session.

That leads to this shape:

```text
Obsidian Plugin UI
        |
        | local WebSocket or equivalent stream transport
        v
termux-bridge
        |
        | PTY / shell process management
        v
shell session inside Termux
```

## Component Boundaries

### Obsidian Plugin

Responsibilities:

- register the custom Obsidian view or pane
- render the terminal surface
- manage user actions such as connect, reconnect, and resize
- translate UI events into bridge requests
- show connection and session state

Non-goals:

- directly spawning shells
- owning PTY logic
- implementing process-level transport concerns

### Termux Bridge

Responsibilities:

- expose a local endpoint, currently expected to stay on `127.0.0.1:11557`
- validate and accept plugin connections
- create or attach to shell-backed terminal sessions
- forward stdin, stdout, stderr, resize, and control events
- keep runtime overhead small and predictable on Android
- stay portable enough that one release binary can be used inside and outside `proot`

Non-goals:

- rendering UI
- carrying Obsidian-specific state
- becoming a general-purpose remote shell platform

## Current Defaults

- mobile target first: Android
- plugin implementation language: TypeScript
- bridge implementation language: Rust
- bridge release target: `aarch64-unknown-linux-musl`
- transport direction: plugin connects outward to the local bridge
- bridge binding: localhost only

## Immediate Next Steps

- define the first bridge protocol draft
- decide whether the first session model is single-session or multi-session
- add a minimal bridge server with a health endpoint and one shell session path
- add a plugin-side service layer that can connect and report state without a full terminal renderer
