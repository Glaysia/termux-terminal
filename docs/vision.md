# Termux Terminal Vision

Termux Terminal is a real interactive native Termux shell inside Android
Obsidian. It is for ordinary shell work, Git, and Codex with a hardware
keyboard, without relying on Chrome or code-server input.

## Scope

- Native Termux on Android `aarch64` is the supported runtime.
- The bridge binds only to `127.0.0.1`; users own any port forwarding they
  configure.
- xterm.js owns terminal behavior and the Rust bridge owns PTY lifecycle.
- Terminal leaves are ephemeral. Durable jobs belong in `tmux` or `screen`.

## Non-Goals

- code-server replacement
- command runner or task system
- SSH client or remote desktop
- reimplementing terminal parsing, completion, IME handling, or shell behavior

## Product Requirements

- Tabs display the shell or running process name, beginning with `bash`.
- Hardware keyboards are first-class: Korean composition, completion, resize,
  paste, Ctrl-C, Ctrl-D, Ctrl-S, and terminal close are release gates.
- Terminal-focused Ctrl shortcuts are forwarded to the PTY as control bytes.
- `~/.obsidianrc` runs only for bridge-owned terminal shells. It does not source
  `~/.bashrc` unless the user enables the generated commented line.
- Token authentication is local-only, expires after six months, and never logs
  terminal content by default.

## Distribution

- The bridge target is `aarch64-unknown-linux-musl`.
- The plugin release contains only Obsidian assets. The bridge binary and its
  checksum use a separate bridge release.
- GitHub Actions publishes provenance attestations for release assets.

Current architecture and protocol contracts live under `docs/foundation/`.
Maintainer workflow lives in `CONTRIBUTING.md`.
