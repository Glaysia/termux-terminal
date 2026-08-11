# Termux Terminal

Use a real native Termux shell inside Obsidian on Android. Termux Terminal
renders the terminal in an Obsidian tab while a local Rust bridge owns the PTY,
shell, and process lifecycle.

Korean: [README.ko.md](README.ko.md)

## Requirements

- Android Obsidian with Community plugins enabled
- native Termux on `aarch64`
- a hardware keyboard for the supported terminal workflow

The bridge is local to the device. It is not an SSH client, a remote shell
server, or a code-server replacement.

## Install

After the first GitHub Release is published, run this command in native Termux:

```sh
curl -fsSL https://raw.githubusercontent.com/Glaysia/termux-terminal/main/scripts/install-termux-bridge.sh | sh
```

The installer verifies the published checksum, installs the bridge as a
Termux `runit` service, and prints a connection token once.

In Obsidian:

1. Install **Termux Terminal** from Community plugins.
2. Open its settings and paste the printed bridge token.
3. Use the terminal ribbon icon or the `Open terminal` command.

## Shell Startup

Each terminal tab starts a fresh interactive Bash session. Bridge-owned Bash
sources `~/.obsidianrc` only. It does not automatically source `~/.bashrc`.

The installer creates a commented `source ~/.bashrc` line in `.obsidianrc`.
Uncomment it only when the ordinary Bash setup is appropriate for terminals
opened from Obsidian.

## Security

- The bridge listens on `127.0.0.1` only.
- Every connection requires the installation token stored in
  `~/.termux_terminal_token` with mode `0600`.
- Tokens expire after six months; the shell warns during the final seven days.
- Terminal data, tokens, and shell output are not recorded by default.
- Any network forwarding is configured and secured by the user. The plugin
  never changes the loopback-only bridge binding.

## Operation

Check the native service from Termux:

```sh
SVDIR="$PREFIX/var/service" sv status termux-terminal-bridge
```

Restart it after updating the bridge:

```sh
SVDIR="$PREFIX/var/service" sv restart termux-terminal-bridge
```

## Development

Read [CONTRIBUTING.md](CONTRIBUTING.md) for branch and validation rules.
Release notes are in [CHANGELOG.md](CHANGELOG.md). Security reports are handled
under [SECURITY.md](SECURITY.md).

## License

AGPL-3.0-or-later. See [LICENSE](LICENSE).
