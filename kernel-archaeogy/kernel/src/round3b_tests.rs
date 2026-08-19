// TSCP Admissibility Kernel — Round 3b Bidirectional Correspondence Attack
// Attacks commit a8f38b1 against ADMISSIBILITY_CONTRACT_SPEC v0.2
// Six vectors: false positive, false negative, component divergence,
// boundary/domain, repaired-seam regression, specification gaps

use tscp_admissibility_kernel::*;

fn make_contract_custom(
    types: Vec<String>, roles: Vec<EvidenceRole>,
    min: usize, max: usize, required: Vec<EvidenceRole>,
    canon: &str,
) -> Contract {
    Contract::new(
        "test-c".to_string(), "1.0".to_string(),
        types, roles, min, max, required, canon.to_string(),
    ).unwrap()
}

fn make_contract() -> Contract {
    make_contract_custom(
        vec!["test_result".into(), "audit_log".into()],
        vec![EvidenceRole::Input, EvidenceRole::Attestation, EvidenceRole::Witness],
        2, 5,
        vec![EvidenceRole::Witness],
        "1.0",
    )
}

fn make_ev(role: EvidenceRole, suffix: &str) -> Evidence {
    Evidence {
        digest: format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{}", suffix),
        artifact_type: "test_result".to_string(),
        media_type: Some("application/json".to_string()),
        role,
        canon_version: "1.0".to_string(),
    }
}

fn make_ev_type(role: EvidenceRole, atype: &str, suffix: &str) -> Evidence {
    Evidence {
        digest: format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{}", suffix),
        artifact_type: atype.to_string(),
        media_type: None,
        role,
        canon_version: "1.0".to_string(),
    }
}

fn make_ev_canon(role: EvidenceRole, canon: &str, suffix: &str) -> Evidence {
    Evidence {
        digest: format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{}", suffix),
        artifact_type: "test_result".to_string(),
        media_type: None,
        role,
        canon_version: canon.to_string(),
    }
}

fn make_ev_raw(digest: &str, atype: &str, role: EvidenceRole, canon: &str) -> Evidence {
    Evidence {
        digest: digest.to_string(),
        artifact_type: atype.to_string(),
        media_type: None,
        role,
        canon_version: canon.to_string(),
    }
}

// ============================================================
// VECTOR 1: FALSE POSITIVE — Rust accepts, Spec should reject
// ============================================================

// V1.1: Spec §2.2 says evidence has media_type as string | null.
// Rust uses Option<String>. Does Rust accept evidence missing media_type?
// Spec says media_type is optional, so this is fine — PASS.
#[test]
fn v1_1_media_type_none_accepted() {
    let c = make_contract();
    let ev = vec![
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "test_result", EvidenceRole::Input, "1.0"),
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1bb", "test_result", EvidenceRole::Witness, "1.0"),
    ];
    assert!(admit(&c, &ev).is_ok(), "media_type=None should be accepted per spec");
}

// V1.2: Spec §3.2 Stage 1.2 says digest must be "64-char lowercase hex".
// Does Rust reject uppercase hex? Spec says lowercase.
#[test]
fn v1_2_uppercase_hex_rejected() {
    let c = make_contract();
    let ev = vec![
        make_ev_raw("A1B2C3D4E5F6A7B8C9D0E1F2A3B4C5D6E7F8A9B0C1D2E3F4A5B6C7D8E9F0A1AA", "test_result", EvidenceRole::Input, "1.0"),
        make_ev(EvidenceRole::Witness, "bb"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    // Spec says "64-char lowercase hex" — uppercase must be rejected
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::ContractInvalid);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Validation);
}

// V1.3: Spec §2.5 defines RejectedEvidence.evidence as Evidence (not Option).
// Rust now uses Option<Evidence>. For non-empty evidence, does Rust populate it?
// This is a spec/impl divergence in the RejectedEvidence type itself.
#[test]
fn v1_3_rejected_evidence_populated_for_non_empty() {
    let c = make_contract();
    let bad = vec![make_ev_type(EvidenceRole::Input, "wrong", "11"), make_ev_type(EvidenceRole::Witness, "wrong", "22")];
    let r = admit(&c, &bad).unwrap_err();
    // Spec says RejectedEvidence.evidence: Evidence (non-optional)
    // Rust uses Option<Evidence> — for non-empty evidence, should be Some
    assert!(r[0].evidence.is_some(), "RejectedEvidence.evidence should be Some for non-empty input");
}

