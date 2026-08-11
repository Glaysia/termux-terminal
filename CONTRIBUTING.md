# Contributing

## Branches

- `main` contains approved release history and is changed through pull requests
  only.
- `dev` is the integration branch.
- Keep at most two active `feat/*` branches. Branch each one from `dev`.

Use a pull request from `feat/*` to `dev`. Preserve meaningful feature commits
with a merge commit. Open a separate `dev` to `main` pull request only after
Android validation; squash that PR so `main` keeps a short release history.

## Development Rules

- Use the existing TypeScript/xterm.js and Rust/PTy boundaries. Do not replace
  terminal behavior with a custom input or terminal parser.
- Add or update `docs/specs/<slug>/spec.md`, `plan.md`, and `tasks.md` for
  non-trivial changes.
- Update `docs/foundation/` whenever a shared protocol, runtime, or architecture
  contract changes.
- Never commit a token, terminal output containing private data, or a test-vault
  `data.json` file.

## Validation

Run before opening a pull request:

```sh
pnpm install --frozen-lockfile
pnpm run typecheck:plugin
pnpm run build:plugin
pnpm run check:release
cargo test -p termux-bridge --test websocket_server -- --test-threads=1
```

For a plugin deployment, use a new version and copy matching `main.js`,
`manifest.json`, and `styles.css` together. Validate the Android interaction
that the change affects with a physical keyboard where applicable.
