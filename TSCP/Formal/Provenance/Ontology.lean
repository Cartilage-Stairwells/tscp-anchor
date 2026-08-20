/-
  TSCP Formal — Provenance Ontology
  Seven-operator provenance algebra frozen at 9def88c3.

  This file defines the SEMANTIC content of the seven operators independently
  of the Rust implementation. The Rust crate (tscp-provenance, 4af552b7) is a
  parallel realization of the same ontology; correspondence between Rust and
  Lean is NOT yet claimed and is a separate future task.

  The definitions here are inherited from the B7 empirical cases (18b9e823),
  not invented from acronym decoration. Each operator has a distinct failure
  mode demonstrated by a B7 witness (D1-D8).

  Design principle: invalid provenance states should be difficult to construct,
  while historically valid but non-admissible states remain representable.
-/

namespace TSCP.Formal.Provenance.Ontology

-- ===================================================================
-- PART 1: CORE SEMANTIC TYPES
-- ===================================================================

/--
A claim is a bounded assertion. The scope is the set of states/configurations
the claim covers. A more specific claim has a smaller scope (anti-widening).
-/
structure Claim where
  proposition : String
  scope : String
  frozen : Bool

/--
Evidence is a preserved measurement or observation with methodology.
The receipt is a sealed hash. Evidence without a receipt is not admissible.
-/
structure Evidence where
  receipt : String
  commit : String
  target_cpu : Option String

/--
The window within which evidence is admissible for a claim.
-/
structure Window where
  semantic_scope : String
  valid_from : String

/--
Relationship kind — the typed connection.
D2 witness: DependsOn ≠ DerivesFrom. A crate dependency is NOT a call edge.
-/
inductive RelationshipKind
  | supports : RelationshipKind
  | contradicts : RelationshipKind
  | qualifies : RelationshipKind
  | derivesFrom : RelationshipKind
  | dependsOn : RelationshipKind
  | transforms : RelationshipKind
  | splits : RelationshipKind
  deriving Repr, DecidableEq

/--
A typed connection between two objects.
D8 witness: Relationship ≠ Proper(Relationship). The edge exists as an
object independent of its admissibility evaluation.
-/
structure Relationship where
  kind : RelationshipKind
  has_lineage : Bool
  boundary : String

/--
The complete provenance representation.
INTEGRITY operates on the entire graph.
-/
structure Graph where
  edges : List Relationship

-- ===================================================================
-- PART 2: SCOPE SEMANTICS
-- ===================================================================

/--
Scope containment: scope A is a subset of scope B if A is more specific.
Convention: a more specific scope is a longer string (more qualifications).
Placeholder; structured scope comparison will follow.
-/
def scopeSubset (a b : String) : Bool :=
  b.isPrefixOf a

/--
Anti-widening: the new scope must be a subset of the old scope.
-/
def antiWidening (oldScope newScope : String) : Bool :=
  scopeSubset newScope oldScope

-- ===================================================================
-- PART 3: THE SEVEN OPERATORS
-- ===================================================================

/--
EVIDENCE(C, E, W): Does evidence E support claim C within window W?
D1 witness: same receipt, different claims, different results.
-/
def EvidencePred (claim : Claim) (ev : Evidence) (window : Window) : Prop :=
  scopeSubset claim.scope window.semantic_scope
  ∧ ¬ ev.receipt.isEmpty
  ∧ ev.commit ≠ "uncommitted"

/--
PROPER(r): Is relationship r admissible?
D2 witness: DependsOn is proper, DerivesFrom without lineage is not.
-/
def Proper (r : Relationship) : Prop :=
  match r.kind with
  | RelationshipKind.derivesFrom => r.has_lineage = true
  | RelationshipKind.transforms => r.has_lineage = true
  | RelationshipKind.splits => r.has_lineage = true
  | RelationshipKind.dependsOn => True
  | RelationshipKind.supports => ¬ r.boundary.isEmpty
  | RelationshipKind.contradicts => ¬ r.boundary.isEmpty
  | RelationshipKind.qualifies => ¬ r.boundary.isEmpty

/--
FOLLOW(r, G): Can we traverse edge r in graph G?
D3 witness: edge absent from frozen graph → traversal fails.
-/
def Follow (r : Relationship) (graph : Graph) : Prop :=
  r ∈ graph.edges ∧ Proper r

/--
SHARP(C, E, r, C'): C' is a valid transformation of C.
D4 witness: transformation narrows scope (anti-widening).
-/
def Sharp (claim : Claim) (ev : Evidence) (r : Relationship) (claim' : Claim) : Prop :=
  antiWidening claim.scope claim'.scope
  ∧ (r.kind = RelationshipKind.qualifies ∨ r.kind = RelationshipKind.transforms
     ∨ r.kind = RelationshipKind.splits)
  ∧ Proper r

/--
SHARK(C, C'): Does the transformation preserve historical meaning?
D5 witness: scalar PASS → instruction-equivalent rejected (scope widened).
-/
def Shark (original transformed : Claim) : Prop :=
  antiWidening original.scope transformed.scope

/--
INTEGRITY(G): Does the global representation preserve all distinctions?
D6 witness: observed ≠ admitted at the graph level.
-/
def Integrity (graph : Graph) : Prop :=
  ∀ r ∈ graph.edges, Proper r

/--
OBSERVED vs ADMISSIBLE: the type-level distinction.
-/
inductive Admissibility (E : Type)
  | observed : E → Admissibility E
  | admissible : E → Admissibility E
  | rejected : E → String → Admissibility E

-- ===================================================================
-- PART 4: B7 WITNESS CONSTANTS
-- ===================================================================

def b7_experiment_a_evidence : Evidence :=
  { receipt := "sha256:experiment_a_sealed"
  , commit := "8df0c247"
  , target_cpu := some "x86-64" }

def b7_experiment_a_window : Window :=
  { semantic_scope := "experiment_a.isolated_kernel"
  , valid_from := "2026-08-19" }

def b7_kernel_claim : Claim :=
  { proposition := "7.23x peak AVX-512 over scalar"
  , scope := "experiment_a.isolated_kernel"
  , frozen := true }

def b7_prover_claim : Claim :=
  { proposition := "7.23x at prover level"
  , scope := "experiment_a"
  , frozen := false }

def b7_supports_edge : Relationship :=
  { kind := RelationshipKind.supports
  , has_lineage := true
  , boundary := "Experiment A, isolated kernel" }

def b7_dependency_edge : Relationship :=
  { kind := RelationshipKind.dependsOn
  , has_lineage := false
  , boundary := "monty-31 depends on p3-zksha-rx" }

def b7_improper_derives_edge : Relationship :=
  { kind := RelationshipKind.derivesFrom
  , has_lineage := false
  , boundary := "capability implies reachability?" }

def b7_uncommitted_evidence : Evidence :=
  { receipt := "sha256:5504_calls"
  , commit := "uncommitted"
  , target_cpu := some "znver4" }

end TSCP.Formal.Provenance.Ontology
