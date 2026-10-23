# TSCP SNARK-Wrapper Deployment Path v1.0

**Document:** TSCP_SNARK_WRAPPER_DEPLOYMENT_PATH_v1.0  
**Date:** 2026-10-07  
**Status:** OPTIONAL deployment path — DRAFTED discussion only  
**Relation to native stack:** **Separate.** Does not redefine the native transparent (STARK-family / FRI) evidence object used by TSCP Anchor.

---

## 1. Purpose

Record an optional deployment pattern in which a **STARK-family proof** (or other transparent IOP proof) produced by the native TSCP proving path is **recursively wrapped** into a smaller **SNARK** for environments where on-chain verifier cost or proof size is the dominant constraint.

This document:

- Does **not** change the native Anchor stack (FRI, sumcheck, DEEP-ALI, Poseidon2, Plonky3).
- Does **not** claim that a SNARK wrapper is implemented in `tscp-anchor`.
- Does **not** override VoxCanon S3 constitutional failures.
- Does **not** grant Authority to any proof object.

---

## 2. Motivation

| Concern | Native STARK-family proof | SNARK-wrapped form |
|---------|---------------------------|---------------------|
| Trusted setup | None (transparent) | May introduce setup (trusted, universal, or transparent SNARK variant) |
| Proof size | Larger | Often much smaller |
| On-chain verify cost | Higher | Often lower |
| Post-quantum posture | Stronger hash-based story | Depends on SNARK family (pairing-based ≠ PQ) |
| Audit surface | FRI + AIR + transcript | Native surface **plus** SNARK circuit / setup |

Use a wrapper only when deployment constraints require it. Prefer native transparent verification when setup avoidance and PQ framing dominate.

---

## 3. Architectural placement

```
Native TSCP Anchor path (unchanged)
  oracle-layer / commitment / prover-server
        │
        │  STARK-family proof P_stark
        ▼
[Optional] SNARK wrapper circuit
  Statement: "P_stark verifies under public parameters PP_stark
              for public inputs X"
        │
        │  SNARK proof P_snark
        ▼
Deployment verifier (e.g. constrained on-chain environment)
```

**Rules**

1. `P_stark` remains the **primary evidence object** for TSCP admission discussions unless a deployment contract explicitly accepts `P_snark` as a proxy under stated soundness assumptions.  
2. Wrapper failure does not repair an invalid `P_stark`.  
3. Wrapper success does not clear a VoxCanon S3 plane violation on the underlying artifact.  
4. Named-role strings must not appear in wrapper circuit identifiers that enter hash preimages.

---

## 4. Soundness and trust obligations (DRAFTED checklist)

Before any production use of a wrapper path, the following must be explicitly evidenced (not claimed by this document):

| Obligation | Notes |
|------------|--------|
| Verifier equivalence | Proof that accept(`P_snark`) implies accept(`P_stark`) under stated assumptions |
| Setup policy | Trusted / universal / transparent — disclosed |
| Parameter binding | Public inputs bind to the same artifact hashes / statements as native verify |
| Version gates | Mixed-version rejection (native envelope vs wrapper) documented |
| Audit | Native stack audit **and** wrapper circuit audit treated as separate scopes |
| PQ disclosure | If wrapper is pairing-based, PQ posture of the *deployment path* is weaker than native FRI |

---

## 5. Interaction with drafted protocols

| Protocol | Interaction |
|----------|-------------|
| VoxCanon exception protocol | S2/S3 alignment failures still reject; wrapper cannot override |
| VexScale telemetry | May record `fri_verify_latency_ms` and separately `snark_wrapper_verify_latency_ms` with distinct metric names and claim_class |
| FCO / AdmittedEvidence | Wrapper proof may be referenced as evidence; Authority remains 0 for evidence objects |

---

## 6. Explicit non-claims

- No SNARK wrapper implementation is asserted to exist in the inspected public `tscp-anchor` tree.  
- No end-to-end prover acceleration claim is made.  
- No recommendation that all deployments must use a SNARK wrapper.  
- No substitution of SNARK setup trust for TSCP custody discipline.

---

## 7. When to keep this path unused

Prefer **not** to deploy a wrapper when:

1. Verifier environment can afford native proof size and verify time.  
2. Trusted setup is unacceptable.  
3. Post-quantum posture is a hard requirement and the candidate SNARK is not PQ.  
4. Audit budget cannot cover the additional circuit surface.

---

## 8. Evidence boundary

| Item | Label |
|------|-------|
| This deployment-path text | OBSERVED (this file) as optional DRAFTED path |
| Implementation in tscp-anchor | NOT_FOUND_IN_INSPECTED_SCOPE |
| Production readiness | Not claimed |

---

*End of TSCP_SNARK_WRAPPER_DEPLOYMENT_PATH_v1.0*
