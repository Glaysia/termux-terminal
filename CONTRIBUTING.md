# Contributing

## Branches

- `main` contains reviewed, release-ready commits only.
- `dev` is the integration branch and may receive direct maintainer pushes.
- Keep at most two active `feat/*` branches. Merge feature work into `dev` with
  small descriptive commits.
- Merge `dev` into `main` through a pull request using squash merge, then merge
  the resulting `main` commit back into `dev`.

## Development Rules

- Use the existing TypeScript/xterm.js and Rust/PTy boundaries. Do not replace
  terminal behavior with a custom input or terminal parser.
- Add or update `docs/specs/<slug>/spec.md`, `plan.md`, and `tasks.md` for
  non-trivial changes.
- Update `docs/foundation/` whenever a shared protocol, runtime, or architecture
  contract changes.
- Never commit a token, terminal output containing private data, or a test-vault
  `data.json` file.

## Versioning

Every changed deployable plugin build receives a new semantic version. Deploy
matching `main.js`, `manifest.json`, and `styles.css` together. The manifest
and GitHub release tag use `x.y.z` format without a `v` prefix.

## Validation

Before a release, run:

```bash
pnpm run typecheck:plugin
pnpm run build
pnpm run check:release
cargo test -p termux-bridge --test websocket_server -- --test-threads=1
cargo build -p termux-bridge --target aarch64-unknown-linux-musl --release
```

Also validate the Android Obsidian terminal with a hardware keyboard.

## Release

1. Merge the release candidate from `dev` into `main`.
2. Push the matching bare semantic tag, for example `1.0.4`.
3. Confirm GitHub Actions publishes the plugin release, separate bridge release,
   checksums, and provenance attestations.
4. Check the Obsidian Community review result before public promotion.
