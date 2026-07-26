# Execution Trace Receipt

**Artifact type:** Verification custody artifact
**Status:** Template v0.2 (review commit)
**Motivating case:** Issue #27 — Verification Surface Drift in AVX-512 equivalence gate
**Date:** 2026-07-26

---

## Purpose

This artifact records that the implementation target identified in
`IMPLEMENTATION_TARGET_BINDING.md` was **actually executed** — not just
defined, not just compiled, but invoked on real hardware with the expected
backend selected and no silent fallback.

It exists because a symbol can exist without being executed, and a test can
pass without reaching the code it claims to test. Issue #27 exposed both
failures:

1. `butterfly_reference_agreement` cited `"backends": ["scalar", "avx512"]`
   in the receipt, but the test called `butterfly()` → `scalar_butterfly_32()`.
   The SIMD path was never invoked.

2. `fix_test.rs` compared `avx512_radix2_butterfly` (a placeholder that
   delegates to `scalar_radix2_butterfly`) against `scalar_radix2_butterfly`
   itself — a tautological test that proved nothing.

3. `staged_cross_backend_equivalence` does exercise the real SIMD path via
   `ntt_avx512_stage` → `avx512_butterfly_pass_32`, but silently skips when
   `is_avx512_supported()` returns false. On non-AVX-512 hardware (including
   most CI runners), the test passes vacuously.

This artifact prevents all three failures by requiring **evidence of
invocation**, not merely identity.

**Authority neutrality:** This artifact is evidence, not authorization. It
records what was observed, not what is permitted. A consumer treating this
receipt as permission to deploy, ship, or act is overstepping its authority
boundary. See `VERIFICATION_INVARIANTS.md` Invariant 1 (Authority Neutrality).

**Claim scope:** A receipt cannot claim a broader property than the executed
evidence establishes. If the test exercised `avx512_radix2_butterfly_32`, the
receipt cannot claim "entire AVX512 NTT backend correctness." See
`VERIFICATION_INVARIANTS.md` Invariant 6 (Claim Scope Integrity).

---

## Schema

