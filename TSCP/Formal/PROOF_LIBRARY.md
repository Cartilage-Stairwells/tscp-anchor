# PROOF_LIBRARY.md — zkSHA-Rx Lean Formalization Corpus

**State:** B3 frozen (2026-08-03) · 0 axioms · 0 sorries · 75 proven theorems across 9 files

## Architecture

```
ReviewerSemantics.lean    Layer 0 semantics (15 theorems)
        ↓
Montgomery.lean           Montgomery arithmetic (12 theorems)
        ↓
Butterfly.lean            Butterfly algebra (27 theorems)
        ↓
Core.lean                 NTT admissibility (2 theorems)
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
| B4: DIF/DIT stage composition | Next | — | — | — |
| B5: Full NTT correctness | Pending | — | — | — |
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
| ReviewerSemantics.lean | 15 | Layer 0 semantic types |
| Montgomery.lean | 12 | Montgomery representation |
| Examples/NormalizationBridge.lean | 6 | Reference: normalization bridge |
| TSCP_Formal_Backbone.lean | 4 | Custody framework |
| Examples/PropositionalKernel.lean | 4 | Reference: propositional kernel |
| Evidence/ManifestBinding.lean | 3 | Manifest binding |
| Core.lean | 2 | NTT admissibility |
| BridgePreservation.lean | 2 | Semantic bridge |
| **Total** | **75** | |
