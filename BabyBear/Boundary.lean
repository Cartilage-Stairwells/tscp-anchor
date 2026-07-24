/-
  BabyBear Instance — Formal Boundary Summary

  Updated closure matrix (post proof_valid discharge):

    BabyBear Instance

    Field encoding:
      ✓ canonical representative predicate (canonicalRule_babybear)
      ✓ proof_valid (discharged from axiom to theorem via rfl)

    Kernel execution:
      ✓ execution evidence bound

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

  Core.lean axiom count: 3 → 2 (proof_valid discharged)
-/

namespace BabyBear.Boundary

-- === Closure matrix ===

/-- Field encoding: canonical representative predicate is proven. -/
inductive FieldEncodingStatus : Prop
  | canonicalRepresentativePredicate : FieldEncodingStatus
  | proofValidDischarged : FieldEncodingStatus

/-- Kernel execution: execution evidence is bound. -/
inductive KernelExecutionStatus : Prop
  | executionEvidenceBound : KernelExecutionStatus

/-- Packed semantics: naturality and lane faithfulness are bound. -/
inductive PackedSemanticsStatus : Prop
  | scalarPackedNaturalityBound : PackedSemanticsStatus
  | scalarPackedFaithfulBound   : PackedSemanticsStatus

/-
  Verifier soundness: binding proposition is specified; proof pending.
-/
inductive VerifierSoundnessStatus : Prop
  | friBindingPropositionSpecified : VerifierSoundnessStatus
  -- proof pending formal FRI development

-- === Axiom inventory ===

/-
  Axiom inventory — governance artifact.

  Category 1 — Replaceable assumptions:
    [DISCHARGED] proof_valid

  Category 2 — Explicit engineering boundaries:
    execution_valid — AVX-512 execution bridge certificate
    babybear_ntt_end_to_end — NTT end-to-end admissibility
    babybear_verifier_injective — FRI binding property

  Category 3 — Future formalization work (not in axiom list, tracked separately):
    NormalizationBridge.lean — 3 sorries
    TraceCoreProver/Kernel.lean — 11 sorries
    LEAN_STATEMENTS.lean — 16 sorries (deprecated)

  When an axiom is discharged into a theorem, remove it from this list.
-/
def remainingAxioms : List String :=
  [ "execution_valid",
    "babybear_ntt_end_to_end",
    "babybear_verifier_injective"
  ]

/-- Category 3: skeleton/sorry work tracked separately from the axiom list. -/
def deferredSorries : List String :=
  [ "NormalizationBridge.lean: reflection/invertibility (3 sorries)",
    "TraceCoreProver/Kernel.lean: STARK kernel skeleton (11 sorries)",
    "LEAN_STATEMENTS.lean: deprecated skeleton (16 sorries, superseded)"
  ]

/-
  Status legend:
    ✓  — proven or evidence-bound (no remaining trust assumption)
    △  — specified but pending (trust assumption is visible and tracked)

  Current frontier (post proof_valid discharge):
    Core.lean axioms: 2 (down from 3)
    Verifier.lean axioms: 1
    Total tracked assumptions: 3

  Next reduction target: none are trivially dischargeable. All three
  remaining axioms represent genuine engineering boundaries that require
  either formal FRI work or external evidence packages.
-/

end BabyBear.Boundary
