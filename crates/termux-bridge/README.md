# termux-bridge

Small Rust bridge process intended to run inside `Termux`.

Planned responsibilities:

- expose a local endpoint for the Obsidian plugin
- manage shell/session lifecycle
- keep runtime overhead small on Android

Current status:

- crate skeleton only
- no transport or PTY implementation yet
