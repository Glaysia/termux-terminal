#!/data/data/com.termux/files/usr/bin/sh

set -eu

repository="${TERMUX_TERMINAL_REPOSITORY:-Glaysia/termux-terminal}"
asset="termux-bridge-aarch64-unknown-linux-musl"
prefix="${PREFIX:-/data/data/com.termux/files/usr}"
service_name="termux-terminal-bridge"
service_dir="$prefix/var/service/$service_name"
binary="$prefix/bin/$service_name"
temp_dir="$(mktemp -d)"

cleanup() {
  rm -rf "$temp_dir"
}
trap cleanup EXIT INT TERM

if [ "$(uname -m)" != "aarch64" ]; then
  echo "This release supports native aarch64 Termux only." >&2
  exit 1
fi

pkg install -y curl openssl-tool runit
bridge_tag="${TERMUX_TERMINAL_BRIDGE_TAG:-}"
if [ -z "$bridge_tag" ]; then
  bridge_tag="$(curl -fsSL "https://api.github.com/repos/$repository/releases?per_page=100" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\(bridge-[^"]*\)".*/\1/p' | head -n 1)"
fi
if [ -z "$bridge_tag" ]; then
  echo "No bridge release was found for $repository." >&2
  exit 1
fi
release_base="https://github.com/$repository/releases/download/$bridge_tag"
curl -fsSL "$release_base/$asset" -o "$temp_dir/$asset"
curl -fsSL "$release_base/SHA256SUMS" -o "$temp_dir/SHA256SUMS"

(cd "$temp_dir" && grep "  $asset$" SHA256SUMS | sha256sum -c -)
install -Dm700 "$temp_dir/$asset" "$binary"

token_file="$HOME/.termux_terminal_token"
if [ ! -s "$token_file" ]; then
  token="$(openssl rand -hex 32)"
  printf '%s\n%s\n' "$token" "$(date +%s)" > "$token_file"
  chmod 600 "$token_file"
else
  token="$(sed -n '1p' "$token_file")"
fi

rcfile="$HOME/.termux-terminal.bashrc"
printf '%s\n' \
  '# Bridge-owned Bash startup file.' \
  '[ -f "$HOME/.obsidianrc" ] && . "$HOME/.obsidianrc"' > "$rcfile"
chmod 600 "$rcfile"

if [ ! -e "$HOME/.obsidianrc" ]; then
  printf '%s\n' \
    '# Termux Terminal startup commands run only in Obsidian terminal tabs.' \
    '# source ~/.bashrc' \
    '' \
    'issued_at=$(sed -n "2p" "$HOME/.termux_terminal_token" 2>/dev/null || true)' \
    'if [ -n "$issued_at" ] && [ "$(date +%s)" -gt "$((issued_at + 180 * 24 * 60 * 60))" ]; then' \
    '  printf "[Termux Terminal] Token expires soon; rerun the Termux Terminal installer to rotate it.\\n" >&2' \
    'fi' > "$HOME/.obsidianrc"
fi

mkdir -p "$service_dir"
printf '%s\n' '#!/data/data/com.termux/files/usr/bin/sh' "exec $binary" > "$service_dir/run"
chmod 700 "$service_dir/run"

SVDIR="$prefix/var/service"
if [ ! -d "$service_dir/supervise" ]; then
  service-daemon start || true
fi
SVDIR="$SVDIR" sv up "$service_name"

printf '%s\n' "Installed and started $service_name." "Bridge token: $token" "Paste the token into Obsidian Settings > Termux Terminal."
