import TSCP.Formal.Core

namespace TSCP.Formal

open Core

def R : Nat := 2 ^ 32
def R_inv : Nat := 0x38400000
def NEG_INV : Nat := 0x77FFFFFF
theorem R_inv_correct : (R * R_inv) % P = 1 := by decide
theorem montgomery_radix_coprime : Nat.gcd R P = 1 := by decide
theorem neg_inv_correct : (P * NEG_INV + 1) % R = 0 := by decide

structure MontgomeryElem where
  val : Nat
  isLt : val < P

def encodeMontgomery (x : BabyBearElem) : MontgomeryElem :=
  ⟨(x.val * R) % P, by exact Nat.mod_lt _ (by decide)⟩
def decodeMontgomery (x : MontgomeryElem) : BabyBearElem :=
  ⟨(x.val * R_inv) % P, by exact Nat.mod_lt _ (by decide)⟩

theorem decode_encode_roundtrip (x : BabyBearElem) :
    decodeMontgomery (encodeMontgomery x) = x := by
  apply Subtype.ext
  simp [decodeMontgomery, encodeMontgomery]
  rw [Nat.mul_assoc, Nat.mul_mod, R_inv_correct]
  rw [Nat.mod_eq_of_lt x.property, Nat.mul_one]
  exact Nat.mod_eq_of_lt x.property

def montgomeryMul (a b : Nat) : Nat := (a * b * R_inv) % P
theorem montgomeryMul_lt_p (a b : Nat) (ha : a < P) (hb : b < P) :
    montgomeryMul a b < P := by exact Nat.mod_lt _ (by decide)

