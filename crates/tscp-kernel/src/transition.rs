use crate::types::*;
use crate::state::State;
use crate::event::EventEnvelope;

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub ruleset_version: RulesetVersion,
    pub logical_time: u64,
}

pub fn dispatch_event(
    state: &State,
    state_hash: StateHash,
    event: &EventEnvelope,
    _context: &ExecutionContext,
) -> Result<(State, StateHash), TransitionError> {
    // Verify parent linkage
    if event.parent_state_hash!= state_hash {
        return Err(TransitionError::InvalidParent);
    }

    // Simple deterministic transition: increment counter, hash new state
    let mut new_state = state.clone();
    new_state.counter += 1;

    let new_hash = {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(&state_hash);
        hasher.update(&event.hash());
        hasher.update(&new_state.counter.to_le_bytes());
        let mut out = [0u8; 32];
        out.copy_from_slice(hasher.finalize().as_bytes());
        out
    };

    Ok((new_state, new_hash))
}
