/-
  TSCP Formal — Propositional Kernel
  ====================================

  Commit 2 of the minimal verified path.

  The smallest concrete semantic universe. This gives `Truth` something
  concrete to evaluate.

  Structure:
    Formula  (inductive: atom, implies)
    Proof    (inductive: assume, mp)
    TypeOf   (inductive predicate: Proof → Formula → Prop)
    admits   (∃ formula, TypeOf p formula)

  The kernel's admissibility predicate has actual meaning: a proof is
  admissible if it is well-typed (proves a specific formula). Modus ponens
  is only admissible when the formula types match.

  This is NOT a complete logic. It is the minimal concrete universe
  needed to exercise the verification pipeline.
-/

import TSCP.Formal.TSCP_Formal_Backbone

namespace TSCP.Formal.Examples

/- ===================================================================
   FORMULAS
   =================================================================== -/

/-- Propositional logic formulas. -/
inductive Formula where
  | atom : String → Formula
  | implies : Formula → Formula → Formula
deriving DecidableEq, Repr

/- ===================================================================
   PROOFS
   =================================================================== -/

/-- Proof terms (Hilbert-style: assumption + modus ponens). -/
inductive Proof where
  | assume : Formula → Proof
  | mp : Proof → Proof → Proof
deriving Repr

/- ===================================================================
   TYPING RELATION
   A proof is well-typed if it proves a specific formula.
   This is the semantic content of the kernel.
   =================================================================== -/

/-- TypeOf p f means proof p proves formula f.

    - `assume f` proves f.
    - `mp p q` proves b when p proves (a → b) and q proves a.

    The equality condition is built into the constructor: the formula
    that q proves must match the antecedent of the implication that p proves. -/
inductive TypeOf : Proof → Formula → Prop where
  | assume : ∀ (f : Formula), TypeOf (Proof.assume f) f
  | mp : ∀ (p q : Proof) (a b : Formula),
      TypeOf p (Formula.implies a b) →
      TypeOf q a →
      TypeOf (Proof.mp p q) b

/- ===================================================================
   ADMISSIBILITY
   A proof is admissible if it is well-typed (proves some formula).
   =================================================================== -/

/-- Kernel admissibility: the proof is well-typed. -/
def admits (p : Proof) : Prop := ∃ (f : Formula), TypeOf p f

/-- Decidable admissibility via the computable type checker. -/
def type_of : Proof → Option Formula
  | Proof.assume f => some f
  | Proof.mp p q =>
    match type_of p, type_of q with
    | some (Formula.implies a b), some c => if a = c then some b else none
    | _, _ => none

/-- Soundness: type_of returns a formula only if TypeOf holds. -/
theorem type_of_sound : ∀ (p : Proof) (f : Formula),
    type_of p = some f → TypeOf p f := by
  intro p
  induction p with
  | assume form =>
    intro f h
    simp only [type_of] at h
    injection h with h_eq
    rw [← h_eq]
    exact TypeOf.assume form
  | mp p' q' ih_p ih_q =>
    intro f h
    simp only [type_of] at h
    cases hpt : type_of p' with
    | none => simp [hpt] at h
    | some pt =>
      cases hqt : type_of q' with
      | none => simp [hpt, hqt] at h
      | some qt =>
        cases pt with
        | atom s => simp [hpt, hqt] at h
        | implies a b =>
          by_cases h_eq : a = qt
          · subst h_eq
            simp [hpt, hqt] at h
            rw [← h]
            exact TypeOf.mp p' q' a b (ih_p (Formula.implies a b) hpt) (ih_q a hqt)
          · simp [hpt, hqt, h_eq] at h

/-- Completeness: if TypeOf holds, type_of returns the formula. -/
theorem type_of_complete : ∀ (p : Proof) (f : Formula),
    TypeOf p f → type_of p = some f := by
  intro p f ht
  induction ht with
  | assume form => rfl
  | mp p' q' a b htp htq ih_p ih_q =>
    -- TypeOf (mp p' q') b
    -- ih_p : type_of p' = some (implies a b)
    -- ih_q : type_of q' = some a
    simp [type_of, ih_p, ih_q]

/-- Admissibility is decidable (via the computable type checker). -/
instance admits_decidable (p : Proof) : Decidable (admits p) :=
  match hp : type_of p with
  | none => isFalse (by
    intro h_admits
    obtain ⟨f, ht⟩ := h_admits
    rw [type_of_complete p f ht] at hp
    simp at hp)
  | some f => isTrue (by
    exact ⟨f, type_of_sound p f hp⟩)

/- ===================================================================
   KERNEL INSTANCE
   =================================================================== -/

/-- The propositional kernel: admissibility = well-typedness. -/
def propositional_kernel : Kernel Proof where
  admits_proof := admits
  admits_decidable := fun p => admits_decidable p
  nonempty := ⟨Proof.assume (Formula.atom "a"), Formula.atom "a", TypeOf.assume _⟩

/-- A trivial kernel for formulas (everything is admissible). -/
def trivial_formula_kernel : Kernel Formula where
  admits_proof := fun _ => True
  admits_decidable := fun _ => isTrue trivial
  nonempty := ⟨Formula.atom "x", trivial⟩

/-- A trivial kernel for executions. -/
def trivial_exec_kernel : Kernel Unit where
  admits_proof := fun _ => True
  admits_decidable := fun _ => isTrue trivial
  nonempty := ⟨(), trivial⟩

/- ===================================================================
   UNIVERSE INSTANCE
   =================================================================== -/

/-- The propositional universe: proofs, formulas, and executions. -/
def propositional_universe : Universe where
  ProofType := Proof
  FormulaType := Formula
  ExecutionType := Unit
  proof_kernel := propositional_kernel
  formula_kernel := trivial_formula_kernel
  exec_kernel := trivial_exec_kernel

/- ===================================================================
   CONCRETE TRUTH EXAMPLES
   =================================================================== -/

/-- Example: `assume (atom "P")` proves `atom "P"`. -/
def example_proof : Proof := Proof.assume (Formula.atom "P")

/-- The example proof is admissible. -/
theorem example_proof_admits : admits example_proof :=
  ⟨Formula.atom "P", TypeOf.assume _⟩

/-- Truth evaluates to a concrete value: the example proof is true. -/
theorem example_truth : Truth propositional_universe example_proof :=
  example_proof_admits

end TSCP.Formal.Examples
