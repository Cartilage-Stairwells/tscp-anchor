/-
  TSCP Formal — Bridge Preservation Theorem
  ==========================================

  Commit 1 of the minimal verified path.

  Goal: prove the generic invariant — a certified proof crossing a
  certified bridge remains admissible.

  Acceptance criteria (all met):
    - no new assumptions introduced
    - no incomplete proofs
    - no classical escape hatch
    - theorem closes using the certificate interface alone

  This proves the backbone is compositional.
-/

import TSCP.Formal.TSCP_Formal_Backbone

namespace TSCP.Formal

/-- The central preservation theorem.

    If a bridge has a valid certificate, and a proof is certified (kernel-admitted)
    in the source universe, then the transported proof is admissible (true) in
    the target universe.

    This closes by definition unfolding: `cert.proof_preservation` already
    states exactly `∀ p, source admits p → target admits (bridge.proof_map p)`,
    and `Truth U p` is defined as `U.proof_kernel.admits_proof p`.

    The certificate interface is sufficient. -/
theorem bridge_preserves_certified_truth
    {U V : Universe} (f : Bridge U V) (cert : BridgeCertificate f)
    (cp : CertifiedProof U.proof_kernel) :
    Truth V (f.proof_map cp.proof) :=
  cert.proof_preservation cp.proof cp.certified

/-- The transport function: maps a certified proof across a certified bridge,
    carrying the evidence records with it. -/
def transport_certified_proof
    {U V : Universe} (f : Bridge U V) (cert : BridgeCertificate f)
    (cp : CertifiedProof U.proof_kernel) :
    CertifiedProof V.proof_kernel :=
  { proof := f.proof_map cp.proof
  , certified := cert.proof_preservation cp.proof cp.certified
  , evidence := cp.evidence
  }

/-- The transport preserves truth (stated via the transport function). -/
theorem transport_preserves_truth
    {U V : Universe} (f : Bridge U V) (cert : BridgeCertificate f)
    (cp : CertifiedProof U.proof_kernel) :
    Truth V (transport_certified_proof f cert cp).proof :=
  (transport_certified_proof f cert cp).certified

end TSCP.Formal
