# Obsidian Terminal Workbench Spec

## Metadata

- Status: draft
- Owners: repository maintainers
- Last Updated: 2026-08-09
- Applies To: `packages/obsidian-plugin`, `crates/termux-bridge`, Android Obsidian and Termux runtime
- Related Docs: `../../foundation/architecture.md`, `../../foundation/protocol.md`, `../termux-bridge-v1/spec.md`, `plan.md`, `tasks.md`

## Summary

Turn Android Obsidian into the primary interactive shell workspace for this project. The plugin should present a terminal experience that is good enough for shell work, Codex sessions, and project navigation without depending on Chrome's code-server text-input path.

The product remains split deliberately: Obsidian provides the Android-native workspace and terminal UI; the Rust bridge in Termux owns PTYs, processes, and transport.

## Problem

Termux supplies a capable Android shell but does not provide a satisfying multi-pane application shell. Browser-hosted code-server supplies a rich workspace but Korean IME handling in the web input path is unreliable and disruptive. Android Obsidian already provides the desired note/workspace context, native app lifecycle, and plugin surface, but does not itself provide PTY access.

## Goals

- make a real Termux shell usable from an Obsidian pane on Android
- support a general terminal-first workflow; Codex and ordinary CLI tools run as normal shell commands
- allow an operator to work with multiple terminal contexts without falling back to a browser
- preserve the existing localhost-only, Rust-bridge architecture
- keep the first usable release narrow enough to validate on a real Android device

## Non-Goals

- replace the full VS Code editor or implement the VS Code extension API
- expose a Termux shell to the network
- build a general remote-desktop product
- require a native Termux Rust toolchain
- commit a multi-session or persistence protocol before the product decision is made

## Current State

- the Obsidian plugin is only a minimal skeleton
- `termux-bridge` has a V1 localhost-only WebSocket contract with one ephemeral shell per connection
- the V1 contract intentionally excludes reconnect, session persistence, cross-connection session attachment, and binary frames
- the validated bridge artifact remains `aarch64-unknown-linux-musl`, built inside Debian proot and runnable in both supported runtimes

## Proposed Behavior

The initial workbench release should offer a dedicated Obsidian view that connects to the local bridge, renders a full terminal, reports connection and session state, and preserves terminal-native input behavior. It uses xterm.js rather than a custom renderer or input implementation; the plugin manages lifecycle and focus while xterm.js handles terminal input, IME, Unicode, ANSI, and rendering. Shell completion remains provided by the configured Termux shell. The remaining interaction, session, and runtime decisions are tracked in `tasks.md`.

The first release requires a hardware keyboard. Software-keyboard-specific shortcut rows, gesture modifiers, and separate composition input surfaces are out of scope.

The plugin may open multiple terminal views concurrently. Each view owns a dedicated bridge connection and ephemeral shell; tabs use the default shell name, currently `bash`, rather than persistent user-defined session names. Closing a terminal view terminates only its shell, and restarting Obsidian starts fresh shells. Durable sessions and background process retention remain the responsibility of tmux or screen, not the bridge.

Releases publish the plugin and bridge binary through GitHub Releases, with documented Termux setup. The project is licensed under AGPL-3.0-or-later.

The release sequence is:

1. prove one interactive terminal with reliable composition input
2. prove the Android layout and lifecycle under ordinary shell and Codex use
3. add only the workspace/session capabilities justified by that validation

## Interfaces And Contracts

The canonical protocol and architecture documents define the accepted localhost-only connection and ephemeral-session contract. The following contracts remain deferred until corresponding decisions are accepted:

- WebSocket message schema and protocol versioning
- session identity, attach, detach, and persistence semantics
- terminal binary/text data encoding
- plugin settings schema
- bridge launch, discovery, and health-check behavior

When any of these decisions is accepted, update `docs/foundation/protocol.md` and/or `docs/foundation/architecture.md` in the same implementation change.

## Acceptance Criteria

- a decision record exists for the 50 initial product and implementation questions in `tasks.md`
- one selected vertical slice is small enough to implement without redesigning the bridge twice
- the first vertical slice has Android-device acceptance tests for hardware-keyboard input, shell completion, resize, and backgrounding
- all accepted runtime or protocol decisions are reflected in foundation docs before implementation relying on them lands

## Risks And Open Questions

- Android Obsidian's webview may constrain external-keyboard handling differently from Chrome
- session IDs or reattachment require a bridge protocol revision; concurrent independent views do not
- keeping an Android process alive across backgrounding may require a user-visible Termux service policy
- a terminal-first workbench can succeed without embedding the full code-server product; that boundary must remain explicit
