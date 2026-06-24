use crate::types::*;
use crate::state::State;
use crate::event::EventEnvelope;
use crate::transition::{dispatch_event, ExecutionContext};

pub struct ReplayEngine {
    state: State,
    state_hash: StateHash,
    context: ExecutionContext,
    receipts: Vec<TransitionReceipt>,
}

impl ReplayEngine {
    pub fn new(ruleset_version: RulesetVersion) -> Self {
        Self {
            state: State::default(),
            state_hash: GENESIS_STATE,
            context: ExecutionContext { ruleset_version, logical_time: 0 },
            receipts: Vec::new(),
        }
    }

    pub fn apply(&mut self, event: &EventEnvelope) -> Result<&TransitionReceipt, TransitionError> {
        let (new_state, receipt) = dispatch_event(&self.state, self.state_hash, event, &self.context)?;
        self.state = new_state;
        self.state_hash = receipt.child_state_hash;
        self.context.logical_time += 1;
        self.receipts.push(receipt);
        Ok(self.receipts.last().unwrap())
    }

    pub fn replay(events: &[EventEnvelope], ruleset_version: RulesetVersion) -> Result<Vec<TransitionReceipt>, TransitionError> {
        let mut engine = Self::new(ruleset_version);
        for ev in events {
            engine.apply(ev)?;
        }
        Ok(engine.receipts)
    }

    pub fn current_hash(&self) -> StateHash { self.state_hash }
    pub fn receipts(&self) -> &[TransitionReceipt] { &self.receipts }
}
