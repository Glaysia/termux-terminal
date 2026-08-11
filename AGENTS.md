# AGENTS

## Scope

This repository contains:

- an Obsidian plugin under `packages/obsidian-plugin`
- a Rust bridge binary under `crates/termux-bridge`

## Spec-Driven Development

For non-trivial work, start by creating or updating the relevant spec folder under:

- `docs/specs/<slug>/`

Each change folder should contain:

- `spec.md`
- `plan.md`
- `tasks.md`

When architecture, protocol, or runtime contracts change, update the relevant canonical foundation docs under:

- `docs/foundation/`

## Runtime Contract

- Public runtime: native Termux on Android `aarch64`.
- Release bridge target: `aarch64-unknown-linux-musl`.
- The bridge binds only to `127.0.0.1`.
- Native Termux-side Rust tooling is not required to run the released bridge.

## Build Guidance

When working on `crates/termux-bridge`, use:

```bash
cargo build -p termux-bridge --target aarch64-unknown-linux-musl --release
```

## Documentation Rule

If the runtime or distribution contract changes, update:

- `docs/vision.md`
- `docs/foundation/`
- this file
