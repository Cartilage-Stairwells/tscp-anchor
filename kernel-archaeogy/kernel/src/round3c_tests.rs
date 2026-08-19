// TSCP Admissibility Kernel — Round 3c Amendment Attack
// Attacks Amendments A & B + their interaction with existing predicates
// Per Aria Round 3b disposition: "Did the amendments eliminate ambiguity
// without introducing a new divergence?"

use tscp_admissibility_kernel::*;

fn make_contract_custom(types: Vec<String>, roles: Vec<EvidenceRole>,
    min: usize, max: usize, required: Vec<EvidenceRole>, canon: &str) -> Contract {
    Contract::new("test-c".into(), "1.0".into(), types, roles, min, max, required, canon.into()).unwrap()
}

fn make_contract() -> Contract {
    make_contract_custom(
        vec!["test_result".into()],
        vec![EvidenceRole::Input, EvidenceRole::Witness],
        2, 5, vec![EvidenceRole::Witness], "1.0",
    )
}

fn make_ev(role: EvidenceRole, suffix: &str) -> Evidence {
    Evidence {
        digest: format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{}", suffix),
        artifact_type: "test_result".to_string(),
        media_type: None, role, canon_version: "1.0".to_string(),
    }
}

fn make_ev_canon(role: EvidenceRole, canon: &str, suffix: &str) -> Evidence {
    Evidence {
        digest: format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{}", suffix),
        artifact_type: "test_result".to_string(),
        media_type: None, role, canon_version: canon.to_string(),
    }
}

fn make_ev_raw(digest: &str, atype: &str, role: EvidenceRole, canon: &str) -> Evidence {
    Evidence { digest: digest.to_string(), artifact_type: atype.to_string(),
        media_type: None, role, canon_version: canon.to_string() }
}

// ============================================================
// A. CANON-VERSION CONVERSE (Amendment A)
// ============================================================

// A.1: "1.0" accepted (spec accepts ↔ Rust accepts)
#[test]
fn a1_accepted_version_1_0() {
    let c = make_contract();
    let ev = vec![make_ev(EvidenceRole::Input, "aa"), make_ev(EvidenceRole::Witness, "bb")];
    assert!(admit(&c, &ev).is_ok());
}

// A.2: Contract with "1.0" constructs successfully
#[test]
fn a2_contract_with_1_0() {
    let c = Contract::new("c".into(), "1.0".into(), vec!["t".into()], vec![EvidenceRole::Input],
        1, 5, vec![], "1.0".into());
    assert!(c.is_ok());
}

// A.3: empty string rejected (spec rejects ↔ Rust rejects)
#[test]
fn a3_empty_canon_rejected() {
    let c = Contract::new("c".into(), "1.0".into(), vec!["t".into()], vec![EvidenceRole::Input],
        1, 5, vec![], "".into());
    assert!(c.is_err());
}

// A.4: "1.1" rejected (not in AcceptedCanonVersions)
#[test]
fn a4_version_1_1_rejected() {
    let c = Contract::new("c".into(), "1.0".into(), vec!["t".into()], vec![EvidenceRole::Input],
        1, 5, vec![], "1.1".into());
    assert!(c.is_err(), "1.1 not in AcceptedCanonVersions = {{1.0}}");
}

// A.5: "0.9" rejected
#[test]
fn a5_version_0_9_rejected() {
    let c = Contract::new("c".into(), "1.0".into(), vec!["t".into()], vec![EvidenceRole::Input],
        1, 5, vec![], "0.9".into());
    assert!(c.is_err(), "0.9 not in AcceptedCanonVersions");
}

// A.6: "garbage" rejected
#[test]
fn a6_garbage_rejected() {
    let c = Contract::new("c".into(), "1.0".into(), vec!["t".into()], vec![EvidenceRole::Input],
        1, 5, vec![], "garbage".into());
    assert!(c.is_err(), "arbitrary string not in AcceptedCanonVersions");
}

// A.7: case variant "1.0" with whitespace rejected
#[test]
fn a7_whitespace_canon_rejected() {
    let c = Contract::new("c".into(), "1.0".into(), vec!["t".into()], vec![EvidenceRole::Input],
        1, 5, vec![], " 1.0".into());
    assert!(c.is_err(), "whitespace variant must not match");
}

// A.8: evidence canon_version must match contract canon_version (both "1.0")
#[test]
fn a8_evidence_canon_matches_contract() {
    let c = make_contract();
    let ev = vec![make_ev_canon(EvidenceRole::Input, "1.0", "aa"), make_ev_canon(EvidenceRole::Witness, "1.0", "bb")];
    assert!(admit(&c, &ev).is_ok());
}

// A.9: evidence canon_version "1.1" rejected (contract is "1.0")
#[test]
fn a9_evidence_canon_1_1_rejected() {
    let c = make_contract();
    let ev = vec![make_ev_canon(EvidenceRole::Input, "1.1", "aa"), make_ev(EvidenceRole::Witness, "bb")];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::CanonVersionMismatch);
}

