# Termux Terminal

An interactive Termux shell inside Obsidian on Android. The Obsidian plugin
renders xterm.js; a small Rust bridge owns the local PTY and shell.

Korean: [README.ko.md](README.ko.md)

## Status

`1.0.0` is under Android release-candidate testing. The public runtime target
is native aarch64 Termux. The bridge listens only on `127.0.0.1:11557`.

## Install

After the first GitHub release, run this in native Termux:

```sh
curl -fsSL https://raw.githubusercontent.com/Glaysia/termux-terminal/main/scripts/install-termux-bridge.sh | sh
```

The installer downloads the release binary, verifies `SHA256SUMS`, creates a
`runit` service, creates `~/.termux_terminal_token` with mode `0600`, and
prints the token once. Paste that token into Obsidian Settings > Termux
Terminal. Then use the ribbon terminal icon or the `Open terminal` command.

## Shell Startup

Bridge-owned Bash sessions source `~/.obsidianrc`. They do not automatically
source `~/.bashrc`. The generated template includes a commented
`source ~/.bashrc` line for users who want their ordinary interactive setup.

## Security

- The bridge binds to loopback only.
- Every production connection must provide the installation token in its first
  WebSocket message.
- Tokens are valid for six months, with a seven-day shell warning period.
- Default logs never record tokens, terminal input, or terminal output.

## Development

Feature development happens on `feat/terminal-vertical-slice`; accepted release
candidates are squash-merged into `main`. See [GOAL.md](GOAL.md) and
`docs/specs/public-release-preparation/`.

```sh
pnpm install --frozen-lockfile
pnpm run check:release
pnpm run typecheck:plugin
pnpm run build:plugin
cargo test -p termux-bridge
```

Build the release bridge inside Debian `proot`:

```sh
cargo build -p termux-bridge --target aarch64-unknown-linux-musl --release
```

The resulting native Termux artifact is
`target/aarch64-unknown-linux-musl/release/termux-bridge`.

## License

AGPL-3.0-or-later. See [LICENSE](LICENSE).
