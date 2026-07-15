#!/usr/bin/env bash
# history-scrub-culturalcodex.sh
#
# Removes .env (revoked Polygon private key + Infura key) from CulturalCodex
# git history. Keys are already dead — this is provenance cleanup, not incident
# response. The scrub report is kept as an auditable record of what was removed.

set -euo pipefail

REPO_PATTERN="CulturalCodex"
MIGRATION_DATE="2026-07-14"
TAG_NAME="pre-env-scrub-${MIGRATION_DATE}"

if ! git remote -v 2>/dev/null | grep -q "$REPO_PATTERN"; then
  echo "ERROR: Does not appear to be a CulturalCodex clone. Aborting." >&2; exit 1
fi
if ! command -v git-filter-repo &>/dev/null; then
  echo "ERROR: git-filter-repo not found. Install: pip install git-filter-repo" >&2; exit 1
fi

echo "==================================================================="
echo "  CulturalCodex .env history scrub — Date: $MIGRATION_DATE"
echo "  Provenance cleanup (keys already revoked)"
echo "==================================================================="

BEFORE_SIZE=$(du -sh .git | cut -f1)
BEFORE_HEAD=$(git rev-parse HEAD)

# Collect old blob IDs for the scrub report before rewriting
OLD_ENV_BLOBS=$(git rev-list --objects --all \
  | git cat-file --batch-check='%(objecttype) %(objectname) %(rest)' \
  | awk '/blob .* \.env$/{print $2}' || true)

echo "Pre-scrub: HEAD=$BEFORE_HEAD  .git=$BEFORE_SIZE"
echo "Old .env blob IDs:"
echo "$OLD_ENV_BLOBS" | sed 's/^/  /' || echo "  (none found — may already be clean)"

echo ""
echo "Step 1: Annotated checkpoint tag..."
git tag -a "$TAG_NAME" HEAD \
  -m "Pre-scrub checkpoint $MIGRATION_DATE. Rollback anchor for .env removal." \
  2>/dev/null || echo "  Tag already exists — skipping."

echo "Step 2: Pushing tag to remote..."
git push origin "$TAG_NAME"

git fetch origin --tags --quiet
git rev-parse "refs/tags/$TAG_NAME" >/dev/null 2>&1 \
  || { echo "ERROR: Remote tag not found. Aborting before rewrite." >&2; exit 1; }
echo "  Remote tag verified: refs/tags/$TAG_NAME"

echo ""
echo "Step 3: Removing .env from all commits..."
git filter-repo --path .env --invert-paths --force

echo "Step 4: Removing .env.* variants..."
git filter-repo --path-glob '.env.*' --invert-paths --force 2>/dev/null || true

echo ""
echo "Step 5: Verifying..."
REMAINING=$(git rev-list --objects --all \
  | git cat-file --batch-check='%(objecttype) %(rest)' \
  | grep -c ' \.env' || true)
echo "  .env references remaining: $REMAINING"
[ "$REMAINING" -eq 0 ] && echo "  CLEAN." || {
  echo "  WARNING: residual .env references:"
  git rev-list --objects --all \
    | git cat-file --batch-check='%(objecttype) %(rest)' | grep ' \.env'
}

echo ""
echo "Step 6: Confirming .gitignore covers .env..."
if grep -q "^\.env$" .gitignore 2>/dev/null; then
  echo "  .env already in .gitignore"
else
  echo ".env" >>.gitignore
  echo ".env.*" >>.gitignore
  git add .gitignore
  git commit -m "chore: ensure .env in .gitignore [scrub $MIGRATION_DATE]" || true
  echo "  Added .env to .gitignore"
fi

echo ""
echo "Step 7: Repacking..."
git gc --aggressive --prune=now

AFTER_SIZE=$(du -sh .git | cut -f1)
AFTER_HEAD=$(git rev-parse HEAD)

# Scrub report — keep this as the auditable cleanup record
REPORT_FILE="./env-scrub-report-${MIGRATION_DATE}.txt"
cat >"$REPORT_FILE" <<EOF
CulturalCodex .env History Scrub Report
=========================================
Date:           $MIGRATION_DATE
Checkpoint tag: $TAG_NAME
Reason:         Provenance cleanup. Credentials were already revoked prior to scrub.

Pre-scrub:  HEAD=$BEFORE_HEAD  .git=$BEFORE_SIZE
Post-scrub: HEAD=$AFTER_HEAD   .git=$AFTER_SIZE

Removed paths:
  .env
  .env.* (glob)

Old blob IDs removed:
$OLD_ENV_BLOBS

Residual .env references post-scrub: $REMAINING

Credentials affected (already revoked before this scrub):
  - Polygon Mainnet private key (PRIVATE_KEY)
  - Infura API key (RPC_URL endpoint)

Rollback:
  git checkout $TAG_NAME
  (only meaningful before force-push; after force-push, rollback requires
   restoring from the pre-scrub tag on a fresh clone)
EOF

echo ""
echo "==================================================================="
echo "  Scrub complete. $BEFORE_SIZE → $AFTER_SIZE"
echo "  Report: $REPORT_FILE"
echo "==================================================================="
echo ""
echo "NEXT STEPS:"
echo "  1. Review report: cat $REPORT_FILE"
echo "  2. Force-push:"
echo "       git push --force-with-lease origin main"
echo "  3. Retain $REPORT_FILE as the auditable cleanup record."
echo "  4. Consider deleting pinata-api-keys repo after confirming rotated."
