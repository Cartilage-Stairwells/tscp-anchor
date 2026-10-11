//! Oracle Bridge — the sole integration point between tscp-verifier and oracle-layer.
//!
//! This module is the ONLY file in tscp-verifier that imports `oracle_layer::` types.
//! It wraps the oracle-layer's `fri_prove` / `fri_verify` functions behind a typed
//! interface that:
//!
//! 1. Captures phase timings at execution boundaries using PhaseTimer.
//! 2. Returns typed errors instead of panicking.
//! 3. Serializes proof and transcript bytes for the emitter's digest chain.
//! 4. Does NOT import any artifact or schema types.
//!
//! Architectural invariant:
//!   oracle_bridge.rs → imports oracle_layer (proving)
//!   artifact.rs      → sees only Vec<u8> (no oracle-layer types)
//!   emitter.rs       → consumes both, builds PulseArtifact

use crate::timing::{phases, PhaseTimer};

use oracle_layer::fri_protocol::{Challenger, FriCommitment};
use oracle_layer::fri_query::{fri_prove, fri_verify, FriProof, FriQueryRound};
use oracle_layer::merkle::Commitment;
use oracle_layer::oracle::MleOracle;
use oracle_layer::sumcheck::sumcheck_round;

use p3_baby_bear::BabyBear;
use p3_field::PrimeField32;

/// Error type for all oracle-bridge operations.
#[derive(Debug)]
pub enum BridgeError {
    InvalidInput(String),
    VerificationFailed,
    SerializationError(String),
}

impl std::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BridgeError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            BridgeError::VerificationFailed => write!(f, "Proof verification failed"),
            BridgeError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for BridgeError {}

// ---------------------------------------------------------------------------
// tscp-fri-proof-serialization-v1 (Issue #37, ARCHER Findings 29 + 30)
//
// Contract: tscp-fri-proof-serialization-v1-verification/CONTRACT.md.
// The CONTRACT is the authority; this implementation conforms to it.
// Supersedes the pre-v1 basis (JSON with Debug-formatted roots, Merkle
// openings omitted). Old-basis digests preserved in issue #37,
// comment 6107484068.
// ---------------------------------------------------------------------------

pub const FRI_PROOF_MAGIC: [u8; 8] = *b"tscp-fri";
pub const FS_TRANSCRIPT_PROXY_MAGIC: [u8; 8] = *b"tscp-fst";
pub const FRI_PROOF_FORMAT_VERSION: u16 = 1;
/// Digest width of the BatchMerkle commitment: 8 BabyBear elements
/// (DIGEST_ELEMS of MerkleTreeMmcs<_, _, PaddingFreeSponge<...,16,8,8>, .., 2, 8>).
pub const COMMITMENT_DIGEST_ELEMS: usize = 8;

type Digest = [BabyBear; COMMITMENT_DIGEST_ELEMS];

fn write_u16(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn write_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn write_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn write_elem(out: &mut Vec<u8>, e: BabyBear) {
    write_u32(out, e.as_canonical_u32());
}

fn write_digest(out: &mut Vec<u8>, d: &Digest) {
    for e in d.iter() {
        write_elem(out, *e);
    }
}

/// Bounded cursor: every read checks the remaining byte budget first,
/// so no untrusted length prefix can trigger unbounded allocation.
struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], BridgeError> {
        let end = self
            .pos
            .checked_add(n)
            .filter(|e| *e <= self.buf.len())
            .ok_or_else(|| BridgeError::SerializationError("truncated input".to_string()))?;
        let s = &self.buf[self.pos..end];
        self.pos = end;
        Ok(s)
    }

    fn read_u16(&mut self) -> Result<u16, BridgeError> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    fn read_u32(&mut self) -> Result<u32, BridgeError> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn read_u64(&mut self) -> Result<u64, BridgeError> {
        let b = self.take(8)?;
        let mut w = [0u8; 8];
        w.copy_from_slice(b);
        Ok(u64::from_le_bytes(w))
    }

    /// Canonical BabyBear element: value must be in [0, ORDER).
    fn read_elem(&mut self) -> Result<BabyBear, BridgeError> {
        let v = self.read_u32()?;
        if v >= BabyBear::ORDER_U32 {
            return Err(BridgeError::SerializationError(format!(
                "field value {} >= BabyBear ORDER {}",
                v,
                BabyBear::ORDER_U32
            )));
        }
        Ok(BabyBear::new(v))
    }

    fn read_digest(&mut self) -> Result<Digest, BridgeError> {
        let mut d = [BabyBear::new(0); COMMITMENT_DIGEST_ELEMS];
        for e in d.iter_mut() {
            *e = self.read_elem()?;
        }
        Ok(d)
    }

    /// Length prefix bounded by the remaining byte budget divided by the
    /// per-item width: a count larger than could possibly fit is rejected
    /// before any allocation.
    fn read_bounded_len(&mut self, item_width: usize) -> Result<u32, BridgeError> {
        let n = self.read_u32()?;
        let remaining = self.buf.len() - self.pos;
        if n as usize > remaining / item_width {
            return Err(BridgeError::SerializationError(format!(
                "length prefix {} exceeds remaining budget ({} bytes left, item width {})",
                n, remaining, item_width
            )));
        }
        Ok(n)
    }

    fn read_usize_from_u64(&mut self, what: &str) -> Result<usize, BridgeError> {
        let v = self.read_u64()?;
        usize::try_from(v).map_err(|_| {
            BridgeError::SerializationError(format!("{} {} does not fit usize", what, v))
        })
    }
}

