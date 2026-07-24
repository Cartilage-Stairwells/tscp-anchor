/-
  BabyBear Instance — Formal Boundary Summary

  Updated closure matrix (post proof_valid discharge + axiom audit):

    BabyBear Instance

    Field encoding:
      ✓ canonical representative predicate (canonicalRule_babybear)
      ✓ proof_valid (discharged from axiom to theorem via rfl)

    Kernel execution:
      △ execution_valid (evidence boundary, Path B target)

    Packed semantics:
      ✓ scalar_packed_naturality evidence bound
      ✓ scalar_packed_faithful evidence bound

    Verifier soundness:
      △ FRI binding proposition specified (babybear_verifier_injective)
      △ proof pending formal FRI development

  Trust surface across all repos (categorized):

    Category 1 — Replaceable assumptions (already-proven facts exist elsewhere):
      [DISCHARGED] proof_valid — replaced with rfl theorem in Core.lean

    Category 2 — Explicit engineering boundaries (real evidence, formal proof pending):
      △ execution_valid — AVX-512 execution bridge certificate (Core.lean)
      △ babybear_ntt_end_to_end — NTT end-to-end admissibility (Core.lean)
      △ babybear_verifier_injective — FRI binding property (Verifier.lean)

    Category 3 — Future formalization work (sorries / skeleton modules):
      △ NormalizationBridge.lean — 3 sorries (reflection/invertibility)
      △ TraceCoreProver/Kernel.lean — 11 sorries (STARK kernel skeleton)
      △ LEAN_STATEMENTS.lean — 16 sorries (DEPRECATED, superseded by BabyBearVerified.lean)

  Core.lean axiom count: 3 → 2 (proof_valid discharged, audit complete)
-/

namespace BabyBear.Boundary

-- === Trust boundary metadata ===

/-- Status of a trust boundary item. -/
inductive Status
  | proven       -- discharged into a Lean theorem
  | evidenceBound -- backed by external evidence, formal proof pending
  | specified    -- exact proposition stated, proof pending
  | deferred     -- skeleton/sorry work, not part of verified backbone
  | discharged  -- was an axiom, now a theorem

/-- A single trust boundary entry with metadata.

    This structure evolves the ledger from a flat list of axiom names
    (remainingAxioms) into a richer trust surface (remainingTrustSurface)
    because not every trust boundary is an axiom. Some are implementation
    claims backed by artifacts, some are specification obligations, and
    some are deferred skeleton work.

    Fields:
      name        — the axiom/theorem/sorry identifier
      status      — current state (proven, evidenceBound, specified, deferred, discharged)
      evidence    — what external evidence supports this (if any)
      closurePath — how this boundary can be closed (Path A, Path B, or N/A)
-/
structure TrustBoundary where
  name        : String
  status      : Status
  evidence    : String
  closurePath : String

/-- The complete trust surface — replaces the flat axiom list.

    Each entry carries enough metadata that a reviewer can understand not
    just *what* is assumed, but *why* it is assumed and *how* it can be
    discharged. This is the canonical trust ledger.
-/
def remainingTrustSurface : List TrustBoundary :=
  [ { name := "execution_valid"
      status := Status.evidenceBound
      evidence := "AVX-512 butterfly kernel implementation + Commit 3C test suite"
      closurePath := "Path B: attach reproducible build artifact + test digest" }
  , { name := "babybear_ntt_end_to_end"
      status := Status.evidenceBound
      evidence := "NTT round-trip test vectors (forward → inverse → compare)"
      closurePath := "Path B: attach test vector artifacts + input/output digests" }
  , { name := "babybear_verifier_injective"
      status := Status.specified
      evidence := "FRI binding proposition stated with exact types (Verifier.lean)"
      closurePath := "Path A: formalize FRI binding theorem → discharge axiom" }
  ]

/-- Discharged items — no longer part of the trust surface. -/
def dischargedBoundaries : List TrustBoundary :=
  [ { name := "proof_valid"
      status := Status.discharged
      evidence := " rfl: babybear_kernel.admits_proof ≡ x.val < P by definition"
      closurePath := "Closed: axiom → theorem (Core.lean v2.1)" }
  , { name := "canonicalRule_babybear"
      status := Status.proven
      evidence := "rfl: canonicalRule x ≡ x.val < BABYBEAR_P by definition"
      closurePath := "Closed: True placeholder → field predicate (Element.lean)" }
  ]

/-- Category 3: deferred skeleton/sorry work, tracked separately. -/
def deferredBoundaries : List TrustBoundary :=
  [ { name := "NormalizationBridge.lean"
      status := Status.deferred
      evidence := "3 sorries (reflection/invertibility gaps)"
      closurePath := "Future: formalize reflection/invertibility proofs" }
  , { name := "TraceCoreProver/Kernel.lean"
      status := Status.deferred
      evidence := "11 sorries (STARK kernel skeleton)"
      closurePath := "Future: complete STARK kernel implementation" }
  , { name := "LEAN_STATEMENTS.lean"
      status := Status.deferred
      evidence := "16 sorries (DEPRECATED, superseded by BabyBearVerified.lean)"
      closurePath := "N/A: deprecated, do not extend" }
  ]

-- === Legacy list (kept for backward compatibility) ===

/-
  The flat axiom list is retained for simple grep/audit purposes.
  Use remainingTrustSurface for the full governance view.
-/
def remainingAxioms : List String :=
  [ "execution_valid",
    "babybear_ntt_end_to_end",
    "babybear_verifier_injective"
  ]

/-
  Status legend:
    ✓  — proven or evidence-bound (no remaining trust assumption)
    △  — specified but pending (trust assumption is visible and tracked)

  Current frontier (post proof_valid discharge + axiom audit):
    Core.lean axioms: 2 (down from 3, audited 2026-07-23)
    Verifier.lean axioms: 1
    Total tracked assumptions: 3
    Discharged: 2 (proof_valid, canonicalRule_babybear)
    Deferred: 3 files (30 sorries total, not part of verified backbone)

  Next phase: Path B — attach operational evidence to execution_valid
  and babybear_ntt_end_to_end. This does not reduce axiom count but
  makes the boundaries auditable.
-/

end BabyBear.Boundary