// A.10: converse — evidence with "1.0" but contract with "1.1" can't even construct
#[test]
fn a10_contract_1_1_cannot_construct() {
    // AcceptedCanonVersions only has "1.0", so contract with "1.1" fails at construction
    let c = Contract::new("c".into(), "1.0".into(), vec!["t".into()], vec![EvidenceRole::Input],
        1, 5, vec![], "1.1".into());
    assert!(c.is_err());
}

// ============================================================
// B. EMPTY-EVIDENCE CORRESPONDENCE (Amendment B)
// ============================================================

// B.1: empty evidence → rejection with evidence: null (None in Rust)
#[test]
fn b1_empty_evidence_null_representation() {
    let c = make_contract();
    let r = admit(&c, &[]).unwrap_err();
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Completeness);
    assert!(r[0].evidence.is_none(), "Amendment B: empty domain → evidence = null");
}

// B.2: converse — non-empty invalid evidence must carry Some(evidence)
#[test]
fn b2_nonempty_invalid_carries_evidence() {
    let c = make_contract();
    let bad = vec![make_ev_raw("short", "test_result", EvidenceRole::Input, "1.0")];
    let r = admit(&c, &bad).unwrap_err();
    assert_eq!(r[0].error_stage, AdmissibilityStage::Validation);
    assert!(r[0].evidence.is_some(), "Non-empty rejection must carry evidence, not null");
}

// B.3: non-empty valid-structure but wrong type → binding rejection carries evidence
#[test]
fn b3_binding_rejection_carries_evidence() {
    let c = make_contract();
    let bad = vec![
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "wrong_type", EvidenceRole::Input, "1.0"),
        make_ev(EvidenceRole::Witness, "bb"),
    ];
    let r = admit(&c, &bad).unwrap_err();
    assert_eq!(r[0].error_stage, AdmissibilityStage::Binding);
    assert!(r[0].evidence.is_some());
}

// B.4: empty evidence with min=1 → insufficient, evidence=None
#[test]
fn b4_empty_evidence_min_1() {
    let c = make_contract_custom(vec!["t".into()], vec![EvidenceRole::Input], 1, 1, vec![], "1.0");
    let r = admit(&c, &[]).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
    assert!(r[0].evidence.is_none());
}

// ============================================================
// C. INTERACTION ATTACK (Amendments × existing predicates)
// ============================================================

// C.1: empty evidence + valid contract → completeness rejection (not validation)
#[test]
fn c1_empty_evidence_valid_contract() {
    let c = make_contract();
    let r = admit(&c, &[]).unwrap_err();
    // Empty evidence with valid contract should reach Stage 3 (Completeness),
    // not be caught by Stage 1 (Validation) — there's nothing to validate.
    assert_eq!(r[0].error_stage, AdmissibilityStage::Completeness);
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
}

// C.2: malformed evidence + canon mismatch → both collected in validation
#[test]
fn c2_malformed_plus_canon_mismatch() {
    let c = make_contract();
    let ev = vec![
        Evidence {
            digest: "short".to_string(),  // malformed
            artifact_type: "test_result".to_string(),
            media_type: None, role: EvidenceRole::Input,
            canon_version: "2.0".to_string(),  // also wrong canon
        },
        make_ev(EvidenceRole::Witness, "bb"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    // Both structural failure AND canon mismatch should be collected
    let first_item_rejections: Vec<_> = r.iter()
        .filter(|rej| rej.evidence.as_ref().map_or(false, |e| e.digest == "short"))
        .collect();
    assert!(first_item_rejections.len() >= 2,
        "Both structural and canon-version rejections should be collected for same item");
}

// C.3: valid evidence + accepted canon → admission succeeds
#[test]
fn c3_valid_evidence_accepted_canon() {
    let c = make_contract();
    let ev = vec![make_ev(EvidenceRole::Input, "aa"), make_ev(EvidenceRole::Witness, "bb")];
    assert!(admit(&c, &ev).is_ok());
}

// C.4: valid evidence + evidence canon mismatch → validation rejection
#[test]
fn c4_valid_evidence_evidence_canon_mismatch() {
    let c = make_contract();
    let ev = vec![
        make_ev_canon(EvidenceRole::Input, "1.0", "aa"),
        make_ev_canon(EvidenceRole::Witness, "2.0", "bb"),  // mismatch
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::CanonVersionMismatch);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Validation);
}

// C.5: incomplete evidence + accepted canon → completeness rejection
#[test]
fn c5_incomplete_evidence_accepted_canon() {
    let c = make_contract(); // min=2, required: Witness
    let ev = vec![make_ev(EvidenceRole::Witness, "aa")]; // only 1, min=2
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Completeness);
}