fn encode_commitment_roots(out: &mut Vec<u8>, roots: &[Commitment], final_value: BabyBear) {
    write_elem(out, final_value);
    write_u32(out, roots.len() as u32);
    for cap in roots {
        let digests = cap.roots();
        write_u32(out, digests.len() as u32);
        for d in digests {
            write_digest(out, d);
        }
    }
}

/// Encode a FriProof under tscp-fri-proof-serialization-v1 (contract above).
/// Deterministic: identical proofs produce byte-identical output.
pub fn encode_fri_proof_v1(proof: &FriProof) -> Result<Vec<u8>, BridgeError> {
    let mut out = Vec::new();
    out.extend_from_slice(&FRI_PROOF_MAGIC);
    write_u16(&mut out, FRI_PROOF_FORMAT_VERSION);
    encode_commitment_roots(
        &mut out,
        &proof.commitment.roots,
        proof.commitment.final_value,
    );

    write_u32(&mut out, proof.query_indices.len() as u32);
    for idx in &proof.query_indices {
        let v = u64::try_from(*idx).map_err(|_| {
            BridgeError::SerializationError(format!("query index {} does not fit u64", idx))
        })?;
        write_u64(&mut out, v);
    }

    for query in &proof.query_proofs {
        write_u32(&mut out, query.len() as u32);
        for round in query {
            for opening in [
                &round.opening_x,
                &round.opening_neg_x,
                &round.opening_folded,
            ] {
                let idx = u64::try_from(opening.leaf_index).map_err(|_| {
                    BridgeError::SerializationError(format!(
                        "leaf index {} does not fit u64",
                        opening.leaf_index
                    ))
                })?;
                write_u64(&mut out, idx);
                write_elem(&mut out, opening.leaf_value);
                write_u32(&mut out, opening.proof.len() as u32);
                for d in &opening.proof {
                    write_digest(&mut out, d);
                }
            }
        }
    }
    Ok(out)
}

/// Encode the commit-phase transcript proxy (record type 2 in the contract).
pub fn encode_fs_transcript_proxy_v1(proof: &FriProof) -> Result<Vec<u8>, BridgeError> {
    let mut out = Vec::new();
    out.extend_from_slice(&FS_TRANSCRIPT_PROXY_MAGIC);
    write_u16(&mut out, FRI_PROOF_FORMAT_VERSION);
    encode_commitment_roots(
        &mut out,
        &proof.commitment.roots,
        proof.commitment.final_value,
    );
    Ok(out)
}

