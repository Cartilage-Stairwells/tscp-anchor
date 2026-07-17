#!/usr/bin/env bash
# history-cleanup-tscp-canon.sh
#
# Removes historical Cargo target/ build artifacts from tscp-canon git history.
# Reduces .git from ~32MB to actual source size.
# Same pattern as history-cleanup-tscp-anchor.sh — see that script for full notes.

set -euo pipefail

REPO_PATTERN="tscp-canon"
MIGRATION_DATE="2026-07-14"
TAG_NAME="pre-history-cleanup-${MIGRATION_DATE}"

if ! git remote -v 2>/dev/null | grep -q "$REPO_PATTERN"; then
  echo "ERROR: Does not appear to be a tscp-canon clone. Aborting." >&2; exit 1
fi
if ! command -v git-filter-repo &>/dev/null; then
  echo "ERROR: git-filter-repo not found. Install with: pip install git-filter-repo" >&2; exit 1
fi

echo "==================================================================="
echo "  tscp-canon history cleanup — Date: $MIGRATION_DATE"
echo "==================================================================="

BEFORE_SIZE=$(du -sh .git | cut -f1)
BEFORE_HEAD=$(git rev-parse HEAD)
echo "Pre-cleanup: HEAD=$BEFORE_HEAD  .git=$BEFORE_SIZE"

echo ""
echo "Step 1: Annotated checkpoint tag..."
git tag -a "$TAG_NAME" HEAD \
  -m "Migration checkpoint: pre-history-cleanup $MIGRATION_DATE." \
  2>/dev/null || echo "  Tag already exists — skipping."

echo "Step 2: Pushing tag to remote..."
git push origin "$TAG_NAME"

git fetch origin --tags --quiet
git rev-parse "refs/tags/$TAG_NAME" >/dev/null 2>&1 \
  || { echo "ERROR: Remote tag not found after push. Aborting." >&2; exit 1; }
echo "  Remote tag verified: refs/tags/$TAG_NAME"

echo ""
echo "Step 3: Removing target/ from all commits..."
git filter-repo --path target --invert-paths --force

echo ""
echo "Step 4: Verifying..."
TG_COUNT=$(git rev-list --objects --all \
  | git cat-file --batch-check='%(objecttype) %(rest)' \
  | grep -c ' target/' || true)
echo "  target/ references remaining: $TG_COUNT"
[ "$TG_COUNT" -eq 0 ] && echo "  CLEAN." || echo "  WARNING: residual references found."

echo ""
echo "Step 5: Repacking..."
git gc --aggressive --prune=now

AFTER_SIZE=$(du -sh .git | cut -f1)
AFTER_HEAD=$(git rev-parse HEAD)

REPORT_FILE="./cleanup-report-${MIGRATION_DATE}.txt"
cat >"$REPORT_FILE" <<EOF
tscp-canon History Cleanup Report
===================================
Date:           $MIGRATION_DATE
Checkpoint tag: $TAG_NAME

Pre-cleanup:  HEAD=$BEFORE_HEAD  .git=$BEFORE_SIZE
Post-cleanup: HEAD=$AFTER_HEAD   .git=$AFTER_SIZE

Paths removed: target/
Residual target/ references: $TG_COUNT

Rollback: git checkout $TAG_NAME
EOF

echo ""
echo "==================================================================="
echo "  Done. $BEFORE_SIZE → $AFTER_SIZE"
echo "  Report: $REPORT_FILE"
echo "==================================================================="
echo ""
echo "NEXT STEPS:"
echo "  git push --force-with-lease origin master"
