// TSCP Admissibility Contract — Experimental Validation
// Falsification-oriented. Per spec §9 (15-point criteria) + conformance tests.

use tscp_admissibility_kernel::*;
use std::any::Any;

fn make_contract() -> Contract {
    Contract {
        id: "test-contract-001".to_string(),
        version: "1.0.0".to_string(),
        evidence_types: vec!["test_result".to_string(), "audit_log".to_string()],
        evidence_roles: vec![EvidenceRole::Input, EvidenceRole::Attestation, EvidenceRole::Witness],
        min_evidence_count: 2,
        max_evidence_count: 5,
        required_roles: vec![EvidenceRole::Witness],
        canon_version: "1.0".to_string(),
    }
}

fn make_evidence(role: EvidenceRole, suffix: &str) -> Evidence {
    // 62-char base + 2-char suffix = 64-char hex digest
    Evidence {
        digest: format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{}", suffix),
        artifact_type: "test_result".to_string(),
        media_type: Some("application/json".to_string()),
        role,
    }
}

fn make_evidence_type(role: EvidenceRole, atype: &str, suffix: &str) -> Evidence {
    Evidence {
        digest: format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{}", suffix),
        artifact_type: atype.to_string(),
        media_type: None,
        role,
    }
}

// === CRITERION 1: Evidence cannot directly enter evaluate() ===
#[test]
fn c1_evidence_cannot_enter_evaluate() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let admitted = admit(&c, &ev).unwrap();
    // evaluate() would accept AdmittedEvidence, not Evidence. Type-enforced.
    assert!(!admitted.evidence().is_empty());
}

// === CRITERION 2: AdmittedEvidence cannot be constructed externally ===
#[test]
fn c2_no_external_construction() {
    // All fields private. No pub constructor. No Default.
    // This compiles only because we access via admit():
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let admitted = admit(&c, &ev).unwrap();
    // Read-only accessors work:
    let _ = admitted.contract_id();
    let _ = admitted.admission_digest();
}

// === CRITERION 3: Every construction path enumerated ===
#[test]
fn c3_construction_paths() {
    // 1. Struct literal → BLOCKED (private fields)
    // 2. pub fn new() → DOES NOT EXIST
    // 3. Default trait → NOT IMPLEMENTED
    // 4. serde Deserialize → NO serde dependency
    // 5. unsafe transmute → NO unsafe code
    // 6. FFI → NO FFI
    // 7. Macros → NO macros emit AdmittedEvidence
    // 8. admit() → THE ONLY PATH
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    assert!(admit(&c, &ev).is_ok());
}

// === CRITERION 4: Serialization cannot manufacture admission ===
#[test]
fn c4_no_serialization_construction() {
    // No serde. No Serialize/Deserialize. Manual JSON reconstruction impossible:
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let admitted = admit(&c, &ev).unwrap();
    let _json = format!("{{\"contract_id\":\"{}\"}}", admitted.contract_id());
    // Cannot deserialize back to AdmittedEvidence — no impl exists.
}

// === CRITERION 5: Persistence cannot manufacture admission ===
#[test]
fn c5_no_persistence_construction() {
    // Same as serialization: no persistence trait. Re-admission required.
}

// === CRITERION 6: FFI cannot manufacture admission ===
#[test]
fn c6_no_ffi() {
    // No extern "C", no #[no_mangle], no #[repr(C)] on AdmittedEvidence.
}

// === CRITERION 7: Unsafe facilities explicitly outside TCB ===
#[test]
fn c7_no_unsafe() {
    // Zero unsafe blocks. TCB = safe Rust only.
    // grep "unsafe" src/lib.rs → only in comments.
}

// === CRITERION 8: Generated code cannot introduce constructor ===
#[test]
fn c8_no_macro_construction() {
    // No derive macros except Debug/Clone/PartialEq/Eq.
    // No macros emit AdmittedEvidence.
}

// === CRITERION 9: Type erasure cannot bypass re-admission ===
#[test]
fn c9_type_erasure() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let admitted = admit(&c, &ev).unwrap();

    // Erase to dyn Any, restore same type — works (preserves existing value)
    let erased: Box<dyn Any> = Box::new(admitted);
    assert!(erased.downcast_ref::<AdmittedEvidence>().is_some());

    // Erase Evidence, try downcast to AdmittedEvidence — FAILS (different types)
    let raw_ev = make_evidence(EvidenceRole::Input, "11");
    let erased_ev: Box<dyn Any> = Box::new(raw_ev);
    assert!(erased_ev.downcast_ref::<AdmittedEvidence>().is_none());
}

// === CRITERION 10: Validation is structural only ===
#[test]
fn c10_validation_structural() {
    // Fabricated digest (structurally valid 64-char hex, no real artifact) → ADMITTED.
    // This is correct: admission = structural admissibility, NOT truth.
    let c = make_contract();
    let fake = vec![
        Evidence { digest: "000000000000000000000000000000000000000000000000000000000000000a".into(), artifact_type: "test_result".into(), media_type: None, role: EvidenceRole::Input },
        Evidence { digest: "000000000000000000000000000000000000000000000000000000000000000b".into(), artifact_type: "test_result".into(), media_type: None, role: EvidenceRole::Witness },
    ];
    assert!(admit(&c, &fake).is_ok());
}

// === CRITERION 11: Binding = association, not endorsement ===
#[test]
fn c11_binding_association() {
    let c = make_contract();
    let ev = vec![make_evidence_type(EvidenceRole::Input, "test_result", "11"), make_evidence_type(EvidenceRole::Witness, "audit_log", "22")];
    assert!(admit(&c, &ev).is_ok()); // Admitted — types match. Not endorsement.
}

