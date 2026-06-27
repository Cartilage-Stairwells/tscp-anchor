use std::collections::VecDeque;
use std::time::Instant;

pub mod server;

#[derive(Clone, Debug)]
pub struct SensorPayload {
    pub timestamp: u64,
    pub sequence_id: u64,
    pub data_hash: [u8; 32],
}

pub struct EdiaAgent {
    pub max_capacity: usize,
    pub ring_buffer: VecDeque<SensorPayload>,
    pub current_sequence: u64,
    pub is_blocked: bool,
    pub drain_rate_tps: usize,
}

impl EdiaAgent {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            max_capacity,
            ring_buffer: VecDeque::with_capacity(max_capacity),
            current_sequence: 0,
            is_blocked: false,
            drain_rate_tps: 150,
        }
    }

    pub fn ingest_telemetry(&mut self) -> Result<(), &'static str> {
        if self.ring_buffer.len() >= self.max_capacity {
            self.is_blocked = true;
            return Err("HARD_CAP_EXCEEDED: Ingestion blocked via backpressure state.");
        }
        let payload = SensorPayload {
            timestamp: Instant::now().elapsed().as_secs(),
            sequence_id: self.current_sequence,
            data_hash: [0x5A; 32],
        };
        self.ring_buffer.push_back(payload);
        self.current_sequence += 1;
        Ok(())
    }

    pub fn adjust_drain_rate(&mut self, delta_ms: u64) {
        // Phase 4 hook - simple backpressure adjustment
        if delta_ms > 2000 {
            self.drain_rate_tps = self.drain_rate_tps.saturating_sub(10);
        } else {
            self.drain_rate_tps = (self.drain_rate_tps + 1).min(self.max_capacity);
        }
        self.is_blocked = false;
    }

    pub fn drain_batch(&mut self) -> Vec<SensorPayload> {
        let mut batch = Vec::new();
        while let Some(p) = self.ring_buffer.pop_front() {
            batch.push(p);
            if batch.len() >= self.drain_rate_tps { break; }
        }
        batch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_dead_code_warnings() {
        let mut agent = EdiaAgent::new(100);
        assert!(agent.ingest_telemetry().is_ok());
        agent.adjust_drain_rate(1500);
        let _ = agent.drain_batch();
        if let Some(payload) = agent.ring_buffer.front() {
            println!(
                "Verifying Payload -> ID: {}, TS: {}, Hash0: {:x}, Blocked: {}",
                payload.sequence_id, payload.timestamp, payload.data_hash[0], agent.is_blocked
            );
        }
        assert_eq!(agent.max_capacity, 100);
        assert_eq!(agent.drain_rate_tps, 151);
    }
}