// V1.4: Spec §3.2 Stage 2.3 says "no two evidence items may have the same digest".
// Does Rust check all pairs or just consecutive? (HashSet catches all.)
#[test]
fn v1_4_duplicate_digest_non_consecutive() {
    let c = make_contract();
    let ev = vec![
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "test_result", EvidenceRole::Input, "1.0"),
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1bb", "test_result", EvidenceRole::Witness, "1.0"),
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "test_result", EvidenceRole::Witness, "1.0"), // same as first
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::DuplicateDigest);
}

// V1.5: Spec §2.3 validity condition 7 says "canon_version must match an accepted TSCP-CANON-001 version".
// Rust only checks non-empty. Does Rust accept ANY non-empty canon_version?
// This is a FALSE POSITIVE — spec says "accepted version", Rust says "non-empty".
#[test]
fn v1_5_canon_version_acceptance_not_checked() {
    // Spec says canon_version must match an ACCEPTED TSCP-CANON-001 version.
    // Rust's validate_contract only checks is_empty().
    // So "garbage_version" passes contract validation.
    let result = Contract::new(
        "test-c".to_string(), "1.0".to_string(),
        vec!["x".to_string()], vec![EvidenceRole::Input],
        1, 5, vec![EvidenceRole::Input], "garbage_non_canon_version".to_string(),
    );
    // Rust accepts this — but spec says it should check against accepted versions
    assert!(result.is_ok(), "Rust accepts non-empty canon_version — spec requires accepted version check");
    // This is a DIVERGENCE but may be a spec gap (what are "accepted versions"?)
}

// V1.6: Spec §3.2 says "Rejection at any stage halts processing — later stages do not execute."
// Does Rust actually halt at each stage, or does it continue?
// Looking at the code: within a stage, all items are checked and rejections collected.
// Between stages: if rejections exist, return early. This matches spec.
// But: spec says "later stages do not execute" — does Rust check evidence structure
// for ALL evidence items before moving to binding? Yes — it collects all validation
// rejections, then returns if any exist.
// PASS — this matches spec.

// V1.7: Spec §3.4 says "rejection reasons are collected, not first-fail."
// Within each stage, does Rust collect ALL rejections or just the first?
#[test]
fn v1_7_collects_all_type_rejections() {
    let c = make_contract();
    let ev = vec![
        make_ev_type(EvidenceRole::Input, "wrong1", "11"),
        make_ev_type(EvidenceRole::Witness, "wrong2", "22"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    // Both should be rejected for type, not just the first
    assert_eq!(r.len(), 2, "Should collect all type rejections, not first-fail");
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::TypeNotAdmissible);
    assert_eq!(r[1].error_code, AdmissibilityErrorCode::TypeNotAdmissible);
}

// ============================================================
// VECTOR 2: FALSE NEGATIVE — Spec accepts, Rust rejects
// ============================================================

// V2.1: Does Rust reject evidence with media_type = Some("") (empty string)?
// Spec says media_type is "string | null" — empty string is a valid string.
// Rust uses Option<String>, so Some("") should be accepted.
#[test]
fn v2_1_empty_media_type_accepted() {
    let c = make_contract();
    let ev = vec![
        Evidence {
            digest: "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa".to_string(),
            artifact_type: "test_result".to_string(),
            media_type: Some("".to_string()),
            role: EvidenceRole::Input,
            canon_version: "1.0".to_string(),
        },
        make_ev(EvidenceRole::Witness, "bb"),
    ];
    assert!(admit(&c, &ev).is_ok(), "Empty media_type string should be accepted");
}

// V2.2: Does Rust reject evidence with leading zeros in digest?
// "0000...0000" is valid 64-char lowercase hex. Spec says "64-char lowercase hex".
#[test]
fn v2_2_all_zeros_digest_accepted() {
    let c = make_contract();
    let ev = vec![
        make_ev_raw("0000000000000000000000000000000000000000000000000000000000000001", "test_result", EvidenceRole::Input, "1.0"),
        make_ev_raw("0000000000000000000000000000000000000000000000000000000000000002", "test_result", EvidenceRole::Witness, "1.0"),
    ];
    assert!(admit(&c, &ev).is_ok(), "All-zeros digest with valid hex should be accepted");
}

// V2.3: Contract with min == max (exactly N evidence items required)
#[test]
fn v2_3_min_equals_max() {
    let c = make_contract_custom(
        vec!["t".into()], vec![EvidenceRole::Input, EvidenceRole::Witness],
        3, 3, vec![EvidenceRole::Witness], "1.0",
    );
    let exactly_right = vec![
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "t", EvidenceRole::Input, "1.0"),
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1bb", "t", EvidenceRole::Witness, "1.0"),
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1cc", "t", EvidenceRole::Witness, "1.0"),
    ];
    assert!(admit(&c, &exactly_right).is_ok(), "Exactly min==max evidence should be accepted");

    let too_few = vec![
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "t", EvidenceRole::Input, "1.0"),
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1bb", "t", EvidenceRole::Witness, "1.0"),
    ];
    assert!(admit(&c, &too_few).is_err(), "Fewer than min==max should be rejected");
}

