import Lake
open Lake DSL

package «tscp-formal» where

require mathlib from git "https://github.com/leanprover-community/mathlib4.git"

lean_lib «TSCP» where
  roots := #[`TSCP.Formal.TSCP_Formal_Backbone, `TSCP.Formal.BridgePreservation,
    `TSCP.Formal.Examples.PropositionalKernel, `TSCP.Formal.Examples.NormalizationBridge,
    `TSCP.Formal.Evidence.ManifestBinding, `TSCP.Formal.Core, `TSCP.Formal.Montgomery,
    `TSCP.Formal.ReviewerSemantics, `TSCP.Formal.Butterfly, `TSCP.Formal.NTTStage,
    `TSCP.Formal.Provenance.Ontology, `TSCP.Formal.Provenance.SeparationLaws,
    `TSCP.Formal.Provenance.Composition]
