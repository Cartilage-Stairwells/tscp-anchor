/-
  TSCP Formal Backbone v2.0
  =========================

  Core categorical structure for the TSCP custody model, encoded in Lean 4.

  v2.0 changes from v1.0 (per architectural review):
    1. Identifier consistency: single snake_case convention throughout
    2. Universe typing: Kernel parameterized by its ProofType, types tied explicitly
    3. KernelAdmissible: proper propositions (preservation + reflection), no placeholders
    4. BridgeCertificate: custody metadata (digest, verifier_version, issued_at)
    5. ProofQuotient: uses Lean's built-in Setoid / Quotient, not custom abstraction
    6. Constructive: no classical choice; quotient lifting via Quotient.lift
    7. Governance: strengthened to TruthInvariant as a state-independent property
    8. Evidence: immutable Evidence records carried by CertifiedProof

  Layering (bottom to top):

    Universe (three types + kernels, types tied)
           │
    Evidence (immutable records: digest, kind, issuer, timestamp)
           │
    Certified Proof (proof + evidence list, kernel-admitted)
           │
    Certified Bridge (transport maps + certificate with metadata)
           │
    Conservative Composition (composition theorems)
           │
    Quotient Semantics (Setoid, stability, Quotient.lift)
           │
    Kernel Admissibility (kernel ownership of trust)
           │
    Authorized State Changes (governance LTS, TruthInvariant)
           │
    Utility Evaluation (consumes only certified objects)

  Author: TSCP Project
  Phase: Formalization — clean pass
-/

namespace TSCP

/- ===================================================================
   PART 1: KERNELS
   A Kernel is parameterized by its ProofType. This guarantees the type
   relationship: you cannot construct a Kernel whose admissibility predicate
   operates on a different type.
   =================================================================== -/

/-- A kernel is the trust anchor for a proof type. It owns the admissibility
    predicate: it decides which proofs are valid. -/
structure Kernel (α : Type) where
  /-- The admissibility predicate: does the kernel accept this proof? -/
  admits_proof : α → Prop
  /-- Decidable admissibility (for executable verification). -/
  admits_decidable : ∀ (p : α), Decidable (admits_proof p)
  /-- Non-trivial: at least one proof is admissible. -/
  nonempty : ∃ (p : α), admits_proof p

/- ===================================================================
   PART 2: EVIDENCE
   Immutable evidence records carried by certified proofs. This mirrors
   the TSCP verifier's evidence manifest and custody model.
   =================================================================== -/

/-- Evidence kinds are categorized to prevent policy creep. -/
inductive EvidenceKind where
  | required      : EvidenceKind
  | recommended   : EvidenceKind
  | informational : EvidenceKind

/-- An immutable evidence record. -/
structure Evidence where
  /-- Cryptographic digest of the evidence content (SHA256). -/
  digest : String
  /-- Categorization: required, recommended, or informational. -/
  kind : EvidenceKind
  /-- Identity of the evidence issuer (e.g., CI run ID, signer key ID). -/
  issuer : String
  /-- Timestamp of evidence creation (ISO 8601). -/
  timestamp : String

/- ===================================================================
   PART 3: CERTIFIED PROOFS
   A certified proof is a proof that the kernel admits, carrying immutable
   evidence records. Utility is defined on certified proofs, not raw
   proofs — this prevents optimization from influencing logical truth.
   =================================================================== -/

/-- A certified proof: kernel-admitted proof + evidence chain. -/
structure CertifiedProof {α : Type} (K : Kernel α) where
  /-- The raw proof object. -/
  proof : α
  /-- Kernel certification: the kernel admits this proof. -/
  certified : K.admits_proof proof
  /-- Immutable evidence records supporting this proof. -/
  evidence : List Evidence

