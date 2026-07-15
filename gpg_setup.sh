#!/usr/bin/env bash
# gpg_setup.sh — Ed25519 dual-UID GPG key for commit signing across two GitHub accounts
#
# Accounts:
#   Cartilage-Stairwells  → adamantinespine@gmail.com
#   Triune-Oracle         → schlagetorren@gmail.com
#
# Usage:
#   ./gpg_setup.sh
#
# Override either email at runtime without modifying the script:
#   PRIMARY_EMAIL="other@example.com" ./gpg_setup.sh
#   ORACLE_EMAIL="other@example.com" ./gpg_setup.sh

set -euo pipefail

# ── Configurable identity (override via environment) ────────────────────────
REAL_NAME="${REAL_NAME:-Sean Christopher Southwick}"
PRIMARY_EMAIL="${PRIMARY_EMAIL:-adamantinespine@gmail.com}"
ORACLE_EMAIL="${ORACLE_EMAIL:-schlagetorren@gmail.com}"
KEY_EXPIRY="${KEY_EXPIRY:-2y}"
# ────────────────────────────────────────────────────────────────────────────

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
BLUE='\033[0;34m'; BOLD='\033[1m'; NC='\033[0m'

log_info()    { echo -e "${BLUE}[INFO]${NC}    $1"; }
log_success() { echo -e "${GREEN}[OK]${NC}      $1"; }
log_warn()    { echo -e "${YELLOW}[WARN]${NC}    $1"; }
log_error()   { echo -e "${RED}[ERROR]${NC}   $1"; exit 1; }
log_step()    { echo -e "\n${BOLD}── $1 ──${NC}"; }

# ── 1. Dependency check ──────────────────────────────────────────────────────
log_step "Checking dependencies"

if ! command -v gpg &>/dev/null; then
  OS="$(uname -s)"
  if   [ "$OS" = "Darwin" ]; then
    command -v brew &>/dev/null || log_error "Homebrew not found. Install from https://brew.sh"
    brew install gnupg pinentry-mac
  elif [ "$OS" = "Linux" ]; then
    if   command -v apt-get &>/dev/null; then sudo apt-get update -q && sudo apt-get install -y gnupg pinentry-tty
    elif command -v dnf     &>/dev/null; then sudo dnf install -y gnupg2 pinentry
    elif command -v pacman  &>/dev/null; then sudo pacman -Sy --needed gnupg pinentry
    else log_error "Unsupported package manager. Install gnupg manually."
    fi
  else
    log_error "Unsupported OS: $OS"
  fi
fi

log_success "GPG: $(gpg --version | head -n1)"

# ── 2. Idempotency — skip if key already exists for this identity ────────────
log_step "Checking for existing key"

EXISTING=$(gpg --list-secret-keys --with-colons "$PRIMARY_EMAIL" 2>/dev/null | awk -F: '/^sec/{print $5}' | head -1)
if [ -n "$EXISTING" ]; then
  log_warn "Key for $PRIMARY_EMAIL already exists (fingerprint prefix: $EXISTING)."
  log_warn "Skipping generation. Delete it first if you want a fresh key:"
  log_warn "  gpg --delete-secret-and-public-key $EXISTING"
  KEY_FP="$EXISTING"
else
  # ── 3. Key generation ───────────────────────────────────────────────────────
  log_step "Generating Ed25519 key"
  log_info "Identity: $REAL_NAME <$PRIMARY_EMAIL>"
  log_info "Expiry:   $KEY_EXPIRY"

  BATCH=$(mktemp)
  # Note: %no-protection generates without passphrase for unattended use.
  # You will be prompted to add a passphrase by gpg-agent on first use.
  # To add one now: gpg --edit-key <KEY_ID> → passwd → save
  cat >"$BATCH" <<EOF
%echo Generating Ed25519 key for TSCP dual-account signing
Key-Type: EDDSA
Key-Curve: ed25519
Subkey-Type: ECDH
Subkey-Curve: cv25519
Name-Real: ${REAL_NAME}
Name-Email: ${PRIMARY_EMAIL}
Expire-Date: ${KEY_EXPIRY}
%no-protection
%commit
%echo Key generation complete
EOF

  gpg --batch --generate-key "$BATCH"
  rm -f "$BATCH"

  KEY_FP=$(gpg --list-secret-keys --with-colons "$PRIMARY_EMAIL" | awk -F: '/^fpr/{print $10}' | head -1)
  [ -n "$KEY_FP" ] || log_error "Key generation succeeded but fingerprint not found. Run: gpg -k $PRIMARY_EMAIL"
  log_success "Generated key fingerprint: $KEY_FP"
fi

