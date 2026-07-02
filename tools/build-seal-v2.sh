#!/usr/bin/env bash
set -euo pipefail

TS=$(date +%Y%m%d-%H%M%S)

echo "=== TSCP SEAL BUILDER v2 ==="
echo "Timestamp: $TS"

OUT="tscp-core-seal-$TS-core"
WORKDIR=$(mktemp -d)

echo "Workdir: $WORKDIR"

# ------------------------------------------------------------
# 1. Collect build artifacts
# ------------------------------------------------------------

echo "[1] Capturing build metadata..."

mkdir -p "$WORKDIR"

git rev-parse HEAD > "$WORKDIR/CORE_COMMIT"

# ------------------------------------------------------------
# 2. Compute CORE_HASHES (source snapshot hash)
# ------------------------------------------------------------

echo "[2] Computing CORE_HASHES from tscp-polyir-verification"

SRC_DIR="crates/tscp-polyir-verification/src"

if [ ! -d "$SRC_DIR" ]; then
  echo "ERROR: missing source directory: $SRC_DIR"
  exit 1
fi

HASH=$(find "$SRC_DIR" -type f -name "*.rs" -print0 \
  | sort -z \
  | xargs -0 sha256sum \
  | sha256sum \
  | awk '{print $1}')

echo "$HASH" > "$WORKDIR/CORE_HASHES"

# ------------------------------------------------------------
# 3. Copy Cargo.lock (determinism anchor)
# ------------------------------------------------------------

echo "[3] Copying Cargo.lock"
cp Cargo.lock "$WORKDIR/"

# ------------------------------------------------------------
# 4. Capture latest build log if exists
# ------------------------------------------------------------

echo "[4] Capturing build log"

if ls build-*.log 1> /dev/null 2>&1; then
  cp build-*.log "$WORKDIR/"
else
  echo "WARN: no build log found"
fi

# ------------------------------------------------------------
# 5. Embed full source snapshot (CRITICAL FIX)
# ------------------------------------------------------------

echo "[5] Embedding source snapshot"

mkdir -p "$WORKDIR/source_snapshot"
cp -r crates/tscp-polyir-verification "$WORKDIR/source_snapshot/"

# ------------------------------------------------------------
# 6. Seal manifest
# ------------------------------------------------------------

echo "[6] Writing manifest"

cat > "$WORKDIR/SEAL_MANIFEST" <<MANIFEST
TSCP-SEAL-V2
timestamp=$TS
mode=core
determinism=strict
contents=CORE_COMMIT,CORE_HASHES,Cargo.lock,build-log,source_snapshot
MANIFEST

# ------------------------------------------------------------
# 7. Pack final seal
# ------------------------------------------------------------

echo "[7] Creating tarball"

tar -czf "$OUT.tar.gz" -C "$WORKDIR" .

echo ""
echo "=== SEAL COMPLETE ==="
echo "Output: $OUT.tar.gz"
echo "Size:"
ls -lh "$OUT.tar.gz"

echo ""
echo "Done."
