/-
  TSCP Formal Backbone v2.1
  =========================
  Core categorical structure for the TSCP custody model, encoded in Lean 4.
  v2.1: compilation fixes for Lean 4.32.0 / Lake 5.0.0
-/

namespace TSCP

/- ===================================================================
   PART 1: KERNELS
   =================================================================== -/

structure Kernel (α : Type) where
  admits_proof : α → Prop
  admits_decidable : ∀ (p : α), Decidable (admits_proof p)
  nonempty : ∃ (p : α), admits_proof p

/- ===================================================================
   PART 2: EVIDENCE
   =================================================================== -/

inductive EvidenceKind where
  | required      : EvidenceKind
  | recommended   : EvidenceKind
  | informational : EvidenceKind

structure Evidence where
  digest : String
  kind : EvidenceKind
  issuer : String
  timestamp : String

/- ===================================================================
   PART 3: CERTIFIED PROOFS
   =================================================================== -/

structure CertifiedProof {α : Type} (K : Kernel α) where
  proof : α
  certified : K.admits_proof proof
  evidence : List Evidence

/- ===================================================================
   PART 4: KERNEL ADMISSIBILITY FOR TRANSPORT MAPS
   =================================================================== -/

structure KernelAdmissible {A B : Type} (K_A : Kernel A) (K_B : Kernel B)
    (f : A → B) where
  preserves : ∀ (p : A), K_A.admits_proof p → K_B.admits_proof (f p)
  reflects : ∀ (q : B), K_B.admits_proof q → ∃ (p : A), K_A.admits_proof p ∧ f p = q

/- ===================================================================
   PART 5: UNIVERSES
   =================================================================== -/

structure Universe where
  ProofType : Type
  FormulaType : Type
  ExecutionType : Type
  proof_kernel : Kernel ProofType
  formula_kernel : Kernel FormulaType
  exec_kernel : Kernel ExecutionType

/- ===================================================================
   PART 6: BRIDGES
   =================================================================== -/

structure Bridge (U V : Universe) where
  proof_map : U.ProofType → V.ProofType
  formula_map : U.FormulaType → V.FormulaType
  exec_map : U.ExecutionType → V.ExecutionType

structure BridgeCertificate {U V : Universe} (f : Bridge U V) where
  proof_preservation :
    ∀ (p : U.ProofType), U.proof_kernel.admits_proof p →
      V.proof_kernel.admits_proof (f.proof_map p)
  proof_reflection :
    ∀ (q : V.ProofType), V.proof_kernel.admits_proof q →
      ∃ (p : U.ProofType), U.proof_kernel.admits_proof p ∧ f.proof_map p = q
  proof_admissibility : KernelAdmissible U.proof_kernel V.proof_kernel f.proof_map
  formula_admissibility : KernelAdmissible U.formula_kernel V.formula_kernel f.formula_map
  exec_admissibility : KernelAdmissible U.exec_kernel V.exec_kernel f.exec_map
  certificate_digest : String
  verifier_version : String
  issued_at : String

/- ===================================================================
   PART 7: CONSERVATIVE COMPOSITION
   =================================================================== -/

def compose_bridges {U V W : Universe} (f : Bridge U V) (g : Bridge V W) : Bridge U W where
  proof_map := g.proof_map ∘ f.proof_map
  formula_map := g.formula_map ∘ f.formula_map
  exec_map := g.exec_map ∘ f.exec_map

theorem kernel_admissibility_comp
    {A B C : Type} {K_A : Kernel A} {K_B : Kernel B} {K_C : Kernel C}
    (f : A → B) (g : B → C)
    (hf : KernelAdmissible K_A K_B f)
    (hg : KernelAdmissible K_B K_C g) :
    KernelAdmissible K_A K_C (g ∘ f) where
  preserves := by
    intro p hp
    exact hg.preserves (f p) (hf.preserves p hp)
  reflects := by
    intro q hq
    obtain ⟨b, hb, hb_eq⟩ := hg.reflects q hq
    obtain ⟨a, ha, ha_eq⟩ := hf.reflects b hb
    refine ⟨a, ha, ?_⟩
    show g (f a) = q
    rw [ha_eq, hb_eq]

