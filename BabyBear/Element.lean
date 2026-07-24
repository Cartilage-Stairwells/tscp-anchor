/-
  BabyBear Element — Canonical Representative Predicate

  This file replaces the placeholder `canonicalRule := True` with the actual
  BabyBear field constraint: a canonical element satisfies `val < BABYBEAR_P`.

  This is a COMPLETED PROOF OBLIGATION. The encoding validity boundary moves
  from "assumed canonical encoding" to "formally checked canonical encoding".
-/

namespace BabyBear

/-- The BabyBear prime: 2^31 - 2^27 + 1 -/
def BABYBEAR_P : Nat := 2^31 - 2^27 + 1

/-- A raw element carrying an unsigned integer representative. -/
structure Element where
  val : Nat
  deriving DecidableEq

/-
  Canonical representative predicate.

  Previous (placeholder):
    def canonicalRule (x : Element) : Prop := True

  Now (actual field constraint):
    def canonicalRule (x : Element) : Prop := x.val < BABYBEAR_P

  This matches the representation used by the execution layer: an encoded
  field element is canonical iff its raw representative lies in [0, P).
-/
def canonicalRule (x : Element) : Prop := x.val < BABYBEAR_P

/-
  canonicalRule_babybear — the closure target.

  The invariant is exactly:

    canonicalRule x ↔ x.val < BABYBEAR_P

  Proof: immediate by definitional unfolding (rfl). The significance is not
  proof difficulty but that the predicate is now the real field constraint
  rather than `True`. Every downstream consumer of `canonicalRule` now gets
  the actual encoding-validity boundary for free.
-/
theorem canonicalRule_babybear :
  ∀ x : Element,
    canonicalRule x ↔ x.val < BABYBEAR_P := by
  intro x
  rfl

/-
  Weaker statements that were explicitly avoided:

    x.val ≠ 0   — does not bound the upper end; values ≥ P would pass.
    x.val ≤ P   — includes P itself, which is out of range (range is [0, P)).

  Only `x.val < BABYBEAR_P` captures full field-element validity.
-/

end BabyBear
