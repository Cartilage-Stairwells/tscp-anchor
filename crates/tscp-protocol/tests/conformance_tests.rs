use tscp_protocol::proof_envelope::ProofEnvelope;

#[test]
fn test_proof_envelope_serialization_boundary() {
    // Exact same instantiation as the baseline
    let env = ProofEnvelope::seal_061(294373, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let new_bytes = bincode::serialize(&env).expect("Serialization failed");
    
    let baseline_bytes = std::fs::read("../../proof_envelope_baseline.bin").expect("Baseline missing");
    
    assert_eq!(new_bytes, baseline_bytes, "Serialization boundary compromised!");
}
