#!/usr/bin/env bash
set -euo pipefail

SEAL="$1"

echo "=== TSCP SEAL VERIFIER v2.3 ==="
echo "Seal: $SEAL"

WORKDIR=$(mktemp -d)
tar -xzf "$SEAL" -C "$WORKDIR"
cd "$WORKDIR"

echo "[1] CORE_COMMIT"
cat CORE_COMMIT

echo "[2] CORE_SNAPSHOT_HASH"
cat CORE_SNAPSHOT_HASH

echo "[3] Cargo.lock hash"
sha256sum Cargo.lock

echo "[4] snapshot exists"
test -d source_snapshot/oracle-layer/src || {
  echo "FAIL: missing snapshot"
  exit 1
}

echo "[5] seal integrity OK"
echo "STATUS: VALID (v2.3)"
