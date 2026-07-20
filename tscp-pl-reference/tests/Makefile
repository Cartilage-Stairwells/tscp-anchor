# Makefile for TSCP-PL verifier test vectors
#
# Run from repository root:  make -C tests all
# Or from tests/ directory: make all
#
# Test invariants:
#   1. Each vector produces its expected exit code
#   2. good.jsonld and reorderkeys.jsonld produce identical canonical digests
#   3. Repeated verification of good.jsonld produces identical manifests (determinism)
#
# A mismatch in any of these is a regression.

PY = python3
VERIFIER = ../verifier/tscp_verify.py
LEDGER_DIR = ledgers
ART_DIR = artifacts

.PHONY: all setup generate verify clean determinism canonicalization

all: setup generate verify canonicalization determinism

setup:
	@echo "Installing python deps (jsonschema)..."
	$(PY) -m pip install --user jsonschema >/dev/null || true
	@echo "Setup complete."

generate:
	@echo "Generating test artifacts and ledgers..."
	$(PY) generatetestvectors.py

# expect_exit <file> <expected_code>: runs verifier, asserts exit code matches
define expect_exit
	@printf "  %-30s expected=%s " "$(notdir $(1))" "$(2)"
	@$(PY) $(VERIFIER) verify "$(1)" --quiet 2>/dev/null; RC=$$?; \
	if [ $$RC -eq $(2) ]; then \
		printf "actual=%s  %s\n" "$$RC" "PASS"; \
	else \
		printf "actual=%s  %s\n" "$$RC" "FAIL"; \
		exit 1; \
	fi
endef

verify: generate
	@echo ""
	@echo "=== Stage 1: Expected exit code verification ==="
	@echo "--- PASS cases (expect exit 0) ---"
	$(call expect_exit,$(LEDGER_DIR)/good.jsonld,0)
	$(call expect_exit,$(LEDGER_DIR)/reorderkeys.jsonld,0)
	@echo "--- EXPECTED FAIL cases (expect exit 3) ---"
	$(call expect_exit,$(LEDGER_DIR)/mutated.jsonld,3)
	$(call expect_exit,$(LEDGER_DIR)/missingartifact.jsonld,3)
	$(call expect_exit,$(LEDGER_DIR)/bad_hash.jsonld,3)
	@echo "  All test vectors produced expected exit codes ✅"
	@echo ""

# Canonicalization: good.jsonld and reorderkeys.jsonld must produce
# identical canonical digests despite different key order
canonicalization: verify
	@echo "=== Stage 2: Canonicalization invariance ==="
	@D1=$$($(PY) $(VERIFIER) digest "$(LEDGER_DIR)/good.jsonld"); \
	D2=$$($(PY) $(VERIFIER) digest "$(LEDGER_DIR)/reorderkeys.jsonld"); \
	printf "  good digest:         %s\n  reorderkeys digest: %s\n" "$$D1" "$$D2"; \
	if [ "$$D1" = "$$D2" ]; then \
		echo "  Canonicalization invariance: PASS ✅ (digests identical)"; \
	else \
		echo "  Canonicalization invariance: FAIL ❌ (digests differ)"; \
		exit 1; \
	fi
	@echo ""

# Determinism: verify good.jsonld twice, compare manifests
# The timestamp can differ. Everything else should remain identical.
determinism: canonicalization
	@echo "=== Stage 3: Determinism regression ==="
	@$(PY) $(VERIFIER) verify "$(LEDGER_DIR)/good.jsonld" --quiet 2>/dev/null; \
	cp ../evidence/manifest.json /tmp/manifest1.json
	@$(PY) $(VERIFIER) verify "$(LEDGER_DIR)/good.jsonld" --quiet 2>/dev/null; \
	cp ../evidence/manifest.json /tmp/manifest2.json
	@$(PY) determinism_check.py
	@echo ""
	@echo "=== All regression stages passed ==="
	@rm -f /tmp/manifest1.json /tmp/manifest2.json

clean:
	rm -rf $(ART_DIR) $(LEDGER_DIR) ../evidence ../tscp-pl.payload /tmp/manifest1.json /tmp/manifest2.json || true