```yaml
# ============================================================
# Execution Trace Receipt
# ============================================================

artifact_type: EXECUTION_TRACE_RECEIPT
schema_version: "0.3"
target_binding_ref: <file>         # Reference to IMPLEMENTATION_TARGET_BINDING.md

# --- Claim scope ---
# A receipt cannot claim a broader property than the executed evidence
# establishes. The claimed scope must be a subset of the verified scope.
claim_scope:
  claimed_scope: <scope>           # What the receipt claims to prove
    # e.g. "avx512_radix2_butterfly_32 equivalence to scalar reference"
  verified_scope: <scope>          # What the evidence actually establishes
    # e.g. "avx512_radix2_butterfly_32 element-wise output matches
    # butterfly_reference for 10010 cases (10 boundary + 10000 proptest)"
  scope_valid: true                # claimed_scope ⊆ verified_scope
  # If scope_valid is false, the receipt is INVALID.
  # Example of invalid scope:
  #   claimed_scope: "entire AVX512 NTT backend correctness"
  #   verified_scope: "avx512_radix2_butterfly_32 equivalence"
  #   scope_valid: false  → receipt INVALID

# --- Execution state ---
execution:
  invoked: true                   # Was the target symbol actually called?
  backend_selected: <backend>     # e.g. "avx512" — must match target binding's declared backend
  fallback_used: false             # Did execution fall back to a different backend?
  fallback_symbol: <symbol_or_null> # If fallback occurred, what ran instead?

# --- Fallback policy ---
# If fallback_used is true, the receipt is INVALID regardless of test
# results. A CI environment that silently produces:
#   requested: avx512
#   executed: scalar
#   receipt claims: avx512
# is exactly the failure class being eliminated.
fallback_policy:
  forbidden: true                  # Fallback invalidates the receipt. No exceptions.
  # If the backend cannot be executed (e.g. no AVX-512 hardware), the
  # correct action is to NOT generate a receipt claiming that backend,
  # not to generate one with a fallback and mark it as something else.

# --- Hardware verification ---
hardware:
  cpu_feature_required: <feature> # e.g. "avx512f+avx512dq"
  cpu_feature_verified: true       # Was the feature actually present and detected?
  cpu_model: <model_string>        # e.g. "Intel Xeon Platinum 8480+" (for reproducibility)
  feature_probe_method: <method>   # How was the feature detected?
    # e.g. "is_x86_feature_detected!('avx512f') && is_x86_feature_detected!('avx512dq')"
  feature_probe_hash: <sha256>    # Hash of the probe code/output — prevents
    # silent skip when feature is absent. If cpu_feature_verified is false,
    # the receipt is INVALID regardless of test results.

# --- Test invocation ---
test:
  name: <test_name>               # e.g. "avx512_equivalence_real_path"
  test_file: <path>                # e.g. "tests/babybear_domain.rs"
  test_result: PASS                # PASS | FAIL | SKIPPED
  cases_run: <count>               # Number of test cases executed
  cases_passed: <count>
  oracle: <oracle_name>            # e.g. "scalar_butterfly_32" — what the test compares against
  oracle_independence: <notes>    # How is the oracle independent from the target?

# --- Observation method ---
observation:
  method: <method>                # How execution was independently observed.
    # "cargo_asm_verification" — disassembly confirms target_feature instructions present
    # "runtime_feature_probe" — CPU feature probe logged at test time
    # "test_harness_isolation" — test harness is structurally separate from code under test
    # "performance_counters" — perf stat confirms SIMD instructions executed (SUPPORTING ONLY)
  observer: <tool_or_entity>      # What produced the observation?
  observation_hash: <sha256>       # Hash of the observation artifact
  independence_note: <free_text>  # Why is the observation independent from the target?

  # CRITICAL: If the binary being tested produces its own trace, you have
  # reintroduced Self-Attestation. The observer must be structurally separate
  # from the code under test.

  # Observation ladder (strongest minimum is the first three):
  #
  #   Disassembly  —  proves the code IS SIMD, not that it RAN
  #       +
  #   Feature Probe  —  proves the hardware was present at test time
  #       +
  #   Harness Isolation  —  proves the test doesn't share implementation with target
  #       =
  #   [Required Minimum]
  #
  #   Performance Counters  —  SUPPORTING EVIDENCE ONLY
  #     Not proof of identity. AVX instructions may execute in unrelated
  #     code. A scalar path may produce similar timing. Performance
  #     counters confirm execution characteristics, not execution identity.
  #     Useful as corroboration but cannot substitute for the minimum.
  #     Timing ≠ Identity. A fast result does not prove the intended
  #     backend executed.

# --- Domain verification ---
domain:
  input_type: <type>               # Must match target binding
  output_type: <type>
  representation: <rep>
  domain_verified: true           # Were inputs/outputs confirmed to be in the correct domain?

# --- Authority neutrality ---
authority:
  granted: false                  # This receipt grants no authority or permission
  jurisdiction_crossed: false      # This receipt does not cross any authority boundary
  # Receipt ≠ Permission. This field exists to make that explicit in the
  # schema, not merely in prose. A consumer treating this receipt as
  # authorization to deploy, ship, or act is overstepping.

# --- Provenance ---
provenance:
  commit_sha: <40-char hex>        # Commit at which the test was run
  timestamp: <iso_timestamp>       # When the test was executed
  ci_run_id: <id_or_null>          # CI run identifier if applicable
  environment: <description>        # OS, kernel, container details

# --- Receipt lifecycle ---
lifecycle:
  status: GENERATED               # GENERATED → AUDITED → ACTIVE → { SUPERSEDED | REVOKED }
  generated_at: <iso_timestamp>
  audited_at: <iso_or_null>
  superseded_at: <iso_or_null>
  revoked_at: <iso_or_null>
  supersedes: <file_or_null>       # Previous receipt this one replaces
```

---

## Filled Example (Issue #27 corrected case)

