# Protocol Draft

## Status

This document describes the first draft of the plugin-to-bridge contract. It is intentionally conservative and leaves room to change details while the bridge is still a skeleton.

The transport is expected to be a local WebSocket connection to `127.0.0.1:11557`.

The current deployment assumption is that the bridge is distributed as a single `aarch64-unknown-linux-musl` binary so the same artifact can be launched both in native Termux and inside Debian `proot`.

## Connection Model

- the Obsidian plugin is the client
- `termux-bridge` is the server
- all communication stays on localhost
- the plugin establishes the connection when the terminal view is opened or explicitly connected
- the bridge owns session state and shell process state

## Initial Session Assumption

The first implementation should assume a single active interactive shell session per plugin connection.

That keeps the first version small:

- one connection
- one attached session
- explicit resize and input events
- streamed output events back to the client

Multi-session support can be added later with explicit session IDs once the basic flow is stable.

## Message Shape

All messages should use JSON objects with a top-level `type` field.

Client to bridge:

```json
{ "type": "hello", "client": "obsidian-plugin", "version": "0.1.0" }
{ "type": "session.create" }
{ "type": "session.attach" }
{ "type": "terminal.input", "data": "ls -la\n" }
{ "type": "terminal.resize", "cols": 80, "rows": 24 }
{ "type": "session.close" }
```

Bridge to client:

```json
{ "type": "hello.ack", "server": "termux-bridge", "version": "0.1.0" }
{ "type": "session.ready" }
{ "type": "terminal.output", "stream": "stdout", "data": "..." }
{ "type": "terminal.exit", "exitCode": 0 }
{ "type": "error", "code": "SESSION_UNAVAILABLE", "message": "..." }
```

## Behavioral Rules

- `hello` happens first after the socket opens
- `session.create` creates a shell-backed session if one does not exist
- `session.attach` attaches the current connection to the active session
- `terminal.input` carries raw terminal input bytes encoded as UTF-8 strings for now
- `terminal.resize` updates PTY dimensions
- `terminal.output` is streamed from the bridge as data arrives
- `session.close` closes the attached session, not just the socket

## Known Open Questions

- whether `session.create` and `session.attach` should remain separate in v1
- whether binary frames will be needed for terminal output later
- whether reconnect should resume the same shell or create a fresh one
- whether authentication is unnecessary because the bridge is localhost-only or should still use a shared token
