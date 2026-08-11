# Public Release Preparation Plan

## Metadata

- Status: accepted
- Owners: repository maintainers
- Last Updated: 2026-08-10
- Applies To: public release workflow
- Related Docs: `spec.md`, `tasks.md`

## Steps

1. Keep development, Android deployment, and small commits on `feat/terminal-vertical-slice`.
2. Complete terminal quality work: explicit Ctrl-D forwarding, PTY close behavior, tab exit lifecycle, profiles, ribbon entry point, and recovery status.
3. Add the authenticated handshake, token expiry policy, `~/.obsidianrc` startup behavior, native Termux installer, and npm CLI.
4. Rename public metadata to `Termux Terminal` and make `termux-terminal` the marketplace-safe plugin ID.
5. Establish `1.0.0` only after device acceptance, then validate all manifests and release assets together.
6. Squash the accepted feature range into the clean `main` worktree, create the release tag, and push.
7. Submit the public repository through the Obsidian Community directory manually after GitHub authentication is restored.