/// Decode tscp-fri-proof-serialization-v1 bytes into a complete FriProof.
/// Fails closed per the contract's rejection behavior.
pub fn decode_fri_proof_v1(bytes: &[u8]) -> Result<FriProof, BridgeError> {
    let mut r = Reader::new(bytes);

    let magic = r.take(8)?;
    if magic != FRI_PROOF_MAGIC {
        return Err(BridgeError::SerializationError("bad magic".to_string()));
    }
    let version = r.read_u16()?;
    if version != FRI_PROOF_FORMAT_VERSION {
        return Err(BridgeError::SerializationError(format!(
            "unsupported version {}",
            version
        )));
    }

    let final_value = r.read_elem()?;
    let num_roots = r.read_bounded_len(4)?;
    if num_roots < 2 {
        return Err(BridgeError::SerializationError(format!(
            "num_roots {} < 2",
            num_roots
        )));
    }
    let mut roots = Vec::with_capacity(num_roots as usize);
    for _ in 0..num_roots {
        let num_digests = r.read_bounded_len(COMMITMENT_DIGEST_ELEMS * 4)?;
        if num_digests == 0 || !(num_digests as usize).is_power_of_two() {
            return Err(BridgeError::SerializationError(format!(
                "commitment digest count {} invalid (must be >= 1 and a power of two)",
                num_digests
            )));
        }
        let mut digests = Vec::with_capacity(num_digests as usize);
        for _ in 0..num_digests {
            digests.push(r.read_digest()?);
        }
        roots.push(Commitment::new(digests));
    }

    let num_queries = r.read_bounded_len(8)?;
    if num_queries == 0 {
        return Err(BridgeError::SerializationError(
            "num_queries is 0".to_string(),
        ));
    }
    let mut query_indices = Vec::with_capacity(num_queries as usize);
    for _ in 0..num_queries {
        query_indices.push(r.read_usize_from_u64("query index")?);
    }

    let expected_rounds = (num_roots - 1) as usize;
    let mut query_proofs = Vec::with_capacity(num_queries as usize);
    for _ in 0..num_queries {
        let num_rounds = r.read_bounded_len(8)?;
        if num_rounds as usize != expected_rounds {
            return Err(BridgeError::SerializationError(format!(
                "query round count {} != num_roots - 1 ({})",
                num_rounds, expected_rounds
            )));
        }
        let mut rounds = Vec::with_capacity(num_rounds as usize);
        for _ in 0..num_rounds {
            let read_opening =
                |r: &mut Reader| -> Result<oracle_layer::merkle::MerkleOpening, BridgeError> {
                    let leaf_index = r.read_usize_from_u64("leaf index")?;
                    let leaf_value = r.read_elem()?;
                    let auth_len = r.read_bounded_len(COMMITMENT_DIGEST_ELEMS * 4)?;
                    let mut auth = Vec::with_capacity(auth_len as usize);
                    for _ in 0..auth_len {
                        auth.push(r.read_digest()?);
                    }
                    Ok(oracle_layer::merkle::MerkleOpening {
                        leaf_index,
                        leaf_value,
                        proof: auth,
                    })
                };
            let opening_x = read_opening(&mut r)?;
            let opening_neg_x = read_opening(&mut r)?;
            let opening_folded = read_opening(&mut r)?;
            rounds.push(FriQueryRound {
                opening_x,
                opening_neg_x,
                opening_folded,
            });
        }
        query_proofs.push(rounds);
    }

    if r.pos != bytes.len() {
        return Err(BridgeError::SerializationError(
            "trailing bytes after canonical encoding".to_string(),
        ));
    }

    Ok(FriProof {
        commitment: FriCommitment { roots, final_value },
        query_proofs,
        query_indices,
    })
}

/// Convert serialized form back to a partial FriProof for verification.
/// Note: this reconstruction is sufficient for re-running fri_verify because
/// the query proofs are reconstructed from the original proof data.
/// Serialize the proof under tscp-fri-proof-serialization-v1 (canonical
/// bytes, openings included — Issue #37, Findings 29 + 30).
fn serialize_proof(proof: &FriProof) -> Result<Vec<u8>, BridgeError> {
    encode_fri_proof_v1(proof)
}

/// The result of an instrumented proving run.
pub struct ProveResult {
    pub proof_bytes: Vec<u8>,
    pub transcript_bytes: Vec<u8>,
    pub timer: PhaseTimer,
    pub fiat_shamir_rounds: u32,
}

/// The result of an instrumented verification run.
pub struct VerifyResult {
    pub verification_ok: bool,
    pub timer: PhaseTimer,
}

/// Internal: holds the original proof alongside its serialized bytes,
/// so verification can use the original typed proof.
pub struct ProveResultInternal {
    pub proof_bytes: Vec<u8>,
    pub transcript_bytes: Vec<u8>,
    pub timer: PhaseTimer,
    pub fiat_shamir_rounds: u32,
    pub proof: FriProof, // kept for in-process verification
}

impl std::fmt::Debug for ProveResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProveResult")
            .field("proof_bytes_len", &self.proof_bytes.len())
            .field("transcript_bytes_len", &self.transcript_bytes.len())
            .field("fiat_shamir_rounds", &self.fiat_shamir_rounds)
            .finish()
    }
}

