# Terminal Input Diagnostics Spec

## Metadata

- Status: accepted
- Last Updated: 2026-08-11
- Applies To: `packages/obsidian-plugin`

## Problem

Android Obsidian and its embedded webview can consume hardware-keyboard shortcuts before
xterm.js receives them. Failures such as Ctrl-D and Ctrl-S need device evidence that
distinguishes a browser-level shortcut conflict from a bridge or PTY failure.

## Behavior

- When the terminal has focus, the plugin intercepts Ctrl-D and Ctrl-S at the terminal
  surface, prevents host shortcut handling, and sends their standard terminal control
  bytes: `0x04` and `0x13`.
- An opt-in `Input debug log` setting exposes a session-local diagnostic panel below each
  terminal. It records keyboard events seen by the terminal surface and the exact input
  data the plugin attempts to send to the bridge.
- The panel supports copying and clearing its local log. It is not persisted, uploaded,
  or included in bridge messages.
- The log is bounded to the most recent 2,000 entries to avoid unbounded memory use.

## Acceptance Criteria

- Ctrl-D and Ctrl-S are forwarded once while a connected terminal is focused.
- Enabling diagnostics makes key events and sent control bytes observable on-device.
- Diagnostics do not alter input behavior and disappear when the terminal leaf closes.
