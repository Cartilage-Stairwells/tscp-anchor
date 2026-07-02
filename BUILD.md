# TSCP Build Notes — Termux/Bionic Split

## Known issue: tscp-wasm-smoke SIGSEGVs on native Termux

`tscp-wasm-smoke` depends on `wasmtime`/`cranelift-codegen`. On native
Termux (Bionic libc, aarch64), building it reliably crashes rustc's
own parser with SIGSEGV — Cranelift's macro/cfg-dense codegen graph
overflows rustc_driver's parsing recursion depth. This is NOT fixed by
raising RUST_MIN_STACK.

## Fix: split builds

- **Native (Termux)**: everything except tscp-wasm-smoke builds and
  runs fine directly.

  cargo check --workspace --exclude tscp-wasm-smoke

- **tscp-wasm-smoke**: build only inside proot-distro (glibc):

  proot-distro login debian
  cd ~/tscp-anchor
  cargo check -p tscp-wasm-smoke

See scripts/build-check.sh for a wrapper.
