/-
Kernel-level dependency audit for the TSCP formal corpus.

Enumerates every TSCP-prefixed constant in the elaborated environment
(the actual build environment, not source text), runs Lean.collectAxioms
on each, and emits one machine-readable line per constant:

  AUDIT|<kind>|<name>|<class>|<deps>

kind: axiom | theorem | def | inductive | ctor | recursor | quot | opaque | alias | other
class: clean | sorry | project-axiom | classical | foundational

  clean          — no axioms at all
  sorry          — depends on sorryAx (incomplete proof)
  project-axiom  — depends on a TSCP-declared axiom
  classical      — foundational axioms incl. Classical.choice
  foundational  — propext / Quot.sound only

The classification of "foundational" is Lean's three standard axioms
(propext, Classical.choice, Quot.sound); sorryAx is treated separately.
-/
import TSCP.Formal.TSCP_Formal_Backbone
import TSCP.Formal.BridgePreservation
import TSCP.Formal.Butterfly
import TSCP.Formal.Core
import TSCP.Formal.Evidence.ManifestBinding
import TSCP.Formal.Examples.NormalizationBridge
import TSCP.Formal.Examples.PropositionalKernel
import TSCP.Formal.Montgomery
import TSCP.Formal.NTTStage
import TSCP.Formal.ReviewerSemantics
import TSCP.Formal.NTTFull
import Lean.Elab.Command

open Lean Elab Command in
def kindOf (ci : ConstantInfo) : String :=
  match ci with
  | .axiomInfo _ => "axiom"
  | .thmInfo _ => "theorem"
  | .defnInfo _ => "def"
  | .inductInfo _ => "inductive"
  | .ctorInfo _ => "ctor"
  | .recInfo _ => "recursor"
  | .quotInfo _ => "quot"
  | _ => "other"

open Lean Elab Command in
def classify (axs : Array Name) : String :=
  if axs.isEmpty then "clean"
  else if axs.any (fun n => n == `sorryAx) then "sorry"
  else
    let proj := axs.filter (fun n => (`TSCP).isPrefixOf n)
    if !proj.isEmpty then "project-axiom"
    else if axs.any (fun n => n == `Classical.choice) then "classical"
    else "foundational"

open Lean Elab Command in
run_cmd do
  let env ← getEnv
  let mut entries : Array (Name × String) := #[]
  for (n, ci) in env.constants.toList do
    if (`TSCP).isPrefixOf n && !n.toString.contains "✝" && !n.toString.contains "_private" then
      entries := entries.push (n, kindOf ci)
  entries := entries.qsort (fun a b => a.1.toString < b.1.toString)
  for (n, kind) in entries do
    let axs ← liftCoreM <| Lean.collectAxioms n
    let cls := classify axs
    let deps := (axs.toList.map (fun x => x.toString)).foldl (init := "") (fun acc d => if acc.isEmpty then d else acc ++ "," ++ d)
    logInfo m!"AUDIT|{kind}|{n}|{cls}|{deps}"