impl std::fmt::Debug for VerifyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VerifyResult")
            .field("verification_ok", &self.verification_ok)
            .finish()
    }
}

impl std::fmt::Debug for ProveResultInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProveResultInternal")
            .field("proof_bytes_len", &self.proof_bytes.len())
            .field("transcript_bytes_len", &self.transcript_bytes.len())
            .field("fiat_shamir_rounds", &self.fiat_shamir_rounds)
            .finish()
    }
}

/// Run the full FRI proving pipeline with phase-level instrumentation.
///
/// Phase boundaries (each bracketed by PhaseTimer):
///   - transcript_generation: challenger setup + initial Merkle commitment
///   - fri: fold rounds + query opening proofs
///   - proving_total: wall-clock total (brackets the entire prove call)
pub fn prove_instrumented(
    evals: Vec<BabyBear>,
    domain: Vec<BabyBear>,
    num_queries: usize,
) -> Result<ProveResult, BridgeError> {
    let internal = prove_instrumented_internal(evals, domain, num_queries)?;

    Ok(ProveResult {
        proof_bytes: internal.proof_bytes,
        transcript_bytes: internal.transcript_bytes,
        timer: internal.timer,
        fiat_shamir_rounds: internal.fiat_shamir_rounds,
    })
}

/// Internal function that retains the typed proof for in-process verification.
pub fn prove_instrumented_internal(
    evals: Vec<BabyBear>,
    domain: Vec<BabyBear>,
    num_queries: usize,
) -> Result<ProveResultInternal, BridgeError> {
    let n = evals.len();
    if n == 0 {
        return Err(BridgeError::InvalidInput("evals is empty".to_string()));
    }
    if !n.is_power_of_two() {
        return Err(BridgeError::InvalidInput(format!(
            "evals.len() = {} is not a power of two",
            n
        )));
    }
    if domain.len() != n {
        return Err(BridgeError::InvalidInput(format!(
            "domain.len() = {} != evals.len() = {}",
            domain.len(),
            n
        )));
    }

    let mut timer = PhaseTimer::new();

    // --- PHASE BOUNDARY: proving_total (brackets entire prove call) ---
    // File: oracle-layer/src/fri_query.rs, fn fri_prove (line ~90)
    timer.start(phases::PROVING_TOTAL);

    // --- PHASE BOUNDARY: transcript_generation ---
    // File: oracle-layer/src/fri_protocol.rs, fn fri_commit_transcript (line ~100)
    timer.start(phases::TRANSCRIPT_GENERATION);

    let perm = p3_baby_bear::default_babybear_poseidon2_16();
    let mut challenger = Challenger::new(perm);

    // The initial commitment is part of transcript generation.
    timer.stop(); // transcript_generation

    // --- PHASE BOUNDARY: fri (folding + query opening proofs) ---
    // File: oracle-layer/src/fri_query.rs, fn fri_prove (fold loop + query phase)
    timer.start(phases::FRI);

    let proof = fri_prove(evals, domain.clone(), &mut challenger, num_queries);

    timer.stop(); // fri
    timer.stop(); // proving_total

    // --- Serialize proof to canonical bytes ---
    let proof_bytes = serialize_proof(&proof)?;

    // Transcript: commit-phase proxy under tscp-fri-proof-serialization-v1
    // (record type 2) — replaces the pre-v1 {:?}-formatted roots
    let transcript_bytes = encode_fs_transcript_proxy_v1(&proof)?;

    let fiat_shamir_rounds = proof.commitment.roots.len() as u32;

    Ok(ProveResultInternal {
        proof_bytes,
        transcript_bytes,
        timer,
        fiat_shamir_rounds,
        proof,
    })
}

/// Run FRI verification with timing instrumentation, using the original typed proof.
/// This is the primary verification path for in-process benchmarking.
///
/// Phase boundary:
///   - verification: brackets the entire verify call
pub fn verify_instrumented_with_proof(
    domain: &[BabyBear],
    proof: &FriProof,
    num_queries: usize,
) -> Result<VerifyResult, BridgeError> {
    let mut timer = PhaseTimer::new();

    // --- PHASE BOUNDARY: verification ---
    // File: oracle-layer/src/fri_query.rs, fn fri_verify (line ~169)
    timer.start(phases::VERIFICATION);

    let perm = p3_baby_bear::default_babybear_poseidon2_16();
    let mut challenger = Challenger::new(perm);

    let verification_ok = fri_verify(domain, proof, &mut challenger, num_queries);

    timer.stop(); // verification

    Ok(VerifyResult {
        verification_ok,
        timer,
    })
}

