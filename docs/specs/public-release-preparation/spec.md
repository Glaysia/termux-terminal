# Public Release Preparation Spec

## Metadata

- Status: accepted
- Owners: repository maintainers
- Last Updated: 2026-08-10
- Applies To: repository metadata, GitHub releases, native Termux installation
- Related Docs: `../../foundation/architecture.md`, `../../../README.md`, `plan.md`, `tasks.md`

## Goal

Prepare the accepted terminal vertical slice for a first public release and Obsidian Community plugin submission.

## Scope

- publish under the public product name `Termux Terminal`
- use `termux-terminal` as the Obsidian plugin ID and GitHub repository name
- release version `1.0.0` with matching root manifest, plugin manifest, package metadata, and Rust package version
- support native Termux as the public runtime
- release a static `aarch64-unknown-linux-musl` bridge binary and the three required Obsidian plugin assets
- provide a native-Termux installer that installs and starts the local bridge service
- automate tagged GitHub releases
- require a per-installation bridge token and preserve the localhost-only binding
- source `~/.obsidianrc` for bridge-owned shells without automatically sourcing `~/.bashrc`
- provide terminal profiles, connection recovery, explicit shell exit behavior, and an Obsidian ribbon entry point

## Non-Goals

- support running the released bridge inside Debian `proot`
- automate submission or review in the Obsidian Community directory
- expose the bridge on LAN or the Internet
- implement SSH, port forwarding, Windows, or Linux support in this release

## Acceptance Criteria

- the root repository contains a valid Community-plugin manifest whose ID does not contain `obsidian`
- the generated release contains `main.js`, `manifest.json`, `styles.css`, the native Termux bridge binary, and checksums
- the documented installer registers a localhost-only bridge service in native Termux
- release metadata and documentation accurately describe the supported runtime and security boundary
- an installation token is stored with `0600` permissions, expires after six months, and is required during the WebSocket handshake
- `~/.obsidianrc` runs for bridge-owned shells only; a user can opt into `~/.bashrc` from that file
- terminal exits and bridge restarts have visible, tested UI behavior

## Accepted Decisions

### Development And Release

- Work and Android deployments continue on `feat/terminal-vertical-slice`; `main` stays clean until a tested release candidate is squash-merged.
- Every Android deployment increments the plugin version. `1.0.0` is the first public release, not the next development deployment.
- GitHub releases publish `main.js`, `manifest.json`, `styles.css`, `termux-bridge-aarch64-unknown-linux-musl`, and `SHA256SUMS`.
- Public runtime support is native Termux on `aarch64`. Debian `proot` remains an optional build environment only.

### Authentication And Installation

- The installer and npm CLI create `~/.termux_terminal_token` with the token on line one and the issued Unix time on line two. The file mode is `0600`.
- The plugin stores the token in its settings data and sends it as `hello.token`.
- The bridge rejects an invalid token after one error response and closes the socket.
- Tokens are valid for six months. The generated `~/.obsidianrc` warns during a seven-day grace period; the bridge rejects expired tokens after that period.
- The curl installer is the primary documented route. `@willbecat27/termux-terminal` is an equivalent npm installer and updater. Both download release binaries and verify `SHA256SUMS`.

### Shell And Terminal Behavior

- Bridge-owned shells source only `~/.obsidianrc`. The generated file contains a commented `source ~/.bashrc` line for opt-in compatibility.
- `.obsidianrc` follows ordinary Bash source behavior: it can export variables, define functions, and continue after a failing command unless the user changes shell options.
- Profiles contain name, bridge URL, token, and optional shell path. `Termux local` is created by default and cannot be deleted.
- Shell exit prints the exit code and closes the terminal tab after three seconds. A user-closing tab terminates its shell.
- Broken bridge connections retry with a capped exponential backoff, preserve scrollback, and mark a newly created replacement shell after a bridge restart.

### Future Remote Runtime

- Future Windows/Linux ports keep the same native bridge protocol and loopback-only binding.
- Users provide any port forwarding themselves. The plugin does not implement SSH or bind a bridge to a network interface.
