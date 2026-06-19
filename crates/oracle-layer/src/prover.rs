// ✅ SECURE: Observe everything first, then sample
let mut challenger = DuplexChallenger::new(perm);
challenger.observe_slice(&air_digest);
challenger.observe_slice(&fri_params);
challenger.observe(trace_commitment);

let alpha = challenger.sample(); 
nano crates/oracle-layer/src/prover.rs