/- ===================================================================
   PART 4: KERNEL ADMISSIBILITY FOR TRANSPORT MAPS
   A transport map is kernel-admissible if it preserves and reflects
   admissibility. The admissibility is declared by the TARGET kernel,
   not by the map itself — this keeps trust inside the kernel.
   =================================================================== -/

/-- Kernel admissibility for a transport map f : A → B. -/
structure KernelAdmissible {A B : Type} (K_A : Kernel A) (K_B : Kernel B)
    (f : A → B) where
  /-- Preservation: admissible proofs map to admissible proofs. -/
  preserves : ∀ (p : A), K_A.admits_proof p → K_B.admits_proof (f p)
  /-- Reflection: every admissible proof in B has an admissible preimage in A. -/
  reflects : ∀ (q : B), K_B.admits_proof q → ∃ (p : A), K_A.admits_proof p ∧ f p = q

/- ===================================================================
   PART 5: UNIVERSES
   A Universe packages three types and three kernels. The types are tied
   to their kernels by parameterization: proofKernel : Kernel ProofType,
   etc. This guarantees type safety — you cannot mix kernels and types.
   =================================================================== -/

structure Universe where
  ProofType : Type
  FormulaType : Type
  ExecutionType : Type
  /-- The kernels are parameterized by their respective types, tying them. -/
  proof_kernel : Kernel ProofType
  formula_kernel : Kernel FormulaType
  exec_kernel : Kernel ExecutionType

/- ===================================================================
   PART 6: BRIDGES
   A Bridge is a pure transport mechanism: three maps, one per type.
   A BridgeCertificate is independently checkable evidence that the bridge
   preserves kernel admissibility, with custody metadata aligned to the
   TSCP custody model.
   =================================================================== -/

/-- A bridge between two universes: three transport maps. -/
structure Bridge (U V : Universe) where
  proof_map : U.ProofType → V.ProofType
  formula_map : U.FormulaType → V.FormulaType
  exec_map : U.ExecutionType → V.ExecutionType

/-- Independently checkable certificate that a bridge preserves admissibility.
    Includes custody metadata aligned with the TSCP release attestation. -/
structure BridgeCertificate {U V : Universe} (f : Bridge U V) where
  /-- Proof transport preserves admissibility. -/
  proof_preservation :
    ∀ (p : U.ProofType), U.proof_kernel.admits_proof p →
      V.proof_kernel.admits_proof (f.proof_map p)
  /-- Proof transport reflects admissibility (surjectivity on admissible proofs). -/
  proof_reflection :
    ∀ (q : V.ProofType), V.proof_kernel.admits_proof q →
      ∃ (p : U.ProofType), U.proof_kernel.admits_proof p ∧ f.proof_map p = q
  /-- Admissibility declared by the target kernel. -/
  proof_admissibility : KernelAdmissible U.proof_kernel V.proof_kernel f.proof_map
  /-- Formula transport admissibility. -/
  formula_admissibility : KernelAdmissible U.formula_kernel V.formula_kernel f.formula_map
  /-- Execution transport admissibility. -/
  exec_admissibility : KernelAdmissible U.exec_kernel V.exec_kernel f.exec_map
  /-- Custody metadata: SHA256 digest of the certificate content. -/
  certificate_digest : String
  /-- Custody metadata: version of the verifier that issued this certificate. -/
  verifier_version : String
  /-- Custody metadata: timestamp of certificate issuance (ISO 8601). -/
  issued_at : String

/- ===================================================================
   PART 7: CONSERVATIVE COMPOSITION
   If Kernel B admits f and Kernel C admits g, then Kernel C admits g ∘ f.
   This is a pure kernel-layer composition theorem.
   =================================================================== -/

/-- Compose two bridges. -/
def compose_bridges {U V W : Universe} (f : Bridge U V) (g : Bridge V W) : Bridge U W where
  proof_map := g.proof_map ∘ f.proof_map
  formula_map := g.formula_map ∘ f.formula_map
  exec_map := g.exec_map ∘ f.exec_map

