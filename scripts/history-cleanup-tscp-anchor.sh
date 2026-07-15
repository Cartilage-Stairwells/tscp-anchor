#!/usr/bin/env bash
# history-cleanup-tscp-anchor.sh
#
# Removes historical build artifact bloat from tscp-anchor git history.
# Reduces .git size from ~299MB to ~1.3MB (actual working tree).
#
# Bloat sources identified:
#   - node_modules/ binaries (Ethereum/Ganache native .node files)
#   - target/ Cargo build artifacts
#   - .Cargo.toml.swp vim swap file
#
# PREREQUISITES:
#   pip install git-filter-repo
#
# USAGE:
#   Clone a FRESH copy first — do NOT run on your primary working clone.
#   git clone git@github.com:Cartilage-Stairwells/tscp-anchor.git tscp-anchor-clean
#   cd tscp-anchor-clean
#   bash /path/to/history-cleanup-tscp-anchor.sh
#
# After completion: re-clone from the force-pushed remote for local work.

set -euo pipefail

REPO_PATTERN="tscp-anchor"
MIGRATION_DATE="2026-07-14"
TAG_NAME="pre-history-cleanup-${MIGRATION_DATE}"

# ── Guard: correct repo ───────────────────────────────────────────────────────
if ! git remote -v 2>/dev/null | grep -q "$REPO_PATTERN"; then
  echo "ERROR: Does not appear to be a tscp-anchor clone. Aborting." >&2
  exit 1
fi

# ── Guard: git-filter-repo present ───────────────────────────────────────────
if ! command -v git-filter-repo &>/dev/null; then
  echo "ERROR: git-filter-repo not found. Install with: pip install git-filter-repo" >&2
  exit 1
fi

echo "==================================================================="
echo "  tscp-anchor history cleanup"
echo "  Date: $MIGRATION_DATE"
echo "==================================================================="

# ── Step 1: Record pre-cleanup state ─────────────────────────────────────────
BEFORE_SIZE=$(du -sh .git | cut -f1)
BEFORE_HEAD=$(git rev-parse HEAD)
echo ""
echo "Pre-cleanup state:"
echo "  HEAD:     $BEFORE_HEAD"
echo "  .git size: $BEFORE_SIZE"

# ── Step 2: Create annotated checkpoint tag ───────────────────────────────────
echo ""
echo "Step 2: Creating annotated checkpoint tag..."
git tag -a "$TAG_NAME" HEAD \
  -m "Migration checkpoint: pre-history-cleanup $MIGRATION_DATE. Rollback anchor." \
  2>/dev/null || echo "  Tag already exists — skipping creation."
echo "  Tag: $TAG_NAME → $(git rev-parse $TAG_NAME)"

# ── Step 3: Push tag to remote (confirm rollback anchor is remote) ────────────
echo ""
echo "Step 3: Pushing checkpoint tag to origin..."
git push origin "$TAG_NAME"

# Verify the tag exists remotely before proceeding
git fetch origin --tags --quiet
git rev-parse "refs/tags/$TAG_NAME" >/dev/null 2>&1 \
  || { echo "ERROR: Remote tag verification failed. Aborting before rewrite." >&2; exit 1; }
echo "  Remote tag verified: refs/tags/$TAG_NAME"

# ── Step 4: Rewrite history ───────────────────────────────────────────────────
echo ""
echo "Step 4: Removing node_modules/ from all commits..."
git filter-repo --path node_modules --invert-paths --force

echo ""
echo "Step 5: Removing target/ from all commits..."
git filter-repo --path target --invert-paths --force

echo ""
echo "Step 6: Removing .Cargo.toml.swp from all commits..."
git filter-repo --path .Cargo.toml.swp --invert-paths --force 2>/dev/null || true

# ── Step 5: Verify ───────────────────────────────────────────────────────────
echo ""
echo "Step 7: Verifying — checking for residual references..."

NM_COUNT=$(git rev-list --objects --all \
  | git cat-file --batch-check='%(objecttype) %(rest)' \
  | grep -c ' node_modules/' || true)
TG_COUNT=$(git rev-list --objects --all \
  | git cat-file --batch-check='%(objecttype) %(rest)' \
  | grep -c ' target/' || true)

echo "  node_modules/ references remaining: $NM_COUNT"
echo "  target/ references remaining:       $TG_COUNT"

if [ "$NM_COUNT" -gt 0 ] || [ "$TG_COUNT" -gt 0 ]; then
  echo "  WARNING: Residual references found. Review before pushing."
else
  echo "  CLEAN: No build artifact references in history."
fi

# ── Step 6: Repack ───────────────────────────────────────────────────────────
echo ""
echo "Step 8: Repacking..."
git gc --aggressive --prune=now

AFTER_SIZE=$(du -sh .git | cut -f1)
AFTER_HEAD=$(git rev-parse HEAD)

# ── Step 7: Scrub report ─────────────────────────────────────────────────────
REPORT_FILE="./cleanup-report-${MIGRATION_DATE}.txt"
cat >"$REPORT_FILE" <<EOF
tscp-anchor History Cleanup Report
===================================
Date:          $MIGRATION_DATE
Checkpoint tag: $TAG_NAME

Pre-cleanup:
  HEAD:      $BEFORE_HEAD
  .git size: $BEFORE_SIZE

Post-cleanup:
  HEAD:      $AFTER_HEAD
  .git size: $AFTER_SIZE

Paths removed from history:
  - node_modules/
  - target/
  - .Cargo.toml.swp

Residual references:
  node_modules/: $NM_COUNT
  target/:       $TG_COUNT

Rollback:
  git checkout $TAG_NAME
  (requires re-clone after force-push to restore to pre-cleanup state)
EOF

echo ""
echo "==================================================================="
echo "  Cleanup complete"
echo "  Before: $BEFORE_SIZE  →  After: $AFTER_SIZE"
echo "  Report: $REPORT_FILE"
echo "==================================================================="
echo ""
echo "NEXT STEPS:"
echo "  1. Review: git log --oneline -10"
echo "  2. Build check: cargo build (optional sanity)"
echo "  3. Force-push:"
echo "       git push --force-with-lease origin master"
echo "  4. Notify all collaborators to re-clone."
echo "  5. Keep $REPORT_FILE as the auditable cleanup record."
