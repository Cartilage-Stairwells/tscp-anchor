use sha2::{Sha256, Digest};
use tscp_kernel::types::TransitionHash;

pub fn prove_receipt(hash: &TransitionHash) -> Vec<u8> {
    // Experimental proof interface placeholder.
    // This hashes the receipt hash only; it is not a production STARK proof.
    // This will become real plonky3 circuit later
    let mut hasher = Sha256::new();
    hasher.update(hash);
    hasher.finalize().to_vec()
}

pub fn verify_proof(hash: &TransitionHash, proof: &[u8]) -> bool {
    prove_receipt(hash) == proof
}