// V2.4: Contract with required_roles = [] (empty — no required roles)
// Spec doesn't say required_roles must be non-empty. Does Rust accept it?
#[test]
fn v2_4_empty_required_roles() {
    let c = make_contract_custom(
        vec!["t".into()], vec![EvidenceRole::Input, EvidenceRole::Witness],
        1, 5, vec![], "1.0",
    );
    let ev = vec![make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "t", EvidenceRole::Input, "1.0")];
    assert!(admit(&c, &ev).is_ok(), "Empty required_roles should be valid — no role requirements");
}

// V2.5: Does Rust accept evidence where all roles are the same (if no required_roles)?
#[test]
fn v2_5_all_same_roles_no_required() {
    let c = make_contract_custom(
        vec!["t".into()], vec![EvidenceRole::Input, EvidenceRole::Witness],
        2, 5, vec![], "1.0",
    );
    let ev = vec![
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "t", EvidenceRole::Input, "1.0"),
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1bb", "t", EvidenceRole::Input, "1.0"),
    ];
    assert!(admit(&c, &ev).is_ok(), "All same roles with no required_roles should be accepted");
}

// V2.6: Does Rust over-reject on role checking? Spec says each evidence's role
// must be in contract.evidence_roles. Does Rust check this correctly?
#[test]
fn v2_6_role_not_in_contract_rejected() {
    let c = make_contract(); // roles: Input, Attestation, Witness
    let ev = vec![
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "test_result", EvidenceRole::Output, "1.0"), // Output not in contract
        make_ev(EvidenceRole::Witness, "bb"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::RoleNotAdmissible);
}

// ============================================================
// VECTOR 3: COMPONENT PREDICATE DIVERGENCE
// ============================================================

// V3.1: Validation stage — does Rust check ALL evidence items for structure,
// or does it stop at the first failure?
#[test]
fn v3_1_validation_collects_all_failures() {
    let c = make_contract();
    let ev = vec![
        make_ev_raw("short", "test_result", EvidenceRole::Input, "1.0"), // bad digest
        make_ev_raw("", "test_result", EvidenceRole::Witness, "1.0"),   // empty digest
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1cc", "", EvidenceRole::Witness, "1.0"), // empty type
    ];
    let r = admit(&c, &ev).unwrap_err();
    // All three should have validation rejections
    assert!(r.len() >= 3, "Validation should collect all structural failures, not first-fail");
    for rej in &r {
        assert_eq!(rej.error_stage, AdmissibilityStage::Validation);
    }
}

