# Plonky3 Comparison — Scalar Benchmark + AVX-512 Code Review

**Date:** 2026-07-08 (original draft), **2026-07-23** (provenance + measured/inferred separation added per Aria review)
**Status:** Evidence document. Does not modify VERIFIED_CLAIMS.md / INVESTOR_TEASER.md / SBIR_NARRATIVE.md — held per Sean's request pending review of these numbers.
**Scope:** This document enters the evidence chain as a comparison/evidence artifact, not as a funding claim or performance claim.

---

## 0. Benchmark Provenance

| Field | Value |
|---|---|
| Git commit SHA (at time of measurement) | `8f1e001` (`Add PLONKY3_COMPARISON.md...`) |
| rustc version | `1.96.1 (31fca3adb 2026-06-26)` |
| cargo version | `1.96.1 (356927216 2026-06-26)` |
| Target triple | `x86_64-unknown-linux-gnu` |
| CPU model | `unknown` (sandbox VM, gvisor kernel `4.19.0-gvisor`) |
| CPU core count | 1 |
| AVX-512 available | **No** — `/proc/cpuinfo` shows `avx2` only, no `avx512*` flags |
| AVX2 available | Yes |
| Criterion version | `0.5.1` |
| p3-baby-bear version | `0.6.1` |
| p3-field version | `0.6.1` |
| p3-monty-31 version | `0.6.1` |
| Compiler profile | `opt-level = 3, lto = true` (release) |
| Benchmark parameters | 10,000 ops, 100 samples, 3s warmup, 5s measurement |
| Deterministic seed | `0xCAFEBABEDEADBEEF` (LCG: `s = s * 6364136223846793005 + 1442695040888963407`) |
| Reproduction command | `cd plonky3_bench && cargo bench --bench mont_mul_comparison` |

---

## 1. Background

`p3-baby-bear` v0.6.1 and `p3-field` v0.6.1 are real, published crates (confirmed: added as dependencies, built clean, 37 packages resolved from crates.io). They ship a native, production, tested AVX-512 `PackedField` implementation for BabyBear at `p3-monty-31/src/x86_64_avx512/packing.rs`, which is materially different from our kernel:

| | Ours (`babybear_avx512_butterfly.rs`, `m8_polyir_lowering.rs`) | Plonky3 (`p3-monty-31` x86_64_avx512) |
|---|---|---|
| Montgomery domain | R = 2⁶⁴ | R = 2³² (native to the 31-bit prime) |
| REDC form | Addition-form, `-p⁻¹ = 0x77FFFFFF` | Subtraction-form, `p⁻¹ = 0x88000001` |
| REDC steps | Two-step 32-bit (since product needs 2 reductions to clear R=2⁶⁴) | One-step 32-bit (R=2³² fits the u64 product exactly) |
| SIMD lane width | 8×i64 per ZMM register | 16×i32 per ZMM register (odd/even split via `movehdup`) |
| Shuffles needed | None | 4 movehdup-family ops per multiply |

---

## 2. Measured Results (Scalar Path — Criterion, Real Hardware Execution)

These numbers were produced by running `cargo bench --bench mont_mul_comparison` on this sandbox instance. They are real measurements, not estimates or projections.

### 2.1 Scalar Montgomery multiplication (measured)

| Benchmark | Time (10k ops) | ns/op | Melem/s |
|---|---|---|---|
| `ours_r64_native` — our scalar `mont_mul`, values already in our R=2⁶⁴ domain | 38.998 µs | 3.900 | 256.4 |
| `plonky3_r32_native` — `p3_baby_bear::BabyBear` multiplication, their native R=2³² domain | 14.078 µs | 1.408 | 710.3 |

**Plonky3's native scalar multiply is 2.77× faster than ours** in native domain. Mechanically explained: their R=2³² domain needs only one 32-bit REDC step per multiply; our R=2⁶⁴ domain needs two sequential 32-bit REDC steps.

### 2.2 Boundary conversion cost (measured)

| Benchmark | Time (10k ops) | ns/op | Melem/s |
|---|---|---|---|
| `ours_r64_with_boundary_conversion` — our kernel including R32→R64 conversion in, multiply, R64→R32 conversion out (the realistic cost of using our kernel as a drop-in accelerator for Plonky3-resident field elements) | 136.48 µs | 13.648 | 73.3 |

Including the boundary conversion overhead, **Plonky3 is 9.69× faster** than using our kernel to accelerate their field elements. The conversion overhead (two extra Montgomery multiplies per element pair: one `to_our_r64` each for both operands, plus one `from_our_r64` for the result) dominates the arithmetic cost.

### 2.3 Measured conclusion

On the scalar path, Plonky3's own BabyBear implementation is faster than ours by **2.77–9.69×** depending on whether conversion overhead is included. This is expected and mechanically explained — their REDC domain is natively sized for the 31-bit prime.

---

## 3. Inferred Results (AVX-512 Path — Static Code Review, NOT Measured)

**⚠️ This section is analysis of source code, not benchmark output.** This sandbox instance has no AVX-512 hardware (`/proc/cpuinfo` shows `avx2` only). No AVX-512 intrinsics were executed. No AVX-512 timing data was collected. The following is instruction-level structural analysis for engineering context only — it must not be cited as a performance claim.

### 3.1 Plonky3's AVX-512 multiply instruction sequence

