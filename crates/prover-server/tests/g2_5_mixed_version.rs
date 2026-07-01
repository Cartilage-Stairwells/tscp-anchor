use tscp_protocol::proof_envelope::{ProofEnvelope, ProofVersion};

#[test]
fn test_061_envelope_opens_on_061_verifier() {
    let env = ProofEnvelope::seal_061(294373, vec![1, 2, 3]);
    assert_eq!(env.version, ProofVersion::V0_6_1);
    assert!(env.open(ProofVersion::V0_6_1).is_ok());
    assert!(env.verify_golden().is_ok());
}

#[test]
fn test_062_envelope_opens_on_062_verifier() {
    let env = ProofEnvelope::seal_062(294373, vec![42, 99]);
    assert_eq!(env.version, ProofVersion::V0_6_2);
    assert!(env.open(ProofVersion::V0_6_2).is_ok());
    assert!(env.verify_golden().is_ok());
}

#[test]
fn test_061_envelope_rejected_by_062_verifier() {
    let env = ProofEnvelope::seal_061(294373, vec![1, 2, 3]);
    let err = env.open(ProofVersion::V0_6_2).unwrap_err();
    assert!(format!("{err}").contains("Unsupported proof version"));
}

#[test]
fn test_062_envelope_rejected_by_061_verifier() {
    let env = ProofEnvelope::seal_062(294373, vec![42, 99]);
    let err = env.open(ProofVersion::V0_6_1).unwrap_err();
    assert!(format!("{err}").contains("Unsupported proof version"));
}
