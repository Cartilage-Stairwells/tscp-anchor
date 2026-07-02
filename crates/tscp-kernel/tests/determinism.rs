use tscp_kernel::event::EventEnvelope;
use tscp_kernel::replay::ReplayEngine;
use tscp_kernel::types::*;

fn ev(parent: StateHash, id: u8, payload: u8, time: u64) -> EventEnvelope {
    EventEnvelope {
        event_id: [id; 16],
        parent_state_hash: parent,
        payload_hash: [payload; 32],
        logical_time: time,
    }
}

#[test]
fn test_deterministic_replay() {
    let mut eng = ReplayEngine::new(1);
    let e1 = ev(GENESIS_STATE, 1, 2, 1);
    eng.apply(&e1).unwrap();
    let h1 = eng.current_hash();
    let e2 = ev(h1, 2, 3, 2);
    eng.apply(&e2).unwrap();

    let events = vec![e1, e2];
    let r1 = ReplayEngine::replay(&events, 1).unwrap();
    let r2 = ReplayEngine::replay(&events, 1).unwrap();
    let r3 = ReplayEngine::replay(&events, 1).unwrap();

    assert_eq!(r1[1].child_state_hash, r2[1].child_state_hash);
    assert_eq!(r2[1].hash(), r3[1].hash());
}

#[test]
fn test_invalid_parent_rejected() {
    let mut eng = ReplayEngine::new(1);
    let bad = ev([99u8; 32], 9, 5, 1);
    assert!(matches!(
        eng.apply(&bad),
        Err(TransitionError::InvalidParent)
    ));
}

#[test]
fn test_child_mutation_rejected() {
    let r1 = TransitionReceipt {
        parent_state_hash: [1; 32],
        event_hash: [2; 32],
        child_state_hash: [0; 32],
        kernel_version: 1,
        kind: TransitionKind::ClaimCreated,
    };
    let mut r2 = r1.clone();
    r2.child_state_hash = [99; 32];
    assert_ne!(r1.hash(), r2.hash());
}

#[test]
fn test_domain_separation() {
    let r = TransitionReceipt {
        parent_state_hash: [42; 32],
        event_hash: [2; 32],
        child_state_hash: [3; 32],
        kernel_version: 1,
        kind: TransitionKind::ClaimVerified,
    };
    let h = r.hash();
    assert_ne!(h, [42; 32]);
    assert_ne!(h, [2; 32]);
}
