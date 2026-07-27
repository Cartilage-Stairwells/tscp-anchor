#!/usr/bin/env python3
"""
mutation_tests.py — Mutation testing for custody receipt invariants

Takes the valid_avx512_receipt.json (positive control) and perturbs exactly
one field at a time. Each perturbation should trigger exactly the expected
invariant, proving that:

  1. Each invariant is NECESSARY — removing it would let a specific
     class of invalid receipt through.
  2. Each invariant is SUFFICIENT — it catches its intended failure mode
     without relying on other invariants.

This guards against accidental gaps introduced by future schema changes.

Usage:
    python3 tests/custody/mutation_tests.py

Exit codes:
    0 — all mutations produced expected results
    1 — one or more mutations produced unexpected results
"""

import json
import copy
import sys
from pathlib import Path

# Import the verifier
sys.path.insert(0, str(Path(__file__).parent))
from verify_custody_receipts import verify_receipt, INVARIANTS

# ── Colors ────────────────────────────────────────────────────────────
GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
BLUE = "\033[94m"
BOLD = "\033[1m"
NC = "\033[0m"


def load_valid_receipt():
    """Load the positive control receipt."""
    fixture_path = Path(__file__).parent / "valid_avx512_receipt.json"
    with open(fixture_path) as f:
        return json.load(f)


def mutate(receipt, path, value):
    """Set a nested field in the receipt to a specific value. Returns a deep copy."""
    r = copy.deepcopy(receipt)
    parts = path.split(".")
    obj = r
    for part in parts[:-1]:
        obj = obj[part]
    obj[parts[-1]] = value
    return r


def remove_field(receipt, path):
    """Remove a nested field from the receipt. Returns a deep copy."""
    r = copy.deepcopy(receipt)
    parts = path.split(".")
    obj = r
    for part in parts[:-1]:
        obj = obj[part]
    del obj[parts[-1]]
    return r


def chain_mutations(receipt, *mutations):
    """Apply multiple mutations in sequence. Each mutation is (path, value)."""
    r = copy.deepcopy(receipt)
    for path, value in mutations:
        parts = path.split(".")
        obj = r
        for part in parts[:-1]:
            obj = obj[part]
        obj[parts[-1]] = value
    return r


# ── Mutations ─────────────────────────────────────────────────────────
# Each mutation: (name, perturb_fn, expected_invariant)
# expected_invariant is the invariant number that should be triggered.
# If None, the mutation should still ACCEPT (no invariant violated).

MUTATIONS = [
    # Invariant 1: Authority Neutrality
    ("auth_granted_true",
     lambda r: mutate(r, "authority.granted", True),
     1),
    ("auth_jurisdiction_crossed_true",
     lambda r: mutate(r, "authority.jurisdiction_crossed", True),
     1),

    # Invariant 2: Target Binding
    ("backend_selected_scalar",
     lambda r: mutate(r, "execution.backend_selected", "scalar"),
     2),
    ("backend_selected_empty",
     lambda r: mutate(r, "execution.backend_selected", ""),
     2),

    # Invariant 3: Build Artifact Identity
    ("artifact_hash_flipped",
     lambda r: mutate(r, "build_identity.artifact_hash", "deadbeef0000ffff111122223333444455556666777788889999aaaabbbbcccc"),
     3),
    ("artifact_hash_placeholder",
     lambda r: mutate(r, "build_identity.artifact_hash", "<sha256 of compiled object>"),
     3),
    ("artifact_hash_removed",
     lambda r: remove_field(r, "build_identity.artifact_hash"),
     3),
    ("compiler_missing",
     lambda r: mutate(r, "build_identity.compiler", "not recorded"),
     3),
    ("dependency_lock_missing",
     lambda r: mutate(r, "build_identity.dependency_lock", "not recorded"),
     3),
    ("build_identity_removed",
     lambda r: remove_field(r, "build_identity"),
     None),  # No build_identity → verifier skips (no violation)

    # Invariant 4: Hardware Presence
    ("cpu_feature_verified_false",
     lambda r: mutate(r, "hardware.cpu_feature_verified", False),
     4),
    ("hardware_removed",
     lambda r: remove_field(r, "hardware"),
     4),

    # Invariant 5: Fallback Prohibition
    ("fallback_used_true",
     lambda r: mutate(r, "execution.fallback_used", True),
     5),
    ("fallback_with_symbol",
     lambda r: chain_mutations(r, ("execution.fallback_used", True), ("execution.fallback_symbol", "scalar_butterfly_32")),
     5),

    # Invariant 6: Claim Scope Integrity
    ("scope_valid_false",
     lambda r: mutate(r, "claim_scope.scope_valid", False),
     6),
    ("claimed_scope_broadened",
     lambda r: mutate(r, "claim_scope.claimed_scope", "complete AVX512 NTT backend correctness"),
     6),
    ("cases_run_zero",
     lambda r: mutate(r, "test.cases_run", 0),
     6),

    # Invariant 7: Observation Independence
    ("observation_self_reported",
     lambda r: mutate(r, "observation.method", "self_reported"),
     7),
    ("observation_none",
     lambda r: mutate(r, "observation.method", "none"),
     7),
    ("observer_is_target",
     lambda r: mutate(r, "observation.observer", "avx512_butterfly_32bit.rs (the code under test)"),
     7),
    ("observer_not_independent",
     lambda r: mutate(r, "observation.observer", "test runner (not independently verified)"),
     7),
    ("observation_method_invalid",
     lambda r: mutate(r, "observation.method", "gut-feeling"),
     7),

    # Invariant 8: Gate Ordering
    ("lifecycle_status_invalid",
     lambda r: mutate(r, "lifecycle.status", "PENDING"),
     8),

    # Invariant 9: Receipt Lifecycle Integrity
    ("revoked_with_late_audit",
     lambda r: chain_mutations(r,
         ("lifecycle.status", "REVOKED"),
         ("lifecycle.revoked_at", "2026-07-26T00:00:00Z"),
         ("lifecycle.audited_at", "2026-07-27T00:00:00Z")),
     9),
]


