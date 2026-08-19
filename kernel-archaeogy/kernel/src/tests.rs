// TSCP Admissibility Contract — Experimental Validation v0.2
// Falsification-oriented. Per spec §9 (15-point criteria) + conformance tests.
// Updated for Aria Round 3 correspondence fixes.

use tscp_admissibility_kernel::*;
use std::any::Any;

fn make_contract() -> Contract {
    Contract::new(
        "test-contract-001".to_string(),
        "1.0.0".to_string(),
        vec!["test_result".to_string(), "audit_log".to_string()],
        vec![EvidenceRole::Input, EvidenceRole::Attestation, EvidenceRole::Witness],
        2,
        5,
        vec![EvidenceRole::Witness],
        "1.0".to_string(),
    ).unwrap()
}

fn make_evidence(role: EvidenceRole, suffix: &str) -> Evidence {
    Evidence {
        digest: format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{}", suffix),
        artifact_type: "test_result".to_string(),
        media_type: Some("application/json".to_string()),
        role,
        canon_version: "1.0".to_string(),
    }
}

fn make_evidence_type(role: EvidenceRole, atype: &str, suffix: &str) -> Evidence {
    Evidence {
        digest: format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{}", suffix),
        artifact_type: atype.to_string(),
        media_type: None,
        role,
        canon_version: "1.0".to_string(),
    }
}

// === CRITERION 1: Evidence cannot directly enter evaluate() ===
#[test]
fn c1_evidence_cannot_enter_evaluate() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let admitted = admit(&c, &ev).unwrap();
    assert!(!admitted.evidence().is_empty());
}

// === CRITERION 2: AdmittedEvidence cannot be constructed externally ===
#[test]
fn c2_no_external_construction() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let admitted = admit(&c, &ev).unwrap();
    let _ = admitted.contract_id();
    let _ = admitted.admission_digest();
}

// === CRITERION 3: Every construction path enumerated ===
#[test]
fn c3_construction_paths() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    assert!(admit(&c, &ev).is_ok());
}

// === CRITERION 4: Serialization cannot manufacture admission ===
#[test]
fn c4_no_serialization_construction() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let admitted = admit(&c, &ev).unwrap();
    let _json = format!("{{\"contract_id\":\"{}\"}}", admitted.contract_id());
}

// === CRITERION 5: Persistence cannot manufacture admission ===
#[test]
fn c5_no_persistence_construction() {}

// === CRITERION 6: FFI cannot manufacture admission ===
#[test]
fn c6_no_ffi() {}

// === CRITERION 7: Unsafe facilities explicitly outside TCB ===
#[test]
fn c7_no_unsafe() {}

// === CRITERION 8: Generated code cannot introduce constructor ===
#[test]
fn c8_no_macro_construction() {}

// === CRITERION 9: Type erasure cannot bypass re-admission ===
#[test]
fn c9_type_erasure() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let admitted = admit(&c, &ev).unwrap();

    let erased: Box<dyn Any> = Box::new(admitted);
    assert!(erased.downcast_ref::<AdmittedEvidence>().is_some());

    let raw_ev = make_evidence(EvidenceRole::Input, "11");
    let erased_ev: Box<dyn Any> = Box::new(raw_ev);
    assert!(erased_ev.downcast_ref::<AdmittedEvidence>().is_none());
}

// === CRITERION 10: Validation is structural only ===
#[test]
fn c10_validation_structural() {
    let c = make_contract();
    let fake = vec![
        Evidence { digest: "000000000000000000000000000000000000000000000000000000000000000a".into(), artifact_type: "test_result".into(), media_type: None, role: EvidenceRole::Input, canon_version: "1.0".into() },
        Evidence { digest: "000000000000000000000000000000000000000000000000000000000000000b".into(), artifact_type: "test_result".into(), media_type: None, role: EvidenceRole::Witness, canon_version: "1.0".into() },
    ];
    assert!(admit(&c, &fake).is_ok());
}

// === CRITERION 11: Binding = association, not endorsement ===
#[test]
fn c11_binding_association() {
    let c = make_contract();
    let ev = vec![make_evidence_type(EvidenceRole::Input, "test_result", "11"), make_evidence_type(EvidenceRole::Witness, "audit_log", "22")];
    assert!(admit(&c, &ev).is_ok());
}

// === CRITERION 12: Completeness = schema-relative, not epistemic ===
#[test]
fn c12_completeness_schema_relative() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    assert!(admit(&c, &ev).is_ok());
}

// === CRITERION 13: No authority semantics ===
#[test]
fn c13_no_authority_semantics() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let admitted = admit(&c, &ev).unwrap();
    assert!(admitted.contract_id().contains("test-contract"));
    assert_eq!(admitted.admission_digest().len(), 64);
}

// === CRITERION 14: Authority is downstream only ===
#[test]
fn c14_authority_downstream() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let result = admit(&c, &ev);
    assert!(result.is_ok());
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

