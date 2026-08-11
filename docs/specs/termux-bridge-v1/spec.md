# Termux Bridge V1 Spec

## Metadata

- Status: in-progress
- Owners: termux-bridge maintainers
- Last Updated: 2026-04-07
- Applies To: `crates/termux-bridge`
- Related Docs: `docs/foundation/architecture.md`, `docs/foundation/protocol.md`, `AGENTS.md`

## Summary

Build the first meaningful version of `termux-bridge` as a small Rust process that exposes a localhost WebSocket endpoint and owns a single shell-backed session for the Obsidian plugin.

The bridge should be structurally small, testable, and compatible with the native Termux runtime contract.

## Goals

- provide a local WebSocket bridge at `127.0.0.1:11557`
- enforce a small, explicit JSON protocol
- manage one active shell-backed session
- keep process lifecycle and cleanup predictable
- preserve the shared `aarch64-unknown-linux-musl` runtime artifact

## Non-Goals

- multi-session support
- reconnect and session persistence
- remote access beyond localhost
- authentication or shared secrets
- binary WebSocket frames
- plugin UI implementation details
- native Termux Rust toolchain support

## Environment Constraints

- run from native Termux
- use one `aarch64-unknown-linux-musl` binary

## Current State

The repository already has the architectural split, protocol contract, and Rust bridge direction documented. The bridge initiative is the active implementation track, and its execution details are captured in the related plan and tasks documents in this folder.

## Acceptance Criteria

- the bridge listens on `127.0.0.1:11557`
- the bridge follows the V1 protocol documented in the foundation protocol doc
- the bridge owns shell/session lifecycle rather than the plugin
- the implementation remains compatible with native Termux
- documentation stays aligned when protocol, architecture, or environment assumptions change

## Related Docs

- [Architecture](../../foundation/architecture.md)
- [Bridge Protocol V1](../../foundation/protocol.md)
- [Implementation Plan](plan.md)
- [Execution Tasks](tasks.md)
