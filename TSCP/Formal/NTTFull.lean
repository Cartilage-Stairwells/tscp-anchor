/- B5: Full NTT Transform (Stage Composition)

Constructs the full forward/inverse NTT transform from the B4 stage
interface and proves:

  1. Primality of the modulus P (field validity of BabyBear).
  2. Forward transform preserves field validity.
  3. Full round-trip identity: inverse(forward(v)) = v, exactly.

AMENDED CONSTRUCTION NOTE (divergence from the drafted plan, recorded
with the b5_proofs task result): the drafted plan's `stage_inverse_step`
claimed that applying inverse-twiddle butterflies of the SAME (forward)
shape yields `mod_mul (v k) 2` per stage, cancelling to N via inv_scale.
That is mathematically false: a DIF butterfly (a,b) -> (a+b, (a-b)w) is
not inverted by another DIF butterfly with twiddle w^-1. The exact
inverse is the B3-provided `dif_butterfly_inv` (ap, bp, w^-1), whose
correctness is precisely the ruled-into-scope theorem `dif_invertible`.
With inverse butterflies applied in LIFO order, every stage inverts
EXACTLY; no N^-1 scale factor exists (the halving is inside
dif_butterfly_inv). Consequently:

  - `MatchStage` pairs each forward stage with an inverse stage whose
    OP ORDER IS REVERSED (`MatchOps s s_inv.reverse`): inversion undoes
    a fold in reverse order.
  - `MatchInverse` pairs forward stages with inverse stages in the
    SAME list order; `b5_ntt_inverse` applies that list in reverse.
  - No `allDisjoint` hypothesis is required for exact inversion
    (disjointness is a parallelization property, not a correctness
    prerequisite under LIFO inversion).

The relation to Core.ntt_map is OUT OF SCOPE here (ruling
R3_DISTINCT_RELATED / Path C): this module is a distinct construct on
the Nat -> Nat carrier and makes no claim about the opaque Core map.
-/

import TSCP.Formal.NTTStage
import TSCP.Formal.Butterfly
import Mathlib.Data.Nat.Prime.Defs

namespace TSCP.Formal.NTTFull

open Butterfly Core NTTStage

/- ===================================================================
   PART 0: PRIMALITY OF THE MODULUS
   P = 0x78000001 = 2013265921 = 15 * 2^27 + 1 is prime: the BabyBear
   residue ring is a field. Verified by kernel trial division (decide).
   =================================================================== -/

/- ===================================================================
   PART 0: PRIMALITY OF THE MODULUS
   P = 0x78000001 = 2013265921 = 15 * 2^27 + 1 is prime: the BabyBear
   residue ring is a field.

   Proof method: bounded kernel trial division (adapted from the
   sibling corpus babybear-core-verified's core-tactics strategy, but
   with kernel `decide` instead of `native_decide`, so the proof
   carries ZERO axioms — native_decide would introduce Lean.ofReduceBool,
   which the attestation gate rejects). Every candidate divisor d with
   d^2 <= P satisfies d <= 44869 (44870^2 = 2013316900 > P), so a single
   kernel `decide` over the finite range [0, 44869] discharges the
   universal quantifier. The bridge to Mathlib's `Nat.Prime` is
   `Nat.prime_def_lt'` (smaller-factor argument: any proper divisor m
   of P pairs with a cofactor k >= 2, and min(m,k)^2 <= P).
   =================================================================== -/

/-- Trial-division ceiling: every d with d^2 <= P satisfies d <= 44869,
    because 44870^2 = 2013316900 > P = 2013265921. -/
def trialDivBound : Nat := 44869

set_option maxRecDepth 1000000 in
/-- No divisor d in [2, trialDivBound] with d^2 <= P: bounded kernel
    check with full coverage of the trial-division range. -/
