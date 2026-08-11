# Changelog

All notable user-facing changes are recorded here.

## 1.0.3 - Unreleased

### Added

- Interactive xterm.js terminal tabs in Android Obsidian.
- Local Rust PTY bridge with isolated shell sessions per terminal tab.
- Loopback-only token authentication and a native Termux `runit` installer.
- Shell startup through `~/.obsidianrc`.

### Changed

- Terminal reconnect uses bounded backoff while preserving visible scrollback.
- GitHub Release builds install the Rust musl target before cross-compiling the
  bridge.

### Known Limitations

- Native `aarch64` Termux is the supported runtime.
- A hardware keyboard is required for the supported terminal workflow.
- The first public release is pending GitHub Release publication and Community
  directory review.

## 1.0.2 - Unreleased

- Release workflow request failed before publication because the GitHub runner
  did not install the Rust musl target. No GitHub Release assets were published.