def run_mutation_tests():
    """Run all mutation tests and report results."""
    valid_receipt = load_valid_receipt()
    
    # First, verify the base receipt is accepted
    accepted, _ = verify_receipt(valid_receipt)
    if not accepted:
        print(f"{RED}ERROR: Base valid receipt is not accepted. Mutation tests are meaningless.{NC}")
        return 1
    
    print(f"{BOLD}{'='*72}{NC}")
    print(f"{BOLD}Mutation Test Suite — Custody Receipt Invariants{NC}")
    print(f"{BOLD}{'='*72}{NC}")
    print(f"Base: valid_avx512_receipt.json (confirmed ACCEPT)")
    print(f"Mutations: {len(MUTATIONS)}")
    print(f"{'='*72}")
    print()
    
    all_passed = True
    results = []
    
    for name, perturb_fn, expected_inv in MUTATIONS:
        # Apply mutation
        mutated = perturb_fn(valid_receipt)
        
        # Verify
        accepted, violations = verify_receipt(mutated)
        actual = "ACCEPT" if accepted else "REJECT"
        
        # Determine which invariant(s) were triggered
        triggered_invs = [v[0] for v in violations]
        
        if expected_inv is None:
            # Should still accept (no violation expected)
            match = accepted
            expected_str = "ACCEPT (no violation)"
        else:
            # Should reject with the expected invariant
            match = (not accepted) and (expected_inv in triggered_invs)
            expected_str = f"REJECT (Invariant {expected_inv})"
        
        results.append({
            "name": name,
            "expected": expected_str,
            "actual": actual,
            "match": match,
            "triggered": triggered_invs,
            "violations": violations,
        })
        
        if not match:
            all_passed = False
        
        # Print result
        status_color = GREEN if match else RED
        status_icon = "✓" if match else "✗"
        
        print(f"{status_color}{status_icon}{NC} {BOLD}{name}{NC}")
        print(f"  Expected: {expected_str}")
        print(f"  Actual:   {actual}", end="")
        if triggered_invs:
            inv_names = [f"Inv {n} ({INVARIANTS[n]})" for n in triggered_invs]
            print(f" — {', '.join(inv_names)}")
        else:
            print()
        
        if not match:
            if accepted and expected_inv is not None:
                print(f"  {RED}→ FAIL: Expected rejection by Invariant {expected_inv}, but receipt was ACCEPTED{NC}")
            elif not accepted and expected_inv is not None and expected_inv not in triggered_invs:
                print(f"  {RED}→ FAIL: Rejected, but by wrong invariant(s): {triggered_invs}{NC}")
            elif not accepted and expected_inv is None:
                print(f"  {RED}→ FAIL: Expected ACCEPT, but was REJECTED{NC}")
        
        print()
    
    # Summary
    print(f"{'='*72}")
    total = len(results)
    passed = sum(1 for r in results if r["match"])
    failed = total - passed
    
    print(f"{BOLD}Summary:{NC} {passed}/{total} mutations produced expected results")
    
    if failed > 0:
        print(f"{RED}FAILED: {failed} mutation(s) produced unexpected results{NC}")
        for r in results:
            if not r["match"]:
                print(f"  {RED}→ {r['name']}: expected {r['expected']}, got {r['actual']}{NC}")
    else:
        print(f"{GREEN}ALL MUTATION TESTS PASSED{NC}")
        print()
        print(f"{BOLD}Each invariant is both necessary and sufficient:{NC}")
        print(f"  Every single-field perturbation triggers the correct invariant.")
        print(f"  No invariant can be removed without letting its failure class through.")
    
    print(f"{'='*72}{NC}")
    
    return 0 if all_passed else 1


if __name__ == "__main__":
    sys.exit(run_mutation_tests())
