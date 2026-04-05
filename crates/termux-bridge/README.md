# termux-bridge

Small Rust bridge process intended to run inside `Termux`.

Primary build target:

- `aarch64-unknown-linux-musl`
- intended to be the same binary used in native Termux and Debian `proot`

Validated runtime matrix:

- build in Debian `proot`: yes
- run in Debian `proot`: yes
- run in native Termux: yes

Planned responsibilities:

- expose a local endpoint for the Obsidian plugin
- manage shell/session lifecycle
- keep runtime overhead small on Android

Current status:

- crate skeleton only
- no transport or PTY implementation yet

Verified build notes:

```bash
proot-distro login debian --user harry --termux-home -- bash -lc '
cd /data/data/com.termux/files/home/Projects/obsidian-termux
rustup toolchain install stable --profile minimal
rustup default stable
rustup target add aarch64-unknown-linux-musl
cargo build -p termux-bridge --target aarch64-unknown-linux-musl --release
'
```

Built artifact:

```bash
/data/data/com.termux/files/home/Projects/obsidian-termux/target/aarch64-unknown-linux-musl/release/termux-bridge
```

Verification commands:

```bash
file /data/data/com.termux/files/home/Projects/obsidian-termux/target/aarch64-unknown-linux-musl/release/termux-bridge
ldd /data/data/com.termux/files/home/Projects/obsidian-termux/target/aarch64-unknown-linux-musl/release/termux-bridge
```

Expected output shape:

- `file`: `statically linked`
- `ldd`: `not a dynamic executable`

Run in Debian `proot`:

```bash
proot-distro login debian --user harry --termux-home -- bash -lc '
/data/data/com.termux/files/home/Projects/obsidian-termux/target/aarch64-unknown-linux-musl/release/termux-bridge
'
```

Run in native Termux:

```bash
/data/data/com.termux/files/home/Projects/obsidian-termux/target/aarch64-unknown-linux-musl/release/termux-bridge
```
