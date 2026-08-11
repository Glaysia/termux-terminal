# Obsidian Terminal Workbench Plan

## Metadata

- Status: draft
- Owners: repository maintainers
- Last Updated: 2026-08-09
- Applies To: `packages/obsidian-plugin`, `crates/termux-bridge`, Android validation workflow
- Related Docs: `spec.md`, `tasks.md`, `../../foundation/architecture.md`, `../../foundation/protocol.md`

## Summary

Build the product from a single reliable interactive terminal outward. The plan rejects a premature code-server clone: the first value is a stable Android terminal for actual shell and Codex work inside Obsidian.

## Decision Workflow

Each planning session processes 50 decision cards: 10 subjects with 5 agenda items each. Every agenda item has two to five viable options. A card is marked selected with its option and decision date. Add a reason and affected foundation document when a decision changes a shared contract.

Decision order matters:

1. user workflow and terminal input
2. plugin view and renderer
3. bridge session and lifecycle contracts, after device evidence requires them
4. Android service and recovery behavior
5. packaging, security, and validation

Do not implement a card that depends on an unresolved earlier card unless the work is an isolated spike.

## Delivery Phases

## Current Work: Terminal Vertical Slice

Work on `feat/terminal-vertical-slice`. Keep small, reviewable commits on this branch; squash the accepted vertical slice into `main` only after device validation.

### Today's Scope: Part 1 of 3

Implement the plugin-local terminal surface only:

- add xterm.js and the official Fit addon to the Obsidian plugin
- register a dedicated Obsidian terminal view and an open-terminal command
- create and dispose the xterm.js instance with the view lifecycle
- fit the terminal when the view is resized and render local placeholder output

Out of scope for Part 1: WebSocket connection, bridge messages, terminal input forwarding, multi-terminal UI, profiles, and release packaging.

Exit condition: opening the command in Obsidian displays a correctly sized xterm.js terminal and closing the view disposes it without errors.

### Today's Scope: Part 2 of 3

Implement the read-only bridge connection:

- connect to the existing local bridge at `ws://127.0.0.1:11557`
- follow its existing handshake, session creation, and attachment sequence
- render `terminal.output` events in xterm.js
- show connection state with a compact indicator and write bridge errors into the terminal

Out of scope for Part 2: terminal input forwarding, PTY resize messages, reconnect retention, multi-terminal UI, profiles, and release packaging.

Exit condition: a bridge-owned shell prompt and output render in the Obsidian terminal view; closing the view closes its socket.

### Today's Scope: Part 3 of 3

Complete the interactive terminal path:

- forward xterm.js `onData` events as `terminal.input`
- disable local input until bridge attachment succeeds and after disconnects or errors
- forward fitted xterm.js columns and rows as `terminal.resize`
- deploy the built plugin to the Android test vault and run the bridge under Termux `runit`
- verify the native bridge handshake, input, resize, and output flow end to end

Exit condition: the Android test vault has the built plugin installed, Termux supervises the bridge, and the bridge accepts interactive terminal data at its localhost endpoint.

### Follow-up: Concurrent Terminal Views

After the first Android device slice is working:

- label each terminal tab with its default shell name (`bash`) rather than the plugin release version
- display connection and reconnect progress in the terminal while input is unavailable
- allow the bridge to serve concurrent WebSocket connections, with one isolated ephemeral PTY per connection
- retain the existing close behavior: closing a view closes only that view's shell

Out of scope: persistent session names, session IDs, cross-connection attachment, and durable background jobs.

Exit condition: two terminal views can connect and run commands concurrently, and a new or reconnecting view visibly reports that it is waiting for the bridge.

### Phase 0: Product Decisions And Device Spikes

- select the terminal surface, input strategy, and first-session behavior
- validate external-keyboard input in the actual Android Obsidian webview
- validate focus retention and resize behavior
- record selected choices in `tasks.md`

Exit condition: one end-to-end vertical slice is selected and its constraints are known.

### Phase 1: Plugin Vertical Slice

- add a dedicated Obsidian view and command to open it
- add a terminal renderer behind a thin adapter boundary
- manage focus, resize, connection state, and visible errors
- implement the selected Android input path

Exit condition: the plugin can display a connected terminal without pretending to own a PTY.

### Phase 2: Bridge Alignment

- use the existing bridge unchanged for the first terminal slice where it is sufficient
- change transport or session contracts only after a working device slice exposes a concrete limitation
- keep PTY/session lifecycle entirely in Rust
- add bridge health and diagnostic surfaces selected in the decision log

Exit condition: the bridge has explicit, tested behavior for all plugin interactions in scope.

### Phase 3: Android Operations

- define bridge start/stop ownership and Termux service integration
- validate Obsidian foreground/background and reconnect behavior
- define notification, battery, and recovery expectations
- make failure modes visible and actionable in the plugin

Exit condition: ordinary use does not require manually repairing a stale process or socket.

### Phase 4: Workbench Expansion

- add selected multi-session, split, tab, workspace, and Codex affordances; keep bridge sessions ephemeral and leave durable process management to tmux or screen
- add only the note-to-shell interactions that survive real workflow use
- defer editor parity features to separately scoped specs

Exit condition: the workbench improves day-to-day Android shell work rather than duplicating code-server superficially.

## Key Changes

- Obsidian plugin: custom terminal workbench view, renderer adapter, input adapter, settings, connection lifecycle
- Rust bridge: PTY/session lifecycle, protocol versioning where selected, diagnostics, runtime behavior
- Android deployment: explicit Termux service and recovery documentation
- foundation docs: updated only when selected contracts become project-wide truths

## Candidate Technical Shape

- TypeScript plugin based on xterm.js and its official Fit addon; prefer the official WebGL renderer with fallback to xterm.js's default renderer, while the plugin manages lifecycle and xterm.js retains ownership of terminal input handling
- shell completion remains a Termux shell concern (zsh/fish configuration), not a plugin feature
- a hardware keyboard is a first-release prerequisite; software-keyboard-specific shortcut rows and input work are out of scope
- Rust bridge remains localhost-only and owns all PTY interaction
- JSON remains suitable for control messages; terminal stream encoding is a decision card before changing V1
- settings live in Obsidian for UI behavior and in Termux for bridge/runtime behavior

## Plugin Versioning And Deployment

- Every deployable plugin change increments `packages/obsidian-plugin` with `pnpm --filter @obsidian-termux/obsidian-plugin version:patch` before the build.
- The version script updates `package.json` and `manifest.json` together; the build rejects a mismatch.
- The built bundle embeds the same version for bridge compatibility, but terminal tabs use their shell title rather than displaying the release version.
- Deploy only the generated `main.js`, `styles.css`, and matching `manifest.json` as one versioned set.

## Testing And Validation

- Android-device keyboard tests: completion, delete, cursor movement, paste, Ctrl combinations, focus changes, and rapid command entry
- terminal tests: ANSI rendering, resize, long output, interactive prompts, Ctrl/Cmd-equivalent input, and shell exit
- bridge tests: protocol ordering, session lifecycle, disconnection, restart, and invalid messages
- workflow tests: Codex command sessions, Git status/diff, project navigation, and note-to-terminal handoff
- runtime tests: native Termux execution and Android Obsidian connection

## Assumptions

- the first release focuses on Android Obsidian, not desktop parity
- one good terminal is more valuable than early split panes
- reliable external-keyboard interaction is a release gate, not polish
- code-server remains a comparison point and optional companion, not an implementation target
