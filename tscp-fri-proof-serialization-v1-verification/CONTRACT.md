# tscp-fri-proof-serialization-v1 Contract

Authority: this document specifies the byte format. The Rust implementation
in `crates/tscp-verifier/src/oracle_bridge.rs` conforms to it; the
implementation's apparent determinism is NOT the authority. A reference
decoder in a second language (`scripts/fri_proof_v1_reference.py`) must
reproduce identical bytes and digests.

Scope: the FRI proof emitted by `oracle_layer::fri_query::fri_prove` over
BabyBear, and the commit-phase Fiat-Shamir transcript proxy used for
`transcript_digest`. Supersedes the pre-v1 basis (JSON with Debug-formatted
roots; superseded digests recorded in issue #37, comment 6107484068).

## Record type 1: FRI proof, version 1

All integers are unsigned little-endian. All BabyBear elements are encoded
as their canonical representative: `u32` in `[0, ORDER)` where ORDER is the
BabyBear modulus (`<BabyBear as PrimeField32>::ORDER_U32`). Values `>= ORDER`
are invalid. Every commitment digest is exactly 8 BabyBear elements
(DIGEST_ELEMS = 8, fixed by the BatchMerkle instantiation: Poseidon2-16
sponge, 8-word digest).

```
offset  field            width        constraints / notes
------  -----            -----        ----------------------
0       magic            8 bytes      b"tscp-fri" (74 73 63 70 2D 66 72 69)
8       version          u16          must be 1
10      final_value      u32          canonical BabyBear
14      num_roots        u32          >= 2 (fri_verify requires >= 1 fold round)
18      roots            num_roots commitments, each:
          num_digests    u32          >= 1, power of two (MerkleCap invariant)
          digests        num_digests x (8 x u32)
        (the concrete trees use num_digests = 1: cap height 0)
        num_queries     u32          >= 1
        query_indices   num_queries x u64   each must fit usize
        for each query (num_queries times):
          num_rounds     u32          must equal num_roots - 1, every query
          for each round (num_rounds times): three openings in fixed order
            opening_x, opening_neg_x, opening_folded, each:
              leaf_index  u64         must fit usize
              leaf_value  u32         canonical BabyBear
              auth_len    u32
              auth_path   auth_len x (8 x u32)
end     no trailing bytes
```

## Record type 2: commit-phase transcript proxy, version 1

The emitted artifact's `transcript_digest` hashes this record. It is a
documented PROXY for the Fiat-Shamir transcript (the commit phase:
final value + roots, the data the challenger observed).

```
0       magic            8 bytes      b"tscp-fst"
8       version          u16          must be 1
10      final_value      u32
14      num_roots        u32          >= 2
18      roots            same commitment encoding as record type 1
end     no trailing bytes
```

## Digest binding

`proof_digest = sha256(record type 1 bytes)` (header included).
`transcript_digest = sha256(record type 2 bytes)`.

## Rejection behavior (decoder fails closed)

Wrong magic; unsupported version; truncation at any point; trailing bytes;
`num_roots < 2`; `num_queries == 0`; `num_rounds != num_roots - 1` for any
query; `num_digests == 0` or not a power of two; any BabyBeer value `>= ORDER`;
any u64 that does not fit usize; any length prefix exceeding the remaining
byte budget (bounded reads; no allocation on untrusted counts).

## Acceptance gates

Completeness (every verification-relevant field encoded); determinism
(identical proof -> byte-identical output); canonicality (exactly one
accepted encoding per valid proof); round-trip (decoded proof verifies from
bytes alone, fresh challenger, public domain only); mutation sensitivity;
malformed-input rejection; digest binding over the specified bytes;
independent reproducibility (second-language reference implementation);
existing serialization-conformance CI stays green.
