// TSCP Admissibility Contract — Experimental Implementation v0.1
// Falsification-oriented: attempt to break the proposed boundary in a concrete language.
// Per ADMISSIBILITY_CONTRACT_SPEC v0.2 and Aria's red-team review.
//
// Language: Rust 1.97.1
// Safety model: safe code only. No `unsafe`. No FFI. No reflection.
// Dependencies: none. Pure protocol logic.
//
// SEMANTIC NON-IMPLICATIONS (spec §4):
//   AdmittedEvidence means: "the admission contract was satisfied." Nothing more.
//   AdmittedEvidence ⇏ Truth
//   AdmittedEvidence ⇏ Correctness  
//   AdmittedEvidence ⇏ Authenticity
//   AdmittedEvidence ⇏ Authority
//
// FORBIDDEN ARROWS:
//   AdmittedEvidence → Authority    FORBIDDEN — admission is not authority-producing
//   AdmittedEvidence → Truth        FORBIDDEN — admission does not establish truth

use std::collections::HashSet;

// ============================================================================
// CORE TYPES (spec §2)
// ============================================================================

/// SHA-256 digest: 64-char lowercase hex string.
pub type CanonicalDigest = String;

/// Evidence role in the admission context.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EvidenceRole {
    Input,
    Output,
    Attestation,
    Witness,
}

/// Evidence: a claim that a canonical artifact exists with a specific digest.
///
/// CONSTRAINT 1: Evidence contains NO authority fields.
/// No signature, threshold, authorization, weight, priority, or decision.
/// Evidence is purely descriptive — it says WHAT something is, not WHETHER it authorizes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub digest: CanonicalDigest,
    pub artifact_type: String,
    pub media_type: Option<String>,
    pub role: EvidenceRole,
}

/// Contract: immutable specification of admissibility rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contract {
    pub id: String,
    pub version: String,
    pub evidence_types: Vec<String>,
    pub evidence_roles: Vec<EvidenceRole>,
    pub min_evidence_count: usize,
    pub max_evidence_count: usize,
    pub required_roles: Vec<EvidenceRole>,
    pub canon_version: String,
}

// ============================================================================
// ADMITTED EVIDENCE — THE ONE-WAY TYPE (CONSTRAINT 2)
// ============================================================================
//
// CONSTRUCTION PATH ENUMERATION (Aria §8.2):
//   - No `AdmittedEvidence::new()` — does not exist
//   - No `#[derive(Deserialize)]` — no serde, serialization cannot manufacture this type
//   - No `Default` impl — zeroed memory cannot constitute an instance
//   - No public fields — all private, constructor is private
//   - No unsafe code — no transmute/reinterpret
//   - No FFI — no foreign construction
//   - No macros that emit `AdmittedEvidence { ... }`
//   - `admit()` is the ONLY path
//
// SERIALIZATION BOUNDARY (Aria §8.1, §10):
//   AdmittedEvidence does NOT implement Serialize or Deserialize.
//   A serialized representation is NOT automatically the semantic type.
//   Persistence requires a separate PersistedAdmissionRecord + re-admission.

/// AdmittedEvidence: a custody classification, NOT a quality upgrade.
/// Records that evidence crossed a specified boundary. Not "better evidence."
///
/// All fields PRIVATE. The ONLY way to obtain this value is `admit()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedEvidence {
    contract_id: String,
    contract_version: String,
    evidence: Vec<Evidence>,
    admitted_at: String,
    admission_digest: CanonicalDigest,
}

// Read-only accessors — inspection but NOT construction
impl AdmittedEvidence {
    pub fn contract_id(&self) -> &str { &self.contract_id }
    pub fn contract_version(&self) -> &str { &self.contract_version }
    pub fn evidence(&self) -> &[Evidence] { &self.evidence }
    pub fn admitted_at(&self) -> &str { &self.admitted_at }
    pub fn admission_digest(&self) -> &str { &self.admission_digest }
}

// EXPLICITLY NOT IMPLEMENTED:
//   impl Default for AdmittedEvidence  — NO
//   serde::Serialize for AdmittedEvidence  — NO
//   serde::Deserialize for AdmittedEvidence  — NO
//   AdmittedEvidence::new()  — NO

