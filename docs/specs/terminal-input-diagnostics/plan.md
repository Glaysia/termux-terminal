# Terminal Input Diagnostics Plan

1. Add an opt-in plugin setting for session-local input diagnostics.
2. Intercept the affected control combinations at the terminal surface and send their
   terminal control bytes exactly once.
3. Render a bounded, copyable diagnostic log and validate the plugin typecheck/build.
