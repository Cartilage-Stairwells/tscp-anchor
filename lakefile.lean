import Lake
open Lake DSL

package «tscp-formal» where

lean_lib «TSCP» where
  roots := #[`TSCP.Formal.TSCP_Formal_Backbone, `TSCP.Formal.BridgePreservation,
    `TSCP.Formal.Examples.PropositionalKernel, `TSCP.Formal.Examples.NormalizationBridge,
    `TSCP.Formal.Evidence.ManifestBinding, `TSCP.Formal.Core]
