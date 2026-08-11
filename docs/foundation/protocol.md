# Bridge Protocol V1

## Metadata

- Status: accepted
- Owners: termux-bridge maintainers
- Last Updated: 2026-08-09
- Applies To: `crates/termux-bridge`, `packages/obsidian-plugin`
- Related Docs: `docs/foundation/architecture.md`, `docs/specs/termux-bridge-v1/spec.md`, `AGENTS.md`

## Status

This document defines the V1 protocol contract between the Obsidian plugin and `termux-bridge`.

It is intended to be strict enough for implementation, testing, and refactoring work on the Rust bridge. Future versions may extend it, but V1 should be treated as a stable foundation unless there is a concrete implementation blocker.

## Scope

V1 is intentionally small.

Included:

- localhost-only transport
- concurrent localhost WebSocket connections
- one active shell-backed session per WebSocket connection
- JSON text messages with a top-level `type` field
- explicit terminal input, output, resize, and close events

Excluded from V1:

- session IDs, cross-connection attachment, or reconnect to a prior session
- authentication or shared secrets
- binary WebSocket frames
- remote network access
- file transfer features

## Runtime Assumptions

The bridge is expected to run as a local process and bind only to:

- `127.0.0.1:11557`

The current deployment assumption remains:

- one `aarch64-unknown-linux-musl` binary
- built inside Debian `proot`
- runnable inside Debian `proot`
- runnable from native Termux

## Roles

- the Obsidian plugin is the client
- `termux-bridge` is the server
- the bridge owns shell lifecycle and session state
- the plugin owns UI lifecycle and terminal rendering

## Transport

V1 transport is:

- WebSocket
- localhost only
- text frames carrying JSON messages

The plugin connects outward to the bridge. The bridge does not initiate any outbound connection to the plugin.

## Session Model

Each WebSocket connection owns one active interactive shell session. The bridge
may serve multiple connections concurrently, and their session state and PTY
resources are isolated from one another.

The session model remains intentionally explicit:

- `session.create` creates that connection's shell-backed session
- `session.attach` attaches that connection to its session

These remain separate in V1 on purpose. The split makes state transitions easier to test and gives the protocol room to grow later without changing the meaning of `session.create`. A connection cannot create a second session or attach to another connection's session.

## Message Rules

All protocol messages are JSON objects with:

- a top-level `type` string

Additional fields depend on the message type.

Field naming should use JSON camelCase.

Unknown message types must be rejected with a protocol error.

Malformed JSON or structurally invalid payloads must be rejected with a protocol error.

## Client To Bridge Messages

### `hello`

Sent first after the WebSocket connection opens.

Example:

```json
{ "type": "hello", "client": "obsidian-plugin", "version": "0.1.0" }
```

Fields:

- `client`: client identifier string
- `version`: client version string

Rules:

- this must be the first client message
- the server must reject all other message types before `hello`

### `session.create`

Requests creation of the current connection's shell-backed session.

Example:

```json
{ "type": "session.create" }
```

Rules:

- valid only after successful `hello`
- valid only when the current connection has no active session
- creates the shell process and any associated PTY resources

### `session.attach`

Attaches the current connection to its active session.

Example:

```json
{ "type": "session.attach" }
```

Rules:

- valid only after successful `hello`
- valid only when an active session exists for the current connection
- valid only when the current connection is not already attached

### `terminal.input`

Sends terminal input to the attached session.

Example:

```json
{ "type": "terminal.input", "data": "ls -la\n" }
```

Fields:

- `data`: UTF-8 string carrying terminal input

Rules:

- valid only while attached to an active session
- V1 treats input as UTF-8 string data

### `terminal.resize`

Updates terminal dimensions for the attached session.

Example:

```json
{ "type": "terminal.resize", "cols": 80, "rows": 24 }
```

Fields:

- `cols`: integer greater than zero
- `rows`: integer greater than zero

Rules:

- valid only while attached to an active session
- invalid dimensions must be rejected

### `session.close`

Closes the active session.

Example:

```json
{ "type": "session.close" }
```

Rules:

- valid only after successful `hello`
- closes the current connection's active shell session, not only the socket
- after close completes, the session no longer exists