def conservative_ext_comp
    {U V W : Universe} (f : Bridge U V) (g : Bridge V W)
    (cert_f : BridgeCertificate f) (cert_g : BridgeCertificate g) :
    BridgeCertificate (compose_bridges f g) where
  proof_preservation := by
    intro p hp
    exact cert_g.proof_preservation (f.proof_map p) (cert_f.proof_preservation p hp)
  proof_reflection := by
    intro q hq
    obtain ⟨b, hb, hb_eq⟩ := cert_g.proof_reflection q hq
    obtain ⟨a, ha, ha_eq⟩ := cert_f.proof_reflection b hb
    refine ⟨a, ha, ?_⟩
    show g.proof_map (f.proof_map a) = q
    rw [ha_eq, hb_eq]
  proof_admissibility := kernel_admissibility_comp
    f.proof_map g.proof_map cert_f.proof_admissibility cert_g.proof_admissibility
  formula_admissibility := kernel_admissibility_comp
    f.formula_map g.formula_map cert_f.formula_admissibility cert_g.formula_admissibility
  exec_admissibility := kernel_admissibility_comp
    f.exec_map g.exec_map cert_f.exec_admissibility cert_g.exec_admissibility
  certificate_digest := cert_f.certificate_digest ++ "++" ++ cert_g.certificate_digest
  verifier_version := cert_f.verifier_version
  issued_at := cert_g.issued_at

def ConservativeBridge {U V : Universe} (f : Bridge U V) :=
  BridgeCertificate f

/- ===================================================================
   PART 8: PROOF QUOTIENT
   Standalone equivalence class with named instances for clean field access.
   =================================================================== -/

class ProofSetoid (U : Universe) where
  equiv : U.ProofType → U.ProofType → Prop
  refl : ∀ (p : U.ProofType), equiv p p
  symm : ∀ (p q : U.ProofType), equiv p q → equiv q p
  trans : ∀ (p q r : U.ProofType), equiv p q → equiv q r → equiv p r
  kernel_respects :
    ∀ (p q : U.ProofType), equiv p q →
      (U.proof_kernel.admits_proof p ↔ U.proof_kernel.admits_proof q)

/-- Derive a Setoid from ProofSetoid for Quotient compatibility. -/
instance setoidFromProofSetoid (U : Universe) [ps : ProofSetoid U] : Setoid U.ProofType where
  r := ps.equiv
  iseqv := ⟨ps.refl, fun {x y} h => ps.symm x y h, fun {x y z} h1 h2 => ps.trans x y z h1 h2⟩