// V3.2: Does the canon-version check fire for ALL mismatched evidence,
// or only the first?
#[test]
fn v3_2_canon_version_all_mismatches_collected() {
    let c = make_contract(); // canon_version = "1.0"
    let ev = vec![
        make_ev_canon(EvidenceRole::Input, "2.0", "aa"),
        make_ev_canon(EvidenceRole::Witness, "2.0", "bb"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r.len(), 2, "Both evidence items should get canon-version mismatch rejections");
    for rej in &r {
        assert_eq!(rej.error_code, AdmissibilityErrorCode::CanonVersionMismatch);
    }
}

// V3.3: Does Rust check structure AND canon_version in the same validation pass?
// If evidence has both bad digest AND wrong canon_version, does it report both?
#[test]
fn v3_3_multiple_validation_failures_same_item() {
    let c = make_contract();
    let ev = vec![
        Evidence {
            digest: "short".to_string(),          // bad structure
            artifact_type: "test_result".to_string(),
            media_type: None,
            role: EvidenceRole::Input,
            canon_version: "2.0".to_string(),       // also wrong canon
        },
        make_ev(EvidenceRole::Witness, "bb"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    // Should get at least 2 rejections for the first item: bad structure + canon mismatch
    let first_item_rejections: Vec<_> = r.iter().filter(|rej| rej.evidence.as_ref().map_or(false, |e| e.digest == "short")).collect();
    assert!(first_item_rejections.len() >= 2, "Same item should get both structural AND canon-version rejections");
}

// V3.4: Stage ordering — spec says VALIDATION → BINDING → COMPLETENESS.
// If evidence passes validation but fails binding, does it get binding error (not validation)?
#[test]
fn v3_4_stage_ordering_validation_passes_binding_fails() {
    let c = make_contract();
    let ev = vec![
        make_ev_type(EvidenceRole::Input, "wrong_type", "aa"),  // valid structure, wrong type
        make_ev(EvidenceRole::Witness, "bb"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::TypeNotAdmissible);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Binding);
}

// V3.5: Stage ordering — if evidence passes validation and binding but fails completeness
#[test]
fn v3_5_stage_ordering_validation_binding_pass_completeness_fails() {
    let c = make_contract(); // min=2, required: Witness
    let ev = vec![make_ev(EvidenceRole::Witness, "aa")]; // only 1 item, min=2
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
    assert_eq!(r[0].error_stage, AdmissibilityStage::Completeness);
}

// V3.6: Does Rust's completeness stage check min BEFORE required_roles?
// Spec lists: 1. min count, 2. max count, 3. required roles.
// If evidence is both insufficient AND missing required role, which fires?
#[test]
fn v3_6_completeness_order_min_before_required() {
    let c = make_contract(); // min=2, required: Witness
    let ev = vec![make_ev(EvidenceRole::Input, "aa")]; // 1 item (< min=2), no Witness
    let r = admit(&c, &ev).unwrap_err();
    // Spec lists min count first — should get InsufficientEvidence, not MissingRequiredRole
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
}

// ============================================================
// VECTOR 4: BOUNDARY/DOMAIN EDGE CASES
// ============================================================

// V4.1: Empty evidence slice — must not panic (FIX 1 regression)
#[test]
fn v4_1_empty_evidence_no_panic() {
    let c = make_contract();
    let ev: Vec<Evidence> = vec![];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
    assert!(r[0].evidence.is_none(), "Empty evidence should produce evidence: None");
}

// V4.2: Exactly max_evidence_count items
#[test]
fn v4_2_exactly_max() {
    let c = make_contract(); // max=5
    let ev: Vec<Evidence> = (0..5).map(|i| {
        let suffix = format!("{:02x}", i);
        if i == 0 || i == 1 {
            make_ev(EvidenceRole::Witness, &suffix)
        } else {
            make_ev(EvidenceRole::Input, &suffix)
        }
    }).collect();
    assert!(admit(&c, &ev).is_ok(), "Exactly max_evidence_count should be accepted");
}

// V4.3: One over max
#[test]
fn v4_3_one_over_max() {
    let c = make_contract(); // max=5
    let ev: Vec<Evidence> = (0..6).map(|i| {
        let suffix = format!("{:02x}", i);
        make_ev(EvidenceRole::Witness, &suffix)
    }).collect();
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::ExcessEvidence);
}

// V4.4: Contract with min=1, max=1 (single evidence item)
#[test]
fn v4_4_single_evidence_contract() {
    let c = make_contract_custom(
        vec!["t".into()], vec![EvidenceRole::Input],
        1, 1, vec![], "1.0",
    );
    let ev = vec![make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "t", EvidenceRole::Input, "1.0")];
    assert!(admit(&c, &ev).is_ok());
}

// V4.5: Duplicate digests with different artifact_types and roles
#[test]
fn v4_5_duplicate_digest_different_types_roles() {
    let c = make_contract_custom(
        vec!["t1".into(), "t2".into()],
        vec![EvidenceRole::Input, EvidenceRole::Witness],
        2, 5, vec![EvidenceRole::Witness], "1.0",
    );
    let ev = vec![
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "t1", EvidenceRole::Input, "1.0"),
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "t2", EvidenceRole::Witness, "1.0"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::DuplicateDigest);
}

