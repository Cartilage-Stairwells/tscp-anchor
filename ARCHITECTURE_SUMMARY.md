# TSCP — Transparent Succinct Computational Proof

A Rust zero-knowledge proving system using FRI over the BabyBear prime field (p = 2³¹ − 2²⁷ + 1), built on Plonky3 with Poseidon2-based Merkle commitments.

## Repositories

| Repository | Role |
|---|---|
| **[tscp-anchor](https://github.com/Cartilage-Stairwells/tscp-anchor)** | Principal active repository — proving pipeline, verifier, evidence harness |
| **[avx512-butterfly](https://github.com/Cartilage-Stairwells/avx512-butterfly)** | AVX-512 kernel — BabyBear Montgomery arithmetic, NTT/butterfly |
| **[tscp-pl-phase1](https://github.com/Cartilage-Stairwells/tscp-pl-phase1)** | Planning and specifications |

## Current Phase

**Phase 3** — benchmark/evidence-based proving-pipeline verification.

## Three-Layer Evidence Architecture

```
Layer 1 — EVIDENCE        Immutable benchmark artifacts and cryptographic hashes
      ↓
Layer 2 — ATTESTATION     Git commits, tags, releases, GPG signatures
      ↓
Layer 3 — INTERPRETATION   Human/agent analysis (revisable)
```

## Frozen Evidence Baseline

**Identity:** `firebird_74c6e5f` — **IMMUTABLE**

Captured on Intel Ice Lake-SP with explicit `target-cpu=icelake-server`. The benchmark measures a captured FRI proving path. Key findings:

- FRI dominates >99% of measured proving time.
- Timings strongly consistent with O(n) + fixed overhead (R² = 0.99998).
- The NTT/butterfly backend is **not invoked** by the captured proving path.

> The frozen baseline is a valid measurement of the captured FRI proving path, but it is not a measurement of the AVX-512 NTT/butterfly optimization.

## Kernel Refactor Status (avx512-butterfly)

| Commit | Status | Description |
|---|---|---|
| Commit 1 (`00f54d1`) | ✅ Complete | Verified BabyBear Montgomery arithmetic boundary |
| Commit 1.5 (`077ec6b`) | ✅ Complete | Legacy reduction characterization + corpus manifest |
| Commit 2 (`4d6e9e2`) | ✅ Complete | Mechanical canonicalization — one Montgomery implementation |
| Commit 3 | ⬜ Next | SIMD backend equivalence tests |

**Single Montgomery authority:** `field::babybear::montgomery::ScalarBackend`

```rust
pub trait MontgomeryBackend: Copy {
    const MODULUS: u32;
    fn constants(&self) -> MontgomeryConstants;
    fn mul(&self, a: u32, b: u32) -> u32;
    fn reduce(&self, prod: u64) -> u32;
}
```

## Governance Control Plane

Architecture v0.1.0 — baseline, architecture review CLOSED.

```
Agent Action → TAES → Governance Kernel → {Escalation | Transition} → TSCP
```

Disposition: backlog. Activation triggers: TAES live assessments, TSCP consuming receipts, or agent workflow requiring authorization gates.

See: [Governance Control Plane Architecture](../avx512-butterfly/governance/CONTROL_PLANE_v0.1.0.yaml)

## Next Experiments

| Experiment | Question | Status |
|---|---|---|
| **A** (kernel) | Does the AVX-512 butterfly outperform scalar/AVX2 under controlled conditions? | Not started |
| **B** (integration) | Does the optimized kernel improve end-to-end proving performance? | Not started |

Every new experiment gets a new evidence identity. Never modify `firebird_74c6e5f`.

## Continuity Documents

| Document | Location | Role |
|---|---|---|
| Master Continuity Handoff | `tscp-anchor` | Orients a fresh agent |
| Interpretation Report v2 | local | Layer 3 analytical artifact |
| `firebird_74c6e5f` | frozen | Immutable Layer 1 evidence |
