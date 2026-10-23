# PROOF_LIBRARY Lean Census Errata v1.0

**Document:** PROOF_LIBRARY_ERRATA_Lean_census_v1.0  
**Date:** 2026-10-07  
**Status:** DRAFTED errata (documentation correction)  
**Applies to:** `TSCP/Formal/PROOF_LIBRARY.md` as observed at `tscp-anchor` HEAD `49f7f6b` (baseline family `tscp-freeze-0.6.1`)  
**Authority:** None — does not modify theorems; corrects narrative scope only  
**Source of census:** Static inspection reported in receipt `TSCP-TRANSFER-PASS-v1.0` (shallow clone; no `lake build` re-execution in that pass)

---

## 1. Purpose

Record measurable discrepancies between the **PROOF_LIBRARY.md header narrative** and a **file-level declaration census** of `TSCP/Formal/**/*.lean` at the cited commit, so downstream agents do not treat the header line as an exact machine inventory.

---

## 2. Header claim (as written)

From `TSCP/Formal/PROOF_LIBRARY.md`:

> **State:** B3 frozen (2026-08-03) · 0 axioms · 0 sorries · 83 proven theorems across 10 files

---

## 3. Observed census @ `49f7f6b` (declaration count)

Counts are of lines matching `theorem` / `thm` / `lemma` declarations (grep-style), not a kernel-checked “proven” metric.

| File | Declarations | `sorry` (body) |
|------|--------------|----------------|
| BridgePreservation.lean | 2 | 0 |
| Butterfly.lean | 27 | 0 |
| Core.lean | 3 | 0 |
| Evidence/ManifestBinding.lean | 3 | 0 |
| Examples/NormalizationBridge.lean | 6 | **3** |
| Examples/PropositionalKernel.lean | 4 | 0 |
| Montgomery.lean | 12 | 0 |
| NTTStage.lean | 8 | 0 |
| ReviewerSemantics.lean | 15 | 0 |
| TSCP_Formal_Backbone.lean | 4 | 0 |
| **Total (10 files)** | **84** | **3** (all in NormalizationBridge) |

**Discrepancy A — theorem count:** Header says **83**; census sums to **84**.

**Discrepancy B — zero sorries:** Header says **0 sorries**. That holds for the **backbone** set (excluding `Examples/NormalizationBridge.lean`). It does **not** hold for the full 10-file tree if Examples are included: NormalizationBridge carries **3** `sorry` goals labeled as future work (reflection / invertibility).

**Discrepancy C — zero axioms (scope):** Header implies no axioms in the stated corpus. Separately, **outside** the 10-file PROOF_LIBRARY list:

- `BabyBear/Verifier.lean` contains an `axiom` (`babybear_verifier_injective`).
- `TraceCoreProver/Kernel.lean` contains `sorry` placeholders.

Those trees are **not** claimed by the “83 / 0 sorry / 0 axiom” one-liner unless PROOF_LIBRARY explicitly widens scope (it does not).

---

## 4. B5 status (unchanged)

| Item | Status |
|------|--------|
| Full NTT composition theorem | **Not present** — B5 remains **next / unclaimed** |
| Stage-level lemmas (`stage_preserves_validity`, `stage_deterministic`, etc.) | Present; PROOF_LIBRARY tags some as B5 *forward references* |
| Risk | Mild narrative drift if “B5 forward reference” is read as “B5 complete” |

This errata does **not** promote B5.

---

## 5. Corrected narrative (recommended replacement text)

Replace the PROOF_LIBRARY header state line with scoped language, for example:

> **State (scoped):** B3–B4 backbone frozen narrative (see file table).  
> **Backbone** (`Core`, `Montgomery`, `Butterfly`, `NTTStage`, `ReviewerSemantics`, `TSCP_Formal_Backbone`, `BridgePreservation`, `Evidence/*`, `Examples/PropositionalKernel`): **0 `sorry` in census @ 49f7f6b**.  
> **Full 10-file `TSCP/Formal` tree:** **84** `theorem`/`lemma` declarations; **`Examples/NormalizationBridge.lean` holds 3 `sorry` (future work)**.  
> **Axioms:** none in the backbone table above; **`BabyBear/Verifier.lean` axiom is outside this corpus**.  
> **B5 (full NTT correctness):** not claimed.

Exact theorem totals should be re-counted after any merge; do not treat 84 as eternally fixed.

---

## 6. What this errata does *not* claim

- Does not assert every declaration is kernel-checked without `sorry` beyond the census method.  
- Does not re-run `lake build` or CI.  
- Does not change Lean sources.  
- Does not claim B5, end-to-end FRI soundness, or production readiness.

---

## 7. Evidence boundary

| Item | Label |
|------|-------|
| Header text in PROOF_LIBRARY.md | OBSERVED |
| Declaration/`sorry` census methodology | Static grep-style @ `49f7f6b` (agent receipt) |
| Recommended replacement prose | DRAFTED editorial |
| Independent full Lean rebuild in this errata | NOT performed |

---

*End of PROOF_LIBRARY_ERRATA_Lean_census_v1.0*