// V4.6: Same role, different digests (should be valid)
#[test]
fn v4_6_same_role_different_digests() {
    let c = make_contract_custom(
        vec!["t".into()], vec![EvidenceRole::Witness],
        2, 5, vec![EvidenceRole::Witness], "1.0",
    );
    let ev = vec![
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1aa", "t", EvidenceRole::Witness, "1.0"),
        make_ev_raw("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1bb", "t", EvidenceRole::Witness, "1.0"),
    ];
    assert!(admit(&c, &ev).is_ok(), "Same role with different digests should be valid");
}

// V4.7: Mixed canon versions in evidence (some match, some don't)
#[test]
fn v4_7_mixed_canon_versions() {
    let c = make_contract(); // canon = "1.0"
    let ev = vec![
        make_ev_canon(EvidenceRole::Input, "1.0", "aa"),  // matches
        make_ev_canon(EvidenceRole::Witness, "2.0", "bb"), // doesn't match
    ];
    let r = admit(&c, &ev).unwrap_err();
    // Should reject the mismatched one, not the matching one
    assert_eq!(r.len(), 1, "Only the mismatched evidence should be rejected");
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::CanonVersionMismatch);
}

// ============================================================
// VECTOR 5: REPAIRED-SEAM REGRESSION STRESS
// ============================================================

// V5.1: Empty evidence with multiple different contracts
#[test]
fn v5_1_empty_evidence_multiple_contracts() {
    let c1 = make_contract();
    let c2 = make_contract_custom(vec!["x".into()], vec![EvidenceRole::Input], 1, 3, vec![], "1.0");
    let c3 = make_contract_custom(vec!["y".into()], vec![EvidenceRole::Witness], 3, 10, vec![EvidenceRole::Witness], "2.0");

    for c in [&c1, &c2, &c3] {
        let r = admit(c, &[]).unwrap_err();
        assert_eq!(r[0].error_code, AdmissibilityErrorCode::InsufficientEvidence);
        assert!(r[0].evidence.is_none());
    }
}

// V5.2: Canon-version mismatch fires even when only some evidence mismatches
#[test]
fn v5_2_partial_canon_mismatch() {
    let c = make_contract(); // canon = "1.0"
    let ev = vec![
        make_ev_canon(EvidenceRole::Input, "1.0", "aa"),   // matches
        make_ev_canon(EvidenceRole::Witness, "1.0", "bb"),  // matches
        make_ev_canon(EvidenceRole::Witness, "3.0", "cc"),  // doesn't match
    ];
    let r = admit(&c, &ev).unwrap_err();
    assert_eq!(r.len(), 1, "Only the mismatched item should be rejected");
    assert_eq!(r[0].error_code, AdmissibilityErrorCode::CanonVersionMismatch);
}

// V5.3: Contract immutability — verify accessors return correct values
#[test]
fn v5_3_contract_accessor_correctness() {
    let c = make_contract();
    assert_eq!(c.id(), "test-c");
    assert_eq!(c.version(), "1.0");
    assert_eq!(c.min_evidence_count(), 2);
    assert_eq!(c.max_evidence_count(), 5);
    assert_eq!(c.canon_version(), "1.0");
    assert_eq!(c.evidence_types().len(), 2);
    assert_eq!(c.evidence_roles().len(), 3);
    assert_eq!(c.required_roles().len(), 1);
}

// V5.4: Contract::new rejects invalid contracts at construction
#[test]
fn v5_4_contract_new_rejects_invalid() {
    // min > max
    let r = Contract::new("c".into(), "1.0".into(), vec!["t".into()], vec![EvidenceRole::Input], 5, 2, vec![], "1.0".into());
    assert!(r.is_err());

    // required role not in evidence_roles
    let r = Contract::new("c".into(), "1.0".into(), vec!["t".into()], vec![EvidenceRole::Input], 1, 5, vec![EvidenceRole::Witness], "1.0".into());
    assert!(r.is_err());

    // empty id
    let r = Contract::new("".into(), "1.0".into(), vec!["t".into()], vec![EvidenceRole::Input], 1, 5, vec![], "1.0".into());
    assert!(r.is_err());

    // empty canon_version
    let r = Contract::new("c".into(), "1.0".into(), vec!["t".into()], vec![EvidenceRole::Input], 1, 5, vec![], "".into());
    assert!(r.is_err());
}

