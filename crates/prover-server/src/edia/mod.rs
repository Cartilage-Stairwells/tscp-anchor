pub mod server;

use std::collections::VecDeque;
use std::time::Instant;

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

    pub fn adjust_drain_rate(&mut self, settlement_delta_ms: u64) {
        if settlement_delta_ms <= 1500 {
            self.drain_rate_tps = std::cmp::min(self.drain_rate_tps + 25, 350);
        } else if settlement_delta_ms > 2000 {
            self.drain_rate_tps = std::cmp::max(self.drain_rate_tps.saturating_sub(50), 25);
        }
    }

    pub fn drain_batch(&mut self) -> Vec<SensorPayload> {
        let batch_size = std::cmp::min(self.ring_buffer.len(), self.drain_rate_tps);
        let mut batch = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            if let Some(p) = self.ring_buffer.pop_front() {
                batch.push(p);
            }
        }
        if self.ring_buffer.len() < self.max_capacity {
            self.is_blocked = false;
        }
        batch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edia_field_coverage() {
        let mut agent = EdiaAgent::new(100);
        assert!(agent.ingest_telemetry().is_ok());
        if let Some(payload) = agent.ring_buffer.front() {
            assert_eq!(payload.data_hash[0], 0x5A);
            let _ = payload.sequence_id;
            let _ = payload.timestamp;
            let _ = agent.is_blocked;
        }
        assert_eq!(agent.max_capacity, 100);
        assert_eq!(agent.current_sequence, 1);
    }

    #[test]
    fn test_backpressure_blocks_at_capacity() {
        let mut agent = EdiaAgent::new(2);
        assert!(agent.ingest_telemetry().is_ok());
        assert!(agent.ingest_telemetry().is_ok());
        assert!(agent.ingest_telemetry().is_err());
        assert!(agent.is_blocked);
    }

    #[test]
    fn test_drain_rate_adapts() {
        let mut agent = EdiaAgent::new(1000);
        agent.adjust_drain_rate(1000);
        assert_eq!(agent.drain_rate_tps, 175);
        agent.adjust_drain_rate(2500);
        assert_eq!(agent.drain_rate_tps, 125);
    }
}
