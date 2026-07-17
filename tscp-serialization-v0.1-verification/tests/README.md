# Test Suite Map: tscp-serialization-v0.1

Maps each test to its corresponding evidence file and verification predicate.

| Test | Predicate | Type | Evidence File |
|---|---|---|---|
| `test_valid_schema_accepted` | #1 Schema Compliance | Unit / positive | evidence/positive/01_schema_compliance.md |
| `test_invalid_schema_rejected` | #1 Schema Compliance | Unit / negative | evidence/negative/01_invalid_schema_rejection.md |
| `test_equivalent_json_canonical_bytes` | #2 Canonical Encoding | Unit / positive | evidence/positive/02_canonical_encoding.md |
| `test_non_canonical_rejected` | #2 Canonical Encoding | Unit / negative | evidence/negative/02_non_canonical_rejection.md |
| `test_round_trip_hash_stability` | #3 Hash Stability | Unit / positive | evidence/positive/03_hash_stability.md |
| `test_mutation_changes_hash` (proptest) | #4 Mutation Sensitivity | Property / positive | evidence/positive/04_mutation_sensitivity.md |
| `test_mutation_always_detected` (proptest) | #4 Mutation Sensitivity | Property / negative | evidence/negative/04_mutation_detection_enforcement.md |
| `test_different_domains_different_hashes` (proptest) | #5 Context Isolation | Property / positive | evidence/positive/05_context_isolation.md |
| `test_cross_domain_no_collision` (proptest) | #5 Context Isolation | Property / negative | evidence/negative/03_cross_domain_collision_resistance.md |
| `test_reproducible_build` | #6 Toolchain Reproducibility | Environmental | evidence/positive/06_toolchain_reproducibility.md |
| `test_authority_boundary_compile` | #7 Authority Boundary | Type-system | evidence/positive/07_authority_boundary.md |
| `test_custody_expression_blocked` | #7 Authority Boundary | Compile-time / negative | evidence/negative/05_custody_expression_blocked.md |

## Running the Full Suite

```bash
cargo test --workspace --release 2>&1 | tee verification_run.log
```

## Mapping Evidence to Seal

When all tests PASS, copy output into corresponding evidence files,
compute SHA-256 of the log, and populate IDENTITY_SEAL.md.

---
*Part of tscp-serialization-v0.1 verification dossier*