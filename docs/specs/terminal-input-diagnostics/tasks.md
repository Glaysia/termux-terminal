# Terminal Input Diagnostics Tasks

- [x] Define the diagnostics privacy and retention boundary.
- [x] Validate Ctrl-D and Ctrl-S with a physical keyboard in Android Obsidian.
- [x] Confirm the diagnostic log captures the received key event and transmitted byte.

## Follow-Up Regressions

- [ ] Reproduce Ctrl-B and Ctrl-T failures with the primary Android keyboard and record
  whether the plugin receives each key event.
- [ ] Reproduce the Korean composition duplication/corruption issue in both the Obsidian
  terminal and code-server/VS Code path using the same keyboard and IME.
- [ ] Extend diagnostics only as needed to correlate composition, input, and xterm data
  without persisting terminal content.
- [ ] Fix the confirmed divergent input layer and run the Android hardware-keyboard
  regression set.
