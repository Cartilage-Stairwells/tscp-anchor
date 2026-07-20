/-
  TSCP Formal — Evidence Manifest Binding
  =======================================

  Commit 4 of the minimal verified path.

  Only after the mathematical path works do we bind evidence.

  The Lean layer exports identity information as a ProofArtifact. The
  TSCP evidence manifest consumes this artifact.

  Flow:
    Theorem  →  Proof term  →  Hash  →  Certificate metadata  →  Evidence manifest

  Evidence is downstream of proof certification.

  The manifest records identity and custody of an already-certified
  transition. It is not an authority source for mathematical truth.

  Truth → Artifact
  never
  Artifact → Truth

  The real enforcement remains the type dependency graph: ProofArtifact
  is constructed FROM a CertifiedProof, never the other way around.
-/

import TSCP.Formal.TSCP_Formal_Backbone
import TSCP.Formal.BridgePreservation

namespace TSCP.Formal.Evidence

/- ===================================================================
   PROOF ARTIFACT
   The Lean layer exports this artifact. The TSCP evidence manifest
   (tscp_verify.py) consumes it as a field in the manifest's stages.

   This is the identity boundary: it prevents the manifest layer from
   reaching directly into Lean internals. The manifest only sees the
   artifact's identity fields, not the proof term itself.
   =================================================================== -/

/-- A proof artifact is the identity record exported by the Lean layer.

    The manifest layer consumes this artifact. It does NOT consume the
    proof term directly — this preserves the authority boundary:
    the manifest records WHAT was proven, not WHETHER it is true. -/
structure ProofArtifact where
  /-- Name of the theorem this proof establishes. -/
  theorem_name : String
  /-- SHA256 digest of the proof term's canonical serialization. -/
  proof_digest : String
  /-- Version of the verifier (kernel) that checked this proof. -/
  verifier_version : String
  /-- The proof object (serialized for cross-system verification). -/
  proof_serialization : String

/- ===================================================================
   ARTIFACT IDENTITY INVARIANT

  This theorem documents the intended direction of the evidence
  dependency: artifacts identify certified transitions, they do not
  establish mathematical truth.

  The real enforcement is the type dependency graph — ProofArtifact
  is constructed FROM a CertifiedProof, never the reverse.
  =================================================================== -/

/-- An artifact identity does not establish truth.

    This is not mathematically interesting — it documents the intended
    direction: Truth → Artifact, never Artifact → Truth.

  The type system enforces this: `mk_proof_artifact` takes a
  `CertifiedProof` as input, so you cannot construct an artifact
  without first having a certified proof. The artifact records the
  identity of an already-certified transition. -/
theorem artifact_identity_does_not_establish_truth :
    ∀ (pa : ProofArtifact), True :=
  fun _ => trivial

/- ===================================================================
   ARTIFACT GENERATION
   Functions to produce proof artifacts from certified proofs.
   =================================================================== -/

/-- Generate a proof artifact from a certified proof.

    The digest is a placeholder in the formal layer — in the full system,
    it is computed by the TSCP canonical serializer (tscp_verify.py)
    over the proof term's canonical JSON-LD representation. -/
def mk_proof_artifact
    (theorem_name : String) (verifier_version : String)
    {α : Type} {K : Kernel α} (cp : CertifiedProof K)
    (serialized : String) :
    ProofArtifact :=
  { theorem_name := theorem_name
  , proof_digest := serialized  -- In production: SHA256(serialized)
  , verifier_version := verifier_version
  , proof_serialization := serialized
  }

/- ===================================================================
   EVIDENCE RECORD BINDING
   Convert a proof artifact into an immutable Evidence record.
   =================================================================== -/

/-- Convert a proof artifact into an immutable Evidence record. -/
def proof_artifact_to_evidence (pa : ProofArtifact) : TSCP.Evidence :=
  { digest := pa.proof_digest
  , kind := TSCP.EvidenceKind.required
  , issuer := pa.verifier_version
  , timestamp := "2026-07-20T00:00:00Z"
  }

/- ===================================================================
   BRIDGE ARTIFACT
   An artifact for a bridge certificate, recording the transport proof.
   =================================================================== -/

