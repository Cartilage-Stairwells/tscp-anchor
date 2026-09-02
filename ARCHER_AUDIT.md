# ARCHER Audit Summary

## Overview

The ARCHER framework (GENERATE → PERTURB → EXECUTE → OBSERVE → COMPARE → HYPOTHESIZE → FALSIFY → REPRODUCE → REPORT) was applied across all TSCP repositories in August 2026.

**Total findings:** 36
**Fixed:** 23
**Documented as design boundaries:** 13
**Audit status:** COMPLETE — all source files across all TSCP repositories reviewed

## Methodology

ARCHER is an internal adversarial discovery process with AI-assisted analysis under PI direction. It is preliminary engineering evidence, not independent security certification.

## Findings by Phase

### Phase 1 (Findings 1-18)
Initial audit across P0 baseline, tscp-anchor core, and supporting infrastructure.

### Phase 2 (Findings 19-25) — tscp-kernel, tscp-protocol, oracle-layer, batch-merkle
- F19: verify_golden() hardcodes test claim (documented as test-only)
- F20: sumcheck has no verifier (documented → Issue #35)
- F21: dispatch_event uses counter%2 for transition kind (documented as placeholder)
- F22: evaluate_mle is O(n*2^n) exponential (performance limitation)
- F23: additive shift in DEEP-ALI vs multiplicative shift in constraints (documented → Issue #36)
- F24: Transcript struct may use incompatible Poseidon2 constants (warning added)
- F25: two parallel FRI implementations — oracle-layer has real FRI, delta_fri_bridge has scaffold (documented → Issue #34)

### Phase 3 (Findings 26-30) — tscp-verifier, oracle_bridge, poly_ir
- F26: emitter hardcodes fiat_shamir_rounds to 12 (FIXED — passes real value from oracle bridge)
- F27: verifier_unchanged always set true without checking (documented with TODO)
- F28: public_inputs_hash hashes literal string "public_inputs" not actual inputs (documented as placeholder)
- F29: SerializableFriProof omits Merkle openings — artifact digest over incomplete data (documented → Issue #37)
- F30: SerializableFriProof uses debug format {:?} for roots — non-canonical, non-reproducible (documented → Issue #37)
- Also: expression depth validation (max 100) added to poly_ir.rs to prevent stack overflow

### Phase 4 (Findings 31-36) — foxtrot_harness, prover-server, oracle-layer, cross-repo NTT
- F31: foxtrot_harness hardcoded trace commitment (0xDEADBEEF placeholder) (documented)
- F32: prover-server DEEP-ALI additive shift + unchecked usize to u32 cast (documented → Issue #36)
- F33: naive Lagrange interpolation O(n²) (documented → Issue #38)
- F34: poly_div no zero denominator check (FIXED)
- F35: SoundnessAccumulator not cryptographically bound to Fiat-Shamir transcript (documented → Issue #39)
- F36: incompatible Montgomery forms (R=2³² in zksha-rx vs R=2⁶⁴ in tscp-pl-phase1) (documented → Issue #40)

## Critical Finding (P0)

The proof system proves trace properties but does NOT cryptographically bind an external artifact A to computation output Output(C). H(A)=h and Verify(π,C)=true do not yet imply A=Output(C). Nine seams must be closed. This is documented in the paper (v2.1, Section 4.5) and is the central research question of the ONR proposal.

## Open Implementation Issues

| Issue | Finding | Status |
|-------|---------|--------|
| #34 | F25 | DEEP-ALI → FRI prover connection |
| #35 | F20 | Sumcheck verifier |
| #36 | F23, F32 | Additive → multiplicative shift |
| #37 | F29, F30 | Canonical proof serialization |
| #38 | F33 | FFT-based interpolation |
| #39 | F35 | SoundnessAccumulator → Fiat-Shamir binding |
| #40 | F36 | Montgomery form reconciliation |

## Fix Branches

- `archer/fixes-tscp-anchor` — merged to master (commit 6300f4e3, Sept 2, 2026)
- All 23 fixes are now on master
- 13 design-boundary findings are documented in code with TODO comments
