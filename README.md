# obsidian-termux

`obsidian-termux` is a mixed-language repository for an Android Obsidian plugin and a small local bridge process for `Termux`.

Supporting design docs live in [`docs/architecture.md`](docs/architecture.md) and [`docs/protocol.md`](docs/protocol.md).

The project is split by runtime responsibility:

- `packages/obsidian-plugin`
  - TypeScript-based Obsidian plugin
  - owns mobile UI integration, views, settings, and bridge connection lifecycle
- `crates/termux-bridge`
  - Rust-based local bridge process intended to run in `Termux`
  - owns local socket serving, session attach, stream forwarding, and process/runtime concerns

## Why This Structure

- The Obsidian side naturally fits the plugin ecosystem and stays in TypeScript.
- The bridge side should stay small, low-overhead, and runtime-efficient on Android.
- Keeping both in one repository makes protocol and architecture work easier without forcing the bridge into a Node.js runtime.

## Current Status

This repository currently contains only the project skeleton.

Implemented:

- root repository layout
- `pnpm` workspace for the Obsidian plugin
- Rust crate skeleton for the Termux bridge
- minimal plugin entrypoint and manifest

Not implemented yet:

- actual WebSocket bridge
- shell/session handling
- terminal rendering integration
- protocol definition beyond repository structure

## Design Docs

- [`docs/architecture.md`](docs/architecture.md): component boundaries, runtime responsibilities, and data flow
- [`docs/protocol.md`](docs/protocol.md): draft connection model and message shapes between plugin and bridge

## Repository Layout

```text
.
├─ crates/
│  └─ termux-bridge/
├─ packages/
│  └─ obsidian-plugin/
├─ package.json
├─ pnpm-workspace.yaml
└─ Cargo.toml
```

## Tooling

- JavaScript workspace: `pnpm`
- plugin language: TypeScript
- bridge language: Rust

## Getting Started

Plugin side:

```bash
corepack enable
corepack pnpm install
corepack pnpm --filter @obsidian-termux/obsidian-plugin build
```

Bridge side:

```bash
cargo build -p termux-bridge
```