## Bridge To Client Messages

### `hello.ack`

Acknowledges the initial handshake.

Example:

```json
{ "type": "hello.ack", "server": "termux-bridge", "version": "0.1.0" }
```

Fields:

- `server`: server identifier string
- `version`: server version string

### `session.ready`

Indicates that a session exists and is ready for attachment or use.

Example:

```json
{ "type": "session.ready" }
```

Rules:

- sent after successful `session.create`

### `terminal.output`

Streams terminal output from the active session.

Example:

```json
{ "type": "terminal.output", "stream": "stdout", "data": "..." }
```

Fields:

- `stream`: output stream identifier
- `data`: UTF-8 output chunk

V1 stream values:

- `stdout`
- `stderr`

Rules:

- emitted as output becomes available
- ordering should reflect observed read order from the bridge

### `terminal.exit`

Reports that the shell process exited.

Example:

```json
{ "type": "terminal.exit", "exitCode": 0 }
```

Fields:

- `exitCode`: integer exit code

Rules:

- sent when the shell process exits
- once this is emitted, the session is no longer active

### `error`

Reports a protocol or runtime error.

Example:

```json
{ "type": "error", "code": "SESSION_UNAVAILABLE", "message": "No active session exists" }
```

Fields:

- `code`: stable machine-readable error code
- `message`: human-readable explanation

## Required Message Ordering

The expected V1 flow is:

1. socket opens
2. client sends `hello`
3. server sends `hello.ack`
4. client sends `session.create`
5. server sends `session.ready`
6. client sends `session.attach`
7. client sends `terminal.input` and `terminal.resize` as needed
8. server sends `terminal.output` events as data arrives
9. server sends `terminal.exit` when the shell exits, or client sends `session.close`

The bridge should enforce this ordering rather than trying to recover silently.

## State Rules

The bridge should behave as if it has the following logical states:

- `HandshakePending`
- `Ready`
- `SessionCreated`
- `SessionAttached`

The exact Rust enum names do not matter. The behavior does.

Rules:

- before `hello`, only `hello` is allowed
- after `hello.ack` and before session creation, `session.create` is allowed
- `session.attach` requires an active created session
- `terminal.input` requires an attached session
- `terminal.resize` requires an attached session
- `session.close` removes the active session
- after shell exit, the session returns to the no-session state

## Error Codes

V1 should use a stable set of machine-readable error codes.

Required codes:

- `INVALID_MESSAGE`
  - malformed JSON or missing required fields
- `UNSUPPORTED_MESSAGE_TYPE`
  - unknown `type` value
- `HANDSHAKE_REQUIRED`
  - message sent before `hello`
- `INVALID_STATE`
  - message is well-formed but not valid in the current state
- `SESSION_EXISTS`
  - `session.create` requested while a session already exists
- `SESSION_UNAVAILABLE`
  - `session.attach` or session-dependent action requested without an active session
- `ATTACH_REQUIRED`
  - terminal action requested before attachment
- `INVALID_TERMINAL_SIZE`
  - resize values are invalid
- `SPAWN_FAILED`
  - shell process could not be created
- `INTERNAL_ERROR`
  - unexpected bridge-side failure

The exact human-readable `message` text may change. The `code` values should remain stable.

## Socket And Session Lifecycle

V1 behavior on connection loss should remain simple:

- the socket represents the active control connection
- reconnect does not resume a prior session in V1
- if the bridge decides to tear down the active shell on disconnect, that behavior should be documented and tested

Implementation note:

- V1 should prefer explicit cleanup on disconnect rather than trying to preserve hidden background state

## Validation Notes

The Rust implementation should have tests for:

- successful `hello` and `hello.ack`
- rejection of pre-handshake messages
- successful `session.create`
- rejection of duplicate `session.create`
- rejection of `session.attach` without a session
- rejection of `terminal.input` before attach
- rejection of invalid resize values
- transition to exited or closed state after shell termination

## Future Extensions

The following may be added after V1 without changing the basic contract shape:

- session IDs
- reconnect and resume
- binary output frames
- authentication
- multiple concurrent sessions

These are intentionally postponed until the single-session flow is stable.
