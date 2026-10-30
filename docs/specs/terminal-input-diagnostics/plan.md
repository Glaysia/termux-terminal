# Terminal Input Diagnostics Plan

1. Add an opt-in plugin setting for session-local input diagnostics.
2. Intercept the affected control combinations at the terminal surface and send their
   terminal control bytes exactly once.
3. Render a bounded, copyable diagnostic log and validate the plugin typecheck/build.

## Follow-Up Regression Plan

1. Record a minimal reproduction matrix: Android keyboard, selected Korean IME,
   Obsidian terminal, and the matching code-server/VS Code terminal case.
2. Capture the browser and terminal event sequence for Ctrl-B, Ctrl-T, and a Korean
   composition sequence that demonstrates duplication or corruption.
3. Identify the first layer that diverges: host shortcut handling, composition/input
   events, xterm.js, plugin forwarding, bridge, or PTY.
4. Implement the narrowest fix at that layer and verify it does not regress Ctrl-C,
   Ctrl-D, Ctrl-S, completion, or normal Korean composition.
