# tscp-v2.3-sealed — Release Evidence Notes

cargo-build.txt and cargo-test.txt in this folder capture a FAILED build
attempt: cargo hit a rustc SIGSEGV (stack overflow in the parser) while
compiling cranelift-codegen, pulled in transitively via
tscp-wasm-smoke -> wasmtime -> wasmtime-cranelift.

This is a Termux/bionic-libc-specific rustc issue, not a TSCP logic or
protocol failure. Other workspace crates compiled and tested clean in
the same session (see cargo-test.txt for the oracle-layer 35/35 passing
run prior to the cranelift crash).

Fix applied in commit b418edcd (2026-07-02):
- tscp-wasm-smoke/Cargo.toml: wasmtime now uses
  default-features = false, features = ["runtime"]
- Added [[bin]] required-features = ["jit"] so tscp-wasm-smoke is
  skipped by default on Termux builds
- JIT path (cranelift) now only exercised in CI: see
  .github/workflows/wasm-smoke.yml, runs on ubuntu-latest

commit.txt / version.txt / toolchain.txt / git-status.txt / workspace.json
in this folder remain valid as environment/state snapshots at the time
of the tscp-v2.3-sealed tag; only the build/test logs reflect the
cranelift crash, not the workspace as a whole.
