# TSCP-CANON-001: Protocol-Wide Canonical Byte Serialization Specification

**Version:** 1.0
**Status:** PROPOSED (Protocol Primitive)
**canon_version:** 1.0

## 1. Scope

This specification defines the deterministic transformation of Protocol
Objects into unique, invariant byte arrays (`B`) suitable for cryptographic
hashing, signing, and state-root anchoring across all runtime boundaries.

## 2. Abstract Data Model

A "Protocol Object" is defined as a collection of key-value pairs (a map)
where keys are primitive Unicode strings and values are restricted to:

* Primitive Types: Strings, Integers, Booleans, Null.
* Composite Types: Ordered sequences (Arrays) and Maps matching this
  definition.

Floating-point values are **not** part of the Protocol Object data model.
See §3.1.3 for the required encoding of fractional values.

### 2.1 Top-Level Type Constraint

The top-level input to the canonicalization pipeline MUST be a map (JSON
object). A bare top-level primitive (string, integer, boolean, null) or a
bare top-level array MUST be rejected during validation, prior to
normalization. Protocol Objects are always maps; arrays and primitives are
only valid as *values within* a map, never as the canonicalization root.

## 3. Serialization Profiles

The protocol separates logical object representation from physical
serialization. A Serialization Profile dictates how an abstract Protocol
Object maps to a specific format.

### 3.1 Profile: Canonical JSON (Current Default)

When using the Canonical JSON Profile, the input object must be transformed
into bytes conforming to the following deterministic rules:

#### 3.1.1 Key Ordering
All object keys must be sorted by Unicode code point order. For valid UTF-8
encoded text, code point order and byte order are identical, so this rule is
unambiguous across implementations regardless of internal string
representation (UTF-8, UTF-16, UTF-32).

#### 3.1.2 Unicode Normalization
All strings — both object keys and string values — **MUST** be normalized
to Unicode Normalization Form C (NFC) before key sorting and before
serialization.

Rationale: without a mandated normalization form, two semantically
identical strings (e.g. a precomposed `é` vs. an `e` + combining acute
accent, U+0065 U+0301) would serialize to different byte sequences and
therefore hash to different digests. For any system where a cache key,
state root, or signature is derived from this serialization (ledger
checkpoints, EventBus keys, replay digests), that ambiguity is a
data-integrity fault, not a cosmetic concern.

If two distinct keys within the same object normalize to the same NFC
string, the input is malformed and **MUST** be rejected — implementations
must not silently merge or pick one.

#### 3.1.3 Numeric Representation
Only integers are permitted in core protocol state primitives. Integers
**MUST** be represented in base-10 string notation, with an optional
leading `-` for negative values, and no leading zeros (except the literal
digit `0` itself). Floating-point values and exponential notation are
**strictly prohibited** and MUST cause the implementation to reject the
input.

**Fractional values MUST be represented as scaled integers**, never as
floats and never as decimal strings. The scale factor (power of 10 divisor)
must be declared explicitly per field or baked into a self-documenting
type name, and is invariant across implementations. Example:

```json
{ "currency": "USD", "amount": 1999, "scale": 2 }
```

or, for fixed-domain schemas, a scale-bearing type name such as `UsdCents`
or `EthWei` may be used in place of an explicit `scale` field, provided the
scale is documented in the owning schema and never inferred at runtime.

Rationale for scaled integers over decimal-as-string: decimal strings
reintroduce the same class of normalization ambiguity already resolved for
Unicode (trailing zeros, leading zeros, exponential notation, locale
separators), and require bespoke parsing/validation in every
implementation. Scaled integers reduce to native integer arithmetic in
every target language in the current stack (Rust, TypeScript, Lean,
Solidity) with no parsing step and no format-negotiation risk.

#### 3.1.4 Whitespace
No insignificant whitespace is permitted outside of string literals — no
spaces, tabs, or newlines between keys, values, or structural tokens.

#### 3.1.5 Null Handling
`null` is a valid, explicit value and MUST be preserved in output wherever
present in the input. A field that is entirely absent from a Protocol
Object is distinct from a field explicitly set to `null`; implementations
must not conflate "absent" with "null."

#### 3.1.6 Character Encoding
The output MUST be strictly valid UTF-8.

## 4. Canonicalization Pipeline

Regardless of the active Profile, the execution topology must strictly
conform to:

```
[Protocol Object] -> [Serialization Profile (e.g. JSON)] -> [Deterministic Byte Layout] -> [Cryptographic Digest]
```

## 5. Conformance Requirements

An implementation is compliant with TSCP-CANON-001 v1.0 if and only if it
processes every input case defined within the shared Protocol Conformance
Suite (`conformance/canon/fixtures/`) and yields a byte stream matching the
exact cryptographic digest specified in the test manifest
(`conformance/canon/fixtures/manifest.json`).

