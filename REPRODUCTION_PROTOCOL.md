# REPRODUCTION_PROTOCOL.md

**Version:** 0.1
**Date:** 2026-07-23
**Status:** Draft — defines the reproduction contract for external verification of TSCP-PL Phase 1 artifacts.

---

## 1. Purpose

This document defines the exact protocol a third-party verifier must follow to independently reproduce the TSCP-PL Phase 1 evidence chain. It is stricter than a verification guide: another party should be able to reproduce the same evidence years later without consulting the author.

## 2. Required Environment

| Requirement | Specification |
|---|---|
| OS | Linux (any modern distribution with glibc ≥ 2.28) |
| Architecture | x86_64 |
| Rust toolchain | stable, ≥ 1.96.0 (tested with 1.96.1) |
| Cargo | ≥ 1.96.0 |
| Network | Access to crates.io (for Plonky3 comparison benchmarks only) |
| CPU | Any x86_64; AVX-512 NOT required (scalar fallback is the verified path) |
| Memory | ≥ 512 MB free |
| Disk | ≥ 200 MB for build artifacts |

### AVX-512 hardware (optional, for SIMD verification only)

If the verifier has AVX-512 hardware (`/proc/cpuinfo` contains `avx512f`), the AVX-512 differential tests in `m8_polyir_lowering.rs` will execute instead of self-skipping. The scalar reference path is identical regardless.

## 3. Artifact Inventory

The verifier should receive or clone the following files. Each must match its recorded SHA-256 (see `RELEASE_MANIFEST.md`).

### Core kernels
- `butterfly_avx512.rs` — production AVX-512 DIT kernel
- `babybear_avx512_butterfly.rs` — DIT kernel with runtime detection + scalar fallback
- `butterfly_portable.rs` — portable 32-bit DIF kernel
- `butterfly_no_std.rs` — `no_std + alloc` portable version
- `butterfly_xor.rs` — raw i32 XOR butterfly
- `babybear_field_no_std.rs` — `no_std` field parameters
- `plonky3_integration.rs` — Montgomery R=2³²↔R=2⁶⁴ conversion
- `m8_polyir_lowering.rs` — SIMD lowering pass + multi-stage NTT (21 `#[test]` assertions)

### Verification
- `verify_constants.rs` — first-principles constant derivation (4 checks)
- `property_tests.rs` — property-based differential tests

### Formal proof
- `lean/BabyBearVerified.lean` — Lean4 v4.31.0, 27 theorems, 0 `sorry`

### Benchmarks
- `bench_suite/` — Criterion 0.5 suite (authoritative performance-of-record)
- `plonky3_bench/` — Plonky3 comparison benchmark

### Evidence
- `PLONKY3_COMPARISON.md` — measured scalar comparison + AVX-512 code review
- `CRITERION_BENCHMARKS.md` — full benchmark methodology and results
- `VERIFIED_CLAIMS.md` — verified vs projected claims
- `RELEASE_MANIFEST.md` — SHA-256 hashes and test summary
- `EVIDENCE.md` — evidence chain documentation
- `HARD_GATE_PROTOCOL.md` — quality control protocol
- `evidence_log.txt` — raw test + benchmark output

## 4. Exact Reproduction Commands

### Phase 1: Scalar kernel verification (no AVX-512 required)

```bash
# 1. Compile and run all scalar kernel tests
rustc -O butterfly_portable.rs --test -o butterfly_portable_test && ./butterfly_portable_test
# Expected: 8 tests pass, 0 fail

rustc -O property_tests.rs --test -o property_tests && ./property_tests
# Expected: all assertions pass

rustc -O verify_constants.rs -o verify_constants && ./verify_constants
# Expected: all 4 constant derivations pass, no panic

rustc -O butterfly_xor.rs -o butterfly_xor && ./butterfly_xor
# Expected: all tests pass

rustc -O butterfly_no_std.rs --test -o butterfly_no_std_test && ./butterfly_no_std_test
# Expected: 4 tests pass

rustc -O m8_polyir_lowering.rs --test -o m8_test && ./m8_test
# Expected: 21 tests pass (16 executed, 5 AVX-512 tests self-skip if no AVX-512)
```

### Phase 2: Lean4 formal proof verification

```bash
# Requires: Lean4 v4.31.0 (elan or manual install)
cd lean
lean BabyBearVerified.lean
# Expected: compiles with no errors, no warnings about `sorry`
# Verification: grep -c "sorry" BabyBearVerified.lean  (should return 0)
```

### Phase 3: Criterion benchmark reproduction

```bash
# Requires: cargo, network access to crates.io
cd bench_suite && cargo bench
# Expected: BabyBear DIT speedup ≈ 9.15×, XOR speedup ≈ 4.58×
# (exact numbers will vary by hardware; relative ordering should hold)

cd ../plonky3_bench && cargo bench --bench mont_mul_comparison
# Expected: plonky3_r32_native faster than ours_r64_native by ~2.5-3×
# (scalar-only; AVX-512 SIMD paths not executed without AVX-512 hardware)
```

### Phase 4: Plonky3 comparison reproduction

```bash
# The PLONKY3_COMPARISON.md documents the results from Phase 3 above.
# Verify the measured numbers fall within the expected ranges:
# - ours_r64_native: ~3-5 ns/op
# - plonky3_r32_native: ~1-2 ns/op
# - ours_r64_with_boundary_conversion: ~10-15 ns/op
# Relative ordering must hold: plonky3 < ours_native < ours_with_conversion
```

