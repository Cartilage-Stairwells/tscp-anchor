use crate::telemetry::ProverEvent;

/// Walks a list of phase events backwards to locate the first likely divergence.
pub fn reverse_trace_debug(events: &[ProverEvent]) -> Option<String> {
    for event in events.iter().rev() {
        if event.phase.contains("sumcheck") || event.phase.contains("deep") || event.phase.contains("folding") {
            return Some(format!(
                "First divergence likely in phase '{}' (request_id: {})",
                event.phase, event.request_id
            ));
        }
    }
    None
}