// ============================================================================
// REJECTED EVIDENCE (spec §2.5)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissibilityStage {
    Validation,
    Binding,
    Completeness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissibilityErrorCode {
    TypeNotAdmissible,
    RoleNotAdmissible,
    CanonVersionMismatch,
    InsufficientEvidence,
    ExcessEvidence,
    MissingRequiredRole,
    DuplicateDigest,
    ContractInvalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RejectedEvidence {
    pub evidence: Evidence,
    pub contract_id: String,
    pub reason: String,
    pub error_code: AdmissibilityErrorCode,
    pub error_stage: AdmissibilityStage,
}

// ============================================================================
// SHA-256 (minimal, no external dependencies)
// ============================================================================

fn sha256(input: &str) -> CanonicalDigest {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    let bytes = input.as_bytes();
    let bit_len = (bytes.len() as u64) * 8;
    let mut msg = bytes.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 { msg.push(0); }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i*4], chunk[i*4+1], chunk[i*4+2], chunk[i*4+3]]);
        }
        for i in 16..64 {
            let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
            let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
            w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
        }

        let mut a=h[0]; let mut b=h[1]; let mut c=h[2]; let mut d=h[3];
        let mut e=h[4]; let mut f=h[5]; let mut g=h[6]; let mut hh=h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(k[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh=g; g=f; f=e; e=d.wrapping_add(t1); d=c; c=b; b=a; a=t1.wrapping_add(t2);
        }

        h[0]=h[0].wrapping_add(a); h[1]=h[1].wrapping_add(b);
        h[2]=h[2].wrapping_add(c); h[3]=h[3].wrapping_add(d);
        h[4]=h[4].wrapping_add(e); h[5]=h[5].wrapping_add(f);
        h[6]=h[6].wrapping_add(g); h[7]=h[7].wrapping_add(hh);
    }

    h.iter().map(|x| format!("{:08x}", x)).collect()
}

// ============================================================================
// ADMISSION DIGEST (spec §3.5) — non-self-referential
// ============================================================================

fn compute_admission_digest(
    contract_id: &str,
    contract_version: &str,
    evidence_digests: &[CanonicalDigest],
    canon_version: &str,
) -> CanonicalDigest {
    let mut record = String::new();
    record.push_str("{\"admission_record\":{\"canon_version\":\"");
    record.push_str(canon_version);
    record.push_str("\",\"contract_id\":\"");
    record.push_str(contract_id);
    record.push_str("\",\"contract_version\":\"");
    record.push_str(contract_version);
    record.push_str("\",\"evidence_digests\":[");
    for (i, d) in evidence_digests.iter().enumerate() {
        if i > 0 { record.push(','); }
        record.push('"'); record.push_str(d); record.push('"');
    }
    record.push_str("]}}");
    sha256(&record)
}

// ============================================================================
// CONTRACT VALIDATION (spec §2.3)
// ============================================================================

fn validate_contract(c: &Contract) -> Option<AdmissibilityErrorCode> {
    if c.id.is_empty() || c.version.is_empty() { return Some(AdmissibilityErrorCode::ContractInvalid); }
    if c.evidence_types.is_empty() { return Some(AdmissibilityErrorCode::ContractInvalid); }
    if c.evidence_roles.is_empty() { return Some(AdmissibilityErrorCode::ContractInvalid); }
    if c.min_evidence_count < 1 { return Some(AdmissibilityErrorCode::ContractInvalid); }
    if c.max_evidence_count < c.min_evidence_count { return Some(AdmissibilityErrorCode::ContractInvalid); }
    for r in &c.required_roles {
        if !c.evidence_roles.contains(r) { return Some(AdmissibilityErrorCode::ContractInvalid); }
    }
    if c.canon_version.is_empty() { return Some(AdmissibilityErrorCode::ContractInvalid); }
    None
}

// ============================================================================
// THE ADMISSIBILITY FUNCTION (spec §3)
// ============================================================================
//
// SEMANTIC FIREWALL (spec §3.3):
//   VALIDATION → "structural/contractual validity" — NOT truth
//   BINDING    → "association with specification" — NOT endorsement
//   COMPLETENESS → "complete relative to schema" — NOT epistemic completeness
//
//   V ∧ B ∧ C → ADMISSIBLE — NOT TRUE, NOT CORRECT, NOT AUTHORIZED

