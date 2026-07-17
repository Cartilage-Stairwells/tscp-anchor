# tscp-anchor

The **Triune Structured Codex Protocol (TSCP)** is a multi-agent coordination standard that governs state transitions, governance events, and oracle-layer operations across a sovereign AI execution environment. **TSCP Anchor** is its cryptographic proving layer — the component responsible for making every protocol claim independently verifiable.

It implements a zero-knowledge proving stack built on [Plonky3](https://github.com/Plonky3/Plonky3), providing integrity guarantees grounded in cryptographic proof rather than trust in any single party or system.

The system is designed around a principle of *sovereign verifiability*: every claim the protocol makes about its own state can be independently verified by any party with access to the proof and the public parameters, without trusting the prover.

---

## What This Is

tscp-anchor provides three things:

1. **A ZK proving stack** — FRI (Fast Reed-Solomon IOP)-based transparent commitment scheme designed for STARK-style proof systems, sumcheck protocol, DEEP-ALI (Domain Extension for Eliminating Pretenders via Algebraic Linking Identity) constraint verification, and Poseidon2-based Fiat-Shamir transcript, all implemented in Rust on top of Plonky3 0.6.1.

2. **An on-chain anchor** — `TSCPAnchor.sol`, an immutable Solidity registry that records keccak256 hashes of TSCP artifacts on-chain. Deployed on Sepolia at `0x6FDB70F31E4815bE866Fd6aDD32802f90F9B5E06`.

3. **A migration protocol** — a versioned upgrade framework (`ProofEnvelope`, `upgrade_driver.sh`) that governs transitions between Plonky3 versions with golden corpus validation, mixed-version rejection gates, and rollback manifests.

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   prover-server                     │
│  HTTP endpoint (/prove/sumcheck)                    │
│  Sumcheck prover + verifier                         │
│  ProofEnvelope versioning                           │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────┐
│                  oracle-layer                       │
│  FRI commit / query / verify                        │
│  DEEP-ALI constraint checking                       │
│  Multilinear oracle (MleOracle trait)               │
│  Poseidon2 Fiat-Shamir transcript                   │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────┐
│                 batch-merkle                        │
│  Plonky3 BatchMerkle primitives                     │
│  Tamper-evident commitment layer                    │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────┐
│                  commitment                         │
│  TSCP PCS (polynomial commitment scheme)            │
│  Wires DFT + MMCS + FRI params                     │
└─────────────────────────────────────────────────────┘
```

**Supporting systems:**

- **OWSL** (Oracle Witness Status Layer) — operational witness-status daemon used for runtime health gating and execution policy. Writes atomic status to `~/.tscp/owsl_status.json` every 30 seconds. Rust bridge in `src/owsl_bridge.rs`.
- **TSCPAnchor.sol** — Solidity contract for on-chain artifact registration.
- **Lean 4 kernel** (`TraceCoreProver.lean`) — formal semantics layer defining the Mazurkiewicz trace monoid and categorical compiler functor that underpin the protocol's theoretical guarantees.

---

## Soundness Properties

The proving stack enforces the following soundness invariants, each covered by adversarial tests:

| Property | Location | Test |
|---|---|---|
| FRI query: tampered folded value rejected | `oracle-layer` | `tampered_folded_value_fails_verification` |
| FRI query: wrong beta rejected | `oracle-layer` | `wrong_beta_fails_verification` |
| FRI query: forged opening rejected | `oracle-layer` | `forged_opening_with_wrong_root_fails_even_with_correct_arithmetic` |
| Sumcheck: wrong claimed sum rejected | `prover-server` | `wrong_claimed_sum_fails_verification` |
| Sumcheck: tampered round message rejected | `prover-server` | `tampered_round_message_fails_verification` |
| Sumcheck: final-binding gap closed | `prover-server` | `honest_proof_verifies` |
| DEEP-ALI: real trace enforced | `oracle-layer` | `oracle_lift_end_to_end` |
| BatchMerkle: tampered leaf rejected | `batch-merkle` | `tampered_proof_fails_verification` |
| Version gate: 0.6.2 envelope rejected by 0.6.1 verifier | `prover-server` | `test_062_rejected_by_061_verifier` |

---

## Current State

**Baseline:** `tscp-freeze-0.6.1` (tag)

The repository is frozen at Plonky3 0.6.1 pending availability of 0.6.2 on crates.io. All currently identified implementation soundness issues covered by the test suite have been addressed. The test suite passes cleanly with zero test failures.

```
cargo test --workspace
# 44 tests, 0 failures
```

**Migration trigger:** When Plonky3 0.6.2 ships, the upgrade sequence is:

```bash
git status && git tag -n && cargo test --workspace  # confirm baseline
# then begin migration on master via upgrade_driver.sh
```

Golden corpus artifacts for 0.6.1 are archived in `corpus/golden-0.6.1/` with SHA256 checksums under tag `golden-corpus-0.6.1`.

---

## Crate Structure

| Crate | Purpose |
|---|---|
| `oracle-layer` | FRI, DEEP-ALI, MLE oracle, Fiat-Shamir transcript |
| `batch-merkle` | Plonky3 BatchMerkle wrapper |
| `commitment` | TSCP polynomial commitment scheme |
| `prover-server` | HTTP proving service, sumcheck, ProofEnvelope |

---

## Build Environment

Validated build environment:

```
rustc 1.96.0 (ac68faa20 2026-05-25)
cargo 1.96.0 (30a34c682 2026-05-25)
```

```bash
cargo test --workspace  # 44 tests, 0 failures
```

---

## Running the Prover Server

```bash
cd crates/prover-server
cargo run --release
# Listening on 127.0.0.1:3030

# Submit a sumcheck proof request:
curl -X POST http://localhost:3030/prove/sumcheck \
  -H "Content-Type: application/json" \
  -d '{"job_id": "test-1", "col0": [1,2,3,4], "col1": [5,6,7,8], "alpha": 42}'
```

---

## On-Chain Deployment

`TSCPAnchor.sol` is deployed on Sepolia testnet:

```
Address: 0x6FDB70F31E4815bE866Fd6aDD32802f90F9B5E06
Network: Sepolia
```

Verifiable independently via any Sepolia block explorer. This is a development deployment; mainnet hardening and external audit are prerequisites for production use.

Deployment transaction hashes (from `manifest.json`):
```
0x41a7960e6f0b4d241b79320d4a736d24bc31fca8296701a88b92e0dc6749eb78
0x126b44d63d4a28eae731398e1890954cddf3a6e2bf3ce7fc456c7a520ec1927a
```

---

## OWSL Daemon

The Oracle Witness Status Layer is an operational health-gating component. It monitors configured runtime conditions and gates verification workflows accordingly.

```bash
# Bootstrap (write initial status):
python3 src/owsl_ipc.py test

# Check current status:
python3 src/owsl_ipc.py status

# Run as daemon (termux-services):
# Service installed at $SVDIR/owsl
sv status owsl
```

Status is written atomically to `~/.tscp/owsl_status.json`. The Rust bridge (`src/owsl_bridge.rs`) enforces configured execution policy based on the current status.

---

## Migration Protocol

Version transitions are governed by a three-phase protocol:

- **Phase A** — Archive golden corpus for the current version, compute SHA256 checksums, tag.
- **Phase B** — Add versioned `ProofEnvelope`, implement mixed-version rejection gate, test.
- **Phase C** — Run `upgrade_driver.sh` against the new version, validate benchmarks, retag.

Rollback manifests for the 0.6.1 → 0.6.2 transition are in `migration-backups/plonky3-0.6.2-preflight/`.

---

## Security & Audit Status

An external cryptographic audit has not been performed. The soundness properties listed above are covered by adversarial tests within the repository but have not been independently verified by a third party.

Scope of current security coverage:
- Implementation-level soundness: verified via test suite
- Protocol-level soundness: not formally proven
- Build environment: not independently reproduced
- Operational security (key management, deployment hardening): not reviewed

This repository is suitable for research evaluation, technical due diligence, and collaborator review. It is not suitable for production deployment without independent audit and operational hardening.

---

## Current Limitations

- Plonky3 version is pinned to 0.6.1 pending validated migration to 0.6.2.
- External cryptographic audit has not been performed.
- Sepolia deployment is for development verification only.
- Production deployment requires independent review and operational hardening.

---

## Status

| Component | Status |
|---|---|
| FRI prove/verify | Complete |
| Sumcheck (full soundness) | Complete |
| DEEP-ALI | Complete |
| Poseidon2 Fiat-Shamir | Complete |
| BatchMerkle | Complete (using Plonky3 primitives) |
| OWSL daemon | Running |
| On-chain anchor (Sepolia) | Deployed |
| External audit | Not yet performed |
| Mainnet deployment | Pending audit |

---

## Related

- [TSCP Specification](tscp-docs/) — protocol governance documents
- [Lean 4 kernel](TraceCoreProver.lean) — formal semantics
- [Triune-Oracle](https://github.com/Triune-Oracle) — parent organization
