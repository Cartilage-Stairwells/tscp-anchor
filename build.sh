#!/usr/bin/env bash
set -euo pipefail

MODE="core"
if [[ "${1:-}" == "--full" ]]; then MODE="full"; shift; fi

export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1}
export RUSTFLAGS="-C codegen-units=1 -C opt-level=0"
export CARGO_TERM_COLOR=always

TS=$(date +%Y%m%d-%H%M%S)
LOG="build-$TS-$MODE.log"

echo "=== TSCP Build Contract v1 | mode=$MODE | $TS ===" | tee "$LOG"
echo "Commit: $(git rev-parse --short HEAD)" | tee -a "$LOG"

if [[ "$MODE" == "core" ]]; then
  echo "→ CORE MODE: minimal, deterministic" | tee -a "$LOG"
  EXCLUDES=(--exclude tscp-wasm-smoke --exclude prover-server)
  PACKAGES=(-p tscp-polyir-verification -p oracle-layer -p commitment -p tscp-cli -p tscp-kernel)
else
  echo "→ FULL MODE: explicit expansion" | tee -a "$LOG"
  export CARGO_BUILD_JOBS=4
  EXCLUDES=()
  PACKAGES=(--workspace)
fi

cargo test "${EXCLUDES[@]}" "${PACKAGES[@]}" -- --nocapture 2>&1 | tee -a "$LOG"

# Seal
git rev-parse HEAD > CORE_COMMIT
sha256sum crates/tscp-polyir-verification/src/*.rs > CORE_HASHES
tar czf "tscp-core-seal-$TS-$MODE.tar.gz" CORE_COMMIT CORE_HASHES "$LOG" Cargo.lock

echo "✓ Seal: tscp-core-seal-$TS-$MODE.tar.gz" | tee -a "$LOG"