theorem mul_mod_congr (a a' c : Nat) (h : a % P = a' % P) :
    (a * c) % P = (a' * c) % P := by
  rw [Nat.mul_mod a c P, Nat.mul_mod a' c P, h]

theorem montgomeryMul_congr
    (a a' b b' : Nat) (ha : a % P = a' % P) (hb : b % P = b' % P) :
    montgomeryMul a b = montgomeryMul a' b' := by
  unfold montgomeryMul
  rw [Nat.mul_assoc, Nat.mul_assoc]
  rw [mul_mod_congr a a' (b * R_inv) ha]
  have hbi : (b * R_inv) % P = (b' * R_inv) % P := mul_mod_congr b b' R_inv hb
  rw [Nat.mul_mod a' (b * R_inv) P, Nat.mul_mod a' (b' * R_inv) P, hbi]

def montgomeryMulElem (a b : MontgomeryElem) : MontgomeryElem :=
  ⟨montgomeryMul a.val b.val, montgomeryMul_lt_p a.val b.val a.isLt b.isLt⟩

def scalarMul (a b : Nat) : Nat :=
  let t := a * b
  let m := (t % R) * NEG_INV % R
  let u := (t + m * P) / R
  if u ≥ P then u - P else u


/-- Conditional subtraction equals mod P when u < 2P -/
theorem cond_sub_eq_mod (u P : Nat) (h : u < 2 * P) (hP : 0 < P) :
    (if u ≥ P then u - P else u) = u % P := by
  by_cases huP : u ≥ P
  · rw [if_pos huP]
    -- u/P = 1 since P ≤ u < 2P
    have h_comm : u < P * 2 := by omega
    have h_upper : u / P < 2 := Nat.div_lt_of_lt_mul h_comm
    have h_lower : 1 ≤ u / P := by
      rw [Nat.le_div_iff_mul_le hP, Nat.one_mul]; exact huP
    have h_div : u / P = 1 := by omega
    -- u = u/P * P + u%P = 1 * P + u%P = P + u%P
    have h_dam := Nat.div_add_mod u P
    rw [h_div, Nat.mul_one] at h_dam
    -- h_dam : u = P + u%P → u%P = u - P (linear)
    omega
  · rw [if_neg huP, Nat.mod_eq_of_lt (Nat.lt_of_not_ge huP)]

theorem P_neg_inv_mod_R : (P * NEG_INV) % R = R - 1 := by decide

theorem cios_exact (t : Nat) :
    (t + (t % R * NEG_INV % R) * P) % R = 0 := by
  have hred : (t + (t % R * NEG_INV % R) * P) % R
             = (t % R + (t % R * NEG_INV % R) * P) % R := by
    have hR : R = 4294967296 := by decide
    have hP : P = 2013265921 := by decide
    have hN : NEG_INV = 2013265919 := by decide
    have hq : t % R < R := Nat.mod_lt _ (by decide)
    rw [hR, hP, hN]; omega
  rw [hred]
  have hsplit : (t % R + (t % R * NEG_INV % R) * P) % R
                = (t % R + (t % R * NEG_INV % R * P) % R) % R := by
    have hR : R = 4294967296 := by decide
    have hP : P = 2013265921 := by decide
    have hN : NEG_INV = 2013265919 := by decide
    have hq : t % R < R := Nat.mod_lt _ (by decide)
    rw [hR, hP, hN]; omega
  rw [hsplit]
  have h1 : (t % R * NEG_INV % R * P) % R = (t % R * (R - 1)) % R := by
    have hR : R = 4294967296 := by decide
    have hP : P = 2013265921 := by decide
    have hN : NEG_INV = 2013265919 := by decide
    have hpn : (P * NEG_INV) % R = R - 1 := P_neg_inv_mod_R
    have hq : t % R < R := Nat.mod_lt _ (by decide)
    rw [hR, hP, hN]; omega
  rw [h1]
  have h2 : (t % R * (R - 1)) % R = (R - t % R) % R := by
    have hR : R = 4294967296 := by decide
    have hq : t % R < R := Nat.mod_lt _ (by decide)
    rw [hR]; omega
  rw [h2]
  have h3 : (t % R + (R - t % R) % R) % R = 0 := by
    have hR : R = 4294967296 := by decide
    have hq : t % R < R := Nat.mod_lt _ (by decide)
    rw [hR]; omega
  exact h3

theorem montgomeryMul_scalar_correct (a b : Nat) (ha : a < P) (hb : b < P) :
    scalarMul a b = montgomeryMul a b := by
  -- Exact division: T = u * R
  have hexact := cios_exact (a * b)
  have hdiv : (a * b + (a * b % R * NEG_INV % R) * P)
              = (a * b + (a * b % R * NEG_INV % R) * P) / R * R := by
    have key := Nat.div_add_mod (a * b + (a * b % R * NEG_INV % R) * P) R
    rw [hexact, Nat.add_zero] at key
    rw [Nat.mul_comm] at key
    exact key.symm
  -- T % P = a*b % P (correction is 0 mod P)
  have hT_mod_P : (a * b + (a * b % R * NEG_INV % R) * P) % P = a * b % P := by
    have hR : R = 4294967296 := by decide
    have hP : P = 2013265921 := by decide
    have hN : NEG_INV = 2013265919 := by decide
    rw [hR, hP, hN]; omega
  -- u * R % P = a*b % P (from hdiv and hT_mod_P)
  have huR_mod : (a * b + (a * b % R * NEG_INV % R) * P) / R * R % P = a * b % P := by
    rw [← hdiv, hT_mod_P]
  -- Bound: a*b < P*P (nonlinear, manual)
  have hab : a * b < P * P := Nat.mul_lt_mul_of_lt_of_lt ha hb
  -- u < 2P (bound, uses hab + m < R)
  have hu_bound : (a * b + (a * b % R * NEG_INV % R) * P) / R < 2 * P := by
    -- Strategy: T < 2*P*R → T/R < 2*P (by Nat.div_lt_of_lt_mul)
    -- T = a*b + m*P where a ≤ P-1, b ≤ P-1, m ≤ R-1
    -- T ≤ (P-1)*(P-1) + (R-1)*P < 2*P*R (concrete fact)
    have ha_le : a ≤ P - 1 := Nat.le_pred_of_lt ha
    have hb_le : b ≤ P - 1 := Nat.le_pred_of_lt hb
    have hab_le : a * b ≤ (P - 1) * (P - 1) := Nat.mul_le_mul ha_le hb_le
    have hm_lt : (a * b % R * NEG_INV % R) < R := Nat.mod_lt _ (by decide)
    have hm_le : (a * b % R * NEG_INV % R) ≤ R - 1 := Nat.le_pred_of_lt hm_lt
    have hmp_le : (a * b % R * NEG_INV % R) * P ≤ (R - 1) * P := Nat.mul_le_mul_right P hm_le
    have hT_le : a * b + (a * b % R * NEG_INV % R) * P ≤ (P - 1) * (P - 1) + (R - 1) * P := 
      Nat.add_le_add hab_le hmp_le
    have hconcrete : (P - 1) * (P - 1) + (R - 1) * P < R * (2 * P) := by decide
    have hT_lt : a * b + (a * b % R * NEG_INV % R) * P < R * (2 * P) := 
      Nat.lt_of_le_of_lt hT_le hconcrete
    exact Nat.div_lt_of_lt_mul hT_lt
  -- conditional subtraction = u % P when u < 2P
  have hcond : (if (a * b + (a * b % R * NEG_INV % R) * P) / R ≥ P
               then (a * b + (a * b % R * NEG_INV % R) * P) / R - P
               else (a * b + (a * b % R * NEG_INV % R) * P) / R)
              = (a * b + (a * b % R * NEG_INV % R) * P) / R % P :=
    cond_sub_eq_mod _ P hu_bound (by decide)
  -- u % P = montgomeryMul a b (congruence: u*R ≡ a*b mod P → u ≡ a*b*R_inv mod P)
  have hu_mod : (a * b + (a * b % R * NEG_INV % R) * P) / R % P = montgomeryMul a b := by
    unfold montgomeryMul
    -- Multiply huR_mod by R_inv: (u*R*R_inv) % P = (a*b*R_inv) % P
    have hmul := mul_mod_congr
      ((a * b + (a * b % R * NEG_INV % R) * P) / R * R) (a * b) R_inv huR_mod
    -- u*R*R_inv → u*(R*R_inv) by associativity
    rw [Nat.mul_assoc] at hmul
    -- (u*(R*R_inv)) % P → (u%P) * ((R*R_inv)%P) % P by mul_mod
    rw [Nat.mul_mod _ (R * R_inv) P] at hmul
    -- (R*R_inv) % P → 1 by R_inv_correct
    rw [R_inv_correct] at hmul
    -- (u%P) * 1 → u%P → (u%P)%P → u%P
    rw [Nat.mul_one, Nat.mod_mod] at hmul
    -- hmul : u%P = a*b*R_inv % P = goal
    exact hmul

  -- Combine: scalarMul = u%P = montgomeryMul
  show scalarMul a b = montgomeryMul a b
  rw [show scalarMul a b = (if (a * b + (a * b % R * NEG_INV % R) * P) / R ≥ P
                          then (a * b + (a * b % R * NEG_INV % R) * P) / R - P
                          else (a * b + (a * b % R * NEG_INV % R) * P) / R) from rfl]
  rw [hcond, hu_mod]

end TSCP.Formal
