# TSCP v2.4.0 Verification Dossier

**Tag:** `TSCP-v2.4.0-formal-sealed`
**Commit:** `e41096f9464d7d418db57e26c9b0ae8a21ffb744`
**Date:** 2026-07-25
**Status:** SEALED

---

## What This Document Is

This is the single entry point for anyone auditing the TSCP formal verification acceptance decision. It explains what was verified, what was assumed, how to reproduce the verification, and what the trust boundaries are. An external engineer, auditor, or future agent should not need to read the development history to understand the acceptance state.

---

## Acceptance Decision

**ACCEPTED.** The TSCP formal backbone is sealed at v2.4.0.

| Metric | Value |
|--------|-------|
| Modules compiled | 6/6 |
| Compile errors | 0 |
| Classical logic usage | 0 |
| Axioms (explicitly classified) | 2 |
| Sorry (explicitly classified) | 3 |
| Custody invariants verified | 8 |

---

## Three-Layer Provenance Model

No single layer carries the entire trust burden.

```
┌─────────────────────────────────────────────────────────┐
│  LAYER 1: TSCP CUSTODY                                  │
│  Authority: GitHub rulesets                             │
│  - TSCP-CANONICAL-CUSTODY (branch protection)           │
│  - TSCP-RELEASE-CUSTODY (tag immutability)              │
│  Proves: source structure and release immutability      │
├─────────────────────────────────────────────────────────┤
│  LAYER 2: GITHUB ATTESTATION                            │
│  Authority: GitHub Actions CI                           │
│  - Build Provenance attestation (Run #2)                │
│  - All 9 workflow steps passed                          │
│  Proves: the formal backbone compiles in trusted CI     │
├─────────────────────────────────────────────────────────┤
│  LAYER 3: GPG IDENTITY                                  │
│  Authority: Personal cryptographic key                 │
│  - Key: 84692E6294128CC1C4ACCD15E747C3AF22573539        │
│  - Signer: Sean Christopher Southwick                  │
│  Proves: a specific human sealed this release            │
└─────────────────────────────────────────────────────────┘
```

Each layer has a different authority. Compromising one does not compromise the others. The combination provides:
- **Structural integrity** (rulesets prevent tampering)
- **Build provenance** (CI proves compilation)
- **Identity binding** (GPG proves human intent)

---

## Evidence Chain

```
Source commit (f280f35d)
  ↓
GitHub Actions (Run #2, ID 30159889545)
  ↓  Lean 4.32.1 installed, 6/6 modules compiled
  ↓  SHA256 evidence generated (formal-sha256sums.txt)
  ↓  Build Provenance attestation attached
  ↓  Verification receipt uploaded (90-day retention)
Evidence captured on master (e41096f)
  ↓
GPG-signed tag (TSCP-v2.4.0-formal-sealed)
  ↓
This dossier
```

### Bundle Contents

| File | Contents |
|------|----------|
| `manifest.json` | Entry point — layer structure, bundle contents, acceptance summary |
| `formal-verification.json` | Per-module compilation results, Classical check, custody invariants |
| `attestation.json` | GitHub Build Provenance reference, workflow run, verification commands |
| `assumptions.json` | Trust contracts for all 2 axioms and 3 sorry |
| `hashes.txt` | SHA256 of all 6 formal .lean source files at the sealed tag |
| `README.md` | This dossier |

---

## Custody Invariants (8)

These are the structural properties the formal backbone enforces. Each is a type-level constraint or theorem in Lean, not a runtime check.

| # | Invariant | What It Means |
|---|-----------|---------------|
| 1 | ProofArtifact | Evidence has a canonical carrier: digest, kind, provenance |
| 2 | BridgeCertificate.artifact | Every certificate carries a ProofArtifact — it IS a custody object |
| 3 | governance_transition_preserves_truth | Governance changes cannot create truth — governance ≠ truth |
| 4 | UtilityFunction | Ranking/policy is injected, not intrinsic to the kernel |
| 5 | utility_does_not_affect_admissibility | Utility ranking does not affect proof admissibility |
| 6 | PromotionResult.reject : RejectionReason → PromotionResult | Every rejection carries a reason — no rejection without cause |
| 7 | DomainEvidence | Evidence classified by kind — kernel responsibility documented |
| 8 | no_rejection removed | The no_rejection constructor is absent — confirmed by its absence |

These invariants encode the core custody principle:

> authorization ≠ truth, evidence ≠ correctness, performance ≠ trust, metadata ≠ artifact identity

---

## Accepted Gaps

### Axioms (2) — Hardware/Runtime Trust Boundary

These are NOT failures of the formal layer. They are explicit assumptions entering it — the boundary where Lean stops and hardware/runtime trust begins. This distinction is critical: the formal layer does not claim to verify hardware; it claims to verify everything *above* the hardware boundary.

#### AXIOM-1: execution_valid

**What is assumed:** Hardware execution produces correct results for the operations in the formal model.

**Why necessary:** Lean operates at the abstraction level of mathematical objects. It cannot verify the behavior of physical hardware.

**What would invalidate it:** Hardware bug, compiler bug, bit-flip, adversarial hardware.

