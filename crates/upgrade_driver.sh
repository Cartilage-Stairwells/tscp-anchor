#!/usr/bin/env bash
set -euo pipefail

# --- Configuration ---
CRATE_DIR="oracle-layer"
CARGO_TOML="$CRATE_DIR/Cargo.toml"
BACKUP_DIR=".plonky3_upgrade_backup"
TARGET_VERSION="0.6.0"

log_info() { echo -e "\033[32m[INFO]\033[0m $1"; }
log_warn() { echo -e "\033[33m[WARN]\033[0m $1"; }
log_error() { echo -e "\033[31m[ERROR]\033[0m $1"; exit 1; }

# --- Verification & Pre-flight ---
if [ ! -d "$CRATE_DIR" ] || [ ! -f "$CARGO_TOML" ]; then
    log_error "Target crate directory or $CARGO_TOML not found from current path ($PWD)."
fi

mkdir -p "$BACKUP_DIR"

case "${1:-}" in
    phase1)
        log_info "Starting Phase 1: Environment Preparation & Dependency Alignment"
        if git diff --quiet HEAD -- "$CARGO_TOML" 2>/dev/null; then
            cp "$CARGO_TOML" "$BACKUP_DIR/Cargo.toml.bak"
        fi
        log_info "Updating Plonky3 dependencies to version $TARGET_VERSION in $CARGO_TOML..."
        sed -i.tmp -E "s/(p3-[a-zA-Z0-9_-]*) *= *\"[^\"]*\"/\1 = \"$TARGET_VERSION\"/g" "$CARGO_TOML"
        rm -f "${CARGO_TOML}.tmp"
        log_info "Executing local workspace lock alignment..."
        cargo update
        log_info "Phase 1 Complete. Verify dependency tree changes via git diff."
        ;;
    phase2)
        log_info "Starting Phase 2: Monolithic Workspace Isolation & Stubbing"
        log_info "Isolating $CRATE_DIR compilation context..."
        if cargo check -p "$CRATE_DIR" > "$BACKUP_DIR/compiler_errors.log" 2>&1; then
            log_info "Crate already compiles smoothly with Plonky3 $TARGET_VERSION. Skipping stub generation."
            exit 0
        fi
        log_warn "Breaking API surfaces encountered. Extracting targets for isolation..."
        broken_files=$(grep -E "^error\[E[0-9]+\]" -B 1 "$BACKUP_DIR/compiler_errors.log" | grep "src/" | awk -F: '{print $1}' | sort -u)
        if [ -z "$broken_files" ]; then
            log_warn "Could not programmatically isolate error signatures. Checking full log file: $BACKUP_DIR/compiler_errors.log"
        else
            for file in $broken_files; do
                log_info "Backing up and preparing isolation stub for: $file"
                cp "$file" "$BACKUP_DIR/$(basename "$file").bak"
                echo "// PLONKY3_UPGRADE_STUB_REQUIRED: $file" >> "$BACKUP_DIR/stub_registry.txt"
            done
        fi
        log_info "Phase 2 Complete. Baseline isolation complete. Errors cached in $BACKUP_DIR/compiler_errors.log"
        ;;
    phase3)
        log_info "Starting Phase 3: Component-by-Component API Refactoring"
        if [ ! -f "$BACKUP_DIR/compiler_errors.log" ]; then
            log_error "Compiler error baseline log missing. Run 'phase2' first."
        fi
        log_info "Entering localized compilation loop. Fixing core modules..."
        while ! cargo check -p "$CRATE_DIR" --lib; do
            log_warn "Compilation step failed. Resolving breaking traits..."
            echo "--------------------------------------------------------"
            cargo check -p "$CRATE_DIR" --lib 2>&1 | head -n 25
            echo "--------------------------------------------------------"
            log_info "Resolve the uppermost trait or type mismatch error above."
            log_info "Press [ENTER] to re-verify the module, or type 'exit' to pause execution."
            read -r user_input
            if [ "$user_input" = "exit" ]; then
                log_warn "Refactoring cycle suspended by operator."
                exit 0
            fi
        done
        log_info "Phase 3 Complete: Core library API surface aligned and compiling under Plonky3 $TARGET_VERSION."
        ;;
    rollback)
        log_warn "Initiating localized state rollback..."
        if [ -f "$BACKUP_DIR/Cargo.toml.bak" ]; then
            mv "$BACKUP_DIR/Cargo.toml.bak" "$CARGO_TOML"
            log_info "Restored original Cargo.toml"
        fi
        rm -rf "$BACKUP_DIR"
        cargo update
        log_info "Rollback execution finished."
        ;;
    *)
        echo "Usage: $0 {phase1|phase2|phase3|rollback}"
        exit 1
        ;;
esac
