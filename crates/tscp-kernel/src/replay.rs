use crate::types::*;
use crate::state::State;
use crate::event::EventEnvelope;
use crate::transition::{dispatch_event, ExecutionContext};

pub struct ReplayEngine {
    state: State,
    state_hash: StateHash,
    context: ExecutionContext,
}

impl ReplayEngine {
    pub fn new(ruleset_version: RulesetVersion) -> Self {
        Self {
            state: State::default(),
            state_hash: GENESIS_STATE,
            context: ExecutionContext {
                ruleset_version,
                logical_time: 0,
            },
        }
    }
    pub fn apply(&mut self, event: &EventEnvelope) -> Result<(), TransitionError> {
        let (new_state, new_hash) = dispatch_event(&self.state, self.state_hash, event, &self.context)?;
        self.state = new_state;
        self.state_hash = new_hash;
        self.context.logical_time += 1;
        Ok(())
    }
    pub fn replay(events: &[EventEnvelope], ruleset_version: RulesetVersion) -> Result<StateHash, TransitionError> {
        let mut engine = Self::new(ruleset_version);
        for event in events {
            engine.apply(event)?;
        }
        Ok(engine.state_hash)
    }
    pub fn current_state(&self) -> &State { &self.state }
    pub fn current_hash(&self) -> StateHash { self.state_hash }
}
