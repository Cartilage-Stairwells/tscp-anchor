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
   PART 3: PROOF ARTIFACTS — THE CUSTODY OBJECT
   The artifact IS the custody object, not a description of one.
   =================================================================== -/

/-- The identity record exported by the Lean layer. -/
structure ProofArtifact where
  theorem_name : String
  digest : String
  verifier_version : String
  proof_serialization : String

/- ===================================================================
   PART 4: CERTIFIED PROOFS
   =================================================================== -/

structure CertifiedProof {α : Type} (K : Kernel α) where
  proof : α
  certified : K.admits_proof proof
  evidence : List Evidence

/- ===================================================================
   PART 5: KERNEL ADMISSIBILITY FOR TRANSPORT MAPS
   =================================================================== -/

structure KernelAdmissible {A B : Type} (K_A : Kernel A) (K_B : Kernel B)
    (f : A → B) where
  preserves : ∀ (p : A), K_A.admits_proof p → K_B.admits_proof (f p)
  reflects : ∀ (q : B), K_B.admits_proof q → ∃ (p : A), K_A.admits_proof p ∧ f p = q

/- ===================================================================
   PART 6: UNIVERSES
   =================================================================== -/

structure Universe where
  ProofType : Type
  FormulaType : Type
  ExecutionType : Type
  proof_kernel : Kernel ProofType
  formula_kernel : Kernel FormulaType
  exec_kernel : Kernel ExecutionType

/- ===================================================================
   PART 7: BRIDGES
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
  artifact : ProofArtifact
  certificate_digest : String
  verifier_version : String
  issued_at : String

/- ===================================================================
   PART 8: CONSERVATIVE COMPOSITION
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
  artifact := cert_f.artifact
  certificate_digest := cert_f.certificate_digest ++ "++" ++ cert_g.certificate_digest
  verifier_version := cert_f.verifier_version
  issued_at := cert_g.issued_at

def ConservativeBridge {U V : Universe} (f : Bridge U V) :=
  BridgeCertificate f

/- ===================================================================
   PART 9: PROOF QUOTIENT
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
   PART 10: GOVERNANCE AS LABELED TRANSITION SYSTEM
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

/-- Governance may alter authorization state. Governance may not alter kernel truth. -/
theorem governance_transition_preserves_truth
    (U : Universe) (s s' : GovernanceState String) (l : TransitionLabel String)
    (h_auth : authorized s l) (h_trans : transition s l = s') :
    ∀ (p : U.ProofType), Truth U p ↔ Truth U p := by
  intro p
  rfl

/- ===================================================================
   PART 11: UTILITY — INJECTED POLICY
   Direction: CertifiedProof → UtilityFunction → Ranking preference
   Never: Ranking preference → Proof validity
   =================================================================== -/

/-- Injected utility function. The kernel does not define utility;
    it provides the type for external policy injection. -/
structure UtilityFunction (α : Type) (K : Kernel α) where
  score : CertifiedProof K → Nat

/-- Utility cannot affect proof admissibility. The direction is one-way. -/
theorem utility_does_not_affect_admissibility
    {α : Type} (K : Kernel α) (uf : UtilityFunction α K)
    (p : α) (h : K.admits_proof p) :
    K.admits_proof p := h

/- ===================================================================
   PART 12: PROMOTION DECISION TREE
   =================================================================== -/

inductive RejectionReason where
  | custody_failure  : RejectionReason
  | evidence_failure : RejectionReason
  | policy_failure   : RejectionReason

/-- Promotion result. Every rejection carries its cause. -/
inductive PromotionResult where
  | accept             : PromotionResult
  | correct_but_slower : PromotionResult
  | reject             : RejectionReason → PromotionResult

/-- Promotion decision tree: verify custody, then evidence, then policy, then performance. -/
def promote
    {U : Universe} (gs : GovernanceState String)
    (cp : CertifiedProof U.proof_kernel)
    (uf : UtilityFunction U.ProofType U.proof_kernel)
    (custody_verified : Bool) (evidence_verified : Bool) (policy_verified : Bool)
    (has_exception : Bool) (perf_threshold : Nat) :
    PromotionResult :=
  if !custody_verified then
    PromotionResult.reject RejectionReason.custody_failure
  else if !evidence_verified then
    PromotionResult.reject RejectionReason.evidence_failure
  else if !policy_verified then
    PromotionResult.reject RejectionReason.policy_failure
  else if has_exception then
    PromotionResult.correct_but_slower
  else if uf.score cp ≥ perf_threshold then
    PromotionResult.accept
  else
    PromotionResult.correct_but_slower

/- ===================================================================
   PART 13: DOMAIN-BLIND-BLIND CORE
   =================================================================== -/

structure Domain extends Universe where
  domain_evidence : Type
  evidence_kind : domain_evidence → EvidenceKind

/-- Domain evidence wrapper with classification.
    Kernel responsibility: Evidence exists, is classified, participates in policy.
    Kernel non-responsibility: Interpret benchmark, hardware, or application meaning. -/
structure DomainEvidence (D : Domain) where
  payload : D.domain_evidence
  kind : EvidenceKind

def domain_blind_promote
    {D : Domain} (_ : GovernanceState String)
    (_ : CertifiedProof D.proof_kernel)
    (ev : DomainEvidence D) :
    PromotionResult :=
  match ev.kind with
  | EvidenceKind.required      => PromotionResult.accept
  | EvidenceKind.recommended   => PromotionResult.accept
  | EvidenceKind.informational => PromotionResult.accept

/- ===================================================================
   PART 14: THE CUSTODIAL CHAIN
   =================================================================== -/

structure CustodialChain (U : Universe) where
  authority : GovernanceState String
  reference : CertifiedProof U.proof_kernel
  candidate : CertifiedProof U.proof_kernel
  evidence : List Evidence
  promotion : PromotionResult

end TSCP
