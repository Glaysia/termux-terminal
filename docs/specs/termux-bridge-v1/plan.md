# Termux Bridge V1 Plan

## Metadata

- Status: in-progress
- Owners: termux-bridge maintainers
- Last Updated: 2026-04-07
- Applies To: `crates/termux-bridge`
- Related Docs: `docs/specs/termux-bridge-v1/spec.md`, `docs/foundation/architecture.md`, `docs/foundation/protocol.md`, `AGENTS.md`

## Summary

This document defines the implementation plan for `crates/termux-bridge`.

The goal is to build a small Rust bridge that is structurally sound, easy to verify, and compatible with the current environment contract:

- run from native Termux
- ship one `aarch64-unknown-linux-musl` binary

## Product Goal

The first meaningful Rust milestone is:

- a local WebSocket bridge on `127.0.0.1:11557`
- one active shell-backed session
- a clear JSON protocol
- clean process and connection lifecycle

The bridge is not considered ready just because it can spawn a shell once. It should also be understandable, testable, and safe to extend.

## Design Principles

- prefer explicit contracts over implicit behavior
- separate protocol, transport, session state, and PTY concerns
- keep the first state machine small and testable
- avoid Android- or Termux-specific hacks unless required by evidence
- preserve the validated `musl` build path
- document environment assumptions whenever they affect implementation

## Tooling Decisions

Confirmed foundation:

- `cargo`
- `tokio`
- `serde`
- `serde_json`
- `tokio-tungstenite`
- `futures-util`
- `thiserror`
- `tracing`
- `tracing-subscriber`

Confirmed external programs:

- `bash`
- `sh`
- `file`
- `ldd`

Deferred PTY decision:

- prefer `portable-pty`
- fall back to `nix` plus `libc` only if runtime evidence forces it

## Delivery Phases

### Phase 0: Freeze The V1 Contract

Goal:

- turn the current draft into a stable implementation target

Tasks:

- confirm V1 single-session behavior
- confirm WebSocket on `127.0.0.1:11557`
- confirm the JSON message set
- define exact protocol error codes
- keep `session.create` and `session.attach` deliberately separate

### Phase 1: Create The Internal Rust Skeleton

Goal:

- build the crate around stable internal boundaries before adding runtime behavior

Tasks:

- add module files
- introduce shared config constants
- define internal error types
- define protocol enums and structs
- add unit tests for protocol parsing and serialization

### Phase 2: Implement The Minimal Bridge Server Without PTY

Goal:

- prove transport and state-machine behavior before shell process integration

Tasks:

- add a localhost WebSocket server
- enforce `hello` before other client messages
- return `hello.ack`
- implement `session.create` and `session.attach` against a fake session
- return protocol errors for invalid message ordering

### Phase 3: Lock Down The Session State Machine

Goal:

- prevent protocol drift and hidden lifecycle assumptions

Tasks:

- formalize the single-session lifecycle in Rust types
- ensure each client message has an explicit valid-state set
- cover illegal transitions with unit tests
- define socket-close behavior when a session exists

### Phase 4: Integrate Real PTY And Shell Lifecycle

Goal:

- replace the fake session with a real shell-backed session

Tasks:

- choose the PTY crate or system approach based on `musl` and Android compatibility
- spawn the default shell
- connect `terminal.input` to PTY stdin
- stream PTY output back as `terminal.output`
- emit `terminal.exit` when the shell exits
- surface spawn failures as protocol errors

### Phase 5: Add Resize And Robust Cleanup

Goal:

- make the bridge operational rather than merely functional

Tasks:

- implement `terminal.resize`
- implement `session.close`
- clean up shell processes on socket termination when appropriate
- avoid zombie or orphaned processes
- define bridge shutdown behavior

### Phase 6: Runtime Validation On The Supported Environment

Goal:

- prove that the documented environment contract still holds

Tasks:

- build with `aarch64-unknown-linux-musl`
- verify that the binary remains statically linked
- run the same built binary in native Termux
- confirm the bridge starts and basic protocol flow in native Termux

## Testing Strategy

Unit tests:

- protocol parsing
- protocol serialization
- state transitions
- required message ordering
- error code mapping

Integration tests:

- WebSocket handshake flow
- `hello` and `hello.ack`
- `session.create`
- invalid message sequences
- session close behavior

Manual runtime verification:

- `aarch64-unknown-linux-musl` release build
- execution in native Termux
- real shell interaction
- resize behavior

## Risks

- PTY compatibility in native Termux
- over-coupling state and transport logic
- premature feature expansion into reconnect, auth, or multi-session behavior

## Definition Of Done

The Rust side is structurally ready when all of the following are true:

- the bridge boots as a localhost WebSocket server
- protocol messages are typed and tested
- single-session lifecycle rules are explicit and covered by tests
- a real shell can be created, attached, resized, and closed
- cleanup behavior is predictable on exit and disconnect
- the `aarch64-unknown-linux-musl` binary is validated in native Termux
- docs match reality
