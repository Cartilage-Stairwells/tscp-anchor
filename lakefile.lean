import Lake
open Lake DSL

package "oracle-layer" where
  leanOptions := #[
    ⟨`pp.unicode.fun, true⟩,
    ⟨`pp.proofs.withType, false⟩
  ]

lean_lib `TraceCoreProver where
  roots := #["TraceCoreProver"]

require mathlib from git "https://github.com/leanprover-community/mathlib4" @ "master"
