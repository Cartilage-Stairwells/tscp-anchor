/-
  BabyBear Verifier — Binding Property Specification

  This file replaces the opaque placeholder

    axiom babybear_verifier_injective : True

  with a precise statement of the binding (injectivity) property.

  This is a COMPLETED SPECIFICATION OBLIGATION — not a proof obligation.
  The theorem is stated as an axiom with the exact proposition, matching the
  recommended intermediate step:

    axiom with exact proposition → (later) Lean proof from FRI binding theorem

  We do NOT force a fake proof. The honesty boundary is preserved: the axiom
  now says exactly what binding means, even though the proof is pending
  formal FRI development.
-/

namespace BabyBear.Verifier

open BabyBear

/-
  Witness / proof objects.

  These are abstracted here because the concrete RISC Zero proof structure
  (trace commitments, FRI layers, grinding, etc.) is not yet formalized in
  Lean. The types are opaque so that the binding statement is independent of
  implementation details — what matters is the *relationship*.
-/

/-- A complete proof object containing witness data and FRI layers. -/
structure ProofObject where
  witness  : List Element      -- witness trace (encoded field elements)
  friProof : List Element      -- FRI folding layers
  deriving DecidableEq

/-- The verifier's view: commitments derived from the proof via transcript. -/
structure VerifierCommitment where
  traceCommitment : Nat         -- Merkle root / hash commitment to the trace
  friCommitment   : Nat         -- commitment to FRI layers
  transcript      : List Nat    -- Fiat-Shamir transcript challenges
  deriving DecidableEq

/-
  The verifier function.

  Maps a proof object to the verifier's commitment/transcript view. In the
  real system this involves:
    1. Hashing the witness trace → traceCommitment
    2. Running Fiat-Shamir to derive transcript challenges
    3. Executing FRI verification → friCommitment

  Here it is opaque — we only need that it is a *function* from proof
  objects to verifier commitments.
-/
opaque verifier : ProofObject → VerifierCommitment

/-
  Equivalent-witness relation.

  Two proof objects are "equivalent" if they represent the same underlying
  computational claim. This is weaker than syntactic equality — two proofs
  may differ in their FRI layer representations while encoding the same
  witness trace and the same satisfiability claim.

  This is the equality relation being preserved by the binding property.
-/
def equivalentWitness (a b : ProofObject) : Prop :=
  a.witness = b.witness

/-
  babybear_verifier_injective — the binding property.

  Statement:

    ∀ a b,
      verifier a = verifier b →
      equivalentWitness a b

  This says: if two proof objects produce the same verifier commitment
  (same trace commitment, same FRI commitment, same transcript), then their
  witnesses are equivalent. This is the binding / soundness property —
  you cannot produce two genuinely different witnesses that verify to the
  same commitment.

  Status: axiom (specification obligation completed, proof pending).

  The previous placeholder `axiom babybear_verifier_injective : True`
  carried no information. This axiom carries the *exact* proposition.

  Upgrade path:
    axiom (this file)  →  sorry-assisted theorem  →  proof from FRI binding theorem
-/
axiom babybear_verifier_injective :
  ∀ a b : ProofObject,
    verifier a = verifier b →
    equivalentWitness a b

/-
  Closure status for this file:

    Verifier:
      ✓ binding property statement formalized
      △ proof pending formal FRI development
-/

end BabyBear.Verifier