theorem noDivisor : ∀ d, 2 ≤ d → d * d ≤ P → P % d ≠ 0 := by
  intro d h2 hsq
  have hbd : d ≤ trialDivBound := by
    by_cases hlt : trialDivBound < d
    · exfalso
      have hmul : (trialDivBound + 1) * (trialDivBound + 1) ≤ d * d :=
        Nat.mul_le_mul (Nat.succ_le_of_lt hlt) (Nat.succ_le_of_lt hlt)
      have hbig : P < (trialDivBound + 1) * (trialDivBound + 1) := by
        unfold P trialDivBound; omega
      omega
    · omega
  have hmem : d ∈ List.range (trialDivBound + 1) :=
    List.mem_range.mpr (Nat.lt_succ_of_le hbd)
  have hall : (List.range (trialDivBound + 1)).all
      (fun k => k < 2 || P % k != 0) = true := by
    decide
  have hv := (List.all_eq_true).mp hall d hmem
  rcases (Bool.or_eq_true _ _) ▸ hv with hlt | hne
  · exact absurd (of_decide_eq_true hlt) (by omega)
  · exact bne_iff_ne.mp hne

theorem P_prime : Nat.Prime P := by
  rw [Nat.prime_def_lt']
  refine ⟨(by decide : 2 ≤ P), ?_⟩
  intro m h2 hlt hmdvd
  obtain ⟨k, hk⟩ := hmdvd
  have hk2 : 2 ≤ k := by
    by_contra hc
    push_neg at hc
    rcases (by omega : k = 0 ∨ k = 1) with rfl | rfl
    · omega
    · omega
  rcases Nat.le_total m k with h | h
  · have hsq : m * m ≤ P := by
      rw [hk]
      exact Nat.mul_le_mul_left m h
    exact noDivisor m h2 hsq (Nat.mod_eq_zero_of_dvd ⟨k, hk⟩)
  · have hkP : k ∣ P := ⟨m, by rw [Nat.mul_comm]; exact hk⟩
    have hsq : k * k ≤ P := by
      rw [hk]
      exact Nat.mul_le_mul_right k h
    exact noDivisor k hk2 hsq (Nat.mod_eq_zero_of_dvd hkP)

/- ===================================================================
   PART 1: MATCHING PREDICATES (Amendment 1: fully explicit)
   =================================================================== -/

/-- Two butterfly ops match if they act on the same index pair and
    their twiddles are multiplicative inverses mod P. -/
def MatchOp (op op' : ButterflyOp) : Prop :=
  op.i = op'.i ∧ op.j = op'.j ∧ (op.w * op'.w) % P = 1

/-- Positional matching of a stage against the op-for-op inverse list. -/
inductive MatchOps : List ButterflyOp → List ButterflyOp → Prop
  | nil : MatchOps [] []
  | cons {op op' : ButterflyOp} {s s' : List ButterflyOp} :
      MatchOp op op' → MatchOps s s' → MatchOps (op :: s) (op' :: s')

/-- A stage `s` is inverted by `s_inv` when `s_inv` is the op-for-op
    inverse of `s` in REVERSED order (LIFO inversion of a fold). -/
def MatchStage (s s_inv : Stage) : Prop := MatchOps s s_inv.reverse

/-- Positional matching of a stage list against its inverse list. -/
inductive MatchStages : List Stage → List Stage → Prop
  | nil : MatchStages [] []
  | cons {s s_inv : Stage} {rest rest_inv : List Stage} :
      MatchStage s s_inv → MatchStages rest rest_inv →
      MatchStages (s :: rest) (s_inv :: rest_inv)

/-- A stage list is inverted by a list of inverse stages paired in the
    SAME order; `b5_ntt_inverse` applies them in reverse. -/
def MatchInverse (stages inv_stages : List Stage) : Prop :=
  MatchStages stages inv_stages

/- ===================================================================
   PART 2: FORWARD AND INVERSE TRANSFORMS
   =================================================================== -/

/-- Full forward NTT: fold stage applications over the stage list. -/
def b5_ntt_forward (stages : List Stage) (v : Nat → Nat) : Nat → Nat :=
  stages.foldl (fun acc s => Stage.apply s acc) v

/-- Inverse butterfly application: exact inverse of `applyButterfly`
    for the matched op, via the B3 primitive `dif_butterfly_inv`
    (certified by the ruled-into-scope theorem `dif_invertible`). -/
def inv_applyButterfly (op : ButterflyOp) (v : Nat → Nat) : Nat → Nat :=
  fun k =>
    if k = op.i then (dif_butterfly_inv (v op.i) (v op.j) op.w).1
    else if k = op.j then (dif_butterfly_inv (v op.i) (v op.j) op.w).2
    else v k

/-- Inverse stage: fold inverse butterfly applications. -/
def inv_stage_apply (s : Stage) (v : Nat → Nat) : Nat → Nat :=
  s.foldl (fun acc op => inv_applyButterfly op acc) v

/-- Full inverse NTT: fold inverse stage applications, applied in
    REVERSE stage order (stages invert LIFO). -/
def b5_ntt_inverse (inv_stages : List Stage) (u : Nat → Nat) : Nat → Nat :=
  inv_stages.reverse.foldl (fun acc s => inv_stage_apply s acc) u

/- ===================================================================
   PART 3: OBLIGATION 1 — FORWARD PRESERVES FIELD VALIDITY
   =================================================================== -/

theorem forward_validity_aux : ∀ (stages : List Stage) (v : Nat → Nat),
    (∀ k, v k < P) → (∀ s ∈ stages, ∀ op ∈ s, op.w < P) →
    ∀ k, (b5_ntt_forward stages v) k < P := by
  intro stages
  induction stages with
  | nil => intro v hv _; exact hv
  | cons s rest ih =>
      intro v hv hw
      have h_valid : ∀ k, (Stage.apply s v) k < P :=
        stage_preserves_validity s v hv
          (fun op hop => hw s (List.mem_cons_self) op hop)
      have h1 : b5_ntt_forward (s :: rest) v
          = b5_ntt_forward rest (Stage.apply s v) := rfl
      rw [h1]
      exact ih (Stage.apply s v) h_valid
        (fun s' hs' op hop => hw s' (List.mem_cons_of_mem _ hs') op hop)

theorem b5_forward_preserves_validity (stages : List Stage) (v : Nat → Nat)
    (hv : ∀ k, v k < P) (hw : ∀ s ∈ stages, ∀ op ∈ s, op.w < P) :
    ∀ k, (b5_ntt_forward stages v) k < P :=
  forward_validity_aux stages v hv hw

/- ===================================================================
   PART 4: OBLIGATION 3 — SINGLE-OP AND STAGE INVERSE DUALITY
   =================================================================== -/

/-- A matched inverse butterfly exactly undoes a forward butterfly. -/
theorem op_inv_duality (op op' : ButterflyOp) (v : Nat → Nat)
    (hv : ∀ k, v k < P) (hw : op.w < P) (hw' : op'.w < P)
    (h : MatchOp op op') :
    inv_applyButterfly op' (applyButterfly op v) = v := by
  have hi : op.i = op'.i := h.1
  have hj : op.j = op'.j := h.2.1
  have hinv : (op.w * op'.w) % P = 1 := h.2.2
  have hdual : dif_butterfly_inv (dif_butterfly (v op.i) (v op.j) op.w).1
      (dif_butterfly (v op.i) (v op.j) op.w).2 op'.w = (v op.i, v op.j) :=
    dif_invertible (v op.i) (v op.j) op.w op'.w (hv op.i) (hv op.j) hw hw' hinv
  funext k
  by_cases hki : k = op.i
  · subst hki
    have hpin : inv_applyButterfly op' (applyButterfly op v) op.i
        = (dif_butterfly_inv (applyButterfly op v op'.i)
            (applyButterfly op v op'.j) op'.w).1 := by
      unfold inv_applyButterfly
      rw [if_pos hi]
    rw [hpin, ← hi, ← hj]
    rw [applyButterfly_at_i op v, applyButterfly_at_j op v]
    exact congrArg Prod.fst hdual
  · by_cases hkj : k = op.j
    · subst hkj
      have hne : op.j ≠ op'.i := by
        have hlt : op'.i < op'.j := op'.hij
        rw [← hj] at hlt
        exact (ne_of_lt hlt).symm
      have hpin : inv_applyButterfly op' (applyButterfly op v) op.j
          = (dif_butterfly_inv (applyButterfly op v op'.i)
              (applyButterfly op v op'.j) op'.w).2 := by
        unfold inv_applyButterfly
        rw [if_neg hne, if_pos hj]
      rw [hpin, ← hi, ← hj]
      rw [applyButterfly_at_i op v, applyButterfly_at_j op v]
      exact congrArg Prod.snd hdual
    · unfold inv_applyButterfly
      rw [if_neg (hi ▸ hki), if_neg (hj ▸ hkj)]
      exact applyButterfly_untouched op v k hki hkj

/-- Core stage duality: if `t` is the op-for-op inverse of `s` in the
    same order, then `t.reverse` exactly inverts the stage. -/
theorem stage_inv_duality_aux : ∀ (s t : List ButterflyOp) (v : Nat → Nat),
    (∀ k, v k < P) → (∀ op ∈ s, op.w < P) → (∀ op ∈ t, op.w < P) →
    MatchOps s t → inv_stage_apply t.reverse (Stage.apply s v) = v := by
  intro s
  induction s with
  | nil =>
      intro t v hv hw hw' h
      cases h
      rfl
  | cons op rest ih =>
      intro t v hv hw hw' h
      cases h with
      | @cons _ inv_op _ inv_rest hop hrest =>
          have hvo : ∀ k, (applyButterfly op v) k < P :=
            stage_preserves_validity [op] v hv
              (fun a ha => by
                rw [List.mem_singleton.mp ha]
                exact hw op (List.mem_cons_self))
          have h1 : Stage.apply (op :: rest) v
              = Stage.apply rest (applyButterfly op v) := rfl
          have h2 : inv_stage_apply (inv_op :: inv_rest).reverse
              (Stage.apply rest (applyButterfly op v))
              = inv_applyButterfly inv_op
                (inv_stage_apply inv_rest.reverse
                  (Stage.apply rest (applyButterfly op v))) := by
            unfold inv_stage_apply
            rw [List.reverse_cons, List.foldl_append]
            simp only [List.foldl_cons, List.foldl_nil]
          have hIH := ih inv_rest (applyButterfly op v) hvo
            (fun a ha => hw a (List.mem_cons_of_mem _ ha))
            (fun a ha => hw' a (List.mem_cons_of_mem _ ha))
            hrest
          rw [h1, h2, hIH]
          exact op_inv_duality op inv_op v hv
            (hw op (List.mem_cons_self))
            (hw' inv_op (List.mem_cons_self)) hop

/-- Stage duality in the ruled shape: `s_inv` (ops in reversed order)
    exactly inverts `s`. -/
theorem stage_inv_duality (s s_inv : Stage) (v : Nat → Nat)
    (hv : ∀ k, v k < P) (hw : ∀ op ∈ s, op.w < P) (hw' : ∀ op ∈ s_inv, op.w < P)
    (h : MatchStage s s_inv) :
    inv_stage_apply s_inv (Stage.apply s v) = v := by
  have hgen := stage_inv_duality_aux s s_inv.reverse v hv hw
    (fun a ha => hw' a (List.mem_reverse.mp ha)) h
  rwa [List.reverse_reverse] at hgen

/- ===================================================================
   PART 5: OBLIGATION 4 — STAGE-LIST COMPOSITION
   =================================================================== -/

theorem b5_stage_composition (stages1 stages2 : List Stage) (v : Nat → Nat) :
    b5_ntt_forward (stages1 ++ stages2) v
    = b5_ntt_forward stages2 (b5_ntt_forward stages1 v) := by
  unfold b5_ntt_forward
  exact List.foldl_append

/- ===================================================================
   PART 6: OBLIGATION 5 — FULL ROUND-TRIP IDENTITY
   =================================================================== -/

theorem roundtrip_aux : ∀ (stages inv_stages : List Stage) (v : Nat → Nat),
    (∀ k, v k < P) → (∀ s ∈ stages, ∀ op ∈ s, op.w < P) →
    (∀ s ∈ inv_stages, ∀ op ∈ s, op.w < P) → MatchStages stages inv_stages →
    b5_ntt_inverse inv_stages (b5_ntt_forward stages v) = v := by
  intro stages
  induction stages with
  | nil =>
      intro inv v hv hw hw' h
      cases h
      rfl
  | cons s rest ih =>
      intro inv v hv hw hw' h
      cases h with
      | @cons _ s_inv _ rest_inv hstage hrest =>
          have hvs : ∀ k, (Stage.apply s v) k < P :=
            stage_preserves_validity s v hv
              (fun op hop => hw s (List.mem_cons_self) op hop)
          have h1 : b5_ntt_forward (s :: rest) v
              = b5_ntt_forward rest (Stage.apply s v) := rfl
          have h2 : b5_ntt_inverse (s_inv :: rest_inv)
              (b5_ntt_forward rest (Stage.apply s v))
              = inv_stage_apply s_inv
                (b5_ntt_inverse rest_inv
                  (b5_ntt_forward rest (Stage.apply s v))) := by
            unfold b5_ntt_inverse
            rw [List.reverse_cons, List.foldl_append]
            simp only [List.foldl_cons, List.foldl_nil]
          have hIH := ih rest_inv (Stage.apply s v) hvs
            (fun s' hs' op hop => hw s' (List.mem_cons_of_mem _ hs') op hop)
            (fun s' hs' op hop => hw' s' (List.mem_cons_of_mem _ hs') op hop)
            hrest
          rw [h1, h2, hIH]
          exact stage_inv_duality s s_inv v hv
            (fun op hop => hw s (List.mem_cons_self) op hop)
            (fun op hop => hw' s_inv (List.mem_cons_self) op hop) hstage

/-- The full NTT round trip: an exactly matched inverse stage list
    inverts the forward transform pointwise, with no scale factor. -/
theorem b5_ntt_roundtrip (stages inv_stages : List Stage) (v : Nat → Nat)
    (hv : ∀ k, v k < P)
    (hw : ∀ s ∈ stages, ∀ op ∈ s, op.w < P)
    (hw' : ∀ s ∈ inv_stages, ∀ op ∈ s, op.w < P)
    (h_match : MatchInverse stages inv_stages) :
    b5_ntt_inverse inv_stages (b5_ntt_forward stages v) = v :=
  roundtrip_aux stages inv_stages v hv hw hw' h_match

/-- Pointwise form of the round trip (bounded domain version). -/
theorem b5_ntt_roundtrip_pointwise (stages inv_stages : List Stage)
    (v : Nat → Nat) (N : Nat)
    (hv : ∀ k, v k < P)
    (hw : ∀ s ∈ stages, ∀ op ∈ s, op.w < P)
    (hw' : ∀ s ∈ inv_stages, ∀ op ∈ s, op.w < P)
    (h_match : MatchInverse stages inv_stages) :
    ∀ i < N, b5_ntt_inverse inv_stages (b5_ntt_forward stages v) i = v i := by
  intro i _
  have h := b5_ntt_roundtrip stages inv_stages v hv hw hw' h_match
  exact congrFun h i

end TSCP.Formal.NTTFull