# ── 4. Add second UID (Triune-Oracle email) ──────────────────────────────────
log_step "Adding second UID for Triune-Oracle"

EXISTING_UID=$(gpg --list-keys --with-colons "$KEY_FP" 2>/dev/null | grep "uid" | grep "$ORACLE_EMAIL" || true)
if [ -n "$EXISTING_UID" ]; then
  log_warn "UID $ORACLE_EMAIL already attached to key — skipping."
else
  gpg --batch --quick-add-uid "$KEY_FP" "$REAL_NAME <$ORACLE_EMAIL>"
  log_success "Added UID: $REAL_NAME <$ORACLE_EMAIL>"
fi

# ── 5. Set ultimate trust ────────────────────────────────────────────────────
log_step "Setting owner trust"
echo "$KEY_FP:6:" | gpg --import-ownertrust
log_success "Owner trust set to ultimate"

# ── 6. gpg-agent configuration ───────────────────────────────────────────────
log_step "Configuring gpg-agent"
GNUPGHOME="${GNUPGHOME:-$HOME/.gnupg}"
mkdir -p -m 700 "$GNUPGHOME"
AGENT_CONF="$GNUPGHOME/gpg-agent.conf"
touch "$AGENT_CONF"

if [ "$(uname -s)" = "Darwin" ]; then
  for PINENTRY_PATH in /opt/homebrew/bin/pinentry-mac /usr/local/bin/pinentry-mac; do
    if [ -f "$PINENTRY_PATH" ]; then
      grep -q "pinentry-program" "$AGENT_CONF" \
        || echo "pinentry-program $PINENTRY_PATH" >>"$AGENT_CONF"
      log_success "pinentry-mac configured: $PINENTRY_PATH"
      break
    fi
  done
fi

grep -q "default-cache-ttl" "$AGENT_CONF" || echo "default-cache-ttl 7200"  >>"$AGENT_CONF"
grep -q "max-cache-ttl"     "$AGENT_CONF" || echo "max-cache-ttl 28800"     >>"$AGENT_CONF"
gpg-connect-agent reloadagent /bye >/dev/null 2>&1 || true
log_success "gpg-agent configured (2h default cache, 8h max)"

# ── 7. Git global configuration ──────────────────────────────────────────────
log_step "Configuring git globals"

KEY_ID=$(gpg --list-secret-keys --keyid-format LONG "$PRIMARY_EMAIL" \
  | awk -F'[/ ]' '/^sec/{print $3}' | head -1)

git config --global user.signingkey   "$KEY_ID"
git config --global commit.gpgsign    true
git config --global tag.gpgsign       true
git config --global gpg.program       gpg

log_success "Signing key:   $KEY_ID"
log_success "commit.gpgsign true"
log_success "tag.gpgsign    true"

# ── 8. Per-repo override reminder ────────────────────────────────────────────
log_step "Per-repository override (for Triune-Oracle repos)"
echo ""
echo "  Global git config uses $PRIMARY_EMAIL."
echo "  For any local Triune-Oracle repo clone, run:"
echo ""
echo -e "    ${GREEN}git config --local user.email \"$ORACLE_EMAIL\"${NC}"
echo -e "    ${GREEN}git config --local user.signingkey \"$KEY_ID\"${NC}"
echo ""
echo "  GitHub recognises both UIDs on one key — verified badge appears on both accounts."

# ── 9. Export public key ─────────────────────────────────────────────────────
log_step "Exporting public key"
EXPORT_PATH="$HOME/tscp-gpg-public-key.asc"
gpg --armor --export "$KEY_FP" >"$EXPORT_PATH"
log_success "Public key exported to: $EXPORT_PATH"

# ── 10. Instructions ─────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  SETUP COMPLETE — follow these steps to activate signing  ${NC}"
echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
echo ""
echo "  1. Add the public key to BOTH GitHub accounts:"
echo "     GitHub → Settings → SSH and GPG keys → New GPG key"
echo ""
echo -e "     ${BOLD}Cartilage-Stairwells${NC} (adamantinespine@gmail.com)"
echo -e "     ${BOLD}Triune-Oracle${NC}         (schlagetorren@gmail.com)"
echo ""
echo "     Paste the contents of: $EXPORT_PATH"
echo ""
echo "  2. Test signing works:"
echo -e "     ${GREEN}echo 'test' | gpg --clearsign${NC}"
echo ""
echo "  3. SSH signing alternative (not implemented here):"
echo "     git config --global gpg.format ssh"
echo "     git config --global user.signingkey /path/to/id_ed25519.pub"
echo ""
echo -e "${BOLD}════════════════════════════════════════════════════════════${NC}"
echo ""
cat "$EXPORT_PATH"
