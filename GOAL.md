# Termux Terminal Goal

## Product Goal

Deliver `Termux Terminal`: a real, interactive native Termux terminal inside
Android Obsidian. It must be practical for ordinary shell work, Git, and Codex
with a hardware keyboard, without relying on Chrome or code-server's web input
path.

The product is a terminal, not a code-server replacement, command runner, SSH
client, or remote-desktop product. It uses xterm.js for terminal behavior and
a small Rust bridge for PTY ownership. Do not reimplement terminal parsing,
completion, IME handling, or shell behavior that mature components already own.

## Runtime Boundary

- Public runtime: native Termux on Android `aarch64` only.
- Public bridge target: `aarch64-linux-android`, linked against Android Bionic.
- Debian `proot` is neither a public runtime nor a documented supported path.
  It may be used privately as a build tool only when it remains useful.
- The bridge binds only to `127.0.0.1`; it never offers a LAN binding.
- Future Windows and Linux ports use the same native bridge model. Users who
  need remote access configure their own local port forwarding; the plugin does
  not implement SSH.

## Terminal Experience

- Render an xterm.js terminal in a normal Obsidian leaf. The ribbon has a
  terminal icon and there is an `Open terminal` command.
- A terminal leaf owns one ephemeral shell-backed bridge session. Multiple
  leaves may run independently. Durable jobs belong in `tmux` or `screen`, not
  in bridge session persistence.
- Tabs display the running shell or process name when available, beginning with
  `bash`; plugin versions never appear in terminal tab titles.
- Hardware keyboard is the first-class input method. Korean composition, ANSI,
  completion, resize, paste, Ctrl-C, Ctrl-D, and terminal closing are release
  gates.
- While a terminal has focus, Obsidian and webview `Ctrl` shortcuts must not
  consume terminal control keys. Standard Ctrl combinations are forwarded as
  their corresponding raw control bytes to the PTY.
- Ctrl-D and Ctrl-S specifically forward `0x04` and `0x13`. Their shell/TTY
  semantics remain the shell's responsibility.
- On shell exit, print the exit code and close the terminal tab after three
  seconds.
- Reconnect preserves visible scrollback and uses bounded backoff of 1, 2, 5,
  and 10 seconds. Authentication failures and normal terminal exits never show
  a false retry state.
- An opt-in input diagnostics mode records terminal-surface key events and
  exact plugin-to-bridge input data for the current tab. It is local,
  session-only, bounded, copyable, clearable, never persisted, and never
  uploaded. It exists to prove whether a webview or Obsidian shortcut consumed
  an input.

## Shell Startup

- Bridge-owned Bash starts with `~/.termux-terminal.bashrc` and sources
  `~/.obsidianrc` when it exists.
- Ordinary `~/.bashrc` is not sourced automatically. The generated
  `~/.obsidianrc` template includes a commented `source ~/.bashrc` line for
  users who explicitly choose it.
- `.obsidianrc` is only for commands run from an Obsidian-owned terminal.

## Security And Service

- The Termux bridge runs under a native Termux `runit` service.
- Every production WebSocket connection sends the installation token in
  `hello.token`.
- The token is stored in `~/.termux_terminal_token`, with first-line token,
  second-line Unix issue time, and mode `0600` enforced.
- A token lasts six months. The shell warns during its final seven days; the
  bridge rejects it after the grace period.
- Invalid authentication returns a clear error, closes the socket, and does not
  retry.
- Default logs never retain tokens, terminal input, or terminal output. Input
  diagnostics are the explicit, visible, session-local exception above.
- Users own any port forwarding they configure. The project does not make LAN
  exposure safe by default and does not accept responsibility for a user's
  exposed tunnel.

## Installation And Distribution

- The primary install path is a copy-paste Termux `curl | sh` installer from
  GitHub Releases.
- The installer downloads the native Bionic bridge binary, verifies
  `SHA256SUMS`, installs the service, creates or preserves the token, creates
  the startup files, and starts the service.
- Add a complete npm installer later as `@willbecat27/termux-terminal`; npm is
  for convenient installation and updates, not a prerequisite for the first
  public release.
- Publish GitHub release assets, checksums, an Obsidian-compatible plugin
  package, and marketplace metadata. The project license is
  `AGPL-3.0-or-later`.

## Development And Release Discipline

- Develop only in `feat/terminal-vertical-slice` worktree:
  `/home/harry/Projects/obsidian-termux-terminal-vertical-slice`.
- Keep `main` clean in `/home/harry/Projects/termux-terminal`. Squash only an
  Android-accepted release candidate into `main`; tag and publish from `main`,
  never from the feature branch.
- Make small, descriptive commits frequently on the feature branch.
- Every deployable Android plugin build gets a new monotonically increasing
  semantic version. Deploy `main.js`, `styles.css`, and matching `manifest.json`
  together so Obsidian never executes mixed versions.
- Keep root package, plugin package, manifests, Rust bridge, installer asset
  name, release workflow, and documentation version/target metadata aligned.
- Before a public release, update the relevant `docs/specs/<slug>/` files and
  `docs/foundation/` whenever a shared architecture, protocol, or runtime
  contract changes.

## Release Gate

- Native Bionic bridge builds reproducibly and starts through `runit` on an
  Android Termux device.
- Android Obsidian opens an interactive terminal from both the ribbon and the
  command palette.
- Korean input, completion, resize, paste, Ctrl-C, Ctrl-D, Ctrl-S, terminal
  focus shortcut isolation, tab close, and ordinary shell workflow succeed on
  a physical keyboard.
- Input diagnostics prove the relevant key event and control byte when an input
  issue is investigated.
- `~/.obsidianrc` behavior, authentication, installation, token rotation,
  update, reconnect, and invalid-token behavior are tested on device.
- GitHub release assets and checksums are reproducible and the installer can
  consume the published asset.
- The Android test vault uses exactly one matching plugin build version.

## Current Checkpoint

- The feature branch has a working Rust PTY bridge, localhost WebSocket
  transport, token authentication, runit service installer, xterm.js leaf,
  resize, Korean-input validation, and basic reconnect handling.
- The current active work adds terminal-focused Ctrl-shortcut isolation and
  session-local input diagnostics. It still requires Android device validation
  before deployment acceptance.
- Bionic release targeting and the removal of stale `proot`/musl documentation
  are selected work, not yet complete.
