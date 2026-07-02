#!/usr/bin/env bash

echo "=== TSCP SEAL BUILDER v2.3 (STRICT) ==="

TS=$(date +%Y%m%d-%H%M%S)

WORKDIR=$(mktemp -d)

echo "Workdir: $WORKDIR"

# ------------------------------------------------------------
# 1. Capture commit
# ------------------------------------------------------------

git rev-parse HEAD > "$WORKDIR/CORE_COMMIT"

# ------------------------------------------------------------
# 2. Define canonical repo root (IMPORTANT FIX)
# ------------------------------------------------------------

REPO_ROOT=$(git rev-parse --show-toplevel)

echo "Repo root: $REPO_ROOT" > "$WORKDIR/REPO_ROOT"

# ------------------------------------------------------------
# 3. Snapshot source (canonical path binding)
# ------------------------------------------------------------

SRC_DIR="$REPO_ROOT/crates/oracle-layer/src"

if [ ! -d "$SRC_DIR" ]; then
  echo "ERROR: missing source dir: $SRC_DIR"
  exit 1
fi

echo "$SRC_DIR" > "$WORKDIR/SNAPSHOT_PATH"

SNAPSHOT_HASH=$(
  find "$SRC_DIR" -type f -name "*.rs" -print0 \
  | sort -z \
  | xargs -0 sha256sum \
  | sha256sum \
  | awk '{print $1}'
)

echo "$SNAPSHOT_HASH" > "$WORKDIR/CORE_SNAPSHOT_HASH"

# ------------------------------------------------------------
# 4. Lockfile
# ------------------------------------------------------------

cp "$REPO_ROOT/Cargo.lock" "$WORKDIR/"

# ------------------------------------------------------------
# 5. Build logs (all)
# ------------------------------------------------------------

mkdir -p "$WORKDIR/logs"

cp $REPO_ROOT/build-*.log "$WORKDIR/logs/" 2>/dev/null || true

# ------------------------------------------------------------
# 6. Full snapshot copy (for verifier v3 compatibility)
# ------------------------------------------------------------

mkdir -p "$WORKDIR/source_snapshot"

cp -r "$SRC_DIR/.." "$WORKDIR/source_snapshot/oracle-layer"

# ------------------------------------------------------------
# 7. Manifest (now includes path binding)
# ------------------------------------------------------------

cat > "$WORKDIR/SEAL_MANIFEST" <<MANIFEST
TSCP-SEAL-V2.3
timestamp=$TS
mode=strict
repo_root=$REPO_ROOT
snapshot_path=$SRC_DIR
contents=CORE_COMMIT,CORE_SNAPSHOT_HASH,Cargo.lock,logs,source_snapshot
MANIFEST

# ------------------------------------------------------------
# 8. Pack deterministically
# ------------------------------------------------------------

OUT="tscp-core-seal-v2.3-$TS.tar.gz"

tar -czf "$OUT" -C "$WORKDIR" .

echo ""
echo "=== SEAL COMPLETE ==="
echo "Output: $OUT"
ls -lh "$OUT"
