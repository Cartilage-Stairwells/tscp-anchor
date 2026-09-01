# Evidence-to-Prose Audit (Stage 2)

**Manuscript:** TSCP Paper Draft v2.1
**Date:** August 30, 2026
**Method:** Claim ID → exact manuscript language → registered status → evidence needed → overclaim risk → required action

---

## Claim Register

| ID | Section | Manuscript Language | Registered Status | Evidence | Overclaim Risk | Required Action |
|---|---|---|---|---|---|---|
| C-001 | Abstract | "a protocol for cryptographic custody verification" | FORMALLY_MODELED + PARTIALLY IMPLEMENTED | Protocol structure defined; pipeline partially implemented; binding OPEN | LOW — "protocol for" is aspirational, not "system that achieves" | None — acceptable framing |
| C-002 | Abstract | "transforms declared hashes from assertions into independently verified claims" | OPEN | Binding is OPEN; system does not yet transform assertions into verified claims | **HIGH** — present tense implies completed capability | **Fix: change to "is designed to transform" or "aims to transform"** |
| C-003 | Abstract | "categorical formal backbone in Lean 4, proving that verification properties compose correctly" | PROVEN | Composition theorem mechanically proven in Lean 4 | NONE | None |
| C-004 | Abstract | "AVX-512 NTT optimization achieving 9.15x speedup with 61 verification points" | EMPIRICAL | Benchmark data; stage-by-stage differential testing | LOW — speedup claim is empirical, needs benchmark methodology | Add benchmark hardware/compiler/flags details (P2 item) |
| C-005 | Abstract | "83 formal theorems and 104 test vectors" | PROVEN + EMPIRICAL | Lean 4 theorems; Rust test vectors | NONE | None |
| C-006 | Abstract | "36 boundary-level findings... either fixed, explicitly contained, or documented" | EMPIRICAL | Audit commit history; 23 fixed, 13 documented | NONE | None |
| C-007 | Abstract | "binding audit found that the proof system does not yet cryptographically bind an external artifact" | ACCURATE | Source inspection of proof request, envelope, on-chain anchor | NONE | None — honest |
| C-008 | Abstract | "a declared hash is an assertion, not a verification" | CONCEPTUAL | Protocol principle; no evidence needed | NONE | None |
| C-009 | 1.2 | "existing ZK proof systems prove computation correctness, not artifact provenance" | ACCURATE | Known distinction in ZK literature | LOW — could cite specific examples | None — well-established |
| C-010 | 1.3 | "separates artifact identity, artifact resolution, and artifact-to-computation binding into distinct predicates" | FORMALLY_MODELED | Section 1.5 defines the predicates | NONE | None |
| C-011 | 1.3 | "models custody promotion as a sequence of predicate-dependent state transitions" | FORMALLY_MODELED | Section 3.1.1 defines the state machine | NONE | None |
| C-012 | 1.3 | "mechanically verified composition theorem for certified verification bridges" | PROVEN | Lean 4 proof | NONE | None |
| C-013 | 1.3 | "formal verification of the mathematical NTT specification and conditional refinement of the AVX-512 implementation" | PROVEN + CONDITIONAL | Lean 4 proofs (math layer); 3 open axioms (machine layer) | NONE | None — accurately stated |
| C-014 | 1.4 | "ARCHER audit found that the protocol's implementations initially treated several boundary conditions... as verified when they were merely asserted" | EMPIRICAL | Findings 1, 2, 4, 27, 28 | NONE | None |
| C-015 | 1.5 | "this binding is a protocol requirement, not yet satisfied by the implementation" | ACCURATE | Binding audit (Section 4.5) | NONE | None — honest |
| C-016 | 1.5 | "The proof request takes raw trace columns as input with no artifact hash" | ACCURATE | Source: ProofRequest { job_id, col0, col1, alpha } | NONE | None |
| C-017 | 1.5 | "the proof envelope seals a claim value and proof payload with no artifact reference" | ACCURATE | Source: ProofEnvelope { version, plonky3_semver, claim: u64, payload: Vec<u8> } | NONE | None |
| C-018 | 1.5 | "the on-chain anchor commits a batch hash that is not linked to the proof" | ACCURATE | Source: TSCPAnchor.commit(bytes32 batchHash) — no proof reference | NONE | None |
| C-019 | 1.5 | "In the current implementation, only identity and resolution are established" | ACCURATE | Implementation status table (Section 3.6) | NONE | None |
| C-020 | 2.2 | "Plonky3's audited MMCS" | UNVERIFIED | We have not verified that Plonky3's MMCS has been independently audited | **MODERATE** — claims audit status of a third-party library without citation | **Fix: change to "Plonky3's MMCS" or cite the specific audit if one exists** |
| C-021 | 3.1 Stage 3 | "Cryptographically bind the artifact to its claimed computation via the predicate Bound(A, C, W)" | OPEN | Binding is OPEN — this stage does not exist in the implementation | **HIGH** — present tense "Cryptographically bind" implies it works | **Fix: change to "is designed to cryptographically bind" or add "when implemented" qualifier** |
| C-022 | 3.1 Stage 3 | "In TSCP's ZK implementation, this is a sumcheck proof combined with a FRI commitment" | PARTIALLY ACCURATE | Sumcheck prover exists; FRI prover exists in oracle-layer; but neither binds the artifact | **MODERATE** — implies the binding mechanism works | **Fix: add "Note: the current sumcheck/FRI proof proves trace properties, not artifact binding (Section 4.5)"** |
| C-023 | 3.1 Stage 3 | "The binding stage produces a proof artifact that can be independently verified" | OVERCLAIM | The proof produced does not bind the artifact; it proves trace properties | **HIGH** — "can be independently verified" implies the binding is real | **Fix: change to "The binding stage is designed to produce a proof artifact... when implemented"** |
| C-024 | 3.1 Stage 4 | "Independently verify the binding proof" | OVERCLAIM | The binding proof does not exist | **HIGH** — implies a binding proof exists to verify | **Fix: change to "independently verify the proof (when binding is implemented)" or reframe as "verify the proof's cryptographic correctness"** |
| C-025 | 3.1 Stage 4 | "All Merkle openings are checked and fold consistency is verified at each FRI round" | PARTIALLY ACCURATE | True for oracle-layer FRI; false for delta_fri_bridge (scaffold) | LOW — the oracle-layer FRI does do this | **Fix: add "(in the oracle-layer FRI implementation; the delta_fri_bridge is a scaffold)"** |
| C-026 | 3.1 | "No stage promotes an earlier stage's claim merely because it was declared successful" | FORMALLY_MODELED | Pipeline design; not yet fully tested because binding doesn't work | LOW | None — accurately qualified |
| C-027 | 3.1.1 | "The pipeline admits a formal state machine" | FORMALLY_MODELED | State machine defined in Section 3.1.1 | NONE | None |
| C-028 | 3.2 | "Composition Theorem (mechanically proven in Lean 4)" | PROVEN | Lean 4 proof | NONE | None |
| C-029 | 3.2 | "It depends on no axioms beyond the bridge certificates themselves" | PROVEN | Should verify in Lean source that no `sorry` or `axiom` declarations exist outside certificates | LOW | Verify Lean source (minor) |
| C-030 | 3.3.1 | "TSCP's prover implements a sumcheck protocol that self-verifies before emitting a proof" | IMPLEMENTED + EMPIRICAL | Source: prover with self-check in main.rs | NONE | None |
| C-031 | 3.3.1 | "The sumcheck prover is implemented. The sumcheck verifier is not implemented" | ACCURATE | Implementation status table | NONE | None |
| C-032 | 3.3.2 | "oracle-layer FRI implementation (fri_query.rs) is a working prover and verifier" | IMPLEMENTED + EMPIRICAL | Source code + tests | NONE | None |
| C-033 | 3.3.2 | "delta_fri_bridge.rs is a scaffold that unconditionally returns success" | ACCURATE | Finding 14 | NONE | None |
| C-034 | 3.3.3 | "OWSL is... an execution-environment health gate" | IMPLEMENTED | Source code | NONE | None |
| C-035 | 3.3.3 | "Fiat-Shamir challenges are derived deterministically from the transcript" | ACCURATE | Standard cryptographic fact | NONE | None |
| C-036 | 3.3.3 | "OWSL is best understood as a defense against degraded execution environments, not as a proof that low entropy breaks Fiat-Shamir soundness" | ACCURATE | Corrected per Aria's critique | NONE | None |
| C-037 | 3.4 | "Mathematical layer (fully proven)" | PROVEN | Lean 4 theorems | NONE | None |
| C-038 | 3.4 | "Machine layer (3 open axioms)... They are not proven; they are formalized as axioms backed by empirical testing" | ACCURATE | 3 axioms in Lean source; 104 test vectors | NONE | None |
| C-039 | 3.4 | "formal verification of the mathematical NTT specification and conditional refinement of the AVX-512 implementation to that specification, not as full formal verification of the AVX-512 implementation" | ACCURATE | Corrected per Aria's critique | NONE | None |
| C-040 | 3.5 | "five explicitly documented axioms" | ACCURATE | 3 machine + 2 engineering = 5 | NONE | None |
| C-041 | 4.2 | "36 findings... 23 findings were fixed; 13 were documented" | EMPIRICAL | Audit commit history | NONE | None |
| C-042 | 4.2 | "No defects were identified in the tested portions of the cryptographic core" | ACCURATE | Audit results; qualified by "tested portions" | NONE | None |
| C-043 | 4.2 | "This result does not constitute a cryptographic security proof or independent cryptanalysis" | ACCURATE | Explicit disclaimer | NONE | None |
| C-044 | 4.3 | "No fundamental cryptographic flaws were found in the tested portions of the FRI, Merkle, or Montgomery arithmetic implementations" | ACCURATE | Same as C-042 with "fundamental" qualifier | LOW | None — adequately qualified |
| C-045 | 4.3 | "The oracle-layer FRI implementation (fri_query.rs) is cryptographically sound with proper Fiat-Shamir, Merkle verification, and fold consistency checks" | OVERCLAIM | "Cryptographically sound" implies a security proof; we only verified implementation has these checks | **MODERATE** — "cryptographically sound" is a security claim | **Fix: change to "implements proper Fiat-Shamir, Merkle verification, and fold consistency checks" — drop "cryptographically sound"** |
| C-046 | 4.4 | "No external audit has been conducted. The ARCHER audit was performed by an AI agent" | ACCURATE | Fact | NONE | None |
| C-047 | 4.5 | "The proof request takes two trace columns (col0, col1) and an alpha challenge as raw input" | ACCURATE | Source: ProofRequest struct | NONE | None |
| C-048 | 4.5 | "The proof envelope seals a claim value (currently the Fibonacci terminal: 294373)" | ACCURATE | Source: ProofEnvelope struct | NONE | None |
| C-049 | 4.5 | "The batch hash is not cryptographically linked to the proof" | ACCURATE | Source: TSCPAnchor.sol | NONE | None |
| C-050 | 4.5 | "H(A) = h does NOT imply A = Output(C)" | ACCURATE | Binding audit conclusion | NONE | None |
| C-051 | 5.1 | "A promoted artifact has passed the specified custody predicates" | FORMALLY_MODELED | Pipeline design; conditional on implementation | LOW — conditional on complete implementation | **Fix: add "when all stages are implemented" qualifier** |
| C-052 | 5.1 | "Artifact identity is established over canonical bytes" | IMPLEMENTED | P0 baseline canonicalization | NONE | None |
| C-053 | 5.1 | "The binding proof is independently checked against a reconstructed Fiat-Shamir transcript" | OVERCLAIM | The binding proof does not exist | **HIGH** — present tense claims a capability that doesn't exist | **Fix: change to "When implemented, the binding proof will be independently checked"** |
| C-054 | 5.1 | "Failed custody predicates cannot result in promotion" | FORMALLY_MODELED | State machine design; pipeline is fail-closed | LOW — not yet tested end-to-end | None — design claim, not implementation claim |
| C-055 | 5.1 | "Certified bridges preserve the specified admissibility, preservation, and reflection properties" | PROVEN | Composition theorem | NONE | None |
| C-056 | 5.2 | All 8 non-claims | ACCURATE | Each is verified against implementation status | NONE | None |
| C-057 | 6.2 | "SLSA establishes provenance claims through trusted build platforms; TSCP seeks to establish a stronger binding" | ACCURATE | Corrected per Aria's critique | NONE | None |
| C-058 | 7.1 | "The custody pipeline is a new primitive" | CONCEPTUAL | Novelty claim; not formally proven to be novel | LOW — "new" is a reasonable claim for a protocol paper | None — standard academic framing |
| C-059 | 7.3 | "An AI audit yielding a finding is not equivalent to an independent audit yielding a verified security property" | ACCURATE | Protocol principle applied to itself | NONE | None |
| C-060 | 8 | "Making this gap visible — rather than assuming it is implicitly satisfied — is the protocol's first application of its own principle" | CONCEPTUAL | Reasonable meta-observation | NONE | None |
| C-061 | 8 | "Establish artifact-to-proof binding — commit the artifact hash as a public input in the proof statement" | OPEN | Future work item | NONE | None |