/// Run a single sumcheck round with timing.
///
/// Phase boundary:
///   - sumcheck: brackets the sumcheck_round call
pub fn sumcheck_instrumented<O: MleOracle<BabyBear>>(
    oracle: &O,
    prefix: &[BabyBear],
) -> Result<([BabyBear; 2], PhaseTimer), BridgeError> {
    let mut timer = PhaseTimer::new();

    // --- PHASE BOUNDARY: sumcheck ---
    // File: oracle-layer/src/sumcheck.rs, fn sumcheck_round (line ~11)
    timer.start(phases::SUMCHECK);

    let result = sumcheck_round(oracle, prefix);

    timer.stop(); // sumcheck

    Ok((result, timer))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use p3_baby_bear::BabyBear;

    type F = BabyBear;

    #[test]
    fn test_bridge_prove_verify_roundtrip() {
        let n = 256usize;
        let evals: Vec<F> = (1..=n).map(|i| F::new(i as u32)).collect();
        let domain: Vec<F> = (1..=n).map(|i| F::new((i * 5 + 1) as u32)).collect();

        let prove_result =
            prove_instrumented_internal(evals, domain.clone(), 20).expect("prove should succeed");

        assert!(
            !prove_result.proof_bytes.is_empty(),
            "proof bytes must not be empty"
        );
        assert!(
            !prove_result.transcript_bytes.is_empty(),
            "transcript bytes must not be empty"
        );
        assert!(prove_result.timer.get_or_zero(phases::PROVING_TOTAL) >= 0.0);
        assert!(prove_result.timer.get_or_zero(phases::FRI) >= 0.0);
        assert!(
            prove_result
                .timer
                .get_or_zero(phases::TRANSCRIPT_GENERATION)
                >= 0.0
        );
        assert!(
            prove_result.fiat_shamir_rounds > 0,
            "must have at least 1 Fiat-Shamir round"
        );

        // Verify using the original typed proof
        let verify_result = verify_instrumented_with_proof(&domain, &prove_result.proof, 20)
            .expect("verify should succeed");

        assert!(
            verify_result.verification_ok,
            "verification must pass for a valid proof"
        );
        assert!(verify_result.timer.get_or_zero(phases::VERIFICATION) >= 0.0);
    }

    #[test]
    fn test_bridge_rejects_non_power_of_two() {
        let evals: Vec<F> = (0..7).map(F::new).collect();
        let domain: Vec<F> = (0..7).map(F::new).collect();

        let result = prove_instrumented(evals, domain, 20);
        assert!(result.is_err());
        match result.unwrap_err() {
            BridgeError::InvalidInput(msg) => {
                assert!(
                    msg.contains("power of two"),
                    "error should mention power of two: {}",
                    msg
                );
            }
            _ => panic!("expected InvalidInput error"),
        }
    }

    #[test]
    fn test_bridge_rejects_empty_input() {
        let evals: Vec<F> = vec![];
        let domain: Vec<F> = vec![];

        let result = prove_instrumented(evals, domain, 20);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BridgeError::InvalidInput(_)));
    }

    #[test]
    fn test_bridge_rejects_mismatched_lengths() {
        let evals: Vec<F> = (0..8).map(F::new).collect();
        let domain: Vec<F> = (0..4).map(F::new).collect();

        let result = prove_instrumented(evals, domain, 20);
        assert!(result.is_err());
        match result.unwrap_err() {
            BridgeError::InvalidInput(msg) => {
                assert!(
                    msg.contains("domain.len()"),
                    "error should mention length mismatch: {}",
                    msg
                );
            }
            _ => panic!("expected InvalidInput error"),
        }
    }

    #[test]
    fn test_bridge_serialization_is_canonical_binary() {
        let n = 256usize;
        let evals: Vec<F> = (1..=n).map(|i| F::new(i as u32)).collect();
        let domain: Vec<F> = (1..=n).map(|i| F::new((i * 5 + 1) as u32)).collect();

        let result = prove_instrumented(evals, domain, 20).expect("prove should succeed");

        // tscp-fri-proof-serialization-v1: binary, magic + version header,
        // decodable and re-verifiable from bytes alone. JSON is no longer
        // the digest input (Issue #37, Findings 29 + 30).
        assert_eq!(
            &result.proof_bytes[..8],
            &FRI_PROOF_MAGIC[..],
            "proof bytes must start with the v1 magic"
        );
        assert_eq!(
            u16::from_le_bytes([result.proof_bytes[8], result.proof_bytes[9]]),
            FRI_PROOF_FORMAT_VERSION
        );
        assert!(decode_fri_proof_v1(&result.proof_bytes).is_ok());
        assert!(
            !result.proof_bytes.windows(9).any(|w| w == b"MerkleCap"),
            "no Debug-format residue in the digest input"
        );
    }

    #[test]
    fn test_bridge_timings_are_recorded() {
        let n = 1024usize;
        let evals: Vec<F> = (1..=n).map(|i| F::new(i as u32)).collect();
        let domain: Vec<F> = (1..=n).map(|i| F::new((i * 5 + 1) as u32)).collect();

        let result = prove_instrumented(evals, domain, 20).expect("prove should succeed");

        assert!(
            result.timer.get(phases::PROVING_TOTAL).is_some(),
            "proving_total must be recorded"
        );
        assert!(
            result.timer.get(phases::FRI).is_some(),
            "fri must be recorded"
        );
        assert!(
            result.timer.get(phases::TRANSCRIPT_GENERATION).is_some(),
            "transcript_generation must be recorded"
        );
    }

    #[test]
    fn test_bridge_error_display() {
        let err = BridgeError::InvalidInput("test".to_string());
        assert_eq!(format!("{}", err), "Invalid input: test");

        let err = BridgeError::VerificationFailed;
        assert_eq!(format!("{}", err), "Proof verification failed");

        let err = BridgeError::SerializationError("json".to_string());
        assert_eq!(format!("{}", err), "Serialization error: json");
    }

    // -----------------------------------------------------------------
    // tscp-fri-proof-serialization-v1 acceptance gates (Issue #37)
    // Contract: tscp-fri-proof-serialization-v1-verification/CONTRACT.md
    // -----------------------------------------------------------------

    fn v16_vector() -> (Vec<F>, Vec<F>, usize) {
        let n = 16usize;
        let evals: Vec<F> = (1..=n).map(|i| F::new(i as u32)).collect();
        let domain: Vec<F> = (1..=n).map(|i| F::new((i * 5 + 1) as u32)).collect();
        (evals, domain, 4)
    }

    fn v256_vector() -> (Vec<F>, Vec<F>, usize) {
        let n = 256usize;
        let evals: Vec<F> = (1..=n).map(|i| F::new(i as u32)).collect();
        let domain: Vec<F> = (1..=n).map(|i| F::new((i * 5 + 1) as u32)).collect();
        (evals, domain, 20)
    }

    fn v16_bytes() -> Vec<u8> {
        let (evals, domain, q) = v16_vector();
        prove_instrumented(evals, domain, q)
            .expect("prove should succeed")
            .proof_bytes
    }

    fn fresh_challenger() -> Challenger {
        Challenger::new(p3_baby_bear::default_babybear_poseidon2_16())
    }

    /// Gate: completeness — every verification-relevant field of the proof
    /// survives the encode/decode round trip.
    #[test]
    fn gate_completeness_all_fields_survive() {
        let (evals, domain, q) = v256_vector();
        let r = prove_instrumented_internal(evals, domain, q).expect("prove");
        let decoded = decode_fri_proof_v1(&r.proof_bytes).expect("decode");

        assert_eq!(
            decoded.commitment.final_value.as_canonical_u32(),
            r.proof.commitment.final_value.as_canonical_u32()
        );
        assert_eq!(decoded.query_indices, r.proof.query_indices);
        assert_eq!(
            decoded.commitment.roots.len(),
            r.proof.commitment.roots.len()
        );
        for (a, b) in decoded
            .commitment
            .roots
            .iter()
            .zip(r.proof.commitment.roots.iter())
        {
            assert_eq!(a.num_roots(), b.num_roots());
            for (da, db) in a.roots().iter().zip(b.roots().iter()) {
                assert_eq!(da.len(), db.len());
                for (xa, xb) in da.iter().zip(db.iter()) {
                    assert_eq!(xa.as_canonical_u32(), xb.as_canonical_u32());
                }
            }
        }
        assert_eq!(decoded.query_proofs.len(), r.proof.query_proofs.len());
        for (qa, qb) in decoded.query_proofs.iter().zip(r.proof.query_proofs.iter()) {
            assert_eq!(qa.len(), qb.len());
            for (ra, rb) in qa.iter().zip(qb.iter()) {
                let pairs = [
                    (&ra.opening_x, &rb.opening_x),
                    (&ra.opening_neg_x, &rb.opening_neg_x),
                    (&ra.opening_folded, &rb.opening_folded),
                ];
                for (oa, ob) in pairs {
                    assert_eq!(oa.leaf_index, ob.leaf_index);
                    assert_eq!(
                        oa.leaf_value.as_canonical_u32(),
                        ob.leaf_value.as_canonical_u32()
                    );
                    assert_eq!(oa.proof.len(), ob.proof.len());
                    for (da, db) in oa.proof.iter().zip(ob.proof.iter()) {
                        assert_eq!(da.len(), db.len());
                        for (xa, xb) in da.iter().zip(db.iter()) {
                            assert_eq!(xa.as_canonical_u32(), xb.as_canonical_u32());
                        }
                    }
                }
            }
        }
    }

    /// Gate: determinism — identical proof, byte-identical encoding.
    #[test]
    fn gate_determinism_identical_bytes() {
        let (evals, domain, q) = v16_vector();
        let b1 = prove_instrumented(evals.clone(), domain.clone(), q)
            .expect("prove 1")
            .proof_bytes;
        let b2 = prove_instrumented(evals, domain, q)
            .expect("prove 2")
            .proof_bytes;
        assert_eq!(
            b1, b2,
            "two runs of the same vector must encode identically"
        );
    }

    /// Gate: canonicality + rot detection — the live encoder must reproduce
    /// the committed golden fixture exactly, and the digest must match.
    #[test]
    fn gate_canonicality_golden_fixture() {
        let b = v16_bytes();
        let fixture = std::fs::read("../../fixtures/phase3/fri_proof_v1_test_vector.bin")
            .expect("golden fixture must exist");
        assert_eq!(
            b, fixture,
            "live encoder must reproduce the committed golden fixture byte-for-byte"
        );
        assert_eq!(
            crate::provenance::sha256_hex(&b),
            "6c4d9b327a7cfc9a451a17dec48ae67e215059768e2bbef74bab05e1a937ec99"
        );
    }

    /// Gate: digest binding — the emitted digest is sha256 over the canonical
    /// bytes, for both vectors (256-vector digests frozen as a second golden).
    #[test]
    fn gate_digest_binding() {
        let (evals, domain, q) = v256_vector();
        let r = prove_instrumented(evals, domain, q).expect("prove");
        assert_eq!(
            crate::provenance::sha256_hex(&r.proof_bytes),
            "0c5c28d09a166b1cdaa4a174613df5708e952972fc1c40704a0eaa0c9868007a"
        );
        assert_eq!(
            crate::provenance::sha256_hex(&r.transcript_bytes),
            "0a9340aa385c7bab20bc1a1799e8a41d9a5accd5472b957ade58ad30d3346531"
        );
    }

    /// Gate: round-trip — the decoded proof verifies using ONLY the
    /// serialized bytes, a fresh challenger, and the public domain.
    /// The in-memory proof supplies nothing.
    #[test]
    fn gate_roundtrip_verifies_from_bytes_alone() {
        for (evals, domain, q) in [v16_vector(), v256_vector()] {
            let b = prove_instrumented(evals, domain.clone(), q)
                .expect("prove")
                .proof_bytes;
            let proof = decode_fri_proof_v1(&b).expect("decode");
            let mut challenger = fresh_challenger();
            assert!(
                fri_verify(&domain, &proof, &mut challenger, q),
                "decoded proof must verify from bytes alone"
            );
        }
    }

    /// Gate: mutation sensitivity — every single-bit flip in the canonical
    /// bytes either fails decode (fail closed) or breaks verification.
    #[test]
    fn gate_mutation_sensitivity_bit_flip_sweep() {
        let (evals, domain, q) = v16_vector();
        let orig = prove_instrumented(evals, domain.clone(), q)
            .expect("prove")
            .proof_bytes;
        for i in 0..orig.len() {
            let mut m = orig.clone();
            m[i] ^= 0x01;
            if let Ok(proof) = decode_fri_proof_v1(&m) {
                let mut challenger = fresh_challenger();
                assert!(
                    !fri_verify(&domain, &proof, &mut challenger, q),
                    "mutated byte {} decoded but still verified —                      verification-relevant byte not bound",
                    i
                );
            }
        }
    }

    /// Gate: malformed-input rejection — truncation at every prefix length.
    #[test]
    fn gate_malformed_truncation_sweep() {
        let orig = v16_bytes();
        for cut in 0..orig.len() {
            assert!(
                decode_fri_proof_v1(&orig[..cut]).is_err(),
                "truncation at byte {} must be rejected",
                cut
            );
        }
    }

    /// Gate: malformed-input rejection — battery of structural and
    /// byte-level corruption. The decoder fails closed on all of them.
    #[test]
    fn gate_malformed_battery() {
        let orig = v16_bytes();
        let (evals, domain, q) = v16_vector();
        let r = prove_instrumented(evals, domain, q).expect("prove");

        // wrong magic
        let mut m = orig.clone();
        m[0] ^= 0xFF;
        assert!(decode_fri_proof_v1(&m).is_err(), "bad magic");
        // unsupported version
        let mut m = orig.clone();
        m[8] = 0x02;
        assert!(decode_fri_proof_v1(&m).is_err(), "version 2 unsupported");
        // trailing byte
        let mut m = orig.clone();
        m.push(0x00);
        assert!(decode_fri_proof_v1(&m).is_err(), "trailing bytes");
        // length-prefix budget bomb: num_roots = 0xFFFFFFFF (offset 14)
        let mut m = orig.clone();
        m[14] = 0xFF;
        m[15] = 0xFF;
        m[16] = 0xFF;
        m[17] = 0xFF;
        assert!(decode_fri_proof_v1(&m).is_err(), "budget bomb rejected");
        // field value >= ORDER: final_value bytes 10..14 = FF FF FF 7F
        let mut m = orig.clone();
        m[10] = 0xFF;
        m[11] = 0xFF;
        m[12] = 0xFF;
        m[13] = 0x7F;
        assert!(decode_fri_proof_v1(&m).is_err(), "field value >= ORDER");

        // structural: num_queries = 0 (round-trips through the encoder)
        let mut p0 = decode_fri_proof_v1(&r.proof_bytes).expect("decode");
        p0.query_indices.clear();
        p0.query_proofs.clear();
        assert!(decode_fri_proof_v1(&encode_fri_proof_v1(&p0).unwrap()).is_err());

        // structural: round count inconsistent with num_roots - 1
        let mut p1 = decode_fri_proof_v1(&r.proof_bytes).expect("decode");
        p1.query_proofs[0].pop();
        assert!(decode_fri_proof_v1(&encode_fri_proof_v1(&p1).unwrap()).is_err());

        // structural: single root (num_roots < 2)
        let mut p2 = decode_fri_proof_v1(&r.proof_bytes).expect("decode");
        p2.commitment.roots.truncate(1);
        assert!(decode_fri_proof_v1(&encode_fri_proof_v1(&p2).unwrap()).is_err());

        // structural: zero digest count in a commitment — corrupt the
        // num_digests prefix of root 0 at byte level (a count of 0 cannot
        // be built through the public API: MerkleCap asserts power-of-two).
        // The num_digests prefix of root 0 sits at offset 10+4+4 = 18.
        let mut m = orig.clone();
        m[18] = 0x00;
        m[19] = 0x00;
        m[20] = 0x00;
        m[21] = 0x00;
        assert!(decode_fri_proof_v1(&m).is_err(), "zero digest count");
    }

    /// Gate: transcript proxy is canonical — deterministic, versioned, and
    /// free of Debug-format residue.
    #[test]
    fn gate_transcript_proxy_canonical() {
        let (evals, domain, q) = v256_vector();
        let r1 = prove_instrumented(evals.clone(), domain.clone(), q).expect("prove 1");
        let r2 = prove_instrumented(evals, domain, q).expect("prove 2");
        assert_eq!(r1.transcript_bytes, r2.transcript_bytes);
        assert_eq!(
            &r1.transcript_bytes[..8],
            &FS_TRANSCRIPT_PROXY_MAGIC[..],
            "transcript proxy must carry the v1 magic"
        );
        assert!(
            !r1.transcript_bytes.windows(9).any(|w| w == b"MerkleCap"),
            "no Debug-format residue in the transcript digest input"
        );
    }
}