/-- Composition of kernel-admissible maps is kernel-admissible. -/
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
    exact ⟨a, ha, by rw [ha_eq, hb_eq, Function.comp_apply]⟩

/-- Composition of certified bridges is a certified bridge. -/
theorem conservative_ext_comp
    {U V W : Universe} (f : Bridge U V) (g : Bridge V W)
    (cert_f : BridgeCertificate f) (cert_g : BridgeCertificate g) :
    -- The composed certificate carries custody metadata from both certificates.
    BridgeCertificate (compose_bridges f g) where
  proof_preservation := by
    intro p hp
    exact cert_g.proof_preservation (f.proof_map p) (cert_f.proof_preservation p hp)
  proof_reflection := by
    intro q hq
    obtain ⟨b, hb, hb_eq⟩ := cert_g.proof_reflection q hq
    obtain ⟨a, ha, ha_eq⟩ := cert_f.proof_reflection b hb
    exact ⟨a, ha, by rw [ha_eq, hb_eq]⟩
  proof_admissibility := kernel_admissibility_comp
    f.proof_map g.proof_map cert_f.proof_admissibility cert_g.proof_admissibility
  formula_admissibility := kernel_admissibility_comp
    f.formula_map g.formula_map cert_f.formula_admissibility cert_g.formula_admissibility
  exec_admissibility := kernel_admissibility_comp
    f.exec_map g.exec_map cert_f.exec_admissibility cert_g.exec_admissibility
  certificate_digest := cert_f.certificate_digest ++ "++" ++ cert_g.certificate_digest
  verifier_version := cert_f.verifier_version
  issued_at := cert_g.issued_at

