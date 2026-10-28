---
title: TSCP Release A — Claims Statement
summary: Scoped public claims for the B5 formal-verification release, derived from the final audit record (tscp-anchor #61, comment 6132730770). Every claim is bound to a named revision and evidence artifact.
---

# TSCP Release A — Claims Statement

**Release commit (formal corpus):** `Cartilage-Stairwells/tscp-anchor` @ `70e614f3c2cd9f21ea3d2b0021a523921c33f079` (master)
**Implementation repo:** `Cartilage-Stairwells/avx512-butterfly` — evidence-bearing revision `1cdf93a89bcc4a0c325a6d3571c265a9b26ba82f` (B6 differential suite); master lineage since Release A: `2fdf2bdd` (CI fixes, PRs #20-#22) → `1398ad8f` (archive intake) → `f3e97040` (legacy DIT seal) → `cac1c8a9` (Firebird iteration 1: 32-bit-lane subtraction-form REDC kernel, PR #33) → `87468e13` (Firebird iteration 3: software-pipelined fused pass, PR #36). Kernel arithmetic at `87468e13` is the Z3-verified subtraction-form REDC (C2 formulation); the iteration-3 change is a load/store schedule, not arithmetic, and output parity is gate-proven on silicon (run 39137142420).
**Revision:** Rev 2, 2026-10-18 — adds claim C4 (host-named sealed performance evidence, Release B-adjacent) and retires the "no qualified-host benchmark" limitation; correctness claims C1-C3 unchanged; corpus tree of record unchanged from `70e614f3`
**Licenses:** tscp-anchor: Apache-2.0. avx512-butterfly: Apache-2.0/MIT (repo and all dependencies, per the PR #19 pre-merge review).
**Date:** 2026-10-16

## 1. Claims supported by the evidence

### C1 — Exact NTT round-trip theorem (Lean-verified)
For the B5 construct on the `Nat → Nat` carrier (module `TSCP.Formal.NTTFull`): the inverse transform composed with the forward transform is the identity, with no external N⁻¹ scale factor (inversion via `dif_butterfly_inv` in LIFO order with halving internal to the inverse operation). Supporting theorems: `P_prime` (primality of P = 0x78000001 = 15·2²⁷+1, bounded kernel trial division), `b5_forward_preserves_validity`, `stage_inv_duality`, `b5_stage_composition`.

**Evidence:** CI attestation run 38300810290 on the exact release commit — 11/11 modules build clean (Lean v4.33.0-rc2, Mathlib 51e6992e); kernel audit 919 constants, 0 axioms; source classification 101 theorems = 50 clean / 46 foundational / 5 classical / **0 project axioms / 0 sorry-dependent**, of which **96 are substantive and 5 are structural/definitional** (named below); audit gate PASSED vs `audit/expected-gaps.json`; receipt artifact 11771145127 (`tscp-verification-receipt-70e614f3c…`, sha256:7304e3c54293662e64c101104dab5e87231eb91baf11b3086d44bdaf60d2f3b6). Independently re-verified in the final audit (tscp-anchor issue #61, comment 6132730770).

**Structural theorems (5 of 101).** `Core.lean`'s `Universe` layer is scaffolding for later composition, not a model of the NTT: `ntt_map` is `opaque` and both kernels define `admits_proof := fun _ => True`. Five of the 101 declarations are therefore definitional rather than mathematical — `babybear_ntt_end_to_end` (a `True → True` implication), `governance_transition_preserves_truth` (`X ↔ X` by `rfl`, with its authorization and transition hypotheses unused), `proof_valid`, `verified_path_certified`, and `rename_preserves_formula_admits`. They introduce no axioms, conceal no `sorry`, and carry no part of C1–C4. The remaining 96 theorems — Montgomery, butterfly, stage composition and the NTT round-trip corpus — are substantive and are what the claims below rest on.

**Scope boundary:** the theorem holds for the verified model (canonical residues x ∈ [0, P-1], pow2 lengths N = 2^k, k ≤ 27, matched-inverse stage lists). It is a result about the formal NTT construct, not a universal implementation-correctness claim.

**Corpus counts across tags.** The reviewer surface (`zksha-rx-reviewer-access`) cites 88 theorem declarations at tag `formal-corpus-v2026-09-11`; this statement counts 101 at Release A (`70e614f3`). The difference is exact and reproduces from the source:

| Count | Composition |
|---|---|
| **88** at `formal-corpus-v2026-09-11` | 84 inside the build roots + 4 in scaffolds (`TraceCoreProver/Kernel.lean` 3, `BabyBear/Element.lean` 1) |
| **101** at Release A | 84 + 11 (`NTTFull.lean`, new at Release A) + 6 (`NormalizationBridge`, 6 → 12) = 101 |

The delta is 17, not 13, because four of the 88 were scaffold declarations outside any build root. Both figures are correct for their commits and neither revises the other.

### C2 — Montgomery REDC single-lane kernel equivalence (SMT-verified)
The 64-bit REDC kernel (`mont_reduce_epu64`, single lane) is bit-exact equivalent to its modular specification for all inputs in [0, 2⁶⁴−1].

**Evidence:** Z3 5.1.0 (`unsat`, sealed script `mont_reduce_epu64.smt2`, sha256:b43758a8…; independent audit re-run 1m38s) plus mutation control (`mutation_wrong_mu.smt2` returns `sat`, confirming the check has teeth). Labeled SMT-verified, not Lean-verified.

### C3 — SIMD-vs-scalar differential conformance (tested-corpus evidence)
The AVX-512 fused butterfly pass engine agrees bit-exactly with the scalar reference engine (`scalar_engine_fused_butterfly_pass`, the sole baseline) over the tested conformance corpus on native AVX-512 hardware.

**Evidence:** differential suite `tests/b6_conformance_levelc.rs` green on real AVX-512 silicon — CI run 173 on branch head `da90e17a4` (id 38304740467), enforced on master post-merge (AVX-512 Build and Test run 174 on `1cdf93a8`, VEP Validation run 129). Corpus: sizes 2/16/32/64/1024, maximal canonical inputs (P−1), twiddle boundary values, deterministic PRNG vectors pinned by manifest seed.

**Scope boundary:** finite test evidence over the tested corpus on x86-64 hosts with native avx512f+avx512dq. It does NOT establish universal equivalence over all inputs; that stronger claim is where the Layer 2 SMT proof applies (kernel level only).

### C4 — Kernel-throughput vs Plonky3 on named hosts (sealed qualified-runner evidence)
On each named host below, the AVX-512 fused butterfly pass (`avx512_engine_fused_butterfly_pass`, software-pipelined schedule, master `87468e13f`) completes the same-size pass faster than Plonky3 `p3-baby-bear` 0.6.3 (pinned) packed AVX-512 butterfly (`p3_packed_butterfly_pass`) in the sealed capture:

| host | 2^16 | 2^18 | 2^20 |
|---|---|---|---|
| Intel Xeon Platinum 8573C | 1.165x | 1.223x | 1.007x |
| AMD EPYC 9V74 | 1.113x | 1.129x | 1.127x |
| Intel Xeon Platinum 8370C | 1.233x | 1.073x | 1.021x |
| Intel Xeon 6973P-C | 1.070x | 1.006x | 2.120x |

**Evidence:** sealed capture run 39137424483 (workflow "Plonky3 Comparison Capture", 8-attempt matrix, D17 silicon exclusion, named-CPU provenance, per-attempt SHA256SUMS 247/247 verified at adoption); specimen committed measurement-only at `avx512-butterfly` `repro-v0.3/captures/plonky3_comparison_iter3_87468e13/` (PR #37, merge `31b29e1e`); medians table and honest boundaries in the specimen's `VERDICT.md`. Correctness of the measured kernel: silicon differential run 39137142420 (Xeon 8573C) — fused == generic == legacy byte-identical, B6 differential vs scalar trust root, external-p3 parity at 2^20-2^24. Earlier sealed specimens retained unmodified: iteration 1 (`plonky3_comparison_iter1_f8a8a38/`, run 39132995107, kernel relane flipped the prior p3 deficit) and the legacy DIT verdict (`legacy_dit_1398ad8f/`, run showing the 2026-07 9.15x figure is host-dependent and non-portable).

**Scope boundary:** host-named and sealed. Shared GitHub-hosted runners (named CPUs, not dedicated bare metal). Kernel-throughput of one fused butterfly pass only; p3 pinned 0.6.3; no end-to-end NTT, proof-system, or cross-version claim. No universal performance claim is made: performance is host-dependent by nature, and only correctness claims are universal.

## 2. Claims NOT supported by the evidence (do not make)

1. **No universal implementation-equivalence claim.** No theorem or proof binds the Lean B5 construct to the Rust engines as universally equivalent.
2. **No `Core.ntt_map` correspondence claim.** B5 is a distinct construct on the `Nat → Nat` carrier (ruling R3_DISTINCT_RELATED / Path C). `Core.ntt_map` remains opaque; its surjectivity hypothesis is a named, open boundary. Layer 1 scalar-refinement obligations (O-A/O-B/O-C) are specified but unproved.
3. **No performance claim beyond the host-named sealed evidence of C4.** No universal speedup, no end-to-end or whole-system figure, no cost-per-op or energy figure, no comparison against p3 versions other than the pinned 0.6.3, and no extrapolation to unmeasured hosts. The legacy 9.15x figure is host-dependent and non-portable (sealed legacy verdict) and must not be restated as a portable claim.
4. **No corpus-completeness claim.** The audited census covers the 11 build roots under the pinned toolchain; it is not a completeness result for the whole TSCP corpus.

## 3. Reproduction

**Formal side (tscp-anchor @ 70e614f3c):**
```
git clone https://github.com/Cartilage-Stairwells/tscp-anchor && cd tscp-anchor
git checkout 70e614f3c2cd9f21ea3d2b0021a523921c33f079
# toolchain: lean-toolchain pins Lean v4.33.0-rc2 (d8b18978); Mathlib at 51e6992e
lake build          # all 11 roots, incl. TSCP.Formal.NTTFull
python3 audit/check_audit.py   # audit gate: 0 sorryAx, 0 project axioms, census reconciled
```

**Implementation side (avx512-butterfly @ 1cdf93a8):**
```
git clone https://github.com/Cartilage-Stairwells/avx512-butterfly && cd avx512-butterfly
git checkout 1cdf93a89bcc4a0c325a6d3571c265a9b26ba82f
cargo test         # differential suite runs on AVX-512F+DQ hosts; fails closed otherwise
```

**Audit trail:** tscp-anchor issue #61 — B5 merge record (comment 6131572668), master attestation (6131981664), Level C evidence (6132667358), final audit (6132730770). PR #19 merge record: comment 6139873806.

## 4. Known limitations, stated plainly

- Layer 1 scalar refinement: spec only, obligations unproved.
- Layer 3 differential: finite corpus, not universal proof.
- `Core.ntt_map` surjectivity: named open hypothesis (Level C scope).
- Qualified-host benchmark: executed on shared GitHub-hosted runners with named AVX-512 CPUs (owner-accepted physical step; C4). A dedicated-bare-metal specimen remains the stronger evidence class if ever available.
- CI Custody Enforcement check: repaired 2026-10-16 (PRs #20/#21 — JSON-path defect plus a latent boolean-case defect; enforcement live and green since run 102, first green run in workflow history). The ubuntu-latest silicon lottery for fail-closed AVX-512 suites was also repaired (PR #22, D17 extension): master CI is green on any runner.
