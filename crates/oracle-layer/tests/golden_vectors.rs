// Golden Vector Test Suite — BabyBear Field, Butterfly, NTT
// TSCP-PL v1.963 — Orion Boundary Artifact
// Schema: v1

const P: u32 = 0x78000001;
const P_U64: u64 = P as u64;

#[inline]
fn add_mod(a: u32, b: u32) -> u32 {
    let sum = a as u64 + b as u64;
    if sum >= P_U64 { (sum - P_U64) as u32 } else { sum as u32 }
}

#[inline]
fn sub_mod(a: u32, b: u32) -> u32 {
    if a >= b { a - b } else { P - (b - a) }
}

#[inline]
fn mul_mod(a: u32, b: u32) -> u32 {
    ((a as u64 * b as u64) % P_U64) as u32
}

fn mod_pow(base: u32, exp: u32) -> u32 {
    let mut result = 1u64;
    let mut b = base as u64;
    let mut e = exp as u64;
    while e > 0 {
        if e & 1 == 1 { result = (result * b) % P_U64; }
        b = (b * b) % P_U64;
        e >>= 1;
    }
    result as u32
}

#[test]
fn test_prime_properties() {
    assert_eq!(P as u64, P_U64);
    assert_eq!(P_U64, 2013265921);
    assert_eq!(P_U64 - 1, (1u64 << 27) * 15);
}

#[test]
fn test_field_arithmetic_edge_cases() {
    assert_eq!(add_mod(0, 0), 0);
    assert_eq!(add_mod(2013265920, 1), 0);
    assert_eq!(mul_mod(2013265920, 2013265920), 1);
    assert_eq!(add_mod(1, 2), 3);
    assert_eq!(sub_mod(1, 2), 2013265920);
    assert_eq!(add_mod(305419896, 582803183), 888223079);
    assert_eq!(mul_mod(305419896, 582803183), 347321647);
}

#[test]
fn test_butterfly_primitive() {
    let a = 1u32; let b = 2u32; let w = 1592366214u32;
    let wb = mul_mod(w, b);
    assert_eq!(add_mod(a, wb), 1171466508);
    assert_eq!(sub_mod(a, wb), 841799415);
}

#[test]
fn test_omega_8_is_primitive_8th_root() {
    let omega = 1592366214u32;
    assert_eq!(mod_pow(omega, 8), 1);
    assert_ne!(mod_pow(omega, 4), 1);
    assert_ne!(mod_pow(omega, 2), 1);
}

#[test]
fn test_omega_16_is_primitive_16th_root() {
    let omega = 196396260u32;
    assert_eq!(mod_pow(omega, 16), 1);
    assert_ne!(mod_pow(omega, 8), 1);
    assert_eq!(mod_pow(omega, 2), 1592366214);
}

#[test]
fn test_twiddles_n8() {
    let omega = 1592366214u32;
    let expected = [1, 1592366214, 1728404513, 211723194];
    for (i, &exp) in expected.iter().enumerate() {
        assert_eq!(mod_pow(omega, i as u32), exp);
    }
}

#[test]
fn test_inv_twiddles_n8() {
    let inv_omega = 1801542727u32;
    let expected = [1, 1801542727, 284861408, 420899707];
    for (i, &exp) in expected.iter().enumerate() {
        assert_eq!(mod_pow(inv_omega, i as u32), exp);
    }
}

#[test]
fn test_twiddles_n16() {
    let omega = 196396260u32;
    let expected = [1, 196396260, 1592366214, 78945800, 1728404513, 1400279418, 211723194, 1446056615];
    for (i, &exp) in expected.iter().enumerate() {
        assert_eq!(mod_pow(omega, i as u32), exp);
    }
}

#[test]
fn test_inv_twiddles_n16() {
    let inv_omega = 567209306u32;
    let expected = [1, 567209306, 1801542727, 612986503, 284861408, 1934320121, 420899707, 1816869661];
    for (i, &exp) in expected.iter().enumerate() {
        assert_eq!(mod_pow(inv_omega, i as u32), exp);
    }
}
