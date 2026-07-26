# Implementation Target Binding

**Artifact type:** Verification custody artifact
**Status:** Template v0.2 (review commit)
**Motivating case:** Issue #27 — Verification Surface Drift in AVX-512 equivalence gate
**Date:** 2026-07-26

---

## Purpose

This artifact binds a verification claim to a specific implementation target —
the concrete code, at a specific commit, compiled with a specific toolchain —
that the evidence is supposed to exercise.

It exists because a symbol can exist without being executed, and a receipt
can claim a backend without reaching it. Issue #27 exposed both failures: the
receipt claimed `["scalar", "avx512"]` backend coverage, but the cited test
called `butterfly()`, which delegates to `scalar_butterfly_32()`. The AVX-512
SIMD path was never invoked. The binding was never established.

This artifact is the **precondition** for the `EXECUTION_TRACE_RECEIPT.md`.
Target identity must be established before execution can be verified.

**Authority neutrality:** This artifact is evidence, not authorization. It
establishes what was verified, not what is permitted. A consumer treating
this binding as permission to act is overstepping its authority boundary.
See `VERIFICATION_INVARIANTS.md` Invariant 1 (Authority Neutrality).

**Claim scope:** The binding must specify not just what target was verified
but what scope of property was established. A receipt cannot claim a broader
property than the executed evidence establishes. See `VERIFICATION_INVARIANTS.md`
Invariant 6 (Claim Scope Integrity).

---

## Schema

```yaml
# ============================================================
# Implementation Target Binding
# ============================================================

artifact_type: IMPLEMENTATION_TARGET_BINDING
schema_version: "0.3"

# --- Claim being bound ---
claim:
  id: <claim_identifier>          # e.g. "butterfly.oracle_equivalence"
  description: <human_readable>     # e.g. "AVX-512 butterfly equivalent to scalar Montgomery butterfly"
  receipt_ref: <receipt_file>      # e.g. "evidence/tscp_ntt_equivalence_receipt.v1.json"

  # Claim scope — what property does this binding claim to establish?
  # Must be specific. "entire AVX512 NTT backend correctness" is too broad
  # if only the butterfly function was tested. The scope must match the
  # evidence boundary.
  scope: <scope_description>       # e.g. "avx512_radix2_butterfly_32 equivalence to scalar reference"
  # NOT: "entire AVX512 NTT backend correctness"
  # The scope must be a subset of what the evidence actually establishes.

# --- Source identity ---
implementation:
  module: <source_path>            # e.g. "src/avx512_butterfly_32bit.rs"
  symbol: <function_name>          # e.g. "avx512_radix2_butterfly_32"
  commit_sha: <40-char hex>        # e.g. "5e24faa5caea444f5b8db078dee8a1d7ba1c3909"
  source_hash: <sha256>            # SHA-256 of the file content at that commit

# --- Build artifact identity ---
# The artifact identity must explain why the produced binary is the one
# being observed. Source commit alone is insufficient: different compilers,
# flags, CPU features, or dependency versions can produce different codegen
# from the same commit. Issue #27's placeholder and real SIMD function share
# the same target_feature attribute but produce radically different codegen.
build_identity:
  source_commit: <40-char hex>     # Git commit SHA
  compiler: <compiler_version>     # e.g. "rustc 1.78.0 (stable)"
  target_triple: <target>          # e.g. "x86_64-unknown-linux-gnu"
  cpu_features: <features>         # e.g. "avx512f+avx512dq"
  cargo_features: <features>       # e.g. "std" or "no_std" — Cargo feature flags
  dependency_lock: <lockfile_hash> # SHA-256 of Cargo.lock (or equivalent)
  flags: <compile_flags>           # e.g. "-C target-feature=+avx512f,+avx512dq -O2"
  artifact_hash: <sha256>          # SHA-256 of the compiled object file or disassembly

  # The artifact hash is:
  #   Hash(Source, Compiler, Flags, Features, TargetCPU, Dependencies)
  # Without all inputs, the binding is incomplete. Two compilations of the
  # same commit with different dependency versions could produce different
  # codegen. The dependency_lock hash pins the exact dependency tree.

# --- Backend classification ---
backend:
  declared: <backend_name>         # e.g. "avx512"
  category: <category>             # "native_vector_path" | "scalar_fallback" | "reference_oracle"
  cpu_feature_required: <feature>  # e.g. "avx512f+avx512dq" or "none"
  fallback_path: <symbol_or_null>  # e.g. "scalar_butterfly_32" — what runs if this backend is unavailable

# --- Domain specification ---
domain:
  input_type: <type>               # e.g. "MontgomeryBabyBear"
  output_type: <type>              # e.g. "MontgomeryBabyBear"
  representation: <rep>            # e.g. "xR mod p, R = 2^32"
  preconditions:                   # What must hold for correct execution
    - <precondition>               # e.g. "all inputs in [0, p)"
  postconditions:
    - <postcondition>              # e.g. "all outputs in [0, p)"

# --- Binding verification ---
binding:
  method: <method>                 # "static_analysis" | "disassembly_verification" | "symbol_resolution"
  verifier: <tool_or_person>       # e.g. "cargo asm --symbol avx512_radix2_butterfly_32"
  verified_at: <iso_timestamp>
  notes: <free_text>               # Any caveats about the binding

# --- Authority neutrality ---
authority:
  granted: false                  # This artifact grants no authority or permission
  jurisdiction_crossed: false      # This artifact does not cross any authority boundary
  # A consumer treating this binding as authorization to act is
  # overstepping. Receipt ≠ Permission. This field exists to make that
  # explicit in the schema, not merely in prose.

# --- Supersession tracking ---
supersedes:
  previous_binding: <file_or_null>  # Reference to a previous binding this replaces
  supersession_reason: <reason>    # e.g. "Issue #27: original binding tested scalar path, not SIMD"
  previous_status: <status>       # "SUPERSEDED" or "REVOKED" — the previous binding's lifecycle state
```

