import Mathlib

def BABYBEAR_PRIME : Nat := 2013265921
theorem babybear_prime_eq : BABYBEAR_PRIME = (2 ^ 27) * 15 + 1 := by norm_num
theorem babybear_prime_is_prime : Nat.Prime BABYBEAR_PRIME := by norm_num

abbrev BabyBear := ZMod BABYBEAR_PRIME
instance : Field BabyBear := ZMod.instField BABYBEAR_PRIME

@[reducible] def bb (n : Nat) : BabyBear := (n : BabyBear)

def omega_8 : BabyBear := bb 1592366214
def omega_16 : BabyBear := bb 196396260

theorem omega_8_order_eq_8 : omega_8 ^ 8 = 1 := by native_decide
theorem omega_8_not_order_dividing_4 : omega_8 ^ 4 ≠ 1 := by native_decide
theorem omega_16_order_eq_16 : omega_16 ^ 16 = 1 := by native_decide
theorem omega_16_not_order_dividing_8 : omega_16 ^ 8 ≠ 1 := by native_decide
theorem omega_16_sq_eq_omega_8 : omega_16 ^ 2 = omega_8 := by native_decide

theorem field_add_p_minus_1_and_1 : bb 2013265920 + bb 1 = bb 0 := by native_decide
theorem field_mul_p_minus_1_self : bb 2013265920 * bb 2013265920 = bb 1 := by native_decide
theorem field_add_large : bb 305419896 + bb 582803183 = bb 888223079 := by native_decide
theorem field_mul_large : bb 305419896 * bb 582803183 = bb 347321647 := by native_decide

def SCHEMA_VERSION : String := "v1"
theorem schema_locked_v1 : SCHEMA_VERSION = "v1" := rfl

def main : IO Unit := do
  IO.println "Golden Vector Lean Replay — BabyBear Field"
  IO.println s!"Prime: {BABYBEAR_PRIME}"
  IO.println "All theorems proved by native_decide."
