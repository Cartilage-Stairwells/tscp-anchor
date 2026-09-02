use crate::event::EventEnvelope;
use crate::state::State;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub ruleset_version: RulesetVersion,
    pub logical_time: u64,
}

pub fn dispatch_event(
    state: &State,
    state_hash: StateHash,
    event: &EventEnvelope,
    context: &ExecutionContext,
) -> Result<(State, TransitionReceipt), TransitionError> {
    if event.parent_state_hash != state_hash {
        return Err(TransitionError::InvalidParent);
    }

    let mut new_state = state.clone();
    new_state.counter += 1;

    let child_hash = {
        use blake3::Hasher;
        let mut h = Hasher::new();
        h.update(&state_hash);
        h.update(&event.hash());
        h.update(&new_state.counter.to_le_bytes());
        let mut out = [0u8; 32];
        out.copy_from_slice(h.finalize().as_bytes());
        out
    };

    // TODO: This is a placeholder. In the production kernel, the
    // transition kind should be determined by the event's type/content,
    // not by counter parity. Currently any even-numbered event is
    // marked ClaimVerified regardless of whether verification actually
    // occurred. This is acceptable for the P0 scaffold but must be
    // replaced with event-type dispatch before production use.
    let kind = if new_state.counter % 2 == 1 {
        TransitionKind::ClaimCreated
    } else {
        TransitionKind::ClaimVerified
    };

    let receipt = TransitionReceipt {
        parent_state_hash: state_hash,
        event_hash: event.hash(),
        child_state_hash: child_hash,
        kernel_version: context.ruleset_version,
        kind,
    };

    Ok((new_state, receipt))
}
