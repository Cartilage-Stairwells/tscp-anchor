# Verification Contract: tscp-serialization-v0.1

**Status:** FROZEN  
**Version:** 0.1  
**Date frozen:** 2026-07-14

tscp-serialization-v0.1 is VERIFIED if and only if ALL seven predicates below evaluate to PASS
under the conditions specified in ENVIRONMENT.md. A single FAIL invalidates the package.

---

## The Seven Verification Predicates

### 1. Schema Compliance
**Claim:** The serialization engine strictly rejects any payload that does not conform to the
defined schema boundaries. All field types, structure layouts, and length constraints are
enforced at the boundary.

**Test class:** Unit — invalid input rejection  
**Evidence file:** evidence/negative/01_invalid_schema_rejection.md

### 2. Canonical Encoding Correctness
**Claim:** For any given object state, there exists exactly one valid serialized representation.
Equivalent JSON structures produce identical canonical bytes.

**Test class:** Unit — equivalent-input identity  
**Evidence file:** evidence/positive/02_canonical_encoding.md + evidence/negative/02_non_canonical_rejection.md

### 3. Hash Stability
**Claim:** Serialize/deserialize cycles preserve hashes. Deterministic digest across all conforming environments.

**Test class:** Unit — round-trip stability  
**Evidence file:** evidence/positive/03_hash_stability.md

### 4. Mutation Sensitivity
**Claim:** Any single-bit change produces a changed identity. No collisions admitted.

**Test class:** Property-based — mutation detection  
**Evidence file:** evidence/positive/04_mutation_sensitivity.md + evidence/negative/04_mutation_detection_enforcement.md

### 5. Context Isolation
**Claim:** Different TSCP domains produce different hashes. Engine is a pure function.

**Test class:** Property-based — domain separation  
**Evidence file:** evidence/positive/05_context_isolation.md + evidence/negative/03_cross_domain_collision_resistance.md

### 6. Toolchain Reproducibility
**Claim:** Compilation is bitwise reproducible across identical toolchains.

**Test class:** Environmental — reproducible build  
**Evidence file:** evidence/positive/06_toolchain_reproducibility.md

### 7. Authority Boundary Preservation
**Claim:** Evidence records cannot express custody decisions. Containment is structural.

**Test class:** Type-system / compile-time containment  
**Evidence file:** evidence/positive/07_authority_boundary.md + evidence/negative/05_custody_expression_blocked.md

---

## Invariant Summary

| # | Invariant | Test Class | Status |
|---|---|---|---|
| 1 | Schema compliance | Unit | PENDING |
| 2 | Canonical encoding correctness | Unit | PENDING |
| 3 | Hash stability | Unit | PENDING |
| 4 | Mutation sensitivity | Property-based | PENDING |
| 5 | Context isolation | Property-based | PENDING |
| 6 | Toolchain reproducibility | Environmental | PENDING |
| 7 | Authority boundary preservation | Type-system | PENDING |

---

## Architectural Note

```
verified dependency  ≠  verified consumer
artifact identity    ≠  authority
VERIFIED_ARTIFACT_IDENTITY  ≠  VERIFIED_CUSTODY
```

---
*Contract frozen: 2026-07-14*
*Authorized by Triune-Oracle*