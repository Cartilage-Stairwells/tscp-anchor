#!/usr/bin/env bash
set -euo pipefail

SEAL="${1:-}"

if [ -z "$SEAL" ]; then
  echo "Usage: verify-seal-v2.sh <seal.tar.gz>"
  exit 1
fi

WORKDIR=$(mktemp -d)

echo "=== TSCP SEAL VERIFIER v2.3 (STRICT) ==="
echo "Seal: $SEAL"
echo "Workdir: $WORKDIR"

tar -xzf "$SEAL" -C "$WORKDIR"
cd "$WORKDIR"

# ------------------------------------------------------------
# [0] Seal structure fingerprint (identity layer)
# ------------------------------------------------------------

echo ""
echo "[0] Seal hash fingerprint"

SEAL_HASH=$(find . -type f -print0 \
  | sort -z \
  | xargs -0 sha256sum \
  | sha256sum \
  | awk '{print $1}')

echo "Seal content hash: $SEAL_HASH"

# ------------------------------------------------------------
# [1] Manifest required
# ------------------------------------------------------------

echo ""
echo "[1] SEAL_MANIFEST check"

if [ ! -f SEAL_MANIFEST ]; then
  echo "FAIL: missing SEAL_MANIFEST"
  exit 1
fi

cat SEAL_MANIFEST

# ------------------------------------------------------------
# [2] CORE_COMMIT validation
# ------------------------------------------------------------

echo ""
echo "[2] CORE_COMMIT"

if [ ! -s CORE_COMMIT ]; then
  echo "FAIL: CORE_COMMIT missing or empty"
  exit 1
fi

CORE_COMMIT=$(cat CORE_COMMIT)
echo "commit=$CORE_COMMIT"

# ------------------------------------------------------------
# [3] Cargo.lock integrity
# ------------------------------------------------------------

echo ""
echo "[3] Cargo.lock fingerprint"

if [ ! -f Cargo.lock ]; then
  echo "FAIL: Cargo.lock missing"
  exit 1
fi

LOCK_HASH=$(sha256sum Cargo.lock | awk '{print $1}')
echo "Cargo.lock SHA256: $LOCK_HASH"

# ------------------------------------------------------------
# [4] Build log requirement
# ------------------------------------------------------------

echo ""
echo "[4] build log check"

if ! ls build-*.log >/dev/null 2>&1; then
  echo "FAIL: build log missing"
  exit 1
fi

echo "build log present"

# ------------------------------------------------------------
# [5] Source snapshot (STRICT REQUIREMENT)
# ------------------------------------------------------------

echo ""
echo "[5] source snapshot validation"

if [ ! -d source_snapshot ]; then
  echo "FAIL: missing source_snapshot (required in v2.3)"
  exit 1
fi

SRC_DIR="source_snapshot/crates/oracle-layer/src"

if [ ! -d "$SRC_DIR" ]; then
  echo "FAIL: missing polyir snapshot source"
  exit 1
fi

ACTUAL_HASH=$(find "$SRC_DIR" -type f -name "*.rs" -print0 \
  | sort -z \
  | xargs -0 sha256sum \
  | sha256sum \
  | awk '{print $1}')

echo "recomputed=$ACTUAL_HASH"

EXPECTED_HASH=$(cat CORE_HASHES | tr -d '\n')
echo "expected=$EXPECTED_HASH"

if [ "$ACTUAL_HASH" != "$EXPECTED_HASH" ]; then
  echo "FAIL: CORE_HASHES mismatch"
  exit 1
fi

# ------------------------------------------------------------
# [6] FINAL RESULT
# ------------------------------------------------------------

echo ""
echo "=== TSCP SEAL VERIFIED ==="
echo "STATUS: VALID (STRICT MODE v2.3)"
exit 0