// V5.5: Determinism — same input always produces same output
#[test]
fn v5_5_determinism_stress() {
    let c = make_contract();
    let ev = vec![
        make_ev(EvidenceRole::Input, "aa"),
        make_ev(EvidenceRole::Witness, "bb"),
    ];
    let r1 = admit(&c, &ev).unwrap();
    let r2 = admit(&c, &ev).unwrap();
    let r3 = admit(&c, &ev).unwrap();
    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
    assert_eq!(r1.admission_digest(), r2.admission_digest());
}

// V5.6: Admission digest changes when evidence changes
#[test]
fn v5_6_admission_digest_evidence_dependent() {
    let c = make_contract();
    let ev1 = vec![make_ev(EvidenceRole::Input, "aa"), make_ev(EvidenceRole::Witness, "bb")];
    let ev2 = vec![make_ev(EvidenceRole::Input, "cc"), make_ev(EvidenceRole::Witness, "dd")];
    let r1 = admit(&c, &ev1).unwrap();
    let r2 = admit(&c, &ev2).unwrap();
    assert_ne!(r1.admission_digest(), r2.admission_digest(), "Different evidence should produce different admission digests");
}

// ============================================================
// VECTOR 6: SPECIFICATION GAPS (recorded, not resolved in Rust)
// ============================================================

// V6.1: Spec §2.5 defines RejectedEvidence.evidence as Evidence (non-optional).
// Rust uses Option<Evidence> for the empty-domain case.
// This is a TYPE DIVERGENCE — not a behavioral one, but the Rust type
// doesn't match the spec type.
// Test: verify that the type divergence is only exercised for empty evidence.
#[test]
fn v6_1_rejected_evidence_type_divergence() {
    let c = make_contract();
    // Non-empty evidence rejection — evidence should be Some
    let r1 = admit(&c, &[make_ev_type(EvidenceRole::Input, "wrong", "aa")]).unwrap_err();
    assert!(r1[0].evidence.is_some(), "Non-empty rejection should have Some(evidence)");

    // Empty evidence rejection — evidence must be None (no evidence to reference)
    let r2 = admit(&c, &[]).unwrap_err();
    assert!(r2[0].evidence.is_none(), "Empty rejection should have None(evidence) — type divergence from spec");
}

// V6.2: Spec §2.2 defines Evidence without canon_version field.
// Spec §3.2 Stage 1.3 says "Each Evidence item's implicit canon version
// (established upstream during canonicalization) must match..."
// Rust adds an explicit canon_version field to Evidence.
// This is a representation choice — the spec says "implicit", Rust makes it explicit.
// DIVERGENCE: Spec Evidence type has 4 fields, Rust Evidence has 5.
#[test]
fn v6_2_evidence_canon_version_representation() {
    let c = make_contract();
    // Evidence with canon_version field exists and is checked
    let ev = vec![
        make_ev_canon(EvidenceRole::Input, "1.0", "aa"),
        make_ev_canon(EvidenceRole::Witness, "1.0", "bb"),
    ];
    assert!(admit(&c, &ev).is_ok());

    // The spec says "implicit canon version (established upstream during canonicalization)"
    // Rust makes it explicit. This means the Rust Evidence type has a field the spec
    // doesn't define. Is this a divergence or a legitimate representation choice?
    // Classification: REPRESENTATION DIVERGENCE — spec says implicit, Rust says explicit.
}

// V6.3: Duplicate admission — spec defines error code but no mechanism
#[test]
fn v6_3_duplicate_admission_not_implemented() {
    // The spec defines TSCP-ADMIT-DUPLICATE-ADMISSION but Rust doesn't implement it.
    // This is correct — the spec's §3 execution stages don't define the mechanism.
    // The Rust AdmissibilityErrorCode enum doesn't include DuplicateAdmission.
    // This is a SPECIFICATION GAP, not an implementation bug.

    // We can verify the error code doesn't exist by checking that no test
    // produces it. Since it's not in the enum, it can't be produced.
    let c = make_contract();
    let ev = vec![make_ev(EvidenceRole::Input, "aa"), make_ev(EvidenceRole::Witness, "bb")];

    // First admission succeeds
    let r1 = admit(&c, &ev);
    assert!(r1.is_ok());

    // Second admission with same evidence also succeeds — no duplicate detection
    let r2 = admit(&c, &ev);
    assert!(r2.is_ok(), "Duplicate admission not detected — spec gap, not implementation bug");
}