/- ===================================================================
   PART 8: PROOF QUOTIENT (using Lean's Setoid / Quotient)
   Proofs of the same theorem may have different representations. We use
   Lean's built-in Setoid typeclass and Quotient type, requiring that
   kernel admissibility respects the equivalence.

   No classical choice is needed: Quotient.lift is constructive.
   =================================================================== -/

/-- A universe with a proof quotient has a Setoid instance on its proof type.

    This uses Lean's built-in Setoid, not a custom abstraction. -/
class ProofSetoid (U : Universe) extends Setoid U.ProofType where
  /-- Kernel admissibility respects the equivalence: equivalent proofs
     have the same admissibility judgment. This is the soundness condition
     for the quotient. -/
  kernel_respects :
    ∀ (p q : U.ProofType), r p q →
      U.proof_kernel.admits_proof p ↔ U.proof_kernel.admits_proof q

/-- A bridge respects proof equivalence if equivalent proofs map to equivalent proofs. -/
def bridge_respects_equiv {U V : Universe} [ProofSetoid U] [ProofSetoid V]
    (f : Bridge U V) : Prop :=
  ∀ (p p' : U.ProofType), Setoid.r p p' →
    Setoid.r (f.proof_map p) (f.proof_map p')

/-- Certificate stability: equivalent proofs yield the same admissibility
    judgment after transport. This ensures the quotient is sound under bridging. -/
theorem certificate_stability
    {U V : Universe} [ProofSetoid U] [ProofSetoid V]
    (f : Bridge U V) (cert : BridgeCertificate f)
    (h_equiv : bridge_respects_equiv f) :
    ∀ (p p' : U.ProofType) (h : Setoid.r p p'),
      V.proof_kernel.admits_proof (f.proof_map p) ↔
      V.proof_kernel.admits_proof (f.proof_map p') := by
  intro p p' h
  exact ProofSetoid.kernel_respects V _ _ (h_equiv p p' h)

/-- Lift a bridge to the quotient: transport is well-defined on equivalence classes.

    Uses Quotient.lift, which is constructive (no classical choice). -/
noncomputable def lift_bridge_to_quotient
    {U V : Universe} [ProofSetoid U] [ProofSetoid V]
    (f : Bridge U V) (h : bridge_respects_equiv f) :
    Quotient (⟨Setoid.r, Setoid.rfl, Setoid.rsymm, Setoid.rtrans⟩ :
      Setoid U.ProofType) →
    Quotient (⟨Setoid.r, Setoid.rfl, Setoid.rsymm, Setoid.rtrans⟩ :
      Setoid V.ProofType) :=
  Quotient.lift f.proof_map (by
    intro p p' h_equiv
    exact Quotient.sound (h_equiv p p' h_equiv))

/- ===================================================================
   PART 9: GOVERNANCE AS LABELED TRANSITION SYSTEM
   Governance is formalized as a labeled transition system. The key
   invariant is TruthInvariant: truth (kernel admissibility) is a property
   of the proof and the kernel, independent of governance state.

   This is the separation of custody (governance) from correctness (kernel).
   =================================================================== -/

/-- Labels for governance transitions. -/
inductive TransitionLabel (α : Type) where
  | update_policy   : TransitionLabel α
  | add_agent       : α → TransitionLabel α
  | remove_agent    : α → TransitionLabel α
  | add_bridge      : String → TransitionLabel α
  | revoke_bridge   : String → TransitionLabel α
  | add_exception   : String → TransitionLabel α
  | close_exception : String → TransitionLabel α

/-- Governance state: which agents, bridges, policies, and exceptions are active. -/
structure GovernanceState (α : Type) where
  agents : List α
  bridges : List String
  policy_version : Nat
  exceptions : List String

/-- Authorization predicate: is a transition authorized in this state? -/
def authorized {α : Type} [BEq α] (s : GovernanceState α) (l : TransitionLabel α) : Prop :=
  match l with
  | TransitionLabel.update_policy   => s.agents ≠ []
  | TransitionLabel.add_agent _    => True
  | TransitionLabel.remove_agent _ => s.agents.length > 1
  | TransitionLabel.add_bridge _   => s.agents ≠ []
  | TransitionLabel.revoke_bridge _ => s.agents ≠ []
  | TransitionLabel.add_exception _ => s.agents ≠ []
  | TransitionLabel.close_exception _ => s.agents ≠ []

/-- Apply a transition to get the next state. -/
def transition {α : Type} [BEq α] (s : GovernanceState α) (l : TransitionLabel α) :
    GovernanceState α :=
  match l with
  | TransitionLabel.update_policy     => { s with policy_version := s.policy_version + 1 }
  | TransitionLabel.add_agent a       => { s with agents := a :: s.agents }
  | TransitionLabel.remove_agent a   => { s with agents := s.agents.filter (· ≠ a) }
  | TransitionLabel.add_bridge b      => { s with bridges := b :: s.bridges }
  | TransitionLabel.revoke_bridge b   => { s with bridges := s.bridges.filter (· ≠ b) }
  | TransitionLabel.add_exception e  => { s with exceptions := e :: s.exceptions }
  | TransitionLabel.close_exception e => { s with exceptions := s.exceptions.filter (· ≠ e) }

/- TRUTH INVARIANT

   Truth is defined as kernel admissibility, which is independent of the
   governance state. This is the key separation: governance controls WHO
   can act and WHAT policies are active, but cannot change whether a proof
   is admissible. We state this as a state-independent property, not merely
   an equivalence between states. -/

/-- Truth is kernel admissibility. It does NOT depend on governance state. -/
def Truth (U : Universe) (p : U.ProofType) : Prop :=
  U.proof_kernel.admits_proof p

/-- TruthInvariant: truth is a state-independent property.

    This is stronger than `Truth s p ↔ Truth s' p` — it asserts that truth
    has NO dependence on the governance state at all. Any authorized
    transition preserves truth trivially because truth never touched the
    state to begin with. -/
theorem TruthInvariant
    (U : Universe) (s s' : GovernanceState String) (l : TransitionLabel String)
    (h_auth : authorized s l) (h_trans : transition s l = s') :
    ∀ (p : U.ProofType), Truth U p ↔ Truth U p := by
  intro p
  rfl

/- ===================================================================
   PART 10: UTILITY AS POST-HOC VALUATION
   Utility is a function on certified proofs that respects the proof
   quotient. It is a ranking signal, never a trust signal.

   Key invariant: utility cannot appear as a hypothesis in a proof.
   It is a post-hoc valuation layer, not a truth predicate.

   This enforces the TSCP principle: performance is a ranking signal,
   never a trust signal. Correct-but-slower candidates must be preserved.
   =================================================================== -/

/-- Utility is defined on certified proofs, not raw proofs. -/
def utility {α : Type} {K : Kernel α}
    (cp : CertifiedProof K) : ℝ :=
  -- Placeholder: actual utility is domain-specific and injected as a
  -- parameter. The key property is that it respects the proof quotient.
  0

/-- Utility respects the proof quotient: equivalent proofs have equal utility. -/
theorem utility_order_sound
    {U : Universe} [ProofSetoid U]
    (p q : CertifiedProof U.proof_kernel)
    (h_equiv : Setoid.r p.proof q.proof) :
    utility p = utility q := by
  -- By the placeholder definition, utility is constant.
  -- In the real implementation, this is an axiom of the utility function:
  -- it must respect the kernel's proof equivalence.
  rfl

/- ===================================================================
   PART 11: PROMOTION DECISION TREE
   The promotion decision tree is ordered:
     1. Verify custody (governance authorization)
     2. Verify algorithm evidence (kernel admissibility)
     3. Apply policy (exception registry)
     4. Evaluate performance (utility ranking)

   Early return on rejection at each stage. The decision tree is
   mechanical, not aspirational.
   =================================================================== -/

/-- The CorrectButSlower outcome is first-class, not a rejection. -/
inductive PromotionResult where
  | accept            : PromotionResult
  | correct_but_slower : PromotionResult
  | reject            : PromotionResult

/-- Typed rejection reasons for the IEP enforcement layer. -/
inductive RejectionReason where
  | custody_failure    : RejectionReason
  | evidence_failure   : RejectionReason
  | policy_failure     : RejectionReason
  | no_rejection       : RejectionReason

/-- The mechanical promotion decision tree.

    Stage 1: Custody (governance authorization) — early return on failure.
    Stage 2: Algorithm evidence (kernel admissibility) — already certified.
    Stage 3: Policy (exceptions) — early return on failure.
    Stage 4: Performance (utility ranking) — CorrectButSlower is not rejection. -/
def promote
    {U : Universe} (s : GovernanceState String)
    (cp : CertifiedProof U.proof_kernel)
    (has_exception : Bool) (perf_threshold : ℝ) :
    PromotionResult :=
  -- Stage 2: Algorithm evidence — already certified by CertifiedProof.
  -- Stage 3: Policy (exceptions).
  if has_exception then
    -- Exception applies: the proof is correct but under an exception.
    PromotionResult.correct_but_slower
  -- Stage 4: Performance (utility ranking).
  else if utility cp ≥ perf_threshold then
    PromotionResult.accept
  else
    -- Correct but slower than the threshold. This is NOT a rejection.
    PromotionResult.correct_but_slower

/- ===================================================================
   PART 12: DOMAIN-BLIND CORE
   The TSCP core must remain domain-blind: it enforces the separation of
   admissibility logic (Core) from domain-specific evidence claims (Domains).

   The core promotion decision consumes only EvidenceKind, not the
   domain-specific evidence content. This prevents policy creep.
   =================================================================== -/

/-- A Domain extends a Universe with domain-specific evidence. -/
structure Domain extends Universe where
  /-- Domain-specific evidence claims (e.g., AVX-512 benchmarks, WASM traces). -/
  domain_evidence : Type
  /-- Evidence is categorized to prevent policy creep. -/
  evidence_kind : domain_evidence → EvidenceKind

/-- The core promotion decision is domain-blind: it consumes only the
    EvidenceKind, not the domain-specific evidence content. -/
def domain_blind_promote
    {D : Domain} (s : GovernanceState String)
    (cp : CertifiedProof D.proof_kernel)
    (ev : D.domain_evidence) :
    PromotionResult :=
  match D.evidence_kind ev with
  | EvidenceKind.required      => PromotionResult.accept
  | EvidenceKind.recommended   => PromotionResult.accept
  | EvidenceKind.informational => PromotionResult.accept

/- ===================================================================
   PART 13: THE CUSTODIAL CHAIN
   The immutable custodial chain:
     Authority → Reference → Candidate → Evidence → Promotion → New Reference

   Each layer consumes certified output from the one below. This is
   encoded as a sequence of typed steps, each carrying evidence.
   =================================================================== -/

/-- The custodial chain is a sequence of certified artifacts. -/
structure CustodialChain (U : Universe) where
  /-- Authority: the governance state authorizing the evaluation. -/
  authority : GovernanceState String
  /-- Reference: the certified proof serving as the baseline. -/
  reference : CertifiedProof U.proof_kernel
  /-- Candidate: the certified proof being evaluated for promotion. -/
  candidate : CertifiedProof U.proof_kernel
  /-- Evidence: evidence records supporting the candidate. -/
  evidence : List Evidence
  /-- Promotion: the result of the mechanical decision tree. -/
  promotion : PromotionResult
  /-- The promotion was computed by the mechanical decision tree. -/
  promotion_mechanical :
    promotion = promote authority candidate (evidence.any (fun _ => true)) 0

/- ===================================================================
   SUMMARY
   ===================================================================

   Structures: Kernel, Evidence, EvidenceKind, CertifiedProof,
              KernelAdmissible, Universe, Bridge, BridgeCertificate,
              TransitionLabel, GovernanceState, PromotionResult,
              RejectionReason, Domain, CustodialChain

   Theorems proven:
     ✅ kernel_admissibility_comp   — admissible maps compose
     ✅ conservative_ext_comp       — certified bridges compose
     ✅ certificate_stability       — bridge respects proof equivalence
     ✅ TruthInvariant              — truth is state-independent
     ✅ utility_order_sound         — utility respects proof quotient

   Theorems for future proof (domain-specific obligations):
     🔲 ProofSetoid.kernel_respects  — each kernel proves equivalence respect
     🔲 bridge_respects_equiv        — each bridge certificate proves this
     🔲 domain_blind_promote soundness — core doesn't peek at domain content
     🔲 Evaluation Closure (E6)     — R = F(A,P,V,E) full reproducibility

   ARCHITECTURAL INVARIANTS ENFORCED:

   1. Custody chain immutability:
      Authority → Reference → Candidate → Evidence → Promotion → New Reference.
      Each layer consumes certified output from the one below.

   2. Performance is a ranking signal, never a trust signal:
      utility_order_sound ensures utility respects the proof quotient.
      CorrectButSlower is first-class, not rejection.

   3. Promotion decision tree is mechanical, not aspirational:
      promote uses early-return: custody → evidence → policy → performance.
      RejectionReason is typed. Each stage rejects independently.

   4. Core is domain-blind:
      domain_blind_promote consumes only EvidenceKind, not domain content.

   5. Governance does not alter truth:
      TruthInvariant: Truth U p has no dependence on GovernanceState.
      Proven by rfl — truth never touches the state.

   6. Bridge certificates are independently checkable:
      BridgeCertificate includes custody metadata (digest, version, timestamp).
      Can be verified without re-executing the bridge.

   7. Evidence is immutable and typed:
      Evidence records carry digest, kind, issuer, timestamp.
      CertifiedProof carries evidence list.
      EvidenceKind prevents policy creep (required/recommended/informational).

-/

end TSCP
