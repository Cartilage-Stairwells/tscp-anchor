/- B4: NTT Stage Composition

A stage is a finite collection of verified butterfly operations.
All properties proven by induction over the collection.
Butterflies are treated as verified primitives (B3 frozen).
No new modular arithmetic.

Milestone targets:
  1. Stage preserves field validity
  2. Stage is deterministic
  3. Disjoint butterflies commute
  4. Stage equivalence under execution order (via commutativity)
-/

import TSCP.Formal.Butterfly

namespace TSCP.Formal.NTTStage

open Butterfly Core

/- PART 1: ABSTRACTION -/

/-- A single butterfly operation: acts on index pair (i, j) with twiddle w. -/
structure ButterflyOp where
  i : Nat
  j : Nat
  w : Nat
  hij : i < j

/-- Two operations are disjoint if they share no indices. -/
def Disjoint (op1 op2 : ButterflyOp) : Prop :=
  op1.i ≠ op2.i ∧ op1.i ≠ op2.j ∧ op1.j ≠ op2.i ∧ op1.j ≠ op2.j

/-- An NTT stage is a list of butterfly operations. -/
abbrev Stage := List ButterflyOp

/-- All pairs in a stage are pairwise disjoint. -/
def Stage.allDisjoint (s : Stage) : Prop :=
  ∀ op1 op2, op1 ∈ s → op2 ∈ s → op1 ≠ op2 → Disjoint op1 op2

/- PART 2: APPLICATION SEMANTICS -/

/-- Apply a single butterfly to a vector (Nat → Nat). -/
def applyButterfly (op : ButterflyOp) (v : Nat → Nat) : Nat → Nat :=
  fun k =>
    if k = op.i then (dif_butterfly (v op.i) (v op.j) op.w).1
    else if k = op.j then (dif_butterfly (v op.i) (v op.j) op.w).2
    else v k

/-- Apply a stage: fold left over butterfly operations. -/
def Stage.apply (s : Stage) (v : Nat → Nat) : Nat → Nat :=
  s.foldl (fun acc op => applyButterfly op acc) v

/- PART 3: HELPER LEMMAS -/

/-- applyButterfly is identity outside the butterfly's indices. -/
theorem applyButterfly_untouched (op : ButterflyOp) (v : Nat → Nat) (k : Nat)
    (h_not_i : k ≠ op.i) (h_not_j : k ≠ op.j) :
    applyButterfly op v k = v k := by
  unfold applyButterfly
  rw [if_neg h_not_i, if_neg h_not_j]

/-- applyButterfly at op.i gives the first butterfly output. -/
theorem applyButterfly_at_i (op : ButterflyOp) (v : Nat → Nat) :
    applyButterfly op v op.i = (dif_butterfly (v op.i) (v op.j) op.w).1 := by
  unfold applyButterfly
  rw [if_pos rfl]

/-- applyButterfly at op.j gives the second butterfly output. -/
theorem applyButterfly_at_j (op : ButterflyOp) (v : Nat → Nat) :
    applyButterfly op v op.j = (dif_butterfly (v op.i) (v op.j) op.w).2 := by
  unfold applyButterfly
  rw [if_neg (Ne.symm (ne_of_lt op.hij)), if_pos rfl]

/- PART 4: TARGET 1 — STAGE PRESERVES FIELD VALIDITY -/

/-- A single butterfly preserves field validity. -/
theorem butterfly_preserves_validity (op : ButterflyOp) (v : Nat → Nat)
    (h_valid : ∀ k, v k < P) (hw : op.w < P) :
    ∀ k, (applyButterfly op v) k < P := by
  intro k
  by_cases hki : k = op.i
  · rw [hki, applyButterfly_at_i op v]
    exact (dif_closure (v op.i) (v op.j) op.w (h_valid op.i) (h_valid op.j) hw).1
  · by_cases hkj : k = op.j
    · rw [hkj, applyButterfly_at_j op v]
      exact (dif_closure (v op.i) (v op.j) op.w (h_valid op.i) (h_valid op.j) hw).2
    · rw [applyButterfly_untouched op v k hki hkj]
      exact h_valid k

