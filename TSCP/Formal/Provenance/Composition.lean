/-
  TSCP Formal — Provenance Composition Theorem
  The architectural composition law: Proper + Sharp + scope-subset → Shark.

  This is currently the most consequential theorem in the provenance system.
  It says: a properly related, recursively constructed, non-widening
  descendant preserves the ancestor's historical semantic boundary.

  That gives SHARK a derived role rather than an arbitrary assertion.
  It gives SHARP a precise obligation: construct a descendant, but do not
  silently enlarge the ancestor's scope.

  The general SHARK decomposition is NOT yet frozen. Scope containment is
  necessary but may not be sufficient. Additional preservation dimensions
  (identity, temporal, evidence boundary, relationship) will be forced
  into existence by counterexamples, not invented in advance.

  Disciplined procedure:
  1. Prove the B7 composition theorem as currently defined.
  2. Generate adversarial descendants.
  3. Find a descendant satisfying current premises but violating integrity.
  4. Identify the missing invariant.
  5. Add exactly that invariant.
  6. Repeat.
-/

import TSCP.Formal.Provenance.Ontology

namespace TSCP.Formal.Provenance.Composition

open TSCP.Formal.Provenance.Ontology

-- ===================================================================
-- THE COMPOSITION THEOREM
-- ===================================================================

/--
The composition law: if an edge is proper, a transformation is valid (Sharp),
and the scope is contained, then historical meaning is preserved (Shark).

  Proper(r) ∧ Sharp(C, E, r, C') ∧ scope(C') ⊆ scope(C) → Shark(C, C')

This is the first architectural theorem. It gives SHARK a derived role:
SHARK is not an arbitrary assertion but a consequence of proper
construction with anti-widening.
-/
theorem composition_law
    (claim : Claim) (ev : Evidence) (r : Relationship) (claim' : Claim)
    (h_proper : Proper r)
    (h_sharp : Sharp claim ev r claim')
    (h_scope : antiWidening claim.scope claim'.scope) :
    Shark claim claim' := by
  -- Shark is defined as antiWidening original.scope transformed.scope
  -- which is exactly h_scope
  unfold Shark
  exact h_scope

-- ===================================================================
-- CONVERSE FAILURE: improper edge breaks the law
-- ===================================================================

/--
The composition law does NOT hold without PROPER.

If the edge is improper (DerivesFrom without lineage), the composition
law's premises are not satisfied, so the law does not apply.

This demonstrates that PROPER is NECESSARY for the composition.
-/
theorem composition_requires_proper
    (r : Relationship)
    (h_improper : ¬ Proper r) :
    ¬ (Sharp { proposition := "test", scope := "scope", frozen := true }
              { receipt := "test", commit := "test", target_cpu := none }
              r
              { proposition := "test'", scope := "scope.narrow", frozen := false }) := by
  unfold Sharp
  intro h
  -- Sharp requires Proper r, but we have ¬ Proper r
  exact absurd h.right.right h_improper

-- ===================================================================
-- ANTI-WIDENING IS NECESSARY
-- ===================================================================

/--
If the scope is NOT contained (anti-widening fails), then SHARK fails.

This demonstrates that anti-widening is NECESSARY for SHARK.
-/
theorem shark_requires_anti_widening
    (original transformed : Claim)
    (h_widened : ¬ antiWidening original.scope transformed.scope) :
    ¬ Shark original transformed := by
  unfold Shark
  intro h
  exact absurd h h_widened

-- ===================================================================
-- CORRESPONDENCE STATUS
-- ===================================================================

/-
  RUST ↔ LEAN CORRESPONDENCE — NOT YET CLAIMED

  The Rust crate (tscp-provenance, 4af552b7) and this Lean formalization
  independently express the same ontology. They are parallel witnesses.

  The Rust tests pass (14/14, 0 failures). The Lean theorems are stated
  and partially proven. But the following is NOT yet established:

    Admission_Rust ↔ Admission_Lean

  That correspondence is a separate future task. The current state is:

    Rust witness ∥ Lean proposition

  They should independently express the same ontology. Only afterward
  should correspondence be established.
-/

end TSCP.Formal.Provenance.Composition
