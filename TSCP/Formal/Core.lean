/-
  TSCP Formal — Core.lean v2
  BabyBear instantiation of the TSCP Formal Backbone.

  Note: Uses True as the NTT kernel admissibility predicate (the actual
  field element checking is done by the proof_valid theorem). The rest
  of the formal layer (Backbone through ManifestBinding) is axiom-free.

  v2.1: proof_valid discharged from axiom to theorem (rfl). The predicate
  babybear_kernel.admits_proof is defined as babybear_valid, which is
  x.val < P — so the statement holds by definitional equality. This
  corresponds to canonicalRule_babybear in BabyBear/Element.lean.

  v2.2: Formal axiom audit completed. See PART 7 for the audit record.

  v2.3 (2026-10-12, issue #61 Stage 4, ruling B): zero axioms.
  execution_valid converted from axiom to explicit certificate
  hypothesis, threaded through its two dependents; the corpus
  carries the machine-checked sufficiency theorem
  (surjectivity of ntt_map constructs the full certificate).
  babybear_ntt_end_to_end discharged from axiom to theorem
  (trivial: both kernels admit everything; see PART 5 note).
  The surjectivity assumption itself is NOT proven — it is
  relocated, named, and single-purpose. Binding it to the
  AVX-512 kernel is Level C work (see PART 4).
-/

import TSCP.Formal.TSCP_Formal_Backbone
import TSCP.Formal.BridgePreservation
import TSCP.Formal.Evidence.ManifestBinding

namespace TSCP.Formal.Core

open TSCP TSCP.Formal TSCP.Formal.Evidence

/- ===================================================================
   PART 1: BABYBEAR FIELD
   =================================================================== -/

def P : Nat := 0x78000001
def BabyBearElem : Type := { n : Nat // n < P }

def babybear_valid (x : BabyBearElem) : Prop := x.val < P

instance babybear_valid_decidable (x : BabyBearElem) :
    Decidable (babybear_valid x) := by
  show Decidable (x.val < P)
  infer_instance

def babybear_kernel : Kernel BabyBearElem where
  admits_proof := babybear_valid
  admits_decidable := babybear_valid_decidable
  nonempty := by
    refine ⟨⟨0, ?_⟩, ?_⟩
    · decide
    · unfold babybear_valid; decide

/- ===================================================================
   PART 2: THEOREM — proof_valid (discharged from axiom)

   Previous:
     axiom proof_valid : ∀ (x : BabyBearElem),
         babybear_kernel.admits_proof x ↔ x.val < P

   Now:
     theorem proof_valid — same statement, proved by rfl.

   babybear_kernel.admits_proof is defined as babybear_valid, which is
   x.val < P. The statement "admits_proof x ↔ x.val < P" is therefore
   true by definitional unfolding. No new proof burden — the proof
   already existed; the axiom was a placeholder for a trivial theorem.

   Cross-reference: BabyBear/Element.lean defines canonicalRule as
   x.val < BABYBEAR_P and proves canonicalRule_babybear by rfl with
   the same structure. Both express the same invariant: an encoded
   field element is canonical iff its raw representative is < P.
   =================================================================== -/

theorem proof_valid : ∀ (x : BabyBearElem),
    babybear_kernel.admits_proof x ↔ x.val < P := by
  intro x
  rfl

/- ===================================================================
   PART 3: NTT ENCODING MAP
   =================================================================== -/

def BabyBearVec (n : Nat) : Type := Fin n → BabyBearElem

instance inhabitedBabyBearVec (n : Nat) : Inhabited (BabyBearVec n) :=
  ⟨fun _ => ⟨0, by decide⟩⟩

noncomputable opaque ntt_map (n : Nat) : BabyBearVec n → BabyBearVec n

noncomputable def ntt_universe (n : Nat) : Universe where
  ProofType := BabyBearVec n
  FormulaType := Nat
  ExecutionType := BabyBearVec n
  proof_kernel := {
    admits_proof := fun _ => True
    admits_decidable := fun _ => inferInstance
    nonempty := ⟨default, trivial⟩
  }
  formula_kernel := {
    admits_proof := fun k => k = n
    admits_decidable := fun k => inferInstance
    nonempty := ⟨n, rfl⟩
  }
  exec_kernel := {
    admits_proof := fun _ => True
    admits_decidable := fun _ => inferInstance
    nonempty := ⟨default, trivial⟩
  }

noncomputable def ntt_bridge (n : Nat) : Bridge (ntt_universe n) (ntt_universe n) where
  proof_map := ntt_map n
  formula_map := id
  exec_map := ntt_map n

/- ===================================================================
   PART 4: AVX-512 EXECUTION BRIDGE

   v2.2: this was `noncomputable axiom execution_valid (n : Nat) :
   BridgeCertificate (ntt_bridge n)` — a Category-2 engineering
   boundary (evidence: AVX-512 butterfly kernel + test suite).

   v2.3 disposition (issue #61 Stage 4, ruling B): the axiom is
   REMOVED. Its dependents now take the certificate as an explicit
   hypothesis. The axiom's entire mathematical content was surjectivity
   of the opaque `ntt_map` (every other certificate field is trivial
   under ntt_universe's kernels), so the corpus carries that reduction
   as a machine-checked sufficiency theorem below. Surjectivity itself
   is NOT proven: ntt_map is opaque with no in-corpus definition, so
   its proof is Level C work (implementation correspondence). Until
   then, every use of the bridge states its assumption explicitly.

   Closure path for the assumption: Level C — bind ntt_map to the
   AVX-512 kernel and prove surjectivity, then supply the hypothesis.
   =================================================================== -/

/-- SUFFICIENCY (machine-checked): if the opaque `ntt_map` is
    surjective on `BabyBearVec n`, the FULL `BridgeCertificate
    (ntt_bridge n)` can be constructed — all seven fields including
    exec reflection and the custody metadata. Injectivity is not
    required. The remaining fields are trivial: both proof and exec
    kernels admit everything, and formula_map is the identity on
    `k = n`. -/
noncomputable def execution_bridge_certificate_of_surjectivity (n : Nat)
    (hsurj : ∀ q : BabyBearVec n, ∃ p : BabyBearVec n, ntt_map n p = q) :
    TSCP.BridgeCertificate (ntt_bridge n) := {
  proof_preservation := fun _p _h => trivial,
  proof_reflection := fun q _hq => (hsurj q).imp fun p hp => ⟨trivial, hp⟩,
  proof_admissibility := {
    preserves := fun _p _h => trivial,
    reflects := fun q _hq => (hsurj q).imp fun p hp => ⟨trivial, hp⟩
  },
  formula_admissibility := {
    preserves := fun _k hk => hk
    reflects := fun k hk => ⟨k, hk, rfl⟩
  },
  exec_admissibility := {
    preserves := fun _p _h => trivial,
    reflects := fun q _hq => (hsurj q).imp fun p hp => ⟨trivial, hp⟩
  },
  artifact := {
    theorem_name := "TSCP.Formal.Core.execution_bridge_certificate_of_surjectivity",
    digest := "v2.3-surjectivity-sufficiency-machine-checked",
    verifier_version := "lean4-tscp-formal-backbone-v1.0",
    proof_serialization := "hsurj:hypothesis|certificate:constructed"
  },
  certificate_digest := "v2.3-conditional-bridge-certificate",
  verifier_version := "lean4-tscp-formal-backbone-v1.0",
  issued_at := "2026-10-12"
}

/- ===================================================================
   PART 5: DERIVED THEOREMS
   =================================================================== -/

/-- v2.3: takes the certificate explicitly (was `execution_valid n`). -/
noncomputable def ntt_bridge_conservative (n : Nat)
    (hcert : TSCP.BridgeCertificate (ntt_bridge n)) :
    ConservativeBridge (ntt_bridge n) :=
  hcert

/-- v2.3: takes the certificate explicitly (was `execution_valid n`). -/
theorem ntt_preserves_admissibility (n : Nat)
    (hcert : TSCP.BridgeCertificate (ntt_bridge n))
    (v : CertifiedProof (ntt_universe n).proof_kernel) :
    Truth (ntt_universe n) ((ntt_bridge n).proof_map v.proof) :=
  bridge_preserves_certified_truth
    (ntt_bridge n)
    hcert
    v

/- Axiom #2 (v2.2): babybear_ntt_end_to_end
   Type: noncomputable axiom
   Statement: NTT forward transform preserves admissibility end-to-end
   Category: Explicit engineering boundary (Category 2)
   Evidence: NTT round-trip test vectors (forward → inverse → compare)
   Closure path: Path B — attach reproducible test vector artifacts

   v2.3 disposition (issue #61 Stage 4, ruling B): DISCHARGED to a
   theorem. Under ntt_universe's kernels the statement is `True → True`
   (the proof kernel admits everything, and the exec kernel admits
   everything), so it is provable by `trivial` — the axiom had no
   kernel-level content. This discharge does NOT verify the NTT
   round-trip: the round-trip evidence lives at the implementation
   level (Level C), and the formal statement was vacuous with respect
   to it. Statement and binder types are unchanged from the axiom;
   only the declaration kind changed.
-/
theorem babybear_ntt_end_to_end (n : Nat)
    (v : BabyBearVec n)
    (_h : (ntt_universe n).proof_kernel.admits_proof v) :
    (ntt_universe n).exec_kernel.admits_proof (ntt_map n v) :=
  trivial

/- ===================================================================
   PART 6: AXIOM BINDING RECORD
   =================================================================== -/

def babybear_binding_artifact : ProofArtifact where
  theorem_name := "TSCP.Formal.Core.BabyBear.v2.3"
  digest := "core-lean-v2.3-babybear-instantiation-zero-axioms"
  verifier_version := "lean4-tscp-formal-backbone-v1.0"
  proof_serialization := "proof_valid:THEOREM(rfl)|execution_bridge_certificate_of_surjectivity:THEOREM(sufficiency)|babybear_ntt_end_to_end:THEOREM(trivial,v2.3)"

/- ===================================================================
   PART 7: FORMAL AXIOM AUDIT RECORD

   Audit date: 2026-07-23
   Auditor: Miro (TSCP Agent)
   Method: grep-based scan for `axiom`, `sorry`, `admit` in Core.lean

   Results:
     Axioms:     2 (execution_valid, babybear_ntt_end_to_end)
     Theorems:   2 (proof_valid [discharged], ntt_preserves_admissibility [derived])
     Sorries:   0
     Opaque:     1 (ntt_map — noncomputable, not an axiom)
     Admits:     0 (admits_proof/admits_decidable are field accesses, not axioms)

   Axiom inventory:
     #1 execution_valid (line ~107)
        Category 2 — engineering boundary
        Evidence: AVX-512 execution bridge
        Closure: Path B (reproducible artifacts)

     #2 babybear_ntt_end_to_end (line ~126)
        Category 2 — integration boundary
        Evidence: NTT round-trip test vectors
        Closure: Path B (reproducible test artifacts)

   Discharged (no longer axioms):
     proof_valid — v2.1, discharged via rfl
       babybear_kernel.admits_proof is definitionally x.val < P

   Cross-repo axiom total (not in Core.lean):
     babybear_verifier_injective — Verifier.lean, Category 2, FRI binding

   Audit conclusion:
     Core.lean contains exactly 2 axioms, both Category 2 (engineering
     boundaries with real evidence). No Category 1 (replaceable) axioms
     remain. No sorries. No hidden assumptions. The axiom count is final
     pending either Path A (FRI formalization) or Path B (evidence binding).
   =================================================================== -/

end TSCP.Formal.Core
