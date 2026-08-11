# Obsidian Terminal Workbench Decision Backlog

## Metadata

- Status: draft
- Owners: repository maintainers
- Last Updated: 2026-08-09
- Applies To: product scope, plugin UX, bridge protocol, Android runtime
- Related Docs: `spec.md`, `plan.md`, `../../foundation/architecture.md`, `../../foundation/protocol.md`

## Usage

One planning day contains 50 decision cards: 10 subjects, each with 5 agenda items. Select one option per card and mark the card complete. Record a reason only when it affects an architecture or protocol contract; that record must name the required foundation-doc update.

## Subject 1: Primary User Workflow

1. [x] First supported activity
   - A. Interactive shell and Codex only
   - B. Shell, Codex, and Git workflow
   - C. General shell workspace
   - Selected: C (2026-08-09)

2. [x] Primary workspace metaphor
   - A. One terminal pane
   - B. Terminal tabs
   - C. Split panes plus tabs
   - Selected: B (2026-08-09)

3. [x] Relationship to notes
   - A. Terminal is independent of the active note
   - B. Commands can receive the active note path
   - C. Note context is always injected into the shell
   - Selected: A (2026-08-09)

4. [x] Working-directory behavior
   - A. Bridge default directory only
   - B. Per-terminal selectable directory
   - C. Follow the active vault file
   - Selected: C (2026-08-09)

5. [x] First-release success metric
   - A. Complete a Codex task without Chrome
   - B. Complete shell/Git work without Chrome
   - C. Use it as the daily Android shell for one week
   - Selected: C (2026-08-09)

## Subject 2: Android Input And IME

6. [x] Terminal input implementation
   - A. Renderer-native textarea
   - B. Renderer-owned hidden textarea, with focus lifecycle managed by the plugin
   - C. Visible compose-safe input bar
   - Selected: B (2026-08-09)

7. [x] Full terminal implementation baseline
   - A. xterm.js core plus official Fit addon; use its native input, IME, Unicode, and ANSI paths
   - B. xterm.js core plus a narrowly scoped composition adapter only if the Android spike proves a defect
   - C. hterm/libapps terminal stack
   - Selected: A (2026-08-09)

8. [x] Hardware keyboard priority
   - A. Soft keyboard first
   - B. Equal support
   - C. Hardware keyboard first with soft-keyboard fallback
   - Selected: C (2026-08-09)

9. [x] Extra terminal keys
   - A. Minimal Ctrl/Esc/Tab row
   - B. Configurable shortcut row
   - C. No software-keyboard-specific key row in the first release
   - Selected: C (2026-08-09)

10. [x] Paste handling
   - A. Paste immediately
   - B. Confirm multi-line paste
   - C. Configurable threshold confirmation
   - Selected: B (2026-08-09)

## Subject 3: Terminal UI

11. [x] Rendering engine
   - A. xterm.js adapter
   - B. Always use xterm.js WebGL renderer
   - C. Prefer xterm.js WebGL, with automatic fallback to its default renderer
   - Selected: C (2026-08-09)

12. [x] View placement
   - A. Standard Obsidian leaf
   - B. Bottom-drawer terminal
   - C. Full-screen terminal mode
   - Selected: A (2026-08-09)

13. [x] Status presentation
   - A. Compact connection indicator
   - B. Persistent terminal status bar
   - C. Status only on error or reconnect
   - Selected: A (2026-08-09)

14. [x] Font strategy
   - A. Obsidian monospace default
   - B. Bundled configurable monospace font
   - C. Device-installed font selection
   - Selected: A (2026-08-09)

15. [x] Theme behavior
   - A. Follow Obsidian theme colors
   - B. Terminal-specific theme setting
   - C. Import standard terminal themes
   - Selected: A (2026-08-09)

## Subject 4: Session Model

