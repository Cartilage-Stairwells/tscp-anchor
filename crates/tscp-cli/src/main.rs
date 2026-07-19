use std::env;
use std::fs;

use tscp_kernel::{event::EventEnvelope, replay::ReplayEngine};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("produce") => produce(&args),
        Some("replay") => {
            let path = args.get(2).expect("usage: tscp-cli replay <events.cbor>");
            println!("replaying {}", path);
            let bytes = fs::read(path).expect("failed reading event file");
            let events: Vec<EventEnvelope> =
                ciborium::de::from_reader(bytes.as_slice()).expect("invalid cbor");
            let receipts = ReplayEngine::replay(&events, 1).expect("replay failed");
            println!("replay successful");
            println!("receipts: {}", receipts.len());
            println!(
                "final_state: {:?}",
                receipts.last().map(|r| r.child_state_hash)
            );
        }
        _ => {
            println!("usage:");
            println!("  tscp-cli produce --count N --seed X");
            println!("  tscp-cli replay FILE");
        }
    }
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == flag)?;
    args.get(pos + 1).cloned()
}

fn produce(args: &[String]) {
    let count: usize = arg_value(args, "--count")
        .unwrap_or("10".to_string())
        .parse()
        .unwrap();
    let seed: u8 = arg_value(args, "--seed")
        .unwrap_or("1".to_string())
        .parse()
        .unwrap();

    let mut engine = ReplayEngine::new(1);
    let mut events = Vec::new();

    for i in 0..count {
        let event = EventEnvelope {
            event_id: [seed.wrapping_add(i as u8); 16],
            parent_state_hash: engine.current_hash(),
            payload_hash: [seed.wrapping_add(i as u8); 32],
            logical_time: i as u64,
        };
        engine.apply(&event).expect("apply failed");
        events.push(event);
    }

    let mut bytes = Vec::new();
    ciborium::ser::into_writer(&events, &mut bytes).expect("encode failed");
    fs::write("events.cbor", bytes).expect("write failed");
    println!("wrote events.cbor");
    println!("events: {}", count);
}