/// The ONLY way to produce AdmittedEvidence. Pure function.
pub fn admit(
    contract: &Contract,
    evidence: &[Evidence],
) -> Result<AdmittedEvidence, Vec<RejectedEvidence>> {

    // --- Stage 1: VALIDATION ---
    // Establishes: structural/contractual validity. NOT truth.

    if let Some(_) = validate_contract(contract) {
        return Err(evidence.iter().map(|e| RejectedEvidence {
            evidence: e.clone(), contract_id: contract.id.clone(),
            reason: "Contract fails validity conditions".into(),
            error_code: AdmissibilityErrorCode::ContractInvalid,
            error_stage: AdmissibilityStage::Validation,
        }).collect());
    }

    let mut rejections: Vec<RejectedEvidence> = Vec::new();
    for ev in evidence {
        if ev.digest.len() != 64 || !ev.digest.chars().all(|c| c.is_ascii_hexdigit() && (c.is_ascii_digit() || c.is_ascii_lowercase())) {
            rejections.push(RejectedEvidence {
                evidence: ev.clone(), contract_id: contract.id.clone(),
                reason: "Evidence has invalid structure".into(),
                error_code: AdmissibilityErrorCode::ContractInvalid,
                error_stage: AdmissibilityStage::Validation,
            });
        }
        if ev.artifact_type.is_empty() {
            rejections.push(RejectedEvidence {
                evidence: ev.clone(), contract_id: contract.id.clone(),
                reason: "Evidence has empty artifact_type".into(),
                error_code: AdmissibilityErrorCode::ContractInvalid,
                error_stage: AdmissibilityStage::Validation,
            });
        }
    }
    if !rejections.is_empty() { return Err(rejections); }

    // --- Stage 2: BINDING ---
    // Establishes: association with contract. NOT endorsement.

    for ev in evidence {
        if !contract.evidence_types.contains(&ev.artifact_type) {
            rejections.push(RejectedEvidence {
                evidence: ev.clone(), contract_id: contract.id.clone(),
                reason: format!("artifact_type '{}' not admissible", ev.artifact_type),
                error_code: AdmissibilityErrorCode::TypeNotAdmissible,
                error_stage: AdmissibilityStage::Binding,
            });
        }
    }
    if !rejections.is_empty() { return Err(rejections); }

    for ev in evidence {
        if !contract.evidence_roles.contains(&ev.role) {
            rejections.push(RejectedEvidence {
                evidence: ev.clone(), contract_id: contract.id.clone(),
                reason: format!("role {:?} not admissible", ev.role),
                error_code: AdmissibilityErrorCode::RoleNotAdmissible,
                error_stage: AdmissibilityStage::Binding,
            });
        }
    }
    if !rejections.is_empty() { return Err(rejections); }

    let mut seen: HashSet<&str> = HashSet::new();
    for ev in evidence {
        if !seen.insert(&ev.digest) {
            rejections.push(RejectedEvidence {
                evidence: ev.clone(), contract_id: contract.id.clone(),
                reason: format!("duplicate digest: {}", ev.digest),
                error_code: AdmissibilityErrorCode::DuplicateDigest,
                error_stage: AdmissibilityStage::Binding,
            });
        }
    }
    if !rejections.is_empty() { return Err(rejections); }

    // --- Stage 3: COMPLETENESS ---
    // Establishes: complete relative to schema. NOT epistemically complete.

    if evidence.len() < contract.min_evidence_count {
        return Err(vec![RejectedEvidence {
            evidence: evidence[0].clone(), contract_id: contract.id.clone(),
            reason: format!("insufficient: {} items, min {}", evidence.len(), contract.min_evidence_count),
            error_code: AdmissibilityErrorCode::InsufficientEvidence,
            error_stage: AdmissibilityStage::Completeness,
        }]);
    }

    if evidence.len() > contract.max_evidence_count {
        return Err(vec![RejectedEvidence {
            evidence: evidence[0].clone(), contract_id: contract.id.clone(),
            reason: format!("excess: {} items, max {}", evidence.len(), contract.max_evidence_count),
            error_code: AdmissibilityErrorCode::ExcessEvidence,
            error_stage: AdmissibilityStage::Completeness,
        }]);
    }

    let present: HashSet<&EvidenceRole> = evidence.iter().map(|e| &e.role).collect();
    for req in &contract.required_roles {
        if !present.contains(req) {
            return Err(vec![RejectedEvidence {
                evidence: evidence[0].clone(), contract_id: contract.id.clone(),
                reason: format!("missing required role: {:?}", req),
                error_code: AdmissibilityErrorCode::MissingRequiredRole,
                error_stage: AdmissibilityStage::Completeness,
            }]);
        }
    }

    // --- ALL STAGES PASSED ---
    // The ONLY code path that constructs AdmittedEvidence.

    let digests: Vec<CanonicalDigest> = evidence.iter().map(|e| e.digest.clone()).collect();
    let admission_digest = compute_admission_digest(
        &contract.id, &contract.version, &digests, &contract.canon_version,
    );

    Ok(AdmittedEvidence {
        contract_id: contract.id.clone(),
        contract_version: contract.version.clone(),
        evidence: evidence.to_vec(),
        admitted_at: "2026-08-19T03:50:00Z".to_string(),
        admission_digest,
    })
}

// ============================================================================
// DOWNSTREAM PLACEHOLDERS (NOT IMPLEMENTED — out of scope per spec)
// ============================================================================
//
// evaluate(contract, admitted: AdmittedEvidence, predicate, state, transition) → Decision
//   — accepts AdmittedEvidence, NOT raw Evidence (CONSTRAINT 2)
//   — Decision is NOT convertible to Evidence (CONSTRAINT 3)
//
// apply(decision, current_state) → next_state | error
//   — custody state machine (not yet specified)
//
// Authority arises ONLY through: evaluate → Decision → apply → CustodyState → Authority
// AdmittedEvidence is one step. NOT the whole chain.