```yaml
artifact_type: EXECUTION_TRACE_RECEIPT
schema_version: "0.3"
target_binding_ref: "IMPLEMENTATION_TARGET_BINDING.md"

claim_scope:
  claimed_scope: "avx512_radix2_butterfly_32 element-wise equivalence to butterfly_reference oracle"
  verified_scope: >
    avx512_radix2_butterfly_32 output matches butterfly_reference for
    10010 cases (10 boundary + 10000 proptest), all inputs in [0, p)^3,
    Montgomery domain. Butterfly function only — does not cover
    avx512_butterfly_pass_32 tail loop or NTT stage orchestration.
  scope_valid: true
  # NOT: "entire AVX512 NTT backend correctness"
  # The butterfly function equivalence does not establish NTT pipeline
  # correctness. That requires a separate receipt for ntt_avx512_stage
  # covering staged_cross_backend_equivalence with its own claim scope.

execution:
  invoked: true
  backend_selected: "avx512"
  fallback_used: false
  fallback_symbol: null

fallback_policy:
  forbidden: true

hardware:
  cpu_feature_required: "avx512f+avx512dq"
  cpu_feature_verified: true
  cpu_model: "Intel Xeon Platinum 8480+"
  feature_probe_method: "is_x86_feature_detected!('avx512f') && is_x86_feature_detected!('avx512dq')"
  feature_probe_hash: "<sha256 of probe code + runtime output>"

test:
  name: "avx512_equivalence_real_path"
  test_file: "tests/avx512_real_path_equivalence.rs"
  test_result: PASS
  cases_run: 10010
  cases_passed: 10010
  oracle: "butterfly_reference"
  oracle_independence: >
    butterfly_reference is composed from babybear_mul_reference +
    babybear_add/sub_reference, sharing no code with the implementation.
    Reference oracle is in field::babybear::reference, implementation is in
    avx512_butterfly_32bit. No shared code path.

observation:
  method: "cargo_asm_verification + runtime_feature_probe + test_harness_isolation"
  observer: "cargo asm --symbol avx512_radix2_butterfly_32 | grep vpmullq; feature_probe_log.json"
  observation_hash: "<sha256 of disassembly output + probe log>"
  independence_note: >
    Disassembly is produced by the compiler toolchain (cargo asm), not by
    the code under test. Feature probe is a runtime CPUID check logged to
    a file separate from the test binary's stdout. The test harness is in
    tests/avx512_real_path_equivalence.rs, a separate module that imports
    avx512_butterfly_pass_32 but does not share implementation code.

  # Disassembly proves the code IS SIMD but not that it RAN.
  # Feature probe confirms the hardware was present at test time.
  # Harness isolation confirms the test doesn't share implementation.
  # Together: (a) compiled code uses AVX-512 instructions,
  # (b) hardware could execute them, (c) test is structurally independent.
  # All three are needed.

domain:
  input_type: "MontgomeryBabyBear"
  output_type: "MontgomeryBabyBear"
  representation: "xR mod p, R = 2^32"
  domain_verified: true

authority:
  granted: false
  jurisdiction_crossed: false

provenance:
  commit_sha: "5e24faa5caea444f5b8db078dee8a1d7ba1c3909"
  timestamp: "2026-07-26T00:00:00Z"
  ci_run_id: null
  environment: "Ubuntu 22.04, kernel 5.15.0, AVX-512 hardware required"

lifecycle:
  status: GENERATED
  generated_at: "2026-07-26T00:00:00Z"
  audited_at: null
  superseded_at: null
  revoked_at: null
  supersedes: null
```

---

## What this receipt prevents