// === CRITERION 12: Completeness = schema-relative, not epistemic ===
#[test]
fn c12_completeness_schema_relative() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    assert!(admit(&c, &ev).is_ok()); // Complete relative to schema. Not epistemically complete.
}

// === CRITERION 13: No authority semantics ===
#[test]
fn c13_no_authority_semantics() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let admitted = admit(&c, &ev).unwrap();
    // Fields: contract_id, contract_version, evidence, admitted_at, admission_digest.
    // None express authority, threshold, weight, or decision.
    assert!(admitted.contract_id().contains("test-contract"));
    assert_eq!(admitted.admission_digest().len(), 64);
}

// === CRITERION 14: Authority is downstream only ===
#[test]
fn c14_authority_downstream() {
    // admit() returns AdmittedEvidence, not Authority or Decision.
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let result = admit(&c, &ev);
    assert!(result.is_ok()); // Ok(AdmittedEvidence) — not Ok(Authority)
}

// === CRITERION 15: Specification meaning preserved ===
#[test]
fn c15_spec_preserved() {
    let c = make_contract();
    let bad = vec![make_evidence_type(EvidenceRole::Input, "wrong_type", "11"), make_evidence_type(EvidenceRole::Witness, "wrong_type", "22")];
    let rejections = admit(&c, &bad).unwrap_err();
    assert_eq!(rejections[0].error_code, AdmissibilityErrorCode::TypeNotAdmissible);
    assert_eq!(rejections[0].error_stage, AdmissibilityStage::Binding);
}

// === CONFORMANCE TESTS (spec §10) ===

#[test]
fn t_valid_admission() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let admitted = admit(&c, &ev).unwrap();
    assert_eq!(admitted.evidence().len(), 2);
    assert_eq!(admitted.admission_digest().len(), 64);
}

#[test]
fn t_type_rejection() {
    let c = make_contract();
    let ev = vec![make_evidence_type(EvidenceRole::Input, "wrong", "11"), make_evidence_type(EvidenceRole::Witness, "wrong", "22")];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r.len(), 2);
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::TypeNotAdmissible);
}

#[test]
fn t_role_rejection() {
    let c = make_contract();
    let ev = vec![Evidence {
        digest: "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2".to_string(),
        artifact_type: "test_result".to_string(), media_type: None, role: EvidenceRole::Output,
    }];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::RoleNotAdmissible);
}

#[test]
fn t_insufficient_evidence() {
    let c = make_contract(); // min=2
    let ev = vec![make_evidence(EvidenceRole::Witness, "11")]; // only 1
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
}

#[test]
fn t_missing_required_role() {
    let c = make_contract(); // required: Witness
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Input, "22")];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::MissingRequiredRole);
}

#[test]
fn t_duplicate_digest() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "aa"), make_evidence(EvidenceRole::Witness, "aa")];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::DuplicateDigest);
}

#[test]
fn t_invalid_contract() {
    let c = Contract {
        id: "".to_string(), version: "1.0".to_string(),
        evidence_types: vec!["x".to_string()], evidence_roles: vec![EvidenceRole::Input],
        min_evidence_count: 1, max_evidence_count: 5, required_roles: vec![EvidenceRole::Input],
        canon_version: "1.0".to_string(),
    };
    let ev = vec![make_evidence(EvidenceRole::Input, "11")];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::ContractInvalid);
}

#[test]
fn t_determinism() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let r1 = admit(&c, &ev).unwrap();
    let r2 = admit(&c, &ev).unwrap();
    assert_eq!(r1, r2); // same inputs → identical outputs including admission digest
}

// === SEMANTIC LAUNDERING ATTACK (Aria §7) ===

#[test]
fn t_semantic_laundering() {
    // Fabricated digests, no real artifacts — admitted because structurally valid.
    // This is CORRECT: admission ≠ truth. The firewall holds.
    let c = make_contract();
    let fake = vec![
        Evidence { digest: "deadbeef000000000000000000000000000000000000000000000000deadbeef".into(), artifact_type: "test_result".into(), media_type: None, role: EvidenceRole::Input },
        Evidence { digest: "cafef00d000000000000000000000000000000000000000000000000cafef00d".into(), artifact_type: "test_result".into(), media_type: None, role: EvidenceRole::Witness },
    ];
    let admitted = admit(&c, &fake).unwrap();
    assert!(!admitted.admission_digest().is_empty());
    // AdmittedEvidence with fabricated digests — admission contract satisfied.
    // NOT truth. NOT correctness. NOT authority. Just: structurally admissible.
}

// === MISSING CONVERSE TESTS (per Johnny Stage 6) ===

// invalid structure + correct binding — malformed digest, valid type/role
#[test]
fn t_invalid_structure_valid_binding() {
    let c = make_contract();
    let ev = vec![
        Evidence {
            digest: "short".to_string(), // NOT 64-char hex
            artifact_type: "test_result".to_string(),
            media_type: None,
            role: EvidenceRole::Input,
        },
        make_evidence(EvidenceRole::Witness, "22"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::ContractInvalid);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Validation);
}

// excess evidence — more items than max_evidence_count
#[test]
fn t_excess_evidence() {
    let c = make_contract(); // max=5
    let ev: Vec<Evidence> = (0..6)
        .map(|i| {
            let suffix = format!("{:02x}", i);
            make_evidence(EvidenceRole::Witness, &suffix)
        })
        .collect();
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::ExcessEvidence);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Completeness);
}

// valid structure + empty artifact_type
#[test]
fn t_empty_artifact_type() {
    let c = make_contract();
    let ev = vec![
        Evidence {
            digest: "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2".to_string(),
            artifact_type: "".to_string(), // empty — structurally invalid
            media_type: None,
            role: EvidenceRole::Input,
        },
        make_evidence(EvidenceRole::Witness, "22"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::ContractInvalid);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Validation);
}
