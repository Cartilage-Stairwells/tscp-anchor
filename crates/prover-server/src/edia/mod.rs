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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_dead_code_warnings() {
        let mut agent = EdiaAgent::new(100);
        assert!(agent.ingest_telemetry().is_ok());
        if let Some(payload) = agent.ring_buffer.front() {
            println!(
                "Verifying Payload -> ID: {}, TS: {}, Hash0: {:x}, Blocked: {}",
                payload.sequence_id, payload.timestamp, payload.data_hash[0], agent.is_blocked
            );
            assert_eq!(payload.data_hash[0], 0x5A);
        }
        assert_eq!(agent.max_capacity, 100);
        assert_eq!(agent.current_sequence, 1);
        assert_eq!(agent.drain_rate_tps, 150);
        assert!(!agent.is_blocked);
    }
}
