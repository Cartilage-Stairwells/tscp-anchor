/-
  TSCP Formal — Provenance Separation Laws
  Eight separation propositions derived from B7 empirical witnesses (D1-D8).

  These are semantic propositions about the ontology, NOT correspondence
  claims about the Rust implementation. They express the anti-laundering
  laws that prevent semantic confusion in the provenance system.

  Each law is stated as a separation: two things that must not collapse.
  Some are provable from the definitions in Ontology.lean; others require
  additional case analysis and are marked with `sorry`.

  The B7 witnesses are concrete instances, not universal theorems —
  unless the definitions actually justify universality.
-/

import TSCP.Formal.Provenance.Ontology

namespace TSCP.Formal.Provenance.SeparationLaws

open TSCP.Formal.Provenance.Ontology

-- ===================================================================
-- D1: Evidence scope boundary
-- The same receipt supports the kernel claim but not the prover claim.
-- ===================================================================

/--
D1 witness: EVIDENCE(C_kernel, E, W) holds but EVIDENCE(C_prover, E, W) does not.

The kernel claim has scope "experiment_a.isolated_kernel" which is within
the window's semantic_scope. The prover claim has scope "experiment_a"
which is broader and NOT within the window.
-/
theorem d1_evidence_scope_boundary :
    EvidencePred b7_kernel_claim b7_experiment_a_evidence b7_experiment_a_window
    ∧ ¬ EvidencePred b7_prover_claim b7_experiment_a_evidence b7_experiment_a_window := by
  unfold EvidencePred b7_kernel_claim b7_prover_claim b7_experiment_a_window
    b7_experiment_a_evidence
  simp [scopeSubset, String.isPrefixOf]
  -- "experiment_a.isolated_kernel" starts with "experiment_a.isolated_kernel" ✓
  -- "experiment_a" does NOT start with "experiment_a.isolated_kernel" ✗
  constructor
  · -- Kernel claim is in scope
    refine ⟨?_, ?_, ?_⟩
    · decide  -- scopeSubset "experiment_a.isolated_kernel" "experiment_a.isolated_kernel"
    · decide  -- receipt not empty
    · decide  -- commit ≠ "uncommitted"
  · -- Prover claim is NOT in scope — negation
    intro ⟨h1, h2, h3⟩
    -- "experiment_a" is not a prefix of "experiment_a.isolated_kernel"
    -- so scopeSubset should be false
    simp [scopeSubset] at h1
    -- Actually we need to check: does "experiment_a.isolated_kernel".isPrefixOf "experiment_a"?
    -- The window scope is "experiment_a.isolated_kernel" and claim scope is "experiment_a"
    -- scopeSubset claim scope window scope = isPrefixOf(window_scope, claim_scope)
    -- = isPrefixOf("experiment_a.isolated_kernel", "experiment_a") = false
    sorry  -- need to verify String.isPrefixOf semantics

-- ===================================================================
-- D2: Dependency ≠ call edge
-- ===================================================================

/--
D2 witness: PROPER(DependsOn edge) = TRUE but PROPER(DerivesFrom, no lineage) = FALSE.

A crate dependency is a valid relationship (DependsOn is proper).
But DerivesFrom without a lineage chain is improper — no derivation established.
-/
theorem d2_dependency_not_call_edge :
    Proper b7_dependency_edge ∧ ¬ Proper b7_improper_derives_edge := by
  unfold Proper b7_dependency_edge b7_improper_derives_edge
  simp [RelationshipKind.dependsOn, RelationshipKind.derivesFrom]
  constructor
  · trivial  -- DependsOn → True
  · -- ¬ (False) — DerivesFrom with has_lineage=false → false
    intro h
    exact h  -- derivesFrom with has_lineage=false gives False

-- ===================================================================
-- D3: Cannot traverse absent edge
-- ===================================================================

/--
D3 witness: FOLLOW(upgrade_edge, frozen_graph) fails because the edge
is not in the graph.

The frozen graph has no edges. The upgrade edge would require a new
experiment. FOLLOW cannot construct edges — it can only traverse
existing admissible edges.
-/
theorem d3_cannot_traverse_absent_edge :
    ¬ Follow b7_improper_derives_edge { edges := [] } := by
  unfold Follow
  simp [List.mem]
  intro h
  exact h.elim