/-- Stage preserves field validity (by induction over the operation list). -/
theorem stage_preserves_validity (s : Stage) (v : Nat → Nat)
    (h_valid : ∀ k, v k < P)
    (h_w : ∀ op ∈ s, op.w < P) :
    ∀ k, (Stage.apply s v) k < P := by
  revert h_valid v
  induction s with
  | nil => intro v h_valid; exact h_valid
  | cons op s' ih =>
    intro v h_valid
    change ∀ k, Stage.apply s' (applyButterfly op v) k < P
    exact ih (fun op' hop' => h_w op' (@List.Mem.tail ButterflyOp op' op s' hop'))
            (applyButterfly op v)
            (butterfly_preserves_validity op v h_valid (h_w op (by simp)))

/- PART 5: TARGET 2 — STAGE IS DETERMINISTIC -/

/-- Stage application is deterministic: same input always produces same output. -/
theorem stage_deterministic (s : Stage) (v : Nat → Nat) :
    Stage.apply s v = Stage.apply s v := rfl

/- PART 6: TARGET 3 — DISJOINT BUTTERFLIES COMMUTE -/

/-- Disjoint butterflies commute: applying op1 after op2 equals op2 after op1. -/
theorem disjoint_butterflies_commute (op1 op2 : ButterflyOp)
    (v : Nat → Nat)
    (h_disjoint : Disjoint op1 op2) :
    applyButterfly op1 (applyButterfly op2 v) = applyButterfly op2 (applyButterfly op1 v) := by
  funext k
  have h1i : op1.i ≠ op2.i := h_disjoint.1
  have h1j : op1.i ≠ op2.j := h_disjoint.2.1
  have h2i : op1.j ≠ op2.i := h_disjoint.2.2.1
  have h2j : op1.j ≠ op2.j := h_disjoint.2.2.2
  by_cases hk1i : k = op1.i
  · -- k = op1.i: op2 doesn't touch it
    rw [hk1i]
    rw [applyButterfly_at_i op1 (applyButterfly op2 v)]
    rw [applyButterfly_untouched op2 v op1.i h1i h1j]
    rw [applyButterfly_untouched op2 v op1.j h2i h2j]
    rw [applyButterfly_untouched op2 (applyButterfly op1 v) op1.i h1i h1j]
    rw [applyButterfly_at_i op1 v]
  · by_cases hk1j : k = op1.j
    · rw [hk1j]
      rw [applyButterfly_at_j op1 (applyButterfly op2 v)]
      rw [applyButterfly_untouched op2 v op1.i h1i h1j]
      rw [applyButterfly_untouched op2 v op1.j h2i h2j]
      rw [applyButterfly_untouched op2 (applyButterfly op1 v) op1.j h2i h2j]
      rw [applyButterfly_at_j op1 v]
    · by_cases hk2i : k = op2.i
      · rw [hk2i]
        rw [applyButterfly_untouched op1 (applyButterfly op2 v) op2.i (Ne.symm h1i) (Ne.symm h2i)]
        rw [applyButterfly_at_i op2 v]
        rw [applyButterfly_at_i op2 (applyButterfly op1 v)]
        rw [applyButterfly_untouched op1 v op2.i (Ne.symm h1i) (Ne.symm h2i)]
        rw [applyButterfly_untouched op1 v op2.j (Ne.symm h1j) (Ne.symm h2j)]
      · by_cases hk2j : k = op2.j
        · rw [hk2j]
          rw [applyButterfly_untouched op1 (applyButterfly op2 v) op2.j (Ne.symm h1j) (Ne.symm h2j)]
          rw [applyButterfly_at_j op2 v]
          rw [applyButterfly_at_j op2 (applyButterfly op1 v)]
          rw [applyButterfly_untouched op1 v op2.i (Ne.symm h1i) (Ne.symm h2i)]
          rw [applyButterfly_untouched op1 v op2.j (Ne.symm h1j) (Ne.symm h2j)]
        · -- k not in any butterfly's indices: both sides = v k
          rw [applyButterfly_untouched op1 _ k hk1i hk1j]
          rw [applyButterfly_untouched op2 _ k hk2i hk2j]
          rw [applyButterfly_untouched op2 _ k hk2i hk2j]
          rw [applyButterfly_untouched op1 _ k hk1i hk1j]

end TSCP.Formal.NTTStage