Source: `p3-monty-31/src/x86_64_avx512/packing.rs`, function `mul()`. Their own source comments annotate: **6.5 cycles/vector, 2.46 elements/cycle, 21 cycles latency**.

Instruction sequence (13 vector instructions):
1. `vmovshdup` — deinterleave lhs odd lanes
2. `vmovshdup` — deinterleave rhs odd lanes
3. `vpmuludq` — even-lane 32×32→64 product
4. `vpmuludq` — odd-lane 32×32→64 product
5. `vpmuludq` — q_even = prod_even × MU
6. `vpmuludq` — q_odd = prod_odd × MU
7. `vmovshdup` — merge prod_hi (interleave even/odd high halves)
8. `vpmuludq` — q_p_even = q_even × P
9. `vpmuludq` — q_p_odd = q_odd × P
10. `vmovshdup` — merge q_p_hi
11. `vpcmpltud` — underflow mask (prod_hi < q_p_hi?)
12. `vpsubd` — res = prod_hi - q_p_hi
13. `vpaddd` (masked) — conditional add P on underflow

### 3.2 Our AVX-512 multiply instruction sequence

Source: `m8_polyir_lowering.rs`, functions `avx512_butterfly_inner` + `mont_reduce_64_avx512` + initial `vpmuludq`. Also 13 vector instructions, but for 8 elements (not 16):

1. `vpmuludq` — a × b (8 lanes, 64-bit)
2. `vpandq` — mask low 32 bits (step 1 input)
3. `vpmuludq` — m1 = lo × P_INV_NEG (step 1)
4. `vpmuludq` — m1×P (step 1)
5. `vpaddq` — t + m1×P (step 1)
6. `vpsrlq` — >>32 (step 1 output u1)
7. `vpandq` — mask low 32 bits (step 2 input)
8. `vpmuludq` — m2 = lo × P_INV_NEG (step 2)
9. `vpmuludq` — m2×P (step 2)
10. `vpaddq` — u1 + m2×P (step 2)
11. `vpsrlq` — >>32 (step 2 output u2)
12. `vpcmpgeq` — u2 ≥ P? (mask)
13. `vpsubq` (masked) — conditional subtract P

### 3.3 Structural comparison

| Property | Ours (8-wide) | Plonky3 (16-wide) |
|---|---|---|
| Vector instructions | 13 | 13 |
| Elements per call | 8 | 16 |
| Elements per instruction | 0.615 | 1.23 |
| Shuffle instructions | 0 | 4 (movehdup) |
| REDC dependency chain | 2 sequential steps (step 2 needs step 1 output) | 1 step (parallel odd/even) |
| Conditional reduction | Masked subtract (cmpge + mask_sub) | Masked add (cmplt + mask_add) |

### 3.4 Inferred conclusion (unverified — requires hardware run)

**No structural reason to expect our approach wins on AVX-512.** The instruction-count parity (13 = 13) combined with half the lane width (8 vs 16) and a longer dependency chain (2 sequential REDC steps vs 1 parallel) points toward Plonky3 being faster or equal on AVX-512 as well. Their 4 shuffle instructions are a real cost, but 13 instructions with 2× throughput density is a strong tradeoff in their favor.

**This inference must not be cited as a performance claim.** It is a code-review analysis pending verification on real AVX-512 hardware.

---

## 4. Summary — Measured vs Inferred

| Claim | Classification | Evidence |
|---|---|---|
| Plonky3 scalar multiply 2.77× faster than ours (native domain) | **Measured** | Criterion benchmark, this sandbox, commit `8f1e001` |
| Plonky3 scalar multiply 9.69× faster than ours (with conversion) | **Measured** | Same benchmark, includes boundary conversion |
| Our two-step REDC creates a longer critical path than Plonky3's one-step | **Inferred** | Source code review, not executed on AVX-512 |
| Our 8-wide approach has no shuffle overhead vs their 4 shuffles | **Inferred** | Source code review, not executed on AVX-512 |
| Plonky3 AVX-512 throughput ≥ ours | **Unverified** | Structural analysis suggests this, but no AVX-512 hardware available to confirm |

---

## 5. Recommendation for Framing Decision (Sean's Call)

The evidence collected so far does not support a "superior alternative kernel" narrative. It is consistent with "complementary formally-verified research project" — our differentiator is:

1. **Lean4 machine-checked proof of Montgomery correctness** (27 theorems, 0 `sorry` — Plonky3 does not have this)
2. **Exhaustive differential/property test suite** (61 verification points, 300k+ random inputs)
3. **Honest engineering tradeoff documentation** (this document — we measure where we lose, not just where we win)

Not raw throughput. The framing should be: *"a formally constrained BabyBear arithmetic kernel with machine-checked correctness properties; performance comparison shows the engineering tradeoff."*

---

## 6. Reproduction

```bash
# Requires: rustc 1.96.1+, cargo, network access to crates.io
cd plonky3_bench
cargo bench --bench mont_mul_comparison

# Results are saved to target/criterion/scalar_mont_mul/
# Compare the three sub-benchmarks:
#   ours_r64_native/10000
#   plonky3_r32_native/10000
#   ours_r64_with_boundary_conversion/10000
```

Note: scalar results may vary across hardware due to CPU microarchitecture differences. The relative ordering (Plonky3 faster than ours on scalar) is expected to hold across x86_64 hardware because it is mechanically explained by the REDC step count, not by microarchitecture-dependent timing.
