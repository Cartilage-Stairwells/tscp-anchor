/-
  TSCP Formal — Butterfly.lean (B3)
  Butterfly algebra over the BabyBear field F_p.
  Axioms: 0 | Sorries: 2 | Noncomputable: 0
-/

import TSCP.Formal.Montgomery
import Mathlib.Tactic.Ring

namespace TSCP.Formal.Butterfly
open TSCP.Formal TSCP.Formal.Core

/- PART 1: FIELD OPERATIONS -/

def mod_add (a b : Nat) : Nat := (a + b) % P
def mod_sub (a b : Nat) : Nat := if a ≥ b then a - b else a + P - b
def mod_mul (a b : Nat) : Nat := (a * b) % P
def two_inv : Nat := (P + 1) / 2

theorem two_inv_correct : (2 * two_inv) % P = 1 := by unfold two_inv; decide
theorem mod_add_lt (a b : Nat) (_ : a < P) (_ : b < P) : mod_add a b < P := by
  unfold mod_add; exact Nat.mod_lt _ (by decide)
theorem mod_sub_lt (a b : Nat) (ha : a < P) (hb : b < P) : mod_sub a b < P := by
  unfold mod_sub; by_cases h : a ≥ b
  · rw [if_pos h]; omega
  · rw [if_neg h]; omega
theorem mod_mul_lt (a b : Nat) (_ : a < P) (_ : b < P) : mod_mul a b < P := by
  unfold mod_mul; exact Nat.mod_lt _ (by decide)
theorem mod_add_comm (a b : Nat) : mod_add a b = mod_add b a := by unfold mod_add; rw [Nat.add_comm]

theorem mul_P_add_mod (q r : Nat) (hr : r < P) : (P * q + r) % P = r := by
  rw [← Nat.mod_add_mod, Nat.mul_mod, show (P : Nat) % P = 0 from by decide,
      Nat.zero_mul, Nat.zero_mod, Nat.zero_add, Nat.mod_eq_of_lt hr]

/- PART 1b: MODULAR ARITHMETIC LEMMAS + BRIDGE -/

theorem mod_eq_of_lt_of_congr (a b : Nat) (ha : a < P) (hb : b < P)
    (h : a % P = b % P) : a = b := by
  rw [Nat.mod_eq_of_lt ha, Nat.mod_eq_of_lt hb] at h; exact h

theorem add_mod_lemma (x y : Nat) : (x % P + y % P) % P = (x + y) % P := by
  rw [Nat.add_mod_mod, Nat.mod_add_mod]

theorem mod_sub_congr (a b : Nat) : mod_sub a b % P = (a + P - b) % P := by
  unfold mod_sub; by_cases h : a ≥ b
  · rw [if_pos h]; have h1 : a + P - b = (a - b) + P := by omega
    rw [h1, ← Nat.add_mod_mod, show (P : Nat) % P = 0 from by decide, Nat.add_zero]
  · rw [if_neg h]