16. [x] First-release session count
   - A. One bridge-owned session
   - B. One session per terminal view
   - C. Multiple concurrent sessions
   - Selected: C (2026-08-09)
   - Rationale: allow concurrent work through one ephemeral shell per terminal view, but delegate persistence to tmux/screen rather than retaining bridge sessions.

17. [x] Closing a view
   - A. Close the shell
   - B. Detach and retain shell
   - C. Ask every time
   - Selected: A (2026-08-09)

18. [x] App restart behavior
   - A. Start a new shell
   - B. Reattach when the bridge session exists
   - C. Restore named sessions
   - Selected: A (2026-08-09)

19. [x] Session naming
   - A. No persistent session names; display the terminal title or started command when available
   - B. Automatic directory-based names
   - C. User-named sessions
   - Selected: A (2026-08-09)

20. [x] Background process policy
   - A. Permit processes to continue after view close
   - B. Terminate on session close
   - C. Per-session policy
   - Selected: B (2026-08-09)

## Subject 5: Bridge Scope (Deferred)

Defer these implementation contracts until the terminal vertical slice exists and exposes a concrete limitation. Do not choose a wire format, session-ID scheme, reconnect model, or versioning strategy speculatively.

21. [~] Bridge change threshold: change the existing bridge only when the working terminal slice proves it necessary.
22. [~] Terminal stream encoding: retain the existing transport unless device measurement identifies a problem.
23. [~] Session identity: defer until a real multi-terminal UI requires it.
24. [~] Reconnect semantics: defer until foreground/background behavior has been measured on device.
25. [~] Compatibility strategy: defer until a bridge contract actually changes.

## Subject 6: Runtime And Service Ownership

26. [x] Bridge startup owner
   - A. User starts Termux service manually
   - B. Plugin opens a deep link/instructions only
   - C. Companion Android automation starts it
   - Selected: A (2026-08-09)

27. [x] Bridge process supervisor
   - A. Termux `runit` service
   - B. Termux:Boot plus shell script
   - C. Manual foreground process
   - Selected: A (2026-08-09)

28. [x] Background behavior
   - A. Best-effort process survival
   - B. Foreground service/notification where possible
   - C. Stop cleanly and reconnect on demand
   - Selected: A (2026-08-09)

29. [x] Health check
   - A. WebSocket connection attempt only
   - B. Dedicated bridge health endpoint/message
   - C. Android-visible service status file plus socket check
   - Selected: A (2026-08-09)

30. [x] Runtime selection
   - A. Native Termux bridge only
   - B. Desktop bridge only
   - C. Multiple Android runtime modes
   - Selected: A (2026-08-11)

## Subject 7: Terminal-Only Scope

31. [x] Codex entry point
   - A. Normal shell command only
   - B. Plugin command opens a Codex terminal
   - C. Dedicated Codex view mode
   - Selected: A (2026-08-09)

32. [x] Active-project handoff
   - A. No automatic directory change
   - B. Command opens terminal in active file parent
   - C. Command opens terminal in selected project root
   - Selected: A (2026-08-09)

33. [x] Prompt handling
   - A. Terminal text only
   - B. Paste selected note text into terminal
   - C. Structured note/context command generator
   - Selected: A (2026-08-09)

34. [x] Scrollback policy
   - A. xterm.js default scrollback limit
   - B. Fixed higher scrollback limit
   - C. User-configurable scrollback limit
   - Selected: A (2026-08-09)

35. [x] Selection and clipboard behavior
   - A. Native xterm.js selection and clipboard behavior only
   - B. Add explicit copy and paste buttons
   - C. Disable selection in the first release
   - Selected: A (2026-08-09)

## Subject 8: Security And Boundaries

36. [x] Bridge exposure
   - A. Loopback only, no exceptions
   - B. Loopback by default with explicit LAN mode
   - C. Unix socket where supported
   - Selected: A (2026-08-09)

37. [x] Plugin authorization
   - A. No secret under loopback-only model
   - B. Per-install shared token
   - C. User-confirmed pairing
   - Selected: A (2026-08-09)

