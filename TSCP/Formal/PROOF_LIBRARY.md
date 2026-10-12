# PROOF_LIBRARY.md — zkSHA-Rx Lean Formalization Corpus

**State:** B3 frozen (2026-08-03) · B5 full-transform round-trip proven (2026-10-12) · 0 axioms · 0 sorries · 101 theorem declarations across 11 files
> (Stage 3, issue #61 ruling S3-A: the three normalization-reflection sorries were discharged
> constructively via a section hypothesis on the atom-renaming function. Stage 4, ruling B:
> `execution_valid` converted to an explicit certificate hypothesis with a machine-checked
> sufficiency construction in the corpus; `babybear_ntt_end_to_end` discharged to a theorem.)

> **Proof-status record (issue #61, updated at Stage 3 completion, 2026-10-12).**
> The 90 counts declarations, not fully proved results — and, as of Stage 4
> (issue #61, ruling B, 2026-10-12), the corpus declares **zero project axioms**.
> `Core.lean` v2.3: the former `noncomputable axiom execution_valid` is REMOVED;
> its two dependents (`ntt_bridge_conservative`, `ntt_preserves_admissibility`)
> now take the bridge certificate as an explicit hypothesis, and the corpus
> carries the machine-checked sufficiency construction
> (`execution_bridge_certificate_of_surjectivity`: surjectivity of the opaque
> `ntt_map` constructs the full certificate, injectivity not required). The
> former `babybear_ntt_end_to_end` axiom is discharged to a theorem (`trivial`:
> its statement is `True → True` under the trivial kernels, so the axiom carried
> no kernel-level content).
> **These are relocations and discharges of bookkeeping, not new mathematics:**
> surjectivity of `ntt_map` is NOT proven — it is now a named, single-purpose
> hypothesis, unprovable in-corpus while `ntt_map` is opaque; proving it requires
> binding the map to the AVX-512 kernel, which is Level C work (implementation
> correspondence). Likewise the trivial discharge of `babybear_ntt_end_to_end`
> does not verify the NTT round-trip, whose evidence lives at the implementation
> level. Results downstream of the certificate hypothesis are conditional on it.
> The three former `sorry` placeholders in `Examples/NormalizationBridge.lean`
> (proof reflection, proof-admissibility reflection, and formula reflection inside
> `def normalization_certificate`) are **discharged**: the certificate construction
> now takes an explicit section hypothesis (`g : String → String`,
> `hsec : ∀ s, f (g s) = s`) and proves all three reflection obligations
> constructively. The unrestricted reflection contract is false — the
> constant-renaming counterexample is machine-checked on issue #61 — so the
> strengthened construction is the discharged form, not a weakening of the
> generic `BridgeCertificate` contract, which is unchanged.
> `certified_proof_crosses_bridge` now carries the section hypotheses explicitly.
> Kernel-level dependency auditing is executed in CI on every push and pull
> request (PR #62): all ten roots build, every source declaration is enumerated
> from the elaborated environment via `Lean.collectAxioms`, reconciled against a
> source census, and gated against `audit/expected-gaps.json`, which now accepts
> **zero** `sorryAx` dependencies and **zero** project axioms: any axiom-kind
> project constant or sorry-dependent declaration fails CI. Per-class counts live
> in the attestation receipt (the committed CI evidence), not in this document.
> Open work (issue #61): Level C — formal-to-implementation correspondence,
> including the surjectivity assumption now stated explicitly in `Core.lean`.

## Architecture

```
ReviewerSemantics.lean    Layer 0 semantics (15 theorems)
        ↓
Montgomery.lean           Montgomery arithmetic (12 theorems)
        ↓
Butterfly.lean            Butterfly algebra (27 theorems)
        ↓
Core.lean                 NTT admissibility (3 theorems + certificate construction)
TSCP_Formal_Backbone.lean Custody framework (4 theorems)
BridgePreservation.lean   Semantic bridge (2 theorems)
Evidence/*.lean           Manifest binding (3 theorems)
Examples/*.lean           Reference examples (10 theorems)
```

---

## Reusable Lemma Catalog

### 1. Field Constants & Basic Properties

| Lemma | File | Purpose | Expected Reuse |
|-------|------|---------|----------------|
| `two_inv_correct` | Butterfly | (2 * two_inv) % P = 1 | Butterfly invertibility, NTT twiddle inversion |
| `R_inv_correct` | Montgomery | (R * R_inv) % P = 1 | Montgomery multiplication correctness |
| `bezout_identity` | Montgomery | R * R_inv = P * NEG_INV + 1 | Foundation for all Montgomery arithmetic |
| `montgomery_radix_coprime` | Montgomery | gcd(R, P) = 1 | Guarantees Montgomery representation is well-defined |
| `neg_inv_correct` | Montgomery | (P * NEG_INV + 1) % R = 0 | REDC algorithm correctness |

### 2. Modular Arithmetic Primitives

| Lemma | File | Purpose | Expected Reuse |
|-------|------|---------|----------------|
| `mod_add_lt` | Butterfly | mod_add a b < P when a,b < P | Closure proofs for all modular operations |
| `mod_sub_lt` | Butterfly | mod_sub a b < P when a,b < P | Closure proofs, NTT stage bounds |
| `mod_mul_lt` | Butterfly | mod_mul a b < P when a,b < P | Closure proofs, twiddle factor bounds |
| `mod_add_comm` | Butterfly | mod_add a b = mod_add b a | Commutativity in stage composition |

### 3. Modular Identity Lemmas

| Lemma | File | Purpose | Expected Reuse |
|-------|------|---------|----------------|
| `mul_P_add_mod` | Butterfly | (P*q + r) % P = r when r < P | Removing multiples of P — universal in modular arithmetic |
| `mod_eq_of_lt_of_congr` | Butterfly | Two values < P and congruent mod P are equal | Core technique: reduce congruence to equality via bounds |
| `add_mod_lemma` | Butterfly | (x%P + y%P) % P = (x+y) % P | Combining modular sums — every NTT stage |
| `mod_sub_congr` | Butterfly | mod_sub a b % P = (a + P - b) % P | Connecting mod_sub to raw subtraction — every subtraction proof |

### 4. Bridge & Transport Lemmas

| Lemma | File | Purpose | Expected Reuse |
|-------|------|---------|----------------|
| `mod_sub_bridge_left` | Butterfly | If a ≡ a' (mod P), then (a+P-b)%P = (a'+P-b)%P when b < P | Replacing first argument in modular subtraction |
| `mod_sub_congr_transport` | Butterfly | If x ≡ x' and y ≡ y' (mod P), both y,y' < P, then (x+P-y)%P = (x'+P-y')%P | **Critical for B4+**: congruence through subtraction when both args in field range |
| `div_mod_diff_le` | Butterfly | n - n%m ≤ m when n/m ≤ 1 | Bounds for nested subtraction under modulo — prevents omega counterexamples |
| `mul_mod_congr` | Montgomery | If a%P = a'%P, then (a*c)%P = (a'*c)%P | Multiplication preserves congruence — butterfly composition, NTT stages |
| `montgomeryMul_congr` | Montgomery | Montgomery multiplication respects congruence classes | Connecting encoded and raw arithmetic |

### 5. Decomposition Lemmas (Nat subtraction case analysis)

| Lemma | File | Purpose | Expected Reuse |
|-------|------|---------|----------------|
| `decompose_pos` | Butterfly | Nat subtraction decomposition when r1 ≥ r2 | encode_sub, any proof with conditional subtraction |
| `decompose_neg` | Butterfly | Nat subtraction decomposition when r1 < r2 | encode_sub, any proof with conditional subtraction |
| `decompose_rhs_pos` | Butterfly | RHS decomposition for positive case | encode_sub helper |
| `decompose_rhs_neg` | Butterfly | RHS decomposition for negative case | encode_sub helper |

### 6. Montgomery Encoding Layer

| Lemma | File | Purpose | Expected Reuse |
|-------|------|---------|----------------|
| `encode_lt` | Butterfly | encode x < P when x < P | All Montgomery-encoded operations |
| `encode_add` | Butterfly | encode (a+b mod P) = mod_add (encode a) (encode b) | Montgomery addition is homomorphic — NTT stage correctness |
| `encode_sub` | Butterfly | encode (a-b mod P) = mod_sub (encode a) (encode b) | Montgomery subtraction is homomorphic — butterfly subtraction |
| `encode_mul` | Butterfly | encode (a*b mod P) = montgomeryMul (encode a) (encode b) | Montgomery multiplication is homomorphic — twiddle multiplication |
| `decode_encode_roundtrip` | Montgomery | decode (encode x) = x | Representation is sound — conformance testing |

### 7. Cancellation & Inverse Lemmas

| Lemma | File | Purpose | Expected Reuse |
|-------|------|---------|----------------|
| `mod_mul_cancel` | Butterfly | mod_mul (mod_mul x w) w_inv = x when (w*w_inv)%P = 1 | Butterfly inversion, NTT inverse transform |
| `cond_sub_eq_mod` | Montgomery | Conditional subtraction equals mod P when u < 2P | Montgomery reduction correctness |

### 8. Butterfly Properties

| Lemma | File | Purpose | Expected Reuse |
|-------|------|---------|----------------|
| `dif_closure` | Butterfly | DIF butterfly outputs stay in [0, P) | NTT stage closure — B4, B5 |
| `dit_closure` | Butterfly | DIT butterfly outputs stay in [0, P) | NTT stage closure — B4, B5 |
| `mont_dif_closure` | Butterfly | Montgomery DIF butterfly stays in [0, P) | Montgomery NTT — B4, B5 |
| `mont_dif_equivalence` | Butterfly | Montgomery butterfly ≡ mathematical butterfly on encoded values | **Central theorem**: bridges implementation and specification |
| `dif_invertible` | Butterfly | DIF butterfly is invertible given twiddle inverse | NTT inverse correctness — B5 |
| `dif_additive` | Butterfly | DIF butterfly is additive (distributes over input addition) | NTT linearity — B4 stage composition |


### 10. Stage Composition (B4)

| Lemma | File | Purpose | Expected Reuse |
|-------|------|---------|----------------|
| `applyButterfly_untouched` | NTTStage | Identity outside butterfly indices — `k ≠ i ∧ k ≠ j → applyButterfly op v k = v k` | Every stage proof — isolates butterfly effects |
| `applyButterfly_at_i` | NTTStage | Output at index i — `applyButterfly op v op.i = (dif_butterfly ...).1` | Butterfly output extraction |
| `applyButterfly_at_j` | NTTStage | Output at index j — `applyButterfly op v op.j = (dif_butterfly ...).2` | Butterfly output extraction |
| `butterfly_preserves_validity` | NTTStage | Single butterfly preserves field validity (uses B3 `dif_closure`) | Foundation for stage validity |
| `stage_preserves_validity` | NTTStage | Stage (any number of butterflies) preserves field validity — by induction | **B5**: NTT validity, **B6**: conformance |
| `stage_deterministic` | NTTStage | Stage application is deterministic (rfl) | **B5**: NTT determinism |
| `disjoint_butterflies_commute` | NTTStage | Butterflies on disjoint index pairs commute (funext + 4-case split) | **B5**: schedule independence |
| `stage_concat` | NTTStage | Stage concatenation: `apply (s₁ ++ s₂) = apply s₂ ∘ apply s₁` (via `List.foldl_append`) | **B5**: NTT = list of stages, multi-stage composition |

### 11. Full NTT Transform (B5)

Constructed on the `Nat → Nat` carrier as a DISTINCT object (ruling R3_DISTINCT_RELATED / Path C, 2026-10-12): no relation to the opaque `Core.ntt_map` is asserted here; that correspondence is deferred to Level C.

| Lemma/Theorem | File | Purpose | Method |
|---------------|------|---------|--------|
| `P_prime` | NTTFull | `Nat.Prime P` — the BabyBear ring is a field | Bounded kernel trial division (d ≤ 44869 since 44870² > P), `decide` over the finite range, bridged via `Nat.prime_def_lt'`; zero axioms beyond the standard three |
| `b5_ntt_forward` | NTTFull | Full forward NTT: `stages.foldl Stage.apply` | Definition (B4 composition) |
| `inv_applyButterfly` / `inv_stage_apply` / `b5_ntt_inverse` | NTTFull | Exact inverse transform via B3 `dif_butterfly_inv`, applied in LIFO order | Definitions |
| `MatchOp` / `MatchOps` / `MatchStage` / `MatchInverse` | NTTFull | Explicit matching predicates (Amendment 1): same indices, inverse twiddles; stage ops reversed (LIFO); stage lists paired in order, applied in reverse | Definitions |
| `op_inv_duality` | NTTFull | A matched inverse butterfly exactly undoes a forward butterfly | B3 `dif_invertible` + B4 `applyButterfly_at_i/at_j/untouched` |
| `stage_inv_duality` | NTTFull | A matched stage exactly inverts the forward stage | Induction on the op list via `stage_inv_duality_gen` |
| `b5_forward_preserves_validity` | NTTFull | Forward transform preserves field validity | Induction via B4 `stage_preserves_validity` |
| `b5_stage_composition` | NTTFull | Stage-list concatenation folds associatively | `List.foldl_append` |
| `b5_ntt_roundtrip` | NTTFull | **Full round-trip identity**: `b5_ntt_inverse inv_stages (b5_ntt_forward stages v) = v` — exact, no N⁻¹ scale factor | Induction over the stage list; per-stage exact duality |

Amended construction note (recorded with the b5_proofs task result, 2026-10-12): the drafted plan's `stage_inverse_step` (`= mod_mul (v k) 2`, cancelled by an `inv_scale`) is mathematically false — a DIF butterfly is not inverted by a forward-shape butterfly with an inverse twiddle. The exact inverse is `dif_butterfly_inv` (B3, certified by `dif_invertible`); with LIFO ordering every stage inverts exactly and the halving lives inside the inverse butterfly, so no scale factor arises.

### 9. Custody Framework (Layer 0)

| Lemma | File | Purpose | Expected Reuse |
|-------|------|---------|----------------|
| `plane_disjoint` | ReviewerSemantics | Custody and Authority planes are distinct | All plane separation proofs |
| `completeness_gating` | ReviewerSemantics | Incomplete context → Indeterminate (contrapositive) | Evaluation semantics |
| `determinism` | ReviewerSemantics | evaluate is deterministic | All evaluation proofs |
| `indeterminate_implies_incomplete` | ReviewerSemantics | Indeterminate requires incomplete context | Evaluation semantics |
| `evaluation_preserves_equality` | ReviewerSemantics | Equal contexts → equal results | Semantic preservation |
| `authority_unreachability` | ReviewerSemantics | Authority plane unreachable from Custody | Custody isolation |
| `state_equiv_refl/symm/trans` | ReviewerSemantics | StateEquiv is an equivalence relation | State comparison |

---

## Milestone Roadmap

| Milestone | Status | Theorems | Sorries | Axioms |
|-----------|--------|----------|---------|--------|
| B1: Layer 0 types | ✅ Frozen | 15 | 0 | 0 |
| B2: Montgomery arithmetic | ✅ Frozen | 12 | 0 | 0 |
| B3: Butterfly algebra | ✅ Frozen | 27 | 0 | 0 |
| B4: Stage composition | ✅ Frozen | 8 | 0 | 0 |
| B5: Full NTT correctness | Next | — | — | — |
| B6: Conformance vectors | Pending | — | — | — |

## B4 Design Principle

B4 treats the butterfly as a **verified primitive**. No new modular arithmetic.
- Express one NTT stage as a collection of butterflies
- Show butterflies operate on disjoint index pairs
- Prove stage composition preserves invariants
- Establish stage semantics match mathematical NTT specification

If B4 requires new modular arithmetic, that lemma belongs in B3, not B4.

## File Inventory

| File | Theorems | Role |
|------|----------|------|
| Butterfly.lean | 27 | Butterfly algebra + reusable arithmetic |
| NTTStage.lean | 8 | Stage composition (B4) |
| ReviewerSemantics.lean | 15 | Layer 0 semantic types |
| Montgomery.lean | 12 | Montgomery representation |
| Examples/NormalizationBridge.lean | 12 | Reference: normalization bridge (incl. section/reflection lemmas) |
| TSCP_Formal_Backbone.lean | 4 | Custody framework |
| Examples/PropositionalKernel.lean | 4 | Reference: propositional kernel |
| Evidence/ManifestBinding.lean | 3 | Manifest binding |
| Core.lean | 3 | NTT admissibility (incl. discharged round-trip theorem) |
| BridgePreservation.lean | 2 | Semantic bridge |
| **Total** | **90** | |