-- ===================================================================
-- D4: SHARP anti-widening
-- ===================================================================

/--
D4 witness: If SHARP produces C' from C, then scope(C') ⊆ scope(C).

This is the anti-widening property: a transformation may narrow
a claim but may not broaden it. B3 HOLD→PASS added scope containment.
-/
theorem d4_sharp_anti_widening
    (claim : Claim) (ev : Evidence) (r : Relationship) (claim' : Claim)
    (h : Sharp claim ev r claim') :
    antiWidening claim.scope claim'.scope := by
  unfold Sharp at h
  exact h.left

-- ===================================================================
-- D5: SHARK rejects scope widening
-- ===================================================================

/--
D5 witness: SHARK rejects a transformation that widens scope.

scalar_pass has scope "b3.arithmetic_correspondence.scalar"
instruction_equivalent has scope "b3.arithmetic_correspondence" (broader)

The scope widened (dropped ".scalar"), so SHARK = FALSE.
-/
theorem d5_shark_rejects_widening :
    ¬ Shark { proposition := "scalar PASS", scope := "b3.arithmetic_correspondence.scalar", frozen := true }
             { proposition := "instruction-equivalent", scope := "b3.arithmetic_correspondence", frozen := false } := by
  unfold Shark antiWidening scopeSubset
  -- antiWidening old new = scopeSubset new old = isPrefixOf(old, new)
  -- = isPrefixOf("b3.arithmetic_correspondence.scalar", "b3.arithmetic_correspondence")
  -- = false (the longer string is not a prefix of the shorter)
  sorry  -- need to verify String.isPrefixOf direction

-- ===================================================================
-- D6: Observed ≠ Admissible
-- ===================================================================

/--
D6 witness: Evidence can be observed without being admissible.

The 5,504 fused calls have a receipt but are from an uncommitted state.
EVIDENCE checks the committed state and rejects uncommitted evidence.
-/
theorem d6_observed_not_admissible :
    -- The uncommitted evidence has a receipt (observed)
    ¬ b7_uncommitted_evidence.receipt.isEmpty
    -- But it is not admissible (commit = "uncommitted")
    ∧ ¬ EvidencePred b7_kernel_claim b7_uncommitted_evidence b7_experiment_a_window := by
  unfold EvidencePred b7_uncommitted_evidence b7_kernel_claim b7_experiment_a_window
  simp
  constructor
  · decide  -- "sha256:5504_calls" is not empty
  · -- commit = "uncommitted" so the third conjunct fails
    intro h
    exact absurd h.right (by decide)

-- ===================================================================
-- D7: Capability ≠ reachability
-- ===================================================================

/--
D7 witness: A DerivesFrom edge without lineage is improper,
which means capability does not imply reachability through that edge.

The adapter implementing a trait (capability) does not establish
a derivation to the prover (reachability) without a call chain.
-/
theorem d7_capability_not_reachability :
    ¬ Proper { kind := RelationshipKind.derivesFrom, has_lineage := false,
               boundary := "capability implies reachability?" } := by
  unfold Proper
  simp [RelationshipKind.derivesFrom]
  intro h
  exact h  -- derivesFrom with has_lineage=false → False

-- ===================================================================
-- D8: Relationship ≠ Proper(Relationship)
-- ===================================================================

/--
D8 witness: A Relationship is an object that EXISTS independent of
its PROPER evaluation. The edge can be constructed and inspected
even when PROPER returns false.

This is expressed by the fact that we can construct an improper
relationship (it exists as a value) and separately evaluate it.
-/
theorem d8_relationship_exists_independent_of_evaluation :
    -- The improper derives edge exists as a constructible value
    ¬ Proper b7_improper_derives_edge
    -- And it is still a valid Relationship (can be constructed)
    ∧ b7_improper_derives_edge.kind = RelationshipKind.derivesFrom := by
  unfold Proper b7_improper_derives_edge
  simp [RelationshipKind.derivesFrom]
  constructor
  · intro h; exact h
  · rfl

end TSCP.Formal.Provenance.SeparationLaws