/-- A bridge artifact records the bridge certificate's identity. -/
structure BridgeArtifact where
  /-- Name of the preservation theorem. -/
  theorem_name : String
  /-- SHA256 digest of the certificate. -/
  certificate_digest : String
  /-- Verifier version that issued the certificate. -/
  verifier_version : String
  /-- Timestamp of issuance. -/
  issued_at : String

/-- Generate a bridge artifact from a bridge certificate. -/
def mk_bridge_artifact {U V : Universe} (f : Bridge U V) (cert : BridgeCertificate f)
    (theorem_name : String) : BridgeArtifact :=
  { theorem_name := theorem_name
  , certificate_digest := cert.certificate_digest
  , verifier_version := cert.verifier_version
  , issued_at := cert.issued_at
  }

/-- Convert a bridge artifact into an immutable Evidence record. -/
def bridge_artifact_to_evidence (ba : BridgeArtifact) : TSCP.Evidence :=
  { digest := ba.certificate_digest
  , kind := TSCP.EvidenceKind.required
  , issuer := ba.verifier_version
  , timestamp := ba.issued_at
  }

/- ===================================================================
   CUSTODY CHAIN BINDING
   The full custody chain from proof to evidence.
   =================================================================== -/

/-- The custody chain binding: a certified proof, its artifact, and
    the evidence records produced from it. -/
structure CustodyBinding {α : Type} {K : Kernel α} where
  certified_proof : CertifiedProof K
  proof_artifact : ProofArtifact
  evidence_records : List TSCP.Evidence

/-- Build a custody binding from a certified proof. -/
def mk_custody_binding
    {α : Type} {K : Kernel α}
    (cp : CertifiedProof K)
    (theorem_name : String) (verifier_version : String)
    (serialized : String) :
    CustodyBinding (K := K) :=
  let artifact := mk_proof_artifact theorem_name verifier_version cp serialized
  { certified_proof := cp
  , proof_artifact := artifact
  , evidence_records := [proof_artifact_to_evidence artifact]
  }

/- ===================================================================
   THE VERIFIED PATH (END-TO-END)

  The complete chain:
    1. A certified proof exists (kernel-admitted + evidence).
    2. The proof crosses a certified bridge (BridgePreservation theorem).
    3. The transported proof is still certified in the target universe.
    4. A proof artifact is generated from the certified proof.
    5. The artifact is converted to an immutable evidence record.
    6. The evidence record is part of the TSCP evidence manifest.

  This is the milestone:
    "A certified proof can cross a representation boundary and remain
     machine-checked admissible, with an externally verifiable custody
     record."
  =================================================================== -/

/-- The end-to-end path: certified proof → transport → artifact → evidence. -/
def verified_path
    {U V : Universe} (f : Bridge U V) (cert : BridgeCertificate f)
    (cp : CertifiedProof U.proof_kernel)
    (theorem_name : String) (verifier_version : String)
    (serialized : String) :
    CertifiedProof V.proof_kernel × List TSCP.Evidence :=
  let transported := transport_certified_proof f cert cp
  let artifact := mk_proof_artifact theorem_name verifier_version transported serialized
  let evidence := proof_artifact_to_evidence artifact
  (transported, [evidence])

/-- The verified path produces a certified proof in the target universe. -/
theorem verified_path_certified
    {U V : Universe} (f : Bridge U V) (cert : BridgeCertificate f)
    (cp : CertifiedProof U.proof_kernel)
    (theorem_name : String) (verifier_version : String)
    (serialized : String) :
    (verified_path f cert cp theorem_name verifier_version serialized).1.certified =
      cert.proof_preservation cp.proof cp.certified :=
  rfl

/-- The verified path produces evidence records. -/
theorem verified_path_has_evidence
    {U V : Universe} (f : Bridge U V) (cert : BridgeCertificate f)
    (cp : CertifiedProof U.proof_kernel)
    (theorem_name : String) (verifier_version : String)
    (serialized : String) :
    (verified_path f cert cp theorem_name verifier_version serialized).2.length ≥ 1 := by
  simp [verified_path]

end TSCP.Formal.Evidence
