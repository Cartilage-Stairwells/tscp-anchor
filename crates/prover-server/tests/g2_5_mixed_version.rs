use prover_server::proof_envelope::{ProofEnvelope, ProofVersion};

#
fn test_061_envelope_opens_on_061() {
    let env = ProofEnvelope::seal_061(294373, vec![1,2,3]);
    assert_eq!(env.version, ProofVersion::V0_6_1);
    assert!(env.open().is_ok());
    assert!(env.verify_golden().is_ok());
}

#
fn test_062_rejected_by_061_verifier() {
    // Simulate future 0.6.2 envelope
    let env = ProofEnvelope {
        version: ProofVersion::V0_6_2,
        plonky3_semver: [0,6,2],
        claim: 294373,
        payload: vec![],
    };
    let err = env.open().unwrap_err();
    assert!(format!("{err}").contains("Unsupported proof version"));
}
