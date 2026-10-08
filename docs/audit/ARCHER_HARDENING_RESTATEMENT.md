# ARCHER Hardening — Restatement (registration, before implementation)

**Base:** master @ 131b6d8a (post-#34/#35/#46 merges; #34 DEEP-ALI→FRI, #35 sumcheck
verifier, #46 DEEP-ALI quotient wired into FRI)
**Scope:** Issues #36–#40, #47 (the tracked hardening set; #34/#35 closed)
**Method:** each issue's "current state" claim was verified against the code
bytes at the base commit BEFORE restating — per the count-class rule, the
issue list is evidence to check, not evidence to trust. All six claims are
CONFIRMED. One location drift recorded (#38).

**Status:** RESTATED, implementation not started. Attack order proposed at the
end; #40 carries a caller decision.

---

## #36 — Additive → multiplicative shift in DEEP-ALI (F23, F32; soundness)

**Verified current state (CONFIRMED):** `crates/prover-server/src/deep_ali.rs:149-151`
— `evaluate_shifted` computes `shifted_z = z + F::from_canonical_usize(shift)`:
an additive shift. The constraint system's shift convention is multiplicative.

**Restated work:** replace the shifted-point construction with the
multiplicative form (z·g^shift in the trace domain's generator terms), and
align every call site (`deep_ali.rs:252-263` iterates `link.shifts`). The
quotient-polynomial construction affected by #34's merge inherits from this
path, so the change must be proven consistent with the FRI wiring landed in
#46, not just with DEEP-ALI in isolation.

**Acceptance:** existing DEEP-ALI/FRI pipeline tests pass on the multiplicative
form (the honest-pipeline suite from PR #46 is the regression surface); the
shift convention is uniform across constraint system and DEEP-ALI, stated in
one place in the code.

**Classification:** direct fix, no divergence between issue and code.

## #37 — Canonical proof serialization (F29, F30; soundness-adjacent)

**Verified current state (CONFIRMED):** `crates/tscp-verifier/src/oracle_bridge.rs:62-88`
— `SerializableFriProof` carries `roots` and `final_value` as `String`
produced by `format!("{:?}", …)` (non-canonical debug format); Merkle openings
are omitted from the struct (the comment says serialization "is exercised for
digest computation" over the incomplete struct).

**Restated work:** (1) include Merkle openings in the serializable proof;
(2) replace `{:?}` with canonical byte encoding (fixed-width, e.g. canonical
field-element bytes) for roots and final_value; (3) digest computed over the
canonical serialization; re-serialization round-trips bit-identically.

**Acceptance:** a digest computed from a serialized proof is reproducible
across two serialize→deserialize→serialize cycles (byte equality), and the
struct carries the openings.

**Classification:** direct fix; scope note — reconstruction of a full `FriProof`
from bytes for standalone verification is adjacent work (the comment marks
the current reconstruction as benchmark-only); this issue covers the
serialization and digest, not a full out-of-process verifier.

## #38 — FFT-based interpolation (F33; performance)

**Verified current state (CONFIRMED, LOCATION DRIFT):** naive Lagrange is at
`crates/prover-server/src/deep_ali.rs:199` and `:304`
(`interpolate_lagrange_naive`), not in `poly.rs` as the issue text states.
The claim holds; the file named in the issue does not match the current
layout (code moved since the audit).

**Restated work:** add an NTT-based interpolation path (O(n log n)) for
power-of-two-length evaluation vectors; keep the naive path as a
cross-check/test oracle. Selection: FFT path when the evaluation domain is a
multiplicative subgroup (coset or not) of the right size, naive otherwise —
no silent semantic change to DEEP-ALI's interpolation points.

**Acceptance:** an equivalence test — for a randomized set of sizes, FFT
interpolation and naive interpolation agree coefficient-for-coefficient;
benchmarks show the complexity change (not a wall-clock target on this
host; performance claims follow the evidence-custody split: correctness here,
measurement on qualified hardware only).

**Classification:** direct fix with a location correction on the record.

## #39 — SoundnessAccumulator → Fiat-Shamir binding (F35; soundness)

**Verified current state (CONFIRMED):** `crates/prover-server/src/deep_ali.rs:37-43`
defines `SoundnessAccumulator`; used at `:255` and `:392`; the struct's own
doc comment (line 53 vicinity) states it is NOT cryptographically bound to
the Fiat-Shamir transcript.

**Restated work:** fold accumulator state into the transcript — the natural
route is to absorb the accumulated soundness parameters into the Fiat-Shamir
sponge at each accumulation event (or a final cumulative absorption), so any
accumulator modification changes the challenge stream and invalidates the
proof. Verifier side must apply the matching absorption.

**Acceptance:** a tampering test — mutate any accumulated value after
proving, re-verify, and the verification MUST fail; an honest round-trip
still passes. This is the first issue whose acceptance is a negative test,
and the negative test is the point.

**Classification:** direct fix; the transcript format change is
proof-breaking (old proofs do not verify under the new binding) — recorded
as an accepted break, since reproducible digests (#37) land in the same
window.

## #40 — Montgomery form reconciliation (F36; cross-repo, CALLER DECISION)

**Verified current state (CONFIRMED as a cross-repo fact):** zksha-rx uses
R=2³², tscp-pl-phase1's NTT uses R=2⁶⁴ (per the issue; zksha-rx is not in
this workspace, so the zksha-rx side is verified from the issue text, not
from bytes — flagged). The B5 artifact pins BabyBear at R=2³² one-step REDC
(babybear-core-verified, sha256-pinned); the paper does not claim the R=2⁶⁴
skeleton forms.

**Restated work:** (1) caller decides the canonical form — the issue
proposes R=2⁶⁴ "for BabyBabyBear", but the B5 record pins BabyBear at R=2³²
and explicitly declines the 2⁶⁴ skeleton claims; the two fields may warrant
different radices, and that is the decision, not a mechanical
reconciliation; (2) once decided, update the non-canonical side; (3)
cross-implementation test vectors.

**Classification:** PARKED pending the caller's canonical-form decision and
zksha-rx workspace access. No work started. This is the only issue in the
set that cannot be restated to an executable plan today.

## #47 — OWSL gate injection point (test infrastructure)

**Verified current state (CONFIRMED):** `crates/prover-server/src/owsl_bridge.rs:7`
hardwires `OWSL_STATUS_PATH = "~/.tscp/owsl_status.json"`; `:164` a
process-global `lazy_static OWSL_BRIDGE`; no injection point for path or
reader. PR #46's mitigation (atomic temp-file publication, bounded
re-publish/retry) is local to the deep_ali_fri suite.

**Restated work:** add a first-class injection point — an environment
override (e.g. `TSCP_OWSL_STATUS_PATH`) or an explicit reader configuration
— so each test suite owns its gate status file; make the publication atomic
by construction (write-temp + rename) in the bridge helper itself so no
suite re-derives the mitigation; keep the global path as the production
default.

**Acceptance:** two OWSL-dependent suites with disjoint status files run in
parallel (`cargo test` default threading) with zero gate-induced flakes
across repeated runs; no suite writes the global path.

**Classification:** direct fix. ORDERING NOTE: this is proposed FIRST in the
attack order — the shared-file race makes test outcomes non-deterministic,
which pollutes the evidence for every other item's acceptance run.

---

## Proposed attack order (for caller confirmation)

1. **#47** — gate injection point (deterministic tests unlock clean
   acceptance runs for everything else).
2. **#36** — shift consistency (soundness; small, self-contained, but must
   reconcile with #46's FRI wiring).
3. **#39** — accumulator binding (soundness; negative test).
4. **#37** — canonical serialization (pairs with #39's proof-breaking
   window).
5. **#38** — FFT interpolation (performance; equivalence-tested).
6. **#40** — parked, caller decision + zksha-rx access.

Each item lands as its own PR against master with the restatement here as
the scope record. No merges on worker authority; #40 does not start until
the caller rules on the canonical Montgomery form.
