use crate::types::*;
use crate::event::{EventEnvelope, ClaimCreatedPayload, ClaimVerifiedPayload};
use crate::state::State;
use crate::serialization;

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub ruleset_version: RulesetVersion,
    pub logical_time: u64,
}

pub fn dispatch_event(
    current_state: &State,
    current_hash: StateHash,
    event: &EventEnvelope,
    _context: &ExecutionContext,
) -> Result<(State, StateHash), TransitionError> {
    if event.parent_state_hash!= current_hash {
        return Err(TransitionError::InvalidParentState {
            expected: current_hash,
            actual: event.parent_state_hash,
        });
    }
    if event.protocol_version!= PROTOCOL_VERSION {
        return Err(TransitionError::UnsupportedTransition(event.transition_id));
    }
    let mut new_state = current_state.clone();
    match event.transition_id {
        TransitionId::ClaimCreated => {
            let payload: ClaimCreatedPayload = serialization::from_cbor(&event.payload)?;
            new_state.apply_claim_created(payload.claim_id, payload.content_hash);
        }
        TransitionId::ClaimVerified => {
            let payload: ClaimVerifiedPayload = serialization::from_cbor(&event.payload)?;
            new_state.apply_claim_verified(payload.claim_id, event.actor.0.clone(), payload.score);
        }
    }
    let new_hash = new_state.hash();
    Ok((new_state, new_hash))
}