**What would remove it:** A verified runtime that carries proof objects and checks them during execution — reducing the axiom to "proof checking is correct."

#### AXIOM-2: babybear_ntt_end_to_end

**What is assumed:** The NTT implementation produces correct results end-to-end for the BabyBear field.

**Why necessary:** NTT correctness depends on arithmetic properties of a specific finite field implementation that the formal model does not model at that level.

**What would invalidate it:** Incorrect twiddle factors, overflow in field arithmetic, bit-reversal permutation error.

**What would remove it:** Formal verification of the NTT algorithm in Lean (significant effort, likely requires Mathlib's field theory), or test-vector parity proofs (probabilistic).

### Sorry (3) — Reflection Preimage Existence

All three share the same root cause: surjectivity of the rename function. All three are in the **reflection** (reverse, target → source) direction. **None are on the custody path** — custody uses only the forward (preservation) direction.

| ID | Location | Obligation |
|----|----------|------------|
| SORRY-1 | NormalizationBridge.lean:~162 | proof_reflection |
| SORRY-2 | NormalizationBridge.lean:~167 | proof_admissibility.reflects |
| SORRY-3 | NormalizationBridge.lean:~173 | formula_admissibility.reflects |

**What is assumed:** The rename function f is surjective — every target proof/formula has a source preimage.

**Why necessary:** The reflection direction requires finding a source object that maps to a given target object. Without surjectivity, some targets have no preimage.

**What would invalidate it:** Any non-surjective rename function (e.g., f := fun _ => "a" — then only atoms of "a" are in the range).

**What would remove it:** Restricting `normalization_bridge` to bijective renamings. Then surjectivity follows from bijectivity, and all three sorry become theorems simultaneously. This is the recommended resolution.

**Important correction:** Earlier analysis suggested SORRY-3 could be eliminated by proof restructuring (because the trivial kernel makes the admissibility condition trivial). This was incorrect. The trivial kernel simplifies admissibility but does NOT eliminate the need for an existential preimage witness.

Full trust contracts for each gap are in `assumptions.json`.

---

## Reproduction Procedure

An external auditor can reproduce the acceptance decision in 5 steps:

### 1. Clone and checkout

```bash
git clone https://github.com/Cartilage-Stairwells/tscp-anchor.git
cd tscp-anchor
git checkout TSCP-v2.4.0-formal-sealed
```

### 2. Verify the GPG signature

```bash
git tag -v TSCP-v2.4.0-formal-sealed
```

Expected: `Good signature from "SEAN CHRISTOPHER SOUTHWICK"` with key `84692E6294128CC1C4ACCD15E747C3AF22573539`.

### 3. Install Lean and build

```bash
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh -s -- -y --default-toolchain leanprover/lean4:v4.32.1
source ~/.elan/env
lake build
```

Expected: 6/6 modules compile, 0 errors.

### 4. Verify the GitHub attestation

```bash
gh run download 30159889545 --repo Cartilage-Stairwells/tscp-anchor
gh attestation verify formal-sha256sums.txt --repo Cartilage-Stairwells/tscp-anchor
```

### 5. Compare hashes

```bash
find TSCP/Formal -type f -name "*.lean" -exec sha256sum {} + | sort
```

Compare with `release/hashes.txt`. They must match exactly.

---

## Formal Debt Reduction Roadmap

### Track 1: Axioms → Trust Contracts

The two axioms are already classified as trust contracts in `assumptions.json`. The next step is to reduce them:

1. **execution_valid** → Replace with a verified runtime that carries proof objects. The axiom shrinks from "hardware is correct" to "proof checking is correct."
2. **babybear_ntt_end_to_end** → Add test-vector parity proofs (scalar vs NTT). Long-term: formal NTT verification in Lean (separate project, may need Mathlib).

### Track 2: Sorry → Theorems or Accepted Assumptions

All three sorry share the same root cause (surjectivity) and can be simultaneously eliminated by restricting `normalization_bridge` to bijective renamings. If bijectivity is too restrictive for practical use, they are already formally classified as accepted structural assumptions with full trust contracts.

**The key principle: ambiguous trust is the enemy. Every gap is classified, justified, and has a removal path.**

---

## Layer Separation Principle

```
Formal seal (this dossier)
     ↓
Implementation candidate (AVX-512 work)
     ↓
Performance evidence (benchmarks)
     ↓
Optimization promotion (backend status change)
```

Speed is never part of correctness. The formal seal verifies the mathematical structure. The implementation candidate is a separate concern. Performance evidence is collected after correctness is established. Optimization promotion is a policy decision, not a mathematical one.

---

## Contact

- **Repository:** https://github.com/Cartilage-Stairwells/tscp-anchor
- **Sealed by:** Sean Christopher Southwick (GPG key 8469 2E62 9412 8CC1 C4AC CD15 E747 C3AF 2257 3539)
- **Attestation:** GitHub Build Provenance (Run #2, ID 30159889545)

---

*This dossier is the authoritative description of the TSCP v2.4.0 acceptance decision. All other files in the `release/` bundle provide machine-readable evidence referenced here.*
