#!/data/data/com.termux/files/usr/bin/bash
# TSCP build split: native (Bionic) vs wasm-smoke (glibc/proot)
set -e

case "$1" in
  native)
    echo "==> Native check (excludes tscp-wasm-smoke)"
    cargo check --workspace --exclude tscp-wasm-smoke 2>&1 | tee build-native.log
    ;;
  wasm)
    echo "==> WASM check must run inside proot-distro debian"
    echo "    Run: proot-distro login debian"
    echo "    Then: cd ~/tscp-anchor && cargo check -p tscp-wasm-smoke"
    ;;
  all)
    "$0" native
    echo "==> Reminder: run 'proot-distro login debian' separately for wasm-smoke"
    ;;
  *)
    echo "Usage: $0 {native|wasm|all}"
    exit 1
    ;;
esac
