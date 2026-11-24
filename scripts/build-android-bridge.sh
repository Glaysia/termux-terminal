#!/bin/sh
set -eu

ndk="${ANDROID_NDK_HOME:-${ANDROID_NDK_ROOT:-}}"
case "$(uname -s)" in
  Linux) host=linux-x86_64 ;;
  Darwin) host=darwin-x86_64 ;;
  *) echo 'Build the Android bridge on Linux or macOS.' >&2; exit 1 ;;
esac

toolchain="$ndk/toolchains/llvm/prebuilt/$host/bin"
linker="$toolchain/aarch64-linux-android24-clang"
if [ ! -x "$linker" ]; then
  echo 'Set ANDROID_NDK_HOME to Android NDK r30.' >&2
  exit 1
fi

cd "$(dirname "$0")/.."
CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$linker" \
  cargo build --locked -p termux-bridge --target aarch64-linux-android --release
binary="${CARGO_TARGET_DIR:-target}/aarch64-linux-android/release/termux-bridge"
"$toolchain/llvm-strip" "$binary"
header="$("$toolchain/llvm-readelf" --file-header "$binary")"
printf '%s\n' "$header" | grep -E 'Type:[[:space:]]+DYN'
printf '%s\n' "$header" | grep -E 'Machine:[[:space:]]+AArch64'
"$toolchain/llvm-readelf" --program-headers "$binary" | grep -F 'Requesting program interpreter: /system/bin/linker64'
"$toolchain/llvm-readelf" --dynamic "$binary" | grep -E 'FLAGS_1.*PIE'