| Failure mode (from Issue #27) | How this receipt catches it |
|---|---|
| Test calls `butterfly()` → scalar, not SIMD | `execution.backend_selected: "avx512"` must match `avx512_radix2_butterfly_32`, not `scalar_butterfly_32` |
| Test is tautological (function vs itself) | `observation.independence_note` requires structural separation |
| Test silently skips on non-AVX-512 hardware | `hardware.cpu_feature_verified: true` — receipt is INVALID if false |
| Receipt claims backend not exercised | `execution.invoked: true` + `backend_selected` cross-checked against target binding |
| Wrapper delegates to scalar silently | `execution.fallback_used: false` — any fallback invalidates the receipt |
| CI produces scalar but receipt says avx512 | `fallback_policy.forbidden: true` — fallback invalidates, no exceptions |
| Execution evidence treated as authorization | `authority.granted: false` — receipt is evidence, not permission |
| Receipt claims broader scope than evidence | `claim_scope.scope_valid: true` — claimed_scope ⊆ verified_scope required |
| Performance counters mistaken for identity proof | `observation.method` — performance counters are supporting only, Timing ≠ Identity |

---

## Lifecycle

```
GENERATED → AUDITED → ACTIVE → { SUPERSEDED | REVOKED }
```

- **GENERATED**: Receipt produced by a test run with observation.
- **AUDITED**: An independent reviewer confirms the observation method is
  structurally separate from the code under test.
- **ACTIVE**: The receipt is the current authority for its claim.
- **SUPERSEDED**: Replaced by a newer valid artifact. The old receipt was
  correct but is no longer current. Retained for custody history.
- **REVOKED**: Invalidated due to a discovered defect. The receipt's claim
  was false or could not be substantiated. Retained for custody history.

**SUPERSEDED ≠ REVOKED:** SUPERSEDED means "replaced by a newer valid
artifact." REVOKED means "invalidated due to discovered defect." The
Issue #27 receipt should be REVOKED — its claim of avx512 backend coverage
was withdrawn because it was false.

Neither state can be re-activated. A new receipt must be created with an
explicit `supersedes` reference.

### Why not "FAILED"?

A receipt that discovers a target mismatch should NOT be marked "FAILED"
because the artifact itself was generated successfully. The test ran, the
observation was recorded, the receipt was produced. What failed is the
*claim*, not the *artifact*. The historical record proves: "A verification
artifact existed and was later found insufficient." That is useful custody
information.

The distinction:
- **FAILED**: The test crashed or could not execute. The artifact was not produced.
- **REVOKED**: The artifact was produced, audited, and later found to have a false claim.
- **SUPERSEDED**: The artifact was produced, valid, and replaced by a newer one.

---

## Independence requirement

The observation method must be **structurally separate** from the code under
test. If the binary being tested produces its own execution trace, you have
reintroduced Self-Attestation through the execution trace.

### Required minimum (all three):

- **Static disassembly** (`cargo asm`) — proves the compiled code contains
  AVX-512 instructions. Does NOT prove they were executed.
- **Runtime hardware probe** (CPUID check logged externally) — proves the
  hardware feature was available at test time. Does NOT prove the specific
  code path was taken.
- **Test harness isolation** — the test module imports the target function but
  shares no implementation code with it. Confirms the test doesn't depend on
  the target's internal implementation.

### Supporting evidence (optional, not a substitute):

- **Performance counter trace** (`perf stat -e avx512f_inst_retired`) —
  confirms AVX-512 instructions were executed during the test run. Useful as
  corroboration. **Not proof of identity.** AVX instructions may execute in
  unrelated code. A scalar path may produce similar timing. Performance
  counters confirm execution characteristics, not execution identity.
  Cannot substitute for the required minimum. Requires root access and
  specific hardware.

The required minimum is: disassembly + feature probe + harness isolation.
Performance counters add confidence but cannot replace any of the three.

---

## Interaction with the gate chain

This artifact is produced at the **Execution Binding Gate**:

```
Implementation Identity Gate  → IMPLEMENTATION_TARGET_BINDING.md
        |
        v
Execution Binding Gate        → this artifact (EXECUTION_TRACE_RECEIPT.md)
        |
        v
Domain Equivalence Gate       → (confirms Montgomery domain, not canonical)
        |
        v
Backend Parity Gate            → (confirms SIMD output == scalar output)
        |
        v
Semantic Reference Gate       → (confirms output == reference oracle)
        |
        v
Formal Theorem                → (Lean proof, only after all gates pass)
        |
        v
Optimization Receipt           → (performance evidence, post-verification)
```

The Execution Binding Gate must pass before the Domain Equivalence Gate can be
meaningfully evaluated. Without execution binding, domain equivalence is
proving a property of a code path that was never taken.