---

## Summary: Overclaim Risk Distribution

| Risk Level | Count | Claim IDs |
|---|---|---|
| HIGH | 4 | C-002, C-021, C-023, C-024, C-053 |
| MODERATE | 3 | C-020, C-022, C-045 |
| LOW | 7 | C-001, C-004, C-025, C-029, C-044, C-051, C-054, C-058 |
| NONE | 47 | All others |

## Required Fixes (HIGH risk — must fix before submission)

1. **C-002 (Abstract):** "transforms declared hashes from assertions into independently verified claims" → "is designed to transform declared hashes from assertions into independently verified claims"

2. **C-021 (Section 3.1, Stage 3):** "Cryptographically bind the artifact" → "is designed to cryptographically bind the artifact (implementation pending, Section 4.5)"

3. **C-023 (Section 3.1, Stage 3):** "The binding stage produces a proof artifact that can be independently verified" → "The binding stage is designed to produce a proof artifact that can be independently verified (not yet implemented)"

4. **C-024 (Section 3.1, Stage 4):** "Independently verify the binding proof" → "Independently verify the proof's cryptographic correctness (when binding is implemented, verify the binding proof)"

5. **C-053 (Section 5.1, Claim 3):** "The binding proof is independently checked" → "When implemented, the binding proof will be independently checked"

## Required Fixes (MODERATE risk — should fix before submission)

6. **C-020 (Section 2.2):** "Plonky3's audited MMCS" → "Plonky3's MMCS" (unless we can cite a specific audit)

7. **C-022 (Section 3.1, Stage 3):** Add qualifier: "Note: the current sumcheck/FRI proof proves trace properties, not artifact binding (Section 4.5)"

8. **C-025 (Section 3.1, Stage 4):** Add qualifier: "(in the oracle-layer FRI implementation; the delta_fri_bridge is a scaffold)"

9. **C-045 (Section 4.3):** "is cryptographically sound with proper Fiat-Shamir..." → "implements proper Fiat-Shamir, Merkle verification, and fold consistency checks" (drop "cryptographically sound")

## Status: FORMALLY_MODELED additions

The following structures should be explicitly labeled as FORMALLY_MODELED in the paper:

- Custody state machine (Section 3.1.1) — already described as "formal state machine" ✓
- Bound(A, C, W) predicate (Section 1.5) — already stated as "protocol requirement, not yet satisfied" ✓
- Promotion invariant (Section 3.1.1) — already described as formal ✓

No additional FORMALLY_MODELED labels needed — the paper already handles these correctly.