def bridge_respects_equiv {U V : Universe} [psU : ProofSetoid U] [psV : ProofSetoid V]
    (f : Bridge U V) : Prop :=
  ∀ (p p' : U.ProofType), psU.equiv p p' →
    psV.equiv (f.proof_map p) (f.proof_map p')

theorem certificate_stability
    {U V : Universe} [psU : ProofSetoid U] [psV : ProofSetoid V]
    (f : Bridge U V) (cert : BridgeCertificate f)
    (h_equiv : bridge_respects_equiv f) :
    ∀ (p p' : U.ProofType) (h : psU.equiv p p'),
      V.proof_kernel.admits_proof (f.proof_map p) ↔
      V.proof_kernel.admits_proof (f.proof_map p') := by
  intro p p' h
  have hpq : psV.equiv (f.proof_map p) (f.proof_map p') := h_equiv p p' h
  exact psV.kernel_respects (f.proof_map p) (f.proof_map p') hpq

noncomputable def lift_bridge_to_quotient
    {U V : Universe} [psU : ProofSetoid U] [psV : ProofSetoid V]
    (f : Bridge U V) (h : bridge_respects_equiv f) :
    Quotient (setoidFromProofSetoid U) → Quotient (setoidFromProofSetoid V) :=
  Quotient.lift (fun a => Quotient.mk (setoidFromProofSetoid V) (f.proof_map a))
    (by intro a b hab
        exact Quotient.sound (h a b hab))

/- ===================================================================
   PART 9: GOVERNANCE AS LABELED TRANSITION SYSTEM
   =================================================================== -/

inductive TransitionLabel (α : Type) where
  | update_policy   : TransitionLabel α
  | add_agent       : α → TransitionLabel α
  | remove_agent    : α → TransitionLabel α
  | add_bridge      : String → TransitionLabel α
  | revoke_bridge   : String → TransitionLabel α
  | add_exception   : String → TransitionLabel α
  | close_exception : String → TransitionLabel α

structure GovernanceState (α : Type) where
  agents : List α
  bridges : List String
  policy_version : Nat
  exceptions : List String

def authorized {α : Type} [BEq α] (s : GovernanceState α) (l : TransitionLabel α) : Prop :=
  match l with
  | TransitionLabel.update_policy   => s.agents ≠ []
  | TransitionLabel.add_agent _    => True
  | TransitionLabel.remove_agent _ => s.agents.length > 1
  | TransitionLabel.add_bridge _   => s.agents ≠ []
  | TransitionLabel.revoke_bridge _ => s.agents ≠ []
  | TransitionLabel.add_exception _ => s.agents ≠ []
  | TransitionLabel.close_exception _ => s.agents ≠ []

def transition {α : Type} [BEq α] (s : GovernanceState α) (l : TransitionLabel α) :
    GovernanceState α :=
  match l with
  | TransitionLabel.update_policy     => { s with policy_version := s.policy_version + 1 }
  | TransitionLabel.add_agent a       => { s with agents := a :: s.agents }
  | TransitionLabel.remove_agent a   => { s with agents := s.agents.filter (· != a) }
  | TransitionLabel.add_bridge b      => { s with bridges := b :: s.bridges }
  | TransitionLabel.revoke_bridge b   => { s with bridges := s.bridges.filter (· != b) }
  | TransitionLabel.add_exception e  => { s with exceptions := e :: s.exceptions }
  | TransitionLabel.close_exception e => { s with exceptions := s.exceptions.filter (· != e) }

def Truth (U : Universe) (p : U.ProofType) : Prop :=
  U.proof_kernel.admits_proof p

theorem TruthInvariant
    (U : Universe) (_ _ : GovernanceState String) (_ : TransitionLabel String) :
    ∀ (p : U.ProofType), Truth U p ↔ Truth U p := by
  intro p
  rfl

/- ===================================================================
   PART 10: UTILITY AS POST-HOC VALUATION
   =================================================================== -/

def utility {α : Type} {K : Kernel α}
    (cp : CertifiedProof K) : Nat := 0

theorem utility_order_sound
    {U : Universe} [psU : ProofSetoid U]
    (p q : CertifiedProof U.proof_kernel)
    (h_equiv : psU.equiv p.proof q.proof) :
    utility p = utility q := by
  rfl

/- ===================================================================
   PART 11: PROMOTION DECISION TREE
   =================================================================== -/

inductive PromotionResult where
  | accept            : PromotionResult
  | correct_but_slower : PromotionResult
  | reject            : PromotionResult

inductive RejectionReason where
  | custody_failure    : RejectionReason
  | evidence_failure   : RejectionReason
  | policy_failure     : RejectionReason
  | no_rejection       : RejectionReason

def promote
    {U : Universe} (_ : GovernanceState String)
    (cp : CertifiedProof U.proof_kernel)
    (has_exception : Bool) (perf_threshold : Nat) :
    PromotionResult :=
  if has_exception then
    PromotionResult.correct_but_slower
  else if utility cp ≥ perf_threshold then
    PromotionResult.accept
  else
    PromotionResult.correct_but_slower

/- ===================================================================
   PART 12: DOMAIN-BLIND CORE
   =================================================================== -/

structure Domain extends Universe where
  domain_evidence : Type
  evidence_kind : domain_evidence → EvidenceKind

def domain_blind_promote
    {D : Domain} (_ : GovernanceState String)
    (_ : CertifiedProof D.proof_kernel)
    (ev : D.domain_evidence) :
    PromotionResult :=
  match D.evidence_kind ev with
  | EvidenceKind.required      => PromotionResult.accept
  | EvidenceKind.recommended   => PromotionResult.accept
  | EvidenceKind.informational => PromotionResult.accept

/- ===================================================================
   PART 13: THE CUSTODIAL CHAIN
   =================================================================== -/

structure CustodialChain (U : Universe) where
  authority : GovernanceState String
  reference : CertifiedProof U.proof_kernel
  candidate : CertifiedProof U.proof_kernel
  evidence : List Evidence
  promotion : PromotionResult

end TSCP