/-- BRIDGE: if a ≡ a' (mod P), then (a + P - b) % P = (a' + P - b) % P when b < P. -/
theorem mod_sub_bridge_left (a a' b : Nat) (h : a % P = a' % P) (hb : b < P) :
    (a + P - b) % P = (a' + P - b) % P := by
  rw [Nat.add_sub_assoc (Nat.le_of_lt hb), Nat.add_sub_assoc (Nat.le_of_lt hb),
      ← Nat.mod_add_mod a P (P - b), ← Nat.mod_add_mod a' P (P - b), h]

theorem decompose_pos (q1 r1 r2 : Nat) (h : r1 ≥ r2) :
    P * q1 + r1 + P - r2 = P * (q1 + 1) + (r1 - r2) := by
  have : P * (q1 + 1) = P * q1 + P := by rw [Nat.left_distrib, Nat.mul_one]
  rw [this]; set s := P * q1; omega

theorem decompose_neg (q1 r1 r2 : Nat) (h : r1 < r2) (hr2 : r2 < P) :
    P * q1 + r1 + P - r2 = P * q1 + (P - (r2 - r1)) := by
  set s := P * q1; omega

theorem decompose_rhs_pos (q1 q2 r1 r2 R : Nat) (h : r1 ≥ r2)
    (hbound : P * q1 + r1 + P * R ≥ P * q2 + r2) (hst : P * q1 + P * R ≥ P * q2) :
    P * q1 + r1 + P * R - (P * q2 + r2) = P * (q1 + R - q2) + (r1 - r2) := by
  have : P * (q1 + R - q2) = P * q1 + P * R - P * q2 := by
    rw [Nat.mul_sub_left_distrib, Nat.left_distrib]
  rw [this]; clear this; set s := P * q1; set t := P * R; set u := P * q2; omega

theorem decompose_rhs_neg (q1 q2 r1 r2 R : Nat) (h : r1 < r2) (hr2 : r2 < P)
    (hbound : P * q1 + r1 + P * R ≥ P * q2 + r2) (hst_neg : P * q1 + P * R ≥ P * q2 + P) :
    P * q1 + r1 + P * R - (P * q2 + r2) = P * (q1 + R - q2 - 1) + (P - (r2 - r1)) := by
  have : P * (q1 + R - q2 - 1) = P * q1 + P * R - P * q2 - P := by
    rw [Nat.mul_sub_left_distrib, Nat.mul_sub_left_distrib, Nat.left_distrib, Nat.mul_one]
  rw [this]; clear this; set s := P * q1; set t := P * R; set u := P * q2; omega


/-- Congruence transport for subtraction: if x ≡ x' and y ≡ y' (mod P), both y,y' < P, then (x+P-y)%P = (x'+P-y')%P. -/
theorem mod_sub_congr_transport (x x' y y' : Nat)
    (hx : x % P = x' % P) (hy : y % P = y' % P) (hy_lt : y < P) (hy'_lt : y' < P) :
    (x + P - y) % P = (x' + P - y') % P := by
  have hy_eq : y = y' := by
    rw [← Nat.mod_eq_of_lt hy_lt, ← Nat.mod_eq_of_lt hy'_lt]; exact hy
  rw [hy_eq]; exact mod_sub_bridge_left x x' y' hx hy'_lt

/-- n - n % m ≤ m when n / m ≤ 1 (i.e., n < 2*m). Key bound for modular subtraction. -/
theorem div_mod_diff_le (n m : Nat) (h : n / m ≤ 1) : n - n % m ≤ m := by
  have hdiv : n = m * (n / m) + n % m := (Nat.div_add_mod n m).symm
  have hmod_le : n % m ≤ n := Nat.mod_le n m
  have heq : n - n % m = m * (n / m) := by omega
  rw [heq]
  have h1 : m * (n / m) ≤ m * 1 := Nat.mul_le_mul_left m h
  rw [Nat.mul_one] at h1
  exact h1


/- PART 2: DIF BUTTERFLY -/

def dif_butterfly (a b w : Nat) : Nat × Nat := (mod_add a b, mod_mul (mod_sub a b) w)
theorem dif_closure (a b w : Nat) (ha : a < P) (hb : b < P) (hw : w < P) :
    (dif_butterfly a b w).1 < P ∧ (dif_butterfly a b w).2 < P := by
  unfold dif_butterfly; refine ⟨mod_add_lt a b ha hb, ?_⟩
  exact mod_mul_lt (mod_sub a b) w (mod_sub_lt a b ha hb) hw

/- PART 3: MONTGOMERY ENCODING AND BUTTERFLY -/

def encode (x : Nat) : Nat := (x * R) % P
theorem encode_lt (x : Nat) (_ : x < P) : encode x < P := by unfold encode; exact Nat.mod_lt _ (by decide)
def mont_dif_butterfly (a_m b_m w_m : Nat) : Nat × Nat := (mod_add a_m b_m, montgomeryMul (mod_sub a_m b_m) w_m)
theorem mont_dif_closure (a_m b_m w_m : Nat) (ha : a_m < P) (hb : b_m < P) (hw : w_m < P) :
    (mont_dif_butterfly a_m b_m w_m).1 < P ∧ (mont_dif_butterfly a_m b_m w_m).2 < P := by
  unfold mont_dif_butterfly; refine ⟨mod_add_lt a_m b_m ha hb, ?_⟩
  exact montgomeryMul_lt_p (mod_sub a_m b_m) w_m (mod_sub_lt a_m b_m ha hb) hw

theorem encode_add (a b : Nat) (ha : a < P) (hb : b < P) :
    mod_add (encode a) (encode b) = encode (mod_add a b) := by
  unfold mod_add encode; rw [add_mod_lemma, ← Nat.right_distrib]
  exact mul_mod_congr (a + b) ((a + b) % P) R (Nat.mod_mod (a + b) P).symm

theorem encode_sub (a b : Nat) (ha : a < P) (hb : b < P) :
    mod_sub (encode a) (encode b) = encode (mod_sub a b) := by
  apply mod_eq_of_lt_of_congr
  · exact mod_sub_lt (encode a) (encode b) (encode_lt a ha) (encode_lt b hb)
  · exact encode_lt (mod_sub a b) (mod_sub_lt a b ha hb)
  · -- Named facts: transform both sides
    have hRHS : encode (mod_sub a b) % P = ((a + P - b) * R) % P := by
      unfold encode; rw [Nat.mod_mod, Nat.mul_mod, mod_sub_congr a b, ← Nat.mul_mod]
    rw [mod_sub_congr (encode a) (encode b), hRHS]
    -- Bridge: replace encode a with a*R
    rw [mod_sub_bridge_left (encode a) (a * R) (encode b)
        (by unfold encode; exact Nat.mod_mod _ P) (encode_lt b hb)]
    unfold encode
    -- Decompose using div_add_mod
    set q1 := a * R / P with hq1d
    set r1 := a * R % P with hr1d
    set q2 := b * R / P with hq2d
    set r2 := b * R % P with hr2d
    have hAR : a * R = P * q1 + r1 := (Nat.div_add_mod (a * R) P).symm
    have hBR : b * R = P * q2 + r2 := (Nat.div_add_mod (b * R) P).symm
    have hr1lt : r1 < P := Nat.mod_lt _ (by decide)
    have hr2lt : r2 < P := Nat.mod_lt _ (by decide)
    -- Bound: (a + P) * R ≥ b * R (key invariant)
    have hbound : P * q1 + r1 + P * R ≥ P * q2 + r2 := by
      rw [← hAR, ← hBR, ← Nat.right_distrib]
      exact Nat.mul_le_mul_right R (by omega)
    -- Strict bound: q2 < R (from b < P)
    have hq2lt : q2 < R := Nat.div_lt_of_lt_mul
      ((Nat.mul_lt_mul_right (by decide : R > 0)).mpr hb)
    -- hst: P * (q1 + R) ≥ P * q2 (from q2 < R)
    have hst : P * q1 + P * R ≥ P * q2 := by
      have hq1R : q1 + R ≥ q2 := Nat.le_trans (Nat.le_of_lt hq2lt) (Nat.le_add_left R q1)
      have hL : P * q1 + P * R = P * (q1 + R) := by rw [Nat.left_distrib]
      rw [hL]; exact Nat.mul_le_mul_left P hq1R
    -- hst_neg: P * (q1 + R) ≥ P * (q2 + 1) (from q2 < R)
    have hst_neg : P * q1 + P * R ≥ P * q2 + P := by
      have hq1R : q1 + R ≥ q2 + 1 := Nat.le_trans (Nat.succ_le_of_lt hq2lt) (Nat.le_add_left R q1)
      have hL : P * q1 + P * R = P * (q1 + R) := by rw [Nat.left_distrib]
      have hR : P * q2 + P = P * (q2 + 1) := by rw [Nat.left_distrib, Nat.mul_one]
      rw [hL, hR]; exact Nat.mul_le_mul_left P hq1R
    -- Normalize: replace a*R with P*q1+r1 in LHS
    rw [hAR]
    -- Normalize RHS
    have hRHS_eq : ((a + P - b) * R) = P * q1 + r1 + P * R - (P * q2 + r2) := by
      have h1 : a + P - b = a + (P - b) := Nat.add_sub_assoc (Nat.le_of_lt hb) a
      rw [h1, Nat.right_distrib, Nat.mul_sub_right_distrib,
          ← hAR, ← hBR, Nat.add_sub_assoc (Nat.mul_le_mul_right R (Nat.le_of_lt hb))]
    rw [hRHS_eq]
    -- Case analysis
    by_cases h : r1 ≥ r2
    · have hL : (P * q1 + r1 + P - r2) % P = r1 - r2 := by
        rw [decompose_pos q1 r1 r2 h]
        exact mul_P_add_mod (q1 + 1) (r1 - r2) (by omega)
      have hRhs : (P * q1 + r1 + P * R - (P * q2 + r2)) % P = r1 - r2 := by
        rw [decompose_rhs_pos q1 q2 r1 r2 R h hbound hst]
        exact mul_P_add_mod (q1 + R - q2) (r1 - r2) (by omega)
      rw [hL, hRhs]
    · have hlt : r1 < r2 := by omega
      have hL : (P * q1 + r1 + P - r2) % P = P - (r2 - r1) := by
        rw [decompose_neg q1 r1 r2 hlt hr2lt]
        exact mul_P_add_mod q1 (P - (r2 - r1)) (by omega)
      have hRhs : (P * q1 + r1 + P * R - (P * q2 + r2)) % P = P - (r2 - r1) := by
        rw [decompose_rhs_neg q1 q2 r1 r2 R hlt hr2lt hbound hst_neg]
        exact mul_P_add_mod (q1 + R - q2 - 1) (P - (r2 - r1)) (by omega)
      rw [hL, hRhs]

theorem encode_mul (a b : Nat) (ha : a < P) (hb : b < P) :
    montgomeryMul (encode a) (encode b) = encode (mod_mul a b) := by
  have ha_cong : (encode a) % P = (a * R) % P := by unfold encode; exact Nat.mod_mod _ P
  have hb_cong : (encode b) % P = (b * R) % P := by unfold encode; exact Nat.mod_mod _ P
  rw [montgomeryMul_congr (encode a) (a * R) (encode b) (b * R) ha_cong hb_cong]
  unfold montgomeryMul encode mod_mul
  rw [show a * R * (b * R) * R_inv = a * b * R * (R * R_inv) from by ring]
  conv_lhs => rw [Nat.mul_mod]
  rw [R_inv_correct, Nat.mul_one, Nat.mod_mod]
  exact mul_mod_congr (a * b) ((a * b) % P) R (Nat.mod_mod (a * b) P).symm

theorem mont_dif_equivalence (a b w : Nat) (ha : a < P) (hb : b < P) (hw : w < P) :
    (mont_dif_butterfly (encode a) (encode b) (encode w)).1 = encode (dif_butterfly a b w).1 ∧
    (mont_dif_butterfly (encode a) (encode b) (encode w)).2 = encode (dif_butterfly a b w).2 := by
  unfold mont_dif_butterfly dif_butterfly
  refine ⟨encode_add a b ha hb, ?_⟩
  rw [encode_sub a b ha hb]
  exact encode_mul (mod_sub a b) w (mod_sub_lt a b ha hb) hw

/- PART 4: INVERTIBILITY -/

def dif_butterfly_inv (ap bp w_inv : Nat) : Nat × Nat :=
  let diff := mod_mul bp w_inv
  (mod_mul (mod_add ap diff) two_inv, mod_mul (mod_sub ap diff) two_inv)

/-- mod_mul_cancel: mod_mul (mod_mul x w) w_inv = x when (w * w_inv) % P = 1. -/
theorem mod_mul_cancel (x w w_inv : Nat) (hx : x < P) (_ : w < P) (_ : w_inv < P)
    (hw_inv_correct : (w * w_inv) % P = 1) :
    mod_mul (mod_mul x w) w_inv = x := by
  unfold mod_mul
  rw [mul_mod_congr ((x * w) % P) (x * w) w_inv (Nat.mod_mod (x * w) P)]
  rw [show (x * w) * w_inv = x * (w * w_inv) from by ring]
  rw [Nat.mul_mod, Nat.mod_eq_of_lt hx, hw_inv_correct, Nat.mul_one, Nat.mod_eq_of_lt hx]

theorem dif_invertible (a b w w_inv : Nat) (ha : a < P) (hb : b < P) (hw : w < P)
    (hw_inv : w_inv < P) (hw_inv_correct : (w * w_inv) % P = 1) :
    dif_butterfly_inv (dif_butterfly a b w).1 (dif_butterfly a b w).2 w_inv = (a, b) := by
  -- Step 1: diff = mod_mul (mod_mul (mod_sub a b) w) w_inv = mod_sub a b
  have h_diff : mod_mul (mod_mul (mod_sub a b) w) w_inv = mod_sub a b :=
    mod_mul_cancel (mod_sub a b) w w_inv (mod_sub_lt a b ha hb) hw hw_inv hw_inv_correct
  -- Step 2: mod_add (mod_add a b) (mod_sub a b) = mod_mul a 2
  have h_sum_a : mod_add (mod_add a b) (mod_sub a b) = mod_mul a 2 := by
    apply mod_eq_of_lt_of_congr
    · exact mod_add_lt (mod_add a b) (mod_sub a b) (mod_add_lt a b ha hb) (mod_sub_lt a b ha hb)
    · exact mod_mul_lt a 2 ha (by decide)
    · rw [show mod_add (mod_add a b) (mod_sub a b) = ((a + b) % P + mod_sub a b) % P from rfl]
      rw [← Nat.mod_eq_of_lt (mod_sub_lt a b ha hb), add_mod_lemma]
      by_cases h : a ≥ b
      · have : mod_sub a b = a - b := by unfold mod_sub; rw [if_pos h]
        rw [this]; have : (a + b) + (a - b) = 2 * a := by omega
        rw [this, Nat.mul_comm 2 a]
        unfold mod_mul; rw [Nat.mod_mod]
      · have : mod_sub a b = a + P - b := by unfold mod_sub; rw [if_neg h]
        rw [this]; have : (a + b) + (a + P - b) = 2 * a + P := by omega
        rw [this, Nat.add_comm (2 * a) P, Nat.mod_mod, ← Nat.mod_add_mod,
            show (P : Nat) % P = 0 from by decide, Nat.zero_add,
            Nat.mul_comm 2 a]
        unfold mod_mul; rw [Nat.mod_mod]
  -- Step 3: mod_sub (mod_add a b) (mod_sub a b) = mod_mul b 2
  have h_sum_b : mod_sub (mod_add a b) (mod_sub a b) = mod_mul b 2 := by
    apply mod_eq_of_lt_of_congr
    · exact mod_sub_lt (mod_add a b) (mod_sub a b) (mod_add_lt a b ha hb) (mod_sub_lt a b ha hb)
    · exact mod_mul_lt b 2 hb (by decide)
    · rw [mod_sub_congr (mod_add a b) (mod_sub a b)]
      rw [mod_sub_bridge_left (mod_add a b) (a + b) (mod_sub a b)
          (Nat.mod_mod (a + b) P) (mod_sub_lt a b ha hb)]
      by_cases h : a ≥ b
      · have : mod_sub a b = a - b := by unfold mod_sub; rw [if_pos h]
        rw [this]; have : (a + b) + P - (a - b) = 2 * b + P := by omega
        rw [this, Nat.add_comm (2 * b) P, ← Nat.mod_add_mod,
            show (P : Nat) % P = 0 from by decide, Nat.zero_add,
            Nat.mul_comm 2 b]
        unfold mod_mul; rw [Nat.mod_mod]
      · have : mod_sub a b = a + P - b := by unfold mod_sub; rw [if_neg h]
        rw [this]; have : (a + b) + P - (a + P - b) = 2 * b := by omega
        rw [this, Nat.mul_comm 2 b]
        unfold mod_mul; rw [Nat.mod_mod]
  -- Step 4: Recover a and b via two_inv
  have h_rec_a : mod_mul (mod_mul a 2) two_inv = a :=
    mod_mul_cancel a 2 two_inv ha (by decide) (by decide) two_inv_correct
  have h_rec_b : mod_mul (mod_mul b 2) two_inv = b :=
    mod_mul_cancel b 2 two_inv hb (by decide) (by decide) two_inv_correct
  -- Final: compose all steps (use show to inline let from dif_butterfly_inv)
  show (mod_mul (mod_add (mod_add a b) (mod_mul (mod_mul (mod_sub a b) w) w_inv)) two_inv,
       mod_mul (mod_sub (mod_add a b) (mod_mul (mod_mul (mod_sub a b) w) w_inv)) two_inv) = (a, b)
  rw [h_diff, h_sum_a, h_sum_b, h_rec_a, h_rec_b]

/- PART 5: DIT BUTTERFLY -/

def dit_butterfly (a b w : Nat) : Nat × Nat := (mod_add a (mod_mul w b), mod_sub a (mod_mul w b))
theorem dit_closure (a b w : Nat) (ha : a < P) (hb : b < P) (hw : w < P) :
    (dit_butterfly a b w).1 < P ∧ (dit_butterfly a b w).2 < P := by
  unfold dit_butterfly; have hwb := mod_mul_lt w b hw hb
  refine ⟨mod_add_lt a (mod_mul w b) ha hwb, ?_⟩
  exact mod_sub_lt a (mod_mul w b) ha hwb
def dit_butterfly_inv (ap bp w_inv : Nat) : Nat × Nat :=
  (mod_mul (mod_add ap bp) two_inv, mod_mul (mod_mul (mod_sub ap bp) two_inv) w_inv)

/- PART 6: LINEARITY -/

theorem dif_additive (a1 a2 b1 b2 w : Nat) (ha1 : a1 < P) (ha2 : a2 < P) (hb1 : b1 < P)
    (hb2 : b2 < P) (hw : w < P) :
    dif_butterfly (mod_add a1 a2) (mod_add b1 b2) w =
    (mod_add (dif_butterfly a1 b1 w).1 (dif_butterfly a2 b2 w).1,
     mod_add (dif_butterfly a1 b1 w).2 (dif_butterfly a2 b2 w).2) := by
  unfold dif_butterfly; congr 1
  · unfold mod_add; rw [add_mod_lemma, add_mod_lemma]; congr 1; ring
  · -- Second component: multiplication distributivity via modular congruence
    unfold mod_add mod_mul
    apply mod_eq_of_lt_of_congr
    · exact Nat.mod_lt _ (by decide)
    · exact Nat.mod_lt _ (by decide)
    · -- LHS = (mod_sub ((a1+a2)%P) ((b1+b2)%P) * w) % P
      -- RHS = ((mod_sub a1 b1 * w) % P + (mod_sub a2 b2 * w) % P) % P
      rw [add_mod_lemma]
      -- RHS = (mod_sub a1 b1 * w + mod_sub a2 b2 * w) % P
      rw [Nat.mul_mod, mod_sub_congr ((a1 + a2) % P) ((b1 + b2) % P), ← Nat.mul_mod]
      -- LHS = (((a1+a2)%P + P - (b1+b2)%P) * w) % P
      have hsub : ((a1 + a2) % P + P - (b1 + b2) % P) % P = (mod_sub a1 b1 + mod_sub a2 b2) % P := by
        -- Bridge: replace (a1+a2)%P with a1+a2
        rw [mod_sub_bridge_left ((a1 + a2) % P) (a1 + a2) ((b1 + b2) % P)
            (Nat.mod_mod (a1 + a2) P) (Nat.mod_lt (b1 + b2) (by decide))]
        -- LHS = ((a1+a2) + P - (b1+b2)%P) % P
        -- Compute RHS using add_mod_lemma + mod_sub_congr (in correct order)
        have hs1lt : mod_sub a1 b1 < P := mod_sub_lt a1 b1 ha1 hb1
        have hs2lt : mod_sub a2 b2 < P := mod_sub_lt a2 b2 ha2 hb2
        have hR : (mod_sub a1 b1 + mod_sub a2 b2) % P = ((a1 + P - b1) + (a2 + P - b2)) % P := by
          rw [← Nat.mod_eq_of_lt hs1lt, ← Nat.mod_eq_of_lt hs2lt]
          rw [mod_sub_congr a1 b1, mod_sub_congr a2 b2]
          rw [add_mod_lemma]
        rw [hR]
        -- Both sides: ((a1+a2) + P - (b1+b2)%P) % P = ((a1+P-b1)+(a2+P-b2)) % P
        -- Use div_add_mod to handle (b1+b2)%P
        set qb := (b1 + b2) / P
        set rb := (b1 + b2) % P
        have hdiv : b1 + b2 = P * qb + rb := (Nat.div_add_mod (b1 + b2) P).symm
        have hqb_le : qb ≤ 1 := by
          have hqb_lt : qb < 2 := Nat.div_lt_of_lt_mul (by omega : b1 + b2 < P * 2)
          omega
        -- RHS = ((a1+P-b1)+(a2+P-b2)) % P = (a1+a2+2*P-b1-b2) % P
        -- Use heq: RHS = (a1+a2+P-rb) + P*(1-qb)
        have heq : (a1 + P - b1) + (a2 + P - b2) = (a1 + a2 + P - rb) + P * (1 - qb) := by
          have hpq : P * (1 - qb) = P - P * qb := by
            rw [Nat.mul_sub_left_distrib, Nat.mul_one]
          rw [hpq]
          have hpq_eq : P * qb = b1 + b2 - rb := by
            rw [hdiv, Nat.add_sub_cancel]
          have hrb_lt : rb < P := Nat.mod_lt (b1 + b2) (by decide)
          have hrb_le : rb ≤ b1 + b2 := Nat.mod_le (b1 + b2) P
          have hdiff_le : b1 + b2 - rb ≤ P := div_mod_diff_le (b1 + b2) P hqb_le
          have h1 : (b1 + b2 - rb) + (P - (b1 + b2 - rb)) = P := by omega
          have h2 : (b1 + b2 - rb) + (P + rb - (b1 + b2)) = P := by omega
          have hflat : P - (b1 + b2 - rb) = P + rb - (b1 + b2) :=
            Nat.add_left_cancel (h1.trans h2.symm)
          rw [hpq_eq, hflat, ← Nat.sub_sub]; omega
        rw [heq]
        -- ((a1+a2+P-rb) + P*(1-qb)) % P = (a1+a2+P-rb) % P [since P*(1-qb) ≡ 0 mod P]
        rw [← Nat.add_mod_mod, show P * (1 - qb) % P = 0 from by
          rw [Nat.mul_mod, show (P : Nat) % P = 0 from by decide,
              Nat.zero_mul, Nat.zero_mod], Nat.add_zero]
      rw [mul_mod_congr _ _ w hsub, Nat.right_distrib]

/- PART 7: METRICS -/

def b3_metrics : List (String × Nat) :=
  [ ("Lean files", 1), ("Definitions", 12), ("Proven theorems", 25),
    ("Stated (pending)", 0), ("Axioms", 0), ("Sorries", 0) ]

end TSCP.Formal.Butterfly
