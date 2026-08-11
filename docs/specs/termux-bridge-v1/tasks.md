# Termux Bridge V1 Tasks

## Metadata

- Status: in-progress
- Owners: termux-bridge maintainers
- Last Updated: 2026-04-07
- Applies To: `crates/termux-bridge`
- Related Docs: `docs/specs/termux-bridge-v1/spec.md`, `docs/specs/termux-bridge-v1/plan.md`, `docs/foundation/protocol.md`

## Ordered Tasks

1. [x] Freeze the V1 bridge contract and keep the protocol explicit.
2. [x] Create the internal Rust skeleton with protocol and state-machine tests.
3. [ ] Finalize the minimal bridge server without PTY and keep its tests aligned with the protocol.
4. [ ] Lock down session lifecycle behavior and invalid-state handling.
5. [ ] Integrate a real PTY-backed shell session.
6. [ ] Add resize handling and robust cleanup on close, exit, and disconnect.
7. [ ] Validate the supported build and runtime matrix in Debian `proot` and native Termux.

## Validation

- [ ] `cargo test -p termux-bridge`
- [ ] `cargo build -p termux-bridge --target aarch64-unknown-linux-musl --release` inside Debian `proot`
- [ ] `file` reports the release artifact as statically linked
- [ ] `ldd` reports the release artifact as not a dynamic executable
- [ ] runtime smoke check passes in Debian `proot`
- [ ] runtime smoke check passes in native Termux

## Documentation Updates

- [ ] keep `docs/foundation/protocol.md` aligned with actual bridge behavior
- [ ] keep `docs/foundation/architecture.md` aligned if runtime boundaries change
- [ ] update `README.md`, `crates/termux-bridge/README.md`, and `AGENTS.md` if the environment contract changes
