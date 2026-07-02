#!/usr/bin/env bash
set -euo pipefail

SEAL="$1"

if [ -z "$SEAL" ]; then
  echo "Usage: verify-seal-v2.sh <seal.tar.gz>"
  exit 1
fi

WORKDIR=$(mktemp -d)

echo "=== TSCP SEAL VERIFIER v2 ==="
echo "Seal: $SEAL"
echo "Workdir: $WORKDIR"

tar -xzf "$SEAL" -C "$WORKDIR"
cd "$WORKDIR"

echo ""
echo "[1] CORE_COMMIT check"
test -s CORE_COMMIT && cat CORE_COMMIT || {
  echo "FAIL: CORE_COMMIT missing or empty"
  exit 1
}

echo ""
echo "[2] CORE_HASHES check"
test -s CORE_HASHES || {
  echo "FAIL: CORE_HASHES missing"
  exit 1
}

echo ""
echo "[3] Cargo.lock check"
test -f Cargo.lock || {
  echo "FAIL: Cargo.lock missing"
  exit 1
}

echo ""
echo "[4] Build log check"
ls build-*.log >/dev/null 2>&1 || {
  echo "FAIL: build log missing"
  exit 1
}

echo ""
echo "[5] Lockfile fingerprint"
LOCK_HASH=$(sha256sum Cargo.lock | awk '{print $1}')
echo "Cargo.lock SHA256: $LOCK_HASH"

echo ""
echo "[6] CORE_HASHES content"
cat CORE_HASHES

echo ""
echo "=== VERIFICATION COMPLETE ==="
echo "STATUS: VALID SEAL STRUCTURE"
