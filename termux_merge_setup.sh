#!/usr/bin/env bash
# termux_merge_setup.sh — GPG signing + PR merge setup for Termux
#
# This script sets up GPG signing in Termux and prepares the merge of
# PR #29 (docs/custody-boundary-artifacts) into tscp-anchor master.
#
# Prerequisites:
#   - Termux installed on your Android device
#   - GitHub access token (for git push)
#   - Your GPG secret key exported to a file (see step 1)
#
# Usage:
#   bash termux_merge_setup.sh
#
# The script will:
#   1. Install gnupg in Termux
#   2. Import your GPG secret key
#   3. Configure git for signed commits
#   4. Clone the repo (if not already cloned)
#   5. Merge the PR branch with a signed merge commit
#   6. Push to master
#
# You will be prompted for your GPG passphrase during the merge.
set -euo pipefail

REPO="tscp-anchor"
ORG="Cartilage-Stairwells"
BRANCH="docs/custody-boundary-artifacts"
GPG_KEY_ID=""  # Will be detected from imported keys

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log()  { echo -e "${BLUE}[INFO]${NC} $1"; }
ok()   { echo -e "${GREEN}[OK]${NC}   $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
err()  { echo -e "${RED}[ERROR]${NC} $1"; }

# ── Step 1: Install gnupg ──────────────────────────────────────────────
log "Installing gnupg..."
pkg install -y gnupg
ok "gnupg installed"

# ── Step 2: Import GPG secret key ──────────────────────────────────────
# You need to have your GPG secret key exported. If you haven't done this:
#
#   On the machine where your key lives:
#     gpg --export-secret-keys KEY_ID > private.key
#     gpg --export KEY_ID > public.key
#
#   Transfer both files to your Android device (e.g. via Termux:storage
#   or scp), then run this script.
#
#   Or, if your key is already on the device, skip to import.

if [ -f "private.key" ]; then
    log "Importing GPG secret key from private.key..."
    gpg --import private.key
    ok "Secret key imported"
else
    warn "No private.key found. If your key is already imported, this is fine."
    warn "If not, export your key from the machine where it lives and place"
    warn "private.key in this directory, then re-run this script."
    echo ""
    log "Current GPG keys:"
    gpg --list-secret-keys 2>/dev/null || warn "No GPG keys found"
fi

if [ -f "public.key" ]; then
    log "Importing GPG public key from public.key..."
    gpg --import public.key
    ok "Public key imported"
fi

# Detect GPG key ID
GPG_KEY_ID=$(gpg --list-secret-keys --keyid-format=long 2>/dev/null | grep -oP '(?<=sec\s+ed25519/)[A-F0-9]+' | head -1)
if [ -z "$GPG_KEY_ID" ]; then
    # Try RSA
    GPG_KEY_ID=$(gpg --list-secret-keys --keyid-format=long 2>/dev/null | grep -oP '(?<=sec\s+rsa\d+/)[A-F0-9]+' | head -1)
fi

if [ -z "$GPG_KEY_ID" ]; then
    err "No GPG secret key found. Import your key first."
    err "Export from source machine: gpg --export-secret-keys KEY_ID > private.key"
    exit 1
fi
ok "Using GPG key: $GPG_KEY_ID"

# ── Step 3: Configure git ─────────────────────────────────────────────
log "Configuring git for signed commits..."
git config --global user.name "Sean Christopher Southwick"
git config --global user.email "adamantinespine@gmail.com"
git config --global user.signingkey "$GPG_KEY_ID"
git config --global commit.gpgsign true
git config --global gpg.program gpg
ok "Git configured for signing with key $GPG_KEY_ID"

# ── Step 4: Clone or update repo ───────────────────────────────────────
if [ -d "$REPO" ]; then
    log "Repo directory exists, fetching latest..."
    cd "$REPO"
    git fetch origin
    git checkout master
    git pull origin master
else
    log "Cloning $ORG/$REPO..."
    git clone "https://github.com/$ORG/$REPO.git"
    cd "$REPO"
fi
ok "Repo ready at $(pwd)"

# ── Step 5: Merge the PR branch ───────────────────────────────────────
log "Fetching PR branch..."
git fetch origin "$BRANCH"

log "Merging $BRANCH into master with GPG signature..."
git merge "$BRANCH" --no-ff -S -m "docs: formalize implementation custody boundary artifacts (#29)

Merge PR #29: implementation custody boundary artifacts.

Adds three specification artifacts defining the verification custody
contract for implementation target binding, execution trace verification,
and formal invariants. Motivated by Issue #27 (Verification Surface Drift).

Nine invariants in four layers:
  Identity: Authority Neutrality, Target Binding, Build Artifact Identity
  Execution: Hardware Presence, Fallback Prohibition, Claim Scope Integrity
  Observation: Observation Independence
  Governance: Gate Ordering, Receipt Lifecycle Integrity

This PR does not prove AVX-512 equivalence. It establishes the custody
framework required before AVX-512 equivalence can be claimed.

Architecture moves from 'prove the code' to 'prove what code, what binary,
what execution, what claim, and what evidence.'

Issue #27 receipt status: REVOKED (claim was false, not merely replaced).

Closes #27 (custody framework). AVX-512 equivalence gate remains blocked
pending proper execution binding."

ok "Merge commit created with GPG signature"

# ── Step 6: Push to master ────────────────────────────────────────────
log "Pushing to master..."
echo ""
warn "You will be prompted for your GitHub credentials and GPG passphrase."
warn "If push fails due to signing, ensure your GPG key is registered"
warn "with GitHub at https://github.com/settings/keys"
echo ""
read -p "Push to master now? (yes/no): " confirm
if [ "$confirm" = "yes" ]; then
    git push origin master
    ok "Pushed to master. PR #29 is merged."
    echo ""
    ok "Next steps:"
    echo "  1. Version bump all three artifacts to 2.0"
    echo "  2. Update schema_version references"
    echo "  3. Verify CI passes on master"
else
    warn "Push skipped. The signed merge commit is ready locally."
    warn "Run 'git push origin master' when ready."
fi
