# termux-bridge

`termux-bridge` is the native Termux process behind the Termux Terminal
Obsidian plugin. It binds a loopback WebSocket endpoint, authenticates the
plugin, and owns one interactive PTY-backed shell for each terminal tab.

## Runtime Contract

- supported runtime: native `aarch64` Termux
- listener: `127.0.0.1:11557`
- supervisor: Termux `runit`
- authentication: token from `~/.termux_terminal_token`
- shell startup: `~/.termux-terminal.bashrc`, which sources `~/.obsidianrc`

The public installer downloads the release binary and configures the service.
It is the supported installation path; this crate README is not a user build
guide.

## Operation

```sh
SVDIR="$PREFIX/var/service" sv status termux-terminal-bridge
SVDIR="$PREFIX/var/service" sv restart termux-terminal-bridge
```

The bridge deliberately accepts local connections only. Do not alter its
binding to expose it on a network.

## Protocol

The protocol is documented in
[../../docs/foundation/protocol.md](../../docs/foundation/protocol.md). The
bridge performs `hello`, `session.create`, and `session.attach` before it
forwards terminal input, output, resize, or close events.