---

## Filled Example (Issue #27 case)

```yaml
artifact_type: IMPLEMENTATION_TARGET_BINDING
schema_version: "0.3"

claim:
  id: "butterfly.oracle_equivalence.avx512"
  description: "AVX-512 radix-2 butterfly equivalent to scalar Montgomery butterfly"
  receipt_ref: "evidence/tscp_ntt_equivalence_receipt.v1.json"
  scope: "avx512_radix2_butterfly_32 element-wise equivalence to butterfly_reference oracle"
  # NOT: "entire AVX512 NTT backend correctness"
  # The scope is the butterfly function equivalence, not the full NTT pipeline.
  # The NTT pipeline is covered by ntt.stage_equivalence, a separate claim.

implementation:
  module: "src/avx512_butterfly_32bit.rs"
  symbol: "avx512_radix2_butterfly_32"
  commit_sha: "5e24faa5caea444f5b8db078dee8a1d7ba1c3909"
  source_hash: "<sha256 of avx512_butterfly_32bit.rs at commit 5e24faa>"

build_identity:
  source_commit: "5e24faa5caea444f5b8db078dee8a1d7ba1c3909"
  compiler: "rustc 1.78.0 (stable)"
  target_triple: "x86_64-unknown-linux-gnu"
  cpu_features: "avx512f+avx512dq"
  cargo_features: "std"
  dependency_lock: "<sha256 of Cargo.lock>"
  flags: "-C target-feature=+avx512f,+avx512dq -O2"
  artifact_hash: "<sha256 of compiled object>"
  # The artifact hash distinguishes the real SIMD codegen from the
  # placeholder in lib.rs::avx512_impl::avx512_radix2_butterfly which
  # compiles with the same target_feature attribute but delegates to
  # scalar_radix2_butterfly. Same commit, same attribute, different codegen.

backend:
  declared: "avx512"
  category: "native_vector_path"
  cpu_feature_required: "avx512f+avx512dq"
  fallback_path: "scalar_butterfly_32"

domain:
  input_type: "MontgomeryBabyBear"
  output_type: "MontgomeryBabyBear"
  representation: "xR mod p, R = 2^32"
  preconditions:
    - "all inputs in [0, p)"
    - "AVX-512F and AVX-512DQ available at runtime"
  postconditions:
    - "all outputs in [0, p)"

binding:
  method: "disassembly_verification"
  verifier: "cargo asm --symbol avx512_radix2_butterfly_32"
  verified_at: "2026-07-26T00:00:00Z"
  notes: >
    The previous receipt (commit 0205722) claimed avx512 backend coverage
    but the cited test (butterfly_reference_agreement) only invoked
    butterfly() -> scalar_butterfly_32(). The real SIMD path
    (avx512_radix2_butterfly_32 with mont_mul_16) was added in commit
    5e24faa but was never covered by the receipt's evidence chain.
    This binding establishes that the verification target is the real
    SIMD implementation, not the scalar wrapper or the placeholder.

authority:
  granted: false
  jurisdiction_crossed: false

supersedes:
  previous_binding: null
  supersession_reason: "Initial binding created after Issue #27 exposed Verification Surface Drift"
  previous_status: "REVOKED"
  # The old receipt's claim was withdrawn, not merely replaced. Its claim
  # of avx512 backend coverage was false. This is REVOKED, not SUPERSEDED.
```

---

## Lifecycle

```
DRAFTED → VERIFIED → ACTIVE → { SUPERSEDED | REVOKED }
```

- **DRAFTED**: Binding fields filled in but not yet verified.
- **VERIFIED**: The `binding.method` has been applied and confirms the
  symbol resolves to the expected implementation at the expected commit.
- **ACTIVE**: The binding is the current authoritative target for its claim.
- **SUPERSEDED**: Replaced by a newer valid artifact. The old binding was
  correct but is no longer current. Retained for custody history.
- **REVOKED**: Invalidated due to a discovered defect. The binding's claim
  was false or could not be substantiated. Retained for custody history.

**SUPERSEDED ≠ REVOKED:** SUPERSEDED means "replaced by a newer valid
artifact." REVOKED means "invalidated due to discovered defect." Issue #27
is a REVOKED case — the old receipt's claim of avx512 coverage was withdrawn
because it was false, not because a newer binding replaced it.

Neither state can be re-activated. A new binding must be created with an
explicit `supersedes` reference.

---

## Interaction with the gate chain

This artifact is produced at the **Implementation Identity Gate**:

```
Implementation Identity Gate  ← this artifact
        |
        v
Execution Binding Gate        ← EXECUTION_TRACE_RECEIPT.md
        |
        v
Domain Equivalence Gate
        |
        v
Backend Parity Gate
        |
        v
Semantic Reference Gate
        |
        v
Formal Theorem
        |
        v
Optimization Receipt
```

Formalization (Lean) must not begin until this artifact is in the **ACTIVE**
state. Without it, Lean can produce a perfectly valid proof about an
abstraction that does not correspond to production execution.
