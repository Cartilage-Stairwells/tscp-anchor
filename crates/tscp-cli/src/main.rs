use tscp_kernel::{replay::ReplayEngine, event::EventEnvelope};
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { eprintln!("usage: tscp-cli events.cbor"); return; }
    let bytes = fs::read(&args[1]).expect("read");
    let events: Vec<EventEnvelope> = serde_cbor::from_slice(&bytes).expect("cbor");
    let receipts = ReplayEngine::replay(&events, 1).expect("replay");
    for (i, r) in receipts.iter().enumerate() {
        println!("{}: {} -> {}", i, hex::encode(r.parent_state_hash), hex::encode(r.child_state_hash));
        println!(" receipt: {}", hex::encode(r.hash()));
    }
}