// C.6: contract-invalid + empty evidence — does implementation produce a rejection record?
#[test]
fn c6_invalid_contract_empty_evidence() {
    // Can't construct an invalid contract via Contract::new (it validates at construction).
    // But we can try: empty canon_version fails Amendment A check.
    let c_result = Contract::new("c".into(), "1.0".into(), vec!["t".into()],
        vec![EvidenceRole::Input], 1, 5, vec![], "1.0".into());
    assert!(c_result.is_ok());
    // Valid contract + empty evidence → InsufficientEvidence (already tested in B.1)
    // We cannot test invalid-contract + empty-evidence because Contract::new
    // rejects invalid contracts at construction. This is a STAGE 0 gate:
    // contract validation happens at CONSTRUCTION, not at admit() time.
}

// C.7: contract with invalid canon_version → rejected at construction (Stage 0)
#[test]
fn c7_contract_invalid_canon_rejected_at_construction() {
    let c = Contract::new("c".into(), "1.0".into(), vec!["t".into()],
        vec![EvidenceRole::Input], 1, 5, vec![], "BAD_VERSION".into());
    assert!(c.is_err(), "Amendment A: non-accepted canon_version rejected at construction");
    assert_eq!(c.unwrap_err(), AdmissibilityErrorCode::ContractInvalid);
}

// C.8: valid evidence + excess count → excess rejection (completeness, not validation)
#[test]
fn c8_excess_evidence() {
    let c = make_contract_custom(vec!["t".into()], vec![EvidenceRole::Witness],
        1, 2, vec![EvidenceRole::Witness], "1.0");
    let ev: Vec<Evidence> = (0..3).map(|i| {
        Evidence {
            digest: format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{:02x}", i),
            artifact_type: "t".to_string(), media_type: None,
            role: EvidenceRole::Witness, canon_version: "1.0".to_string(),
        }
    }).collect();
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::ExcessEvidence);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Completeness);
}

// C.9: stage ordering — validation errors halt before binding
#[test]
fn c9_validation_halts_before_binding() {
    let c = make_contract();
    let ev = vec![
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "wrong_type", EvidenceRole::Input, "2.0"),
        // This item has BOTH: wrong canon (validation) AND wrong type (binding)
        // Validation should catch canon mismatch; binding should NOT execute
    ];
    let r = admit(&c, &ev).unwrap_err();
    for rej in &r {
        assert_eq!(rej.error_stage, AdmissibilityStage::Validation,
            "Validation must halt before binding executes");
    }
    // The wrong type (binding issue) should NOT appear because validation halts first
    let has_type_error = r.iter().any(|rej| rej.error_code == AdmissibilityErrorCode::TypeNotAdmissible);
    assert!(!has_type_error, "Binding must not execute when validation has rejections");
}

// C.10: binding errors halt before completeness
#[test]
fn c10_binding_halts_before_completeness() {
    let c = make_contract(); // min=2, required: Witness
    let ev = vec![make_ev_type(EvidenceRole::Input, "wrong", "aa")]; // wrong type, only 1 item
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_stage, AdmissibilityStage::Binding);
    // Should NOT get InsufficientEvidence because binding halts first
    let has_insufficient = r.iter().any(|rej| rej.error_code == AdmissibilityErrorCode::InsufficientEvidence);
    assert!(!has_insufficient, "Completeness must not execute when binding has rejections");
}

// C.11: multiple validation failures (structure + canon) collected, not first-fail
#[test]
fn c11_multiple_validation_failures_collected() {
    let c = make_contract();
    let ev = vec![
        make_ev_raw("bad1", "test_result", EvidenceRole::Input, "9.9"),  // bad structure + bad canon
        make_ev_raw("bad2", "test_result", EvidenceRole::Witness, "9.9"), // bad structure + bad canon
    ];
    let r = admit(&c, &ev).unwrap_err();
    // Both items should have both rejections (4 total: 2 structural + 2 canon)
    assert!(r.len() >= 4, "All validation failures should be collected, not first-fail");
}

// C.12: Amendment A doesn't change evidence-level canon check (only contract-level)
#[test]
fn c12_evidence_canon_independent_of_accepted_versions() {
    // Evidence canon_version is checked against the CONTRACT's canon_version,
    // not against AcceptedCanonVersions. The contract's canon_version is "1.0"
    // (validated at construction). Evidence with "1.0" matches.
    let c = make_contract();
    let ev = vec![make_ev_canon(EvidenceRole::Input, "1.0", "aa"), make_ev_canon(EvidenceRole::Witness, "1.0", "bb")];
    assert!(admit(&c, &ev).is_ok());

    // Evidence with "2.0" doesn't match contract "1.0" → rejected
    let ev2 = vec![make_ev_canon(EvidenceRole::Input, "2.0", "aa"), make_ev(EvidenceRole::Witness, "bb")];
    let r = admit(&c, &ev2).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::CanonVersionMismatch);
}

fn make_ev_type(role: EvidenceRole, atype: &str, suffix: &str) -> Evidence {
    Evidence {
        digest: format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{}", suffix),
        artifact_type: atype.to_string(), media_type: None, role,
        canon_version: "1.0".to_string(),
    }
}
