# TSCP Trust Boundary Document

**Commit:** 64df9acc773e1564ce8ddbcc836673012ec8e659
**Date:** 2026-07-25

## Question

> What must be trusted outside Lean?

## Answer

Two axioms and three sorry. All five are explicitly classified, none are on the custody path, and none are hidden.

---

## Axioms (2) — Hardware/Runtime Boundary

### 1. `execution_valid`

**Location:** Core.lean:116

**Statement:** `noncomputable axiom execution_valid (n : Nat) : BridgeCertificate (ntt_bridge n)`

**What it asserts:** The NTT bridge from BabyBear proof space to execution space has a valid `BridgeCertificate`.

**What must be trusted:** That the AVX-512 NTT implementation correctly transforms proofs. This is a hardware trust claim — it cannot be verified within Lean without modeling the AVX-512 instruction set formally.

**Why it's acceptable:** This axiom is the entry point of hardware trust into the formal layer. It does not undermine any custody invariant because:
- The custody path (proof → certificate → evidence → custody record) operates downstream of this axiom
- The axiom provides the bridge certificate that the custody path then binds
- Without this axiom, there is no bridge to certify — the formal layer correctly stops at the hardware boundary

**Replacement path:** Either (a) model AVX-512 semantics in Lean (very expensive), or (b) provide runtime evidence (execution traces, hardware attestation) and convert to a verified assumption with external evidence.

### 2. `babybear_ntt_end_to_end`

**Location:** Core.lean:142

**Statement:** `noncomputable axiom babybear_ntt_end_to_end (n : Nat) (v : BabyBearVec n) (h : (ntt_universe n).proof_kernel.admits_proof v) : (ntt_universe n).exec_kernel.admits_proof (ntt_map n v)`

**What it asserts:** If a proof is admissible in the NTT universe's proof kernel, then the NTT-transformed proof is admissible in the execution kernel.

**What must be trusted:** That the NTT pipeline preserves admissibility end-to-end. This is a runtime correctness property for the Number Theoretic Transform implementation.

**Why it's acceptable:** Same boundary as `execution_valid` — this is the runtime/hardware trust entering the formal layer. The custody invariants (artifact identity, governance truth, utility separation, promotion provenance, evidence taxonomy) do not depend on this axiom.

**Replacement path:** Either (a) formally verify the NTT implementation in Lean, or (b) provide benchmark/test evidence and treat as a verified assumption.

---

## Sorry (3) — NormalizationBridge Reflection

All three sorry are in `TSCP.Formal.Examples.NormalizationBridge` and all are in the **reflection** (reverse) direction of bridge certificates.

### The distinction that matters

A `BridgeCertificate` has two directions:

| Direction | Name | What it proves | Status |
|-----------|------|---------------|--------|
| Forward | preservation | If source admits p, target admits rename f p | **Proven** (no sorry) |
| Reverse | reflection | If target admits q, exists source p with rename f p = q | **sorry** (3 instances) |

**The custody path uses the forward direction only.** Evidence flows:
```
certified proof → bridge (preservation) → evidence binding → custody record
```

The reverse direction (reflection) would say: "if the target has an admissible proof, the source must have had one too." This is a completeness property — useful for bi-interpretability but not required for custody.

### The three sorry

1. **`proof_reflection`** (line 162) — preimage existence for proofs
2. **`proof_admissibility.reflects`** (line 167) — preimage existence for admissibility
3. **`formula_admissibility.reflects`** (line 173) — preimage existence for formulas

### Can they become theorems?

**Conditional yes**, if the rename function is restricted to bijections. Currently `normalization_bridge` accepts arbitrary `String → String`. If constrained to bijective renamings (finite variable set), surjectivity gives the preimage and the reflection proofs go through.

**For `formula_admissibility.reflects` specifically:** The trivial kernel makes `preserves` trivial, but `reflects` still needs `∃ p, rename_formula f p = q` — an existential preimage witness. The trivial admissibility does NOT help with preimage existence. This was verified by analysis: the obligation reduces to surjectivity of `rename_formula f`, which is false for non-surjective `f`.

### Recommendation

| Sorry | Action | Rationale |
|-------|--------|-----------|
| proof_reflection | Quarantine | Not on custody path; NormalizationBridge is an example |
| proof_admissibility.reflects | Quarantine | Same |
| formula_admissibility.reflects | Try to eliminate | May be provable by restructuring |

### If quarantined

Move to `docs/trusted-assumptions.md` with explicit justification:
> Reflection (preimage existence) for the normalization bridge is a trusted assumption. It is not required for custody — the preservation direction suffices. The assumption is that if the target kernel admits a proof, the source kernel had an admissible proof mapping to it. This is expected to hold for bijective renamings but is not proven for arbitrary String → String functions.

---

## Summary

| Gap | Type | Count | On custody path? | Action |
|-----|------|-------|-------------------|--------|
| Hardware execution | Axiom | 1 | No (enters layer) | Document boundary |
| Runtime NTT correctness | Axiom | 1 | No (enters layer) | Document boundary |
| Reflection preimage | Sorry | 3 | No (reverse direction) | Quarantine 2, try to eliminate 1 |

**Total external trust required:** 5 claims, all classified, none hidden, none on the custody path.