Conformance digests in the manifest MUST be generated by actually executing
a canonicalizer against the fixture inputs. A manifest containing
fabricated, hand-written, or otherwise unverified digest values does not
constitute a valid conformance suite and MUST NOT be treated as frozen.

As of this version, the suite includes at minimum:

* Key sorting across mixed-case and non-ASCII keys
* Whitespace elimination (pretty-printed input collapsing to identical
  canonical bytes as compact input)
* Nested object/array structures with array order preservation
* Unicode NFC normalization (decomposed -> precomposed)
* Scaled-integer encoding, including large integers exceeding 64 bits,
  negative values, and zero
* Explicit `null` vs. absent-key handling
* RFC 8259 string escaping (quotes, backslashes, control characters,
  non-BMP characters)
* Rejection of float/exponential-notation input (negative case)
* Rejection of a non-object top-level input (negative case, §2.1)
* Rejection of two distinct keys colliding under NFC normalization
  (negative case, §3.1.2)
* Numeric-looking text *inside string values* (large integers, scientific
  notation, negative-looking text, fullwidth Unicode digits, and
  escaped-Unicode-encoded ASCII digits) must remain strings and must not be
  reinterpreted as numbers by any lexical preprocessing an implementation
  performs. This protects the protocol-level string/number boundary
  independent of how any given implementation parses JSON internally.

## 6. Versioning Policy & Compatibility

* **Specification Versioning:** Governed independently of sub-protocol
  releases. This document is tracked as `canon_version: 1.0`.
* **Breaking Changes:** Any alteration to sorting logic, normalization
  form, or numeric encoding constitutes a major version bump and requires
  an explicit state-transition migration path.

## 8. Deterministic Error Taxonomy

Rejection is part of the protocol contract, not an implementation detail.
Every conformant implementation MUST classify rejections using the
following stages and codes, so that independent implementations reject
*for the same reason*, not merely "somehow."

### 8.1 Stages

| Stage          | When it runs                                          |
|----------------|--------------------------------------------------------|
| `VALIDATION`   | Before normalization; structural/type-level checks     |
| `NORMALIZATION`| During NFC normalization and key-collision detection    |
| `NUMERIC`      | During numeric literal classification (int vs. float)   |

### 8.2 Error Codes

| Code                          | Stage          | Spec Section | Condition                                                    |
|-------------------------------|----------------|--------------|---------------------------------------------------------------|
| `TSCP-CANON-TOPLEVEL-NONMAP`  | `VALIDATION`   | §2.1         | Top-level input is not a JSON object                           |
| `TSCP-CANON-NON-STRING-KEY`   | `VALIDATION`   | §2           | An object key is not a string (not reachable via JSON syntax, but part of the abstract data model contract) |
| `TSCP-CANON-FLOAT-PROHIBITED` | `NUMERIC`      | §3.1.3       | A numeric literal contains `.`, `e`, or `E`                    |
| `TSCP-CANON-KEY-COLLISION`    | `NORMALIZATION`| §3.1.2       | Two distinct input keys normalize to the same NFC string        |

Every REJECT fixture in the conformance corpus MUST record `error_code`,
`error_stage`, and the originating spec section. A bare `status: "REJECT"`
with no further classification is insufficient for conformance purposes —
it permits two implementations to both "reject" the same input for two
different, unrelated reasons, which is not a verified shared contract.

## 9. Conformance Corpus Provenance

Every SUCCESS fixture entry in the manifest MUST record:

* The canonical byte output (`expected_output_file`)
* The SHA-256 digest of those exact bytes (`expected_sha256`)
* The generator implementation and its version (`generated_by`)
* The `canon_version` of this specification the corpus was generated against

Every REJECT fixture entry MUST record `error_code`, `error_stage`, and the
originating spec section (§8.2), in addition to which generator produced
the classification.

A manifest is only valid evidence of conformance if its digests and error
classifications were produced by actually executing a canonicalizer against
the fixture inputs — never hand-authored or asserted.

## 10. Reference Implementations

The Rust implementation at `tools/tscp-canon` is the primary conformance
tool: it generates the golden corpus digests in
`conformance/canon/fixtures/manifest.json` and can independently re-verify
them. A Python implementation (`reference_canon.py`) was used as an
independent oracle during initial spec validation — both implementations
were written separately from this document and cross-checked to agree on
every fixture digest before this spec was treated as frozen-candidate.

Any further language implementation (TypeScript `canonical.ts`, future
Lean 4 formalization, etc.) is conformant if and only if it reproduces
every digest in the manifest, including the rejection cases.
