use tscp_kernel::{
    event::{EventEnvelope, ClaimCreatedPayload, ClaimVerifiedPayload},
    replay::ReplayEngine,
    types::{ActorId, RulesetVersion, TransitionId, GENESIS_STATE},
    serialization,
};

#[test]
fn test_deterministic_replay() {
    let ruleset = RulesetVersion(1);

    let claim_id = "claim_123".to_string();
    let content_hash = [1u8; 32];

    let created_payload = ClaimCreatedPayload {
        claim_id: claim_id.clone(),
        content_hash,
    };
    let created_bytes = serialization::to_canonical_cbor(&created_payload).unwrap();

    let event1 = EventEnvelope::new(
        GENESIS_STATE,
        ActorId("alice".to_string()),
        1,
        TransitionId::ClaimCreated,
        created_bytes,
    );

    let state_after_e1 = {
        let mut engine = ReplayEngine::new(ruleset);
        engine.apply(&event1).unwrap();
        engine.current_hash()
    };

    let verified_payload = ClaimVerifiedPayload {
        claim_id: claim_id.clone(),
        evidence_hash: [2u8; 32],
        score: 95,
    };
    let verified_bytes = serialization::to_canonical_cbor(&verified_payload).unwrap();

    let event2 = EventEnvelope::new(
        state_after_e1,
        ActorId("admin_7".to_string()),
        2,
        TransitionId::ClaimVerified,
        verified_bytes,
    );

    // Three independent verifiers
    let mut v1 = ReplayEngine::new(ruleset);
    v1.apply(&event1).unwrap();
    v1.apply(&event2).unwrap();
    let h1 = v1.current_hash();

    let h2 = ReplayEngine::replay(&[event1.clone(), event2.clone()], ruleset).unwrap();

    let mut v3 = ReplayEngine::new(ruleset);
    v3.apply(&event1).unwrap();
    v3.apply(&event2).unwrap();
    let h3 = v3.current_hash();

    assert_eq!(h1, h2);
    assert_eq!(h2, h3);

    let state = v1.current_state();
    let claim = state.claims.get("claim_123").unwrap();
    assert_eq!(claim.status, tscp_kernel::state::ClaimStatus::Verified);
    assert_eq!(claim.score, Some(95));
    assert_eq!(claim.verified_by, Some("admin_7".to_string()));
}

#[test]
fn test_invalid_parent_rejected() {
    let ruleset = RulesetVersion(1);
    let mut engine = ReplayEngine::new(ruleset);

    let payload = ClaimCreatedPayload {
        claim_id: "test".to_string(),
        content_hash: [0u8; 32],
    };
    let bytes = serialization::to_canonical_cbor(&payload).unwrap();

    let bad_event = EventEnvelope::new(
        [99u8; 32],
        ActorId("alice".to_string()),
        1,
        TransitionId::ClaimCreated,
        bytes,
    );

    let result = engine.apply(&bad_event);
    assert!(result.is_err());
}

#[test]
fn test_child_mutation_rejected() {
    // Same parent + event + kernel must produce consistent child.
    // If a broken transition produces a different child, the receipt hash must differ.
    // (Implementation note: this test will be strengthened once a concrete
    // transition function that can be deliberately broken is available.)
    let parent = [1u8; 32];
    let event = create_test_event(parent);
    let kernel_version = 1u16;

    let receipt1 = build_receipt(parent, event.clone(), kernel_version);
    // Simulate a faulty child (different state hash)
    let mut receipt2 = receipt1.clone();
    receipt2.child_state_hash = [99u8; 32];

    assert_ne!(receipt1.hash(), receipt2.hash(),
        "Child mutation must change the receipt hash");
}

#[test]
fn test_domain_separation() {
    // Receipt hash must not collide with raw state or event hashes.
    let parent = [42u8; 32];
    let event = create_test_event(parent);
    let kernel_version = 1u16;

    let receipt = build_receipt(parent, event.clone(), kernel_version);
    let receipt_hash = receipt.hash();

    // These should never be equal under the current domain separator
    assert_ne!(receipt_hash, parent,
        "Receipt hash must differ from raw parent state hash");
    assert_ne!(receipt_hash, event.payload_hash,
        "Receipt hash must differ from event payload hash");
}

// Helper (temporary until a proper builder is added to the test module)
fn build_receipt(parent: StateHash, event: EventEnvelope, kernel_version: KernelVersion) -> TransitionReceipt {
    // In a real implementation this would call the transition function.
    // For now we construct a deterministic placeholder receipt.
    TransitionReceipt {
        parent_state_hash: parent,
        event_hash: event.payload_hash, // placeholder – replace with real event hash when available
        child_state_hash: [0u8; 32],    // placeholder
        kernel_version,
    }
}