38. [x] Shell command provenance
   - A. Terminal sends raw user input only
   - B. Plugin-generated commands visibly previewed
   - C. Command templates require confirmation
   - Selected: A (2026-08-09)
   - Deferred: plugin-generated commands and task templates are post-MVP work.

39. [~] Sensitive output policy: no decision needed while the plugin has no output persistence or generated-command feature.

40. [x] Crash diagnostics
   - A. In-plugin error only
   - B. Bridge log file plus in-plugin path
   - C. Structured diagnostic bundle export
   - Selected: A (2026-08-09)

## Subject 9: Configuration And Customization

41. [x] Plugin settings scope
   - A. Connection URL (host and port) and UI only
   - B. Connection URL, UI, and global named working-directory profiles
   - C. Include bridge launch settings
   - Selected: B (2026-08-09)

42. [x] Bridge configuration format
   - A. Command-line flags only
   - B. TOML/JSON config file
   - C. Environment variables plus flags
   - Selected: A (2026-08-09)

43. [x] Terminal profiles
   - A. One default shell
   - B. Shell plus working-directory profiles
   - C. Command templates including Codex profiles
   - Selected: B (2026-08-09)
   - Follow-up: resolve the plugin-settings scope required to store profiles.

44. [x] Settings portability
   - A. Per-vault settings only
   - B. Global Obsidian plugin settings only
   - C. Global defaults with per-vault overrides
   - Selected: A (2026-08-09)

45. [x] Reset and recovery
   - A. Clear UI settings only
   - B. Restart bridge from plugin instructions
   - C. Full diagnostic reset workflow
   - Selected: A (2026-08-09)

## Subject 10: Validation And Delivery

46. [x] Android test devices
   - A. One primary device first
   - B. Primary device plus tablet
   - C. Primary device plus different keyboard/IME setup
   - Selected: A (2026-08-09)

47. [x] Hardware-keyboard release gate
   - A. Basic typing, shell completion, Ctrl/C, and copy/paste
   - B. A plus function keys, cursor keys, and modifier combinations
   - C. B plus Korean soft-keyboard composition regression
   - Selected: B (2026-08-09)

48. [x] Performance target
   - A. Subjective daily-use acceptance
   - B. Input-to-PTY latency target
   - C. Frame-rate and memory budget targets
   - Selected: A (2026-08-09)

49. [x] Distribution model
   - A. Manual plugin install and bridge binary
   - B. Plugin release plus documented Termux setup
   - C. Plugin release plus companion setup script
   - Selected: B (2026-08-09)
   - Delivery: GitHub Releases provides plugin and bridge artifacts; Termux setup is documented.

50. [x] First milestone demo
   - A. Run shell commands in one terminal pane
   - B. Complete a Codex task in Obsidian
   - C. Reconnect a retained terminal after app backgrounding
   - Selected: A (2026-08-09)

## Ordered Implementation Tasks

1. [ ] Select cards 1, 6, 7, 11, 16, 26, and 47 before implementation.
2. [ ] Validate terminal input and keyboard shortcuts on the primary Android device with the hardware keyboard.
3. [ ] Implement the selected single-terminal vertical slice in the plugin.
4. [ ] Complete or revise bridge behavior only when the selected vertical slice proves it necessary.
5. [ ] Validate the slice with shell completion, keyboard shortcuts, resize, and app backgrounding.
6. [ ] Update foundation documents for every accepted contract change.
7. [ ] Create a follow-up spec for multi-session/splits only after the one-terminal slice is accepted.

## Documentation Updates

- [ ] Update `docs/foundation/architecture.md` after selecting the terminal view and runtime ownership model.
- [ ] Update `docs/foundation/protocol.md` before implementing any selected session, reconnect, or encoding change.
- [ ] Update `README.md` and `README.ko.md` when an Android installation or operating workflow is validated.
