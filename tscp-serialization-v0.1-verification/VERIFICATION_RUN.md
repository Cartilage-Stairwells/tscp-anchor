# Verification Run Log: tscp-serialization-v0.1

**Date:** 2026-07-17
**Toolchain:** rustc 1.97.1 (8bab26f4f 2026-07-14), cargo 1.97.1
**Platform:** Linux x86_64 (gvisor)
**Command:** `cargo test -p tscp-kernel --test serialization_conformance --release`

## Result

```
running 12 tests
test test_authority_boundary_compile ... ok
test test_cross_domain_no_collision ... ok
test test_different_domains_different_hashes ... ok
test test_custody_expression_blocked ... ok
test test_equivalent_json_canonical_bytes ... ok
test test_invalid_schema_rejected ... ok
test test_mutation_changes_hash ... ok
test test_mutation_always_detected ... ok
test test_non_canonical_rejected ... ok
test test_round_trip_hash_stability ... ok
test test_valid_schema_accepted ... ok
test test_reproducible_build ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## SHA-256 Digests

| Artifact | SHA-256 |
|---|---|
| Verification run log | `34d55c12efc8c59bdd9014f66fb7eef2551b4a4a0dab260fc467567d28833d66` |
| Test suite (`serialization_conformance.rs`) | `0c67a5f962317755c6aa7b48a876a85d4bc8a052f6a65c496c45ed7a77379bd8` |
| Kernel serialization module | `676563d082490dcdd69787446da2e5279917ef97f97047371be817b9ea4da8b6` |
| Kernel types module | `50c19aba274a0d7abcfe8bada22d6026ec4b002d118cc5bff0eaeb4827163c57` |

## Test-to-Predicate Mapping

| Test | Predicate | Result |
|---|---|---|
| `test_valid_schema_accepted` | #1 Schema Compliance (positive) | PASS |
| `test_invalid_schema_rejected` | #1 Schema Compliance (negative) | PASS |
| `test_equivalent_json_canonical_bytes` | #2 Canonical Encoding (positive) | PASS |
| `test_non_canonical_rejected` | #2 Canonical Encoding (negative) | PASS |
| `test_round_trip_hash_stability` | #3 Hash Stability | PASS |
| `test_mutation_changes_hash` | #4 Mutation Sensitivity (positive) | PASS |
| `test_mutation_always_detected` | #4 Mutation Sensitivity (negative) | PASS |
| `test_different_domains_different_hashes` | #5 Context Isolation (positive) | PASS |
| `test_cross_domain_no_collision` | #5 Context Isolation (negative) | PASS |
| `test_reproducible_build` | #6 Toolchain Reproducibility | PASS |
| `test_authority_boundary_compile` | #7 Authority Boundary (positive) | PASS |
| `test_custody_expression_blocked` | #7 Authority Boundary (negative) | PASS |

## Verdict

All 12 tests PASS. All 7 verification predicates satisfied.
**VERIFICATION_PACKAGE_PASS**