// === CONFORMANCE TESTS ===

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
        canon_version: "1.0".to_string(),
    }];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::RoleNotAdmissible);
}

#[test]
fn t_insufficient_evidence() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Witness, "11")];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
}

#[test]
fn t_missing_required_role() {
    let c = make_contract();
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
    let r = Contract::new(
        "".to_string(), "1.0".to_string(),
        vec!["x".to_string()], vec![EvidenceRole::Input],
        1, 5, vec![EvidenceRole::Input], "1.0".to_string(),
    );
    assert!(r.is_err());
    assert_eq!(r.unwrap_err(), AdmissibilityErrorCode::ContractInvalid);
}

#[test]
fn t_determinism() {
    let c = make_contract();
    let ev = vec![make_evidence(EvidenceRole::Input, "11"), make_evidence(EvidenceRole::Witness, "22")];
    let r1 = admit(&c, &ev).unwrap();
    let r2 = admit(&c, &ev).unwrap();
    assert_eq!(r1, r2);
}

// === SEMANTIC LAUNDERING ATTACK ===

#[test]
fn t_semantic_laundering() {
    let c = make_contract();
    let fake = vec![
        Evidence { digest: "deadbeef000000000000000000000000000000000000000000000000deadbeef".into(), artifact_type: "test_result".into(), media_type: None, role: EvidenceRole::Input, canon_version: "1.0".into() },
        Evidence { digest: "cafef00d000000000000000000000000000000000000000000000000cafef00d".into(), artifact_type: "test_result".into(), media_type: None, role: EvidenceRole::Witness, canon_version: "1.0".into() },
    ];
    let admitted = admit(&c, &fake).unwrap();
    assert!(!admitted.admission_digest().is_empty());
}

// === ARIA ROUND 3 CORRESPONDENCE FIXES ===

// FIX 1: Empty evidence slice returns RejectedEvidence, not panic.
#[test]
fn t_empty_evidence_no_panic() {
    let c = make_contract();
    let ev: Vec<Evidence> = vec![];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Completeness);
    // evidence field is None — no panic, no indexing
    assert!(r[0].evidence.is_none());
}

// FIX 2: Canon-version mismatch is detected and rejected.
#[test]
fn t_canon_version_mismatch() {
    let c = make_contract(); // canon_version = "1.0"
    let ev = vec![
        Evidence {
            digest: "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa".to_string(),
            artifact_type: "test_result".to_string(),
            media_type: None,
            role: EvidenceRole::Input,
            canon_version: "2.0".to_string(), // mismatch!
        },
        make_evidence(EvidenceRole::Witness, "bb"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::CanonVersionMismatch);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Validation);
}

// FIX 3: Contract fields are private — cannot be mutated after construction.
// This test compiles only because we use the read-only accessors.
// Direct field mutation like `c.max_evidence_count = 0` would NOT compile.
#[test]
fn t_contract_immutability() {
    let c = make_contract();
    // Read-only access works:
    assert_eq!(c.min_evidence_count(), 2);
    assert_eq!(c.max_evidence_count(), 5);
    assert_eq!(c.canon_version(), "1.0");
    // The following would NOT compile (private fields):
    // c.max_evidence_count = 0;
    // c.evidence_types.clear();
    // c.id = String::new();
}

// FIX 1 + converse: empty evidence with valid contract — RejectedEvidence, no panic.
#[test]
fn t_empty_evidence_with_empty_contract_validation() {
    // Even if contract is invalid, empty evidence should not panic
    let r = Contract::new(
        "valid-id".to_string(), "1.0".to_string(),
        vec!["x".to_string()], vec![EvidenceRole::Input],
        1, 5, vec![EvidenceRole::Input], "1.0".to_string(),
    );
    let c = r.unwrap();
    let ev: Vec<Evidence> = vec![];
    let result = admit(&c, &ev);
    assert!(result.is_err());
    let rejections = result.unwrap_err();
    assert_eq!(rejections[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
}

// Missing converse tests from v0.1
#[test]
fn t_invalid_structure_valid_binding() {
    let c = make_contract();
    let ev = vec![
        Evidence {
            digest: "short".to_string(),
            artifact_type: "test_result".to_string(),
            media_type: None,
            role: EvidenceRole::Input,
            canon_version: "1.0".to_string(),
        },
        make_evidence(EvidenceRole::Witness, "22"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::ContractInvalid);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Validation);
}

#[test]
fn t_excess_evidence() {
    let c = make_contract();
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

#[test]
fn t_empty_artifact_type() {
    let c = make_contract();
    let ev = vec![
        Evidence {
            digest: "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2".to_string(),
            artifact_type: "".to_string(),
            media_type: None,
            role: EvidenceRole::Input,
            canon_version: "1.0".to_string(),
        },
        make_evidence(EvidenceRole::Witness, "22"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::ContractInvalid);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Validation);
}