## 5. Expected Outputs

| Command | Expected Output |
|---|---|
| `butterfly_portable_test` | `8 passed; 0 failed` |
| `property_tests` | all assertions pass (no panic) |
| `verify_constants` | all 4 derivations pass (no panic) |
| `butterfly_xor` | all tests pass |
| `butterfly_no_std_test` | `4 passed; 0 failed` |
| `m8_test` (no AVX-512) | `21 passed; 0 failed` (5 AVX-512 tests log skip, count as pass) |
| `m8_test` (with AVX-512) | `21 passed; 0 failed` (all 21 executed) |
| `lean BabyBearVerified.lean` | exit 0, no `sorry` in output |
| `cargo bench` (bench_suite) | BabyBear DIT ≈ 9× speedup, XOR ≈ 4.5× speedup |
| `cargo bench` (plonky3_bench) | plonky3_r32 faster than ours_r64 by ~2.5-3× |

## 6. Failure Modes

| Failure | Likely Cause | Resolution |
|---|---|---|
| `SIGILL` on AVX-512 test | CPU lacks AVX-512, but test didn't self-skip | Check `is_x86_feature_detected!` gate; verify `/proc/cpuinfo` |
| Lean4 compile error | Wrong Lean4 version | Must use v4.31.0; `elan default 4.31.0` |
| `cargo bench` fails | Network blocked to crates.io | Download crates offline or use `--offline` with vendored deps |
| `verify_constants` panic | Constant mismatch — indicates code was tampered | Check SHA-256 against `RELEASE_MANIFEST.md` |
| Criterion numbers wildly different | Different CPU microarchitecture | Report hardware details; relative ordering should still hold |

## 7. Acceptable Deviations

| Measurement | Acceptable Range | Rationale |
|---|---|---|
| Scalar multiply ns/op | ±50% of reported value | Microarchitecture-dependent; depends on u128 multiply latency, branch prediction |
| Criterion speedup ratios | ±20% of reported value | Both scalar and AVX-512 scale with the same CPU |
| Lean4 compile time | Any (no timeout) | Not a performance metric |
| Test pass/fail counts | **Exact match required** | Binary: pass or fail, no tolerance |

## 8. Controlled Falsification Tests

For adversarial verification, a verifier should attempt the following mutations and confirm each produces the expected failure:

### 8.1 Modify one byte in an evidence artifact
```
# Modify a single byte in property_tests.rs
sed -i 's/P = 0x78000001/P = 0x78000002/' property_tests.rs
rustc -O property_tests.rs --test -o pt_fuzz && ./pt_fuzz
# Expected: FAIL — constant mismatch, tests fail
# Restore: git checkout property_tests.rs
```

### 8.2 Replace the Montgomery constant
```
# Change P_INV_NEG from 0x77FFFFFF to 0x77FFFFFE in m8_polyir_lowering.rs
sed -i 's/0x77FF_FFFF/0x77FF_FFFE/g' m8_polyir_lowering.rs
rustc -O m8_polyir_lowering.rs --test -o m8_fuzz && ./m8_fuzz
# Expected: FAIL — Montgomery reduction produces wrong results
# Restore: git checkout m8_polyir_lowering.rs
```

### 8.3 Remove a test assertion
```
# Comment out a test in m8_polyir_lowering.rs
# Expected: test count drops from 21 to 20 — detectable via test runner output
```

### 8.4 Modify NTT twiddle ordering
```
# Swap twiddle generation to use DIF instead of DIT ordering
# Expected: NTT round-trip test fails — forward + inverse ≠ identity
```

These falsification tests are more valuable than the success case because they demonstrate the verification boundary is real — the tests are not vacuously passing.

## 9. Maturity Model

| Version | Definition | Status |
|---|---|---|
| v0.1 | Initial artifact created | ✅ Achieved |
| v0.2 | Internally proven artifact (all tests pass, evidence chain documented) | ✅ Achieved |
| v0.3 | Independently reproduced artifact (third party runs this protocol successfully) | 📌 Next |
| v0.4 | Adversarially falsified (third party runs §8 mutations, all fail as expected) | 📌 Future |
| v1.0 | Independently reproduced + adversarially falsified + formally bound claims | 📌 Future |

## 10. Record Template

The verifier should fill out the following upon completion:

```
REPRODUCTION RECORD
===================
Date:           YYYY-MM-DD
Verifier:       [name/organization]
Environment:    [OS, CPU model, RAM, rustc version]
AVX-512:        [yes/no]
Results:
  butterfly_portable:   [N passed / N failed]
  property_tests:        [pass/fail]
  verify_constants:      [pass/fail]
  butterfly_xor:         [pass/fail]
  butterfly_no_std:      [N passed / N failed]
  m8_polyir_lowering:    [N passed / N failed]
  lean_proof:            [compiles / error]
  bench_suite:           [DIT speedup: Nx, XOR speedup: Nx]
  plonky3_bench:         [plonky3 vs ours: Nx faster]
Falsification tests:     [all 4 failed as expected / which ones did not]
Deviations:              [any deviations from expected outputs]
Signature:               [verifier attestation]
```

This record becomes the evidence that v0.3 maturity was achieved.
