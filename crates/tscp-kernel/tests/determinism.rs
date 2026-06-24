use tscp_kernel::types::*;
use tscp_kernel::event::EventEnvelope;
use tscp_kernel::replay::ReplayEngine;

fn create_test_event(parent: StateHash, id: u8, payload: u8, time: u64) -> EventEnvelope {
    EventEnvelope {
        event_id: [id; 16],
        parent_state_hash: parent,
        payload_hash: [payload; 32],
        logical_time: time,
    }
}

fn build_receipt(parent: StateHash, event: EventEnvelope, kernel_version: KernelVersion) -> TransitionReceipt {
    TransitionReceipt {
        parent_state_hash: parent,
        event_hash: event.payload_hash,
        child_state_hash: [0u8; 32],
        kernel_version,
    }
}

#[test]
fn test_deterministic_replay() {
    // Build a proper chain: each event's parent = previous state hash
    let mut engine = ReplayEngine::new(1);
    let e1 = create_test_event(GENESIS_STATE, 1, 2, 1);
    engine.apply(&e1).unwrap();
    let h1 = engine.current_hash();

    let e2 = create_test_event(h1, 2, 3, 2);
    engine.apply(&e2).unwrap();
    let final_hash = engine.current_hash();

    // Replay the same two events from scratch three times
    let events = vec![e1, e2];
    let hash1 = ReplayEngine::replay(&events, 1).unwrap();
    let hash2 = ReplayEngine::replay(&events, 1).unwrap();
    let hash3 = ReplayEngine::replay(&events, 1).unwrap();

    assert_eq!(hash1, final_hash);
    assert_eq!(hash1, hash2);
    assert_eq!(hash2, hash3);
}

#[test]
fn test_invalid_parent_rejected() {
    let mut engine = ReplayEngine::new(1);
    let bad_event = create_test_event([99u8; 32], 9, 5, 1);
    let result = engine.apply(&bad_event);
    assert!(matches!(result, Err(TransitionError::InvalidParent)));
}

#[test]
fn test_child_mutation_rejected() {
    let parent = [1u8; 32];
    let event = create_test_event(parent, 1, 2, 1);
    let receipt1 = build_receipt(parent, event.clone(), 1);
    let mut receipt2 = receipt1.clone();
    receipt2.child_state_hash = [99u8; 32];
    assert_ne!(receipt1.hash(), receipt2.hash());
}

#[test]
fn test_domain_separation() {
    let parent = [42u8; 32];
    let event = create_test_event(parent, 1, 2, 1);
    let receipt = build_receipt(parent, event.clone(), 1);
    let receipt_hash = receipt.hash();
    assert_ne!(receipt_hash, parent);
    assert_ne!(receipt_hash, event.payload_hash);
}