// V6.4: Spec §3.4 says "admitted_at field is informational only and must not
// affect the admission decision." Rust uses a hardcoded timestamp.
// Does this break determinism? No — same hardcoded value every time.
// But: it's not RFC 3339 UTC from a clock. It's a static string.
// DIVERGENCE: Spec says RFC 3339 UTC timestamp; Rust uses hardcoded "2026-08-19T06:00:00Z".
#[test]
fn v6_4_admitted_at_static() {
    let c = make_contract();
    let ev = vec![make_ev(EvidenceRole::Input, "aa"), make_ev(EvidenceRole::Witness, "bb")];
    let r = admit(&c, &ev).unwrap();
    // The admitted_at is hardcoded, not dynamic
    assert_eq!(r.admitted_at(), "2026-08-19T06:00:00Z");
    // This is fine for determinism (spec says it must not affect decision)
    // but it's not a real RFC 3339 timestamp from a clock
}

// V6.5: Spec §2.3 condition 7: "canon_version must match an accepted TSCP-CANON-001 version"
// Rust only checks non-empty. This is a SPECIFICATION GAP — what are "accepted versions"?
#[test]
fn v6_5_canon_version_acceptance_list_undefined() {
    // The spec says canon_version must match an "accepted" TSCP-CANON-001 version
    // but doesn't define what "accepted" means or provide a version list.
    // Rust checks non-empty, which is weaker but the spec doesn't give enough
    // information to implement the full check.
    // CLASSIFICATION: SPECIFICATION GAP — accepted version list undefined
    let c = make_contract_custom(
        vec!["t".into()], vec![EvidenceRole::Input],
        1, 1, vec![], "any_arbitrary_version_string".into(),
    );
    assert!(c.id() == "test-c"); // contract accepted despite arbitrary canon_version
}

// V6.6: Spec §3.2 Stage 1.2 says "valid role" — but what defines valid?
// Rust uses a closed enum (EvidenceRole with 4 variants). The spec says
// "input" | "output" | "attestation" | " witness". Rust matches exactly.
// PASS — no divergence.
#[test]
fn v6_6_role_definition_matches() {
    // Rust's EvidenceRole has exactly 4 variants matching the spec
    let roles = [EvidenceRole::Input, EvidenceRole::Output, EvidenceRole::Attestation, EvidenceRole::Witness];
    let c = make_contract_custom(
        vec!["t".into()], roles.to_vec(),
        1, 4, vec![], "1.0",
    );
    let ev: Vec<Evidence> = roles.iter().enumerate().map(|(i, r)| {
        make_ev_raw(&format!("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1{:02x}", i), "t", r.clone(), "1.0")
    }).collect();
    assert!(admit(&c, &ev).is_ok(), "All 4 spec-defined roles should be accepted");
}

// V6.7: Spec §3.2 says "Rejection at any stage halts processing — later stages do not execute"
// BUT spec also says "rejection reasons are collected, not first-fail" (§3.4).
// These could conflict: if we collect ALL rejections within a stage, do we "halt" at that stage?
// Rust's interpretation: collect all within a stage, halt between stages.
// This seems correct but the spec could be clearer.
// CLASSIFICATION: SPEC AMBIGUITY — "halt" vs "collect" within stages
#[test]
fn v6_7_halt_vs_collect_interpretation() {
    let c = make_contract();
    // Evidence that fails BOTH validation (bad structure) AND would fail binding (wrong type)
    let ev = vec![
        make_ev_raw("short", "wrong_type", EvidenceRole::Input, "1.0"),
        make_ev(EvidenceRole::Witness, "bb"),
    ];
    let r = admit(&c, &ev).unwrap_err();
    // Should get validation rejection (bad structure) but NOT binding rejection (wrong type)
    // because validation halts before binding runs
    for rej in &r {
        assert_eq!(rej.error_stage, AdmissibilityStage::Validation,
            "Should halt at validation, not proceed to binding");
    }
}
