# AGENTS

## Scope

This repository contains:

- an Obsidian plugin under `packages/obsidian-plugin`
- a Rust bridge binary under `crates/termux-bridge`

## Current Environment Contract

The bridge is currently intended to be built inside Debian `proot` and run in both:

- Debian `proot`
- native Termux

The validated shared runtime artifact is:

- `aarch64-unknown-linux-musl`

Native Termux-side Rust tooling is not required for running the built bridge binary.

## Verified Setup

Verified working on 2026-04-05 with:

- native Termux used as the outer host/runtime
- Debian `proot` used as the build environment
- Debian `rustup` toolchain with target `aarch64-unknown-linux-musl`

The resulting bridge binary was verified to:

- build successfully inside Debian `proot`
- report as `statically linked`
- run inside Debian `proot`
- run from native Termux using the same built artifact

## Build Guidance

When working on `crates/termux-bridge`, prefer:

```bash
proot-distro login debian --user harry --termux-home -- bash -lc '
cd /data/data/com.termux/files/home/Projects/obsidian-termux
cargo build -p termux-bridge --target aarch64-unknown-linux-musl --release
'
```

Avoid reintroducing old `prootrust` wrapper paths or Termux-native Rust shims unless there is a clear reason.

## Documentation Rule

If the validated build or runtime environment changes, update:

- `README.md`
- `crates/termux-bridge/README.md`
- this file
