# Formal Verification Release Note — Montgomery Bézout Identity

**Date:** 2026-07-27
**Commit:** 82bfd1f8 (original), this follow-up commit (documentation)
**File:** `TSCP/Formal/Montgomery.lean`

## Summary

The Montgomery arithmetic formalization was strengthened by deriving both
modular inverse properties from a single Bézout identity, rather than
proving them independently.

## Background

The original proof structure had two independent `by decide` proofs:

```lean
theorem R_inv_correct : (R * R_inv) % P = 1 := by decide
theorem neg_inv_correct : (P * NEG_INV + 1) % R = 0 := by decide
```

These are computationally verified but do not expose *why* they are true
or how the constants relate to each other.

## The Bézout Identity

The new structure introduces a root identity:

```lean
theorem bezout_identity : R * R_inv = P * NEG_INV + 1 := by decide
```

From this single fact, both inverse properties are derived:

- **Semantic inverse** (taking mod P):
  `(R * R_inv) % P = (P * NEG_INV + 1) % P = 1`
  → `R_inv_correct: R * R_inv ≡ 1 (mod P)`

- **REDC inverse** (taking mod R):
  `(P * NEG_INV + 1) % R = 0`
  → `neg_inv_correct: P * NEG_INV ≡ -1 (mod R)`

## Why This Matters

1. **Proof engineering quality:** One authoritative arithmetic fact instead
   of three independent computational checks. A reviewer can see the
   mathematical structure, not just that Lean says "yes."

2. **Constant duality:** The Bézout coefficient is `K = -NEG_INV = -(P - 2)`,
   meaning `R_inv` and `NEG_INV` are not independent — they are dual inverses
   in complementary rings (mod P and mod R respectively).

3. **Reduced duplication:** The `rw [bezout_identity]` tactic in both derived
   theorems makes the dependency explicit. A change to one constant
   propagates through the identity to both proofs.

## Derivation (Extended Euclidean Algorithm)

```
R = 2·P + 268435454
P = 7·268435454 + 134217743
268435454 = 1·134217743 + 134217711
... (full EEA back-substitution yields R_inv = 943718400)
```

## Verification

- `bezout_identity` proven by `decide` (computational)
- `R_inv_correct` derived via `rw [bezout_identity]; decide`
- `neg_inv_correct` derived via `rw [← bezout_identity]; decide`
- 0 `sorry` in the file
- 0 new axioms

## Canonical Repository

This work lives on `Cartilage-Stairwells/tscp-anchor` (master branch).
This is the canonical public artifact for all TSCP formal verification.

The `Triune-Oracle/tscp-anchor` repository is the historical predecessor
and does not contain the formal verification backbone.
