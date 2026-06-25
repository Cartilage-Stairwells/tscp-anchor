use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;
use sha2::{Digest, Sha256};

use tscp_kernel::event::EventEnvelope;
use tscp_kernel::replay::ReplayEngine;

#[derive(Parser)]
#[command(name = "tscp", version, about = "TSCP deterministic verifier")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Verify { bundle: PathBuf },
}

const EXIT_PASS: u8 = 0;
const EXIT_MODIFIED_RECEIPT: u8 = 1;
const EXIT_MALFORMED_BUNDLE: u8 = 5;
const EXIT_CHECKSUM_MISMATCH: u8 = 6;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Verify { bundle } => {
            ExitCode::from(run_verify(&bundle))
        }
    }
}

fn run_verify(bundle_path: &PathBuf) -> u8 {
    let bytes = match std::fs::read(bundle_path) {
        Ok(b) => b,
        Err(_) => {
            println!("FAIL: malformed_bundle");
            return EXIT_MALFORMED_BUNDLE;
        }
    };

    let sha_path = PathBuf::from(
        format!("{}.sha256", bundle_path.display())
    );

    if sha_path.exists() {
        let expected = match std::fs::read_to_string(&sha_path) {
            Ok(v) => v,
            Err(_) => {
                println!("FAIL: malformed_bundle");
                return EXIT_MALFORMED_BUNDLE;
            }
        };

        let expected_hash = expected
            .split_whitespace()
            .next()
            .unwrap_or("");

        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let actual = hex::encode(hasher.finalize());

        if actual != expected_hash {
            println!("FAIL: checksum_mismatch");
            return EXIT_CHECKSUM_MISMATCH;
        }
    }

    let events: Vec<EventEnvelope> =
        match serde_cbor::from_slice(&bytes) {
            Ok(v) => v,
            Err(_) => {
                println!("FAIL: malformed_bundle");
                return EXIT_MALFORMED_BUNDLE;
            }
        };

    let mut engine = ReplayEngine::new(1);

    for event in &events {
        if engine.apply(event).is_err() {
            println!("FAIL: modified_receipt");
            return EXIT_MODIFIED_RECEIPT;
        }
    }

    println!("PASS");
    EXIT_PASS
}
