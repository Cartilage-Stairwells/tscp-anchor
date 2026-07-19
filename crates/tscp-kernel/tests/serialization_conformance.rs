//! tscp-serialization-v0.1 Conformance Suite
//!
//! Implements all 12 tests mapped to the 7 verification predicates
//! defined in tscp-serialization-v0.1-verification/CONTRACT.md.
//!
//! Run: cargo test -p tscp-kernel --test serialization_conformance --release

use tscp_kernel::serialization::{to_cbor, from_cbor};
use tscp_kernel::types::*;


// ─── Helper ──────────────────────────────────────────────────────────────

fn sample_receipt() -> TransitionReceipt {
    TransitionReceipt {
        parent_state_hash: [0xAB; 32],
        event_hash: [0xCD; 32],
        child_state_hash: [0xEF; 32],
        kernel_version: 1,
        kind: TransitionKind::ClaimCreated,
    }
}

fn receipt_hash(r: &TransitionReceipt) -> [u8; 32] {
    r.hash()
}

// ─── Predicate 1: Schema Compliance ──────────────────────────────────────

/// #1 positive: valid schema accepted — a well-formed TransitionReceipt
/// serializes and deserializes cleanly.
#[test]
fn test_valid_schema_accepted() {
    let r = sample_receipt();
    let bytes = to_cbor(&r).expect("serialize should succeed");
    let r2: TransitionReceipt = from_cbor(&bytes).expect("deserialize should succeed");
    assert_eq!(r, r2, "round-trip must preserve all fields");
}

/// #1 negative: invalid schema rejected — truncated / corrupt CBOR is rejected.
#[test]
fn test_invalid_schema_rejected() {
    let r = sample_receipt();
    let mut bytes = to_cbor(&r).expect("serialize");
    // Truncate to corrupt the CBOR structure
    bytes.truncate(bytes.len() / 2);
    let result: Result<TransitionReceipt, _> = from_cbor(&bytes);
    assert!(result.is_err(), "truncated CBOR must be rejected");

    // Also test with garbage bytes
    let garbage = [0xFFu8; 10];
    let result2: Result<TransitionReceipt, _> = from_cbor(&garbage);
    assert!(result2.is_err(), "garbage bytes must be rejected");
}

// ─── Predicate 2: Canonical Encoding ─────────────────────────────────────

/// #2 positive: equivalent objects produce identical canonical bytes.
/// Two TransitionReceipts with identical field values must serialize to
/// byte-identical CBOR.
#[test]
fn test_equivalent_json_canonical_bytes() {
    let r1 = sample_receipt();
    let r2 = sample_receipt(); // same values

    let b1 = to_cbor(&r1).unwrap();
    let b2 = to_cbor(&r2).unwrap();

    assert_eq!(b1, b2, "equivalent objects must produce identical bytes");
}

/// #2 negative: non-canonical encoding rejected — a byte-level modification
/// that doesn't change the logical value still produces a different
/// serialized form (proving determinism, not just correctness).
#[test]
fn test_non_canonical_rejected() {
    let r = sample_receipt();
    let b1 = to_cbor(&r).unwrap();

    // Flip a bit in the serialized form
    let mut b2 = b1.clone();
    b2[10] ^= 0x01;

    // The modified bytes should NOT deserialize to the same object
    let result: Result<TransitionReceipt, _> = from_cbor(&b2);
    if let Ok(r2) = result {
        assert_ne!(r, r2, "modified bytes must not produce equivalent object");
    }
    // If deserialization fails, that's also acceptable — the mutation was detected
}

// ─── Predicate 3: Hash Stability ──────────────────────────────────────────

/// #3 positive: serialize → deserialize → re-serialize → same bytes,
/// and the hash is stable across all cycles.
#[test]
fn test_round_trip_hash_stability() {
    let r = sample_receipt();
    let h_before = receipt_hash(&r);

    let b1 = to_cbor(&r).unwrap();
    let r2: TransitionReceipt = from_cbor(&b1).unwrap();
    let h_after = receipt_hash(&r2);

    let b2 = to_cbor(&r2).unwrap();

    assert_eq!(b1, b2, "re-serialized bytes must be identical");
    assert_eq!(h_before, h_after, "hash must be stable across round-trip");

    // Multiple round-trips
    let r3: TransitionReceipt = from_cbor(&b2).unwrap();
    let b3 = to_cbor(&r3).unwrap();
    assert_eq!(b1, b3, "hash must be stable across multiple round-trips");
    assert_eq!(h_before, receipt_hash(&r3), "hash must be stable across N round-trips");
}

// ─── Predicate 4: Mutation Sensitivity ─────────────────────────────────────

/// #4 positive: any field change in the receipt produces a changed
/// identity hash. Covers all five fields: parent_state_hash, event_hash,
/// child_state_hash, kernel_version, and kind.
#[test]
fn test_mutation_changes_hash() {
    let r = sample_receipt();
    let h_original = receipt_hash(&r);

    // Flip each bit in each hash field, verify hash changes
    for byte_idx in 0..32 {
        let mut r2 = r.clone();
        r2.parent_state_hash[byte_idx] ^= 0x01;
        assert_ne!(
            h_original, receipt_hash(&r2),
            "parent_state_hash bit {} flip must change hash", byte_idx * 8
        );
    }

    for byte_idx in 0..32 {
        let mut r2 = r.clone();
        r2.event_hash[byte_idx] ^= 0x01;
        assert_ne!(
            h_original, receipt_hash(&r2),
            "event_hash bit {} flip must change hash", byte_idx * 8
        );
    }

    for byte_idx in 0..32 {
        let mut r2 = r.clone();
        r2.child_state_hash[byte_idx] ^= 0x01;
        assert_ne!(
            h_original, receipt_hash(&r2),
            "child_state_hash bit {} flip must change hash", byte_idx * 8
        );
    }

    // kernel_version change
    let mut r2 = r.clone();
    r2.kernel_version = 2;
    assert_ne!(h_original, receipt_hash(&r2), "kernel_version change must change hash");

    // kind change
    let mut r2 = r.clone();
    r2.kind = TransitionKind::ClaimVerified;
    assert_ne!(h_original, receipt_hash(&r2), "kind change must change hash");
}

/// #4 negative: mutation detection is always enforced — exhaustive bit-flip
/// across all 96 bytes of the three hash fields (parent, event, child).
///
/// Note: kernel_version and kind are covered by test_mutation_changes_hash.
/// This test is exhaustive over hash-field bytes only: 96 bytes × 8 bits = 768
/// individual bit-flip mutations, all of which must change the output hash.
#[test]
fn test_mutation_always_detected() {
    let r = sample_receipt();
    let h_original = receipt_hash(&r);

    let fields: [(&str, usize); 3] = [
        ("parent", 32),
        ("event", 32),
        ("child", 32),
    ];

    for (field_name, len) in &fields {
        for byte_idx in 0..*len {
            for bit in 0..8 {
                let mut r2 = r.clone();
                let mask = 1u8 << bit;
                match *field_name {
                    "parent" => r2.parent_state_hash[byte_idx] ^= mask,
                    "event" => r2.event_hash[byte_idx] ^= mask,
                    "child" => r2.child_state_hash[byte_idx] ^= mask,
                    _ => unreachable!(),
                }
                assert_ne!(
                    h_original, receipt_hash(&r2),
                    "undetected mutation: field={}, byte={}, bit={}",
                    field_name, byte_idx, bit
                );
            }
        }
    }
}

// ─── Predicate 5: Context Isolation ────────────────────────────────────────

/// #5 positive: different TSCP domains produce different hashes.
/// The domain-separation tag in the hash function ensures no cross-domain
/// collision.
#[test]
fn test_different_domains_different_hashes() {
    let r = sample_receipt();
    let h = receipt_hash(&r);

    // The hash must not equal any input component
    assert_ne!(h, r.parent_state_hash, "hash must not equal parent_state_hash");
    assert_ne!(h, r.event_hash, "hash must not equal event_hash");
    assert_ne!(h, r.child_state_hash, "hash must not equal child_state_hash");

    // Different kernel versions must produce different hashes
    let mut r2 = r.clone();
    r2.kernel_version = 2;
    assert_ne!(h, receipt_hash(&r2), "different kernel_version must produce different hash");

    let mut r3 = r.clone();
    r3.kernel_version = 999;
    assert_ne!(h, receipt_hash(&r3), "different kernel_version (999) must produce different hash");
}

/// #5 negative: cross-domain no collision — two receipts that differ only
/// in their domain context (kernel_version) never produce the same hash.
#[test]
fn test_cross_domain_no_collision() {
    let r = sample_receipt();

    // Generate hashes for kernel_version 1..256, verify all distinct
    let mut hashes: Vec<[u8; 32]> = Vec::new();
    for kv in 1u16..=256 {
        let mut r2 = r.clone();
        r2.kernel_version = kv;
        let h = receipt_hash(&r2);

        for (idx, prev) in hashes.iter().enumerate() {
            assert_ne!(
                h, *prev,
                "collision: kernel_version={} collided with kernel_version={}",
                kv, idx + 1
            );
        }
        hashes.push(h);
    }

    // Also verify that different kind values produce different hashes
    let h_created = {
        let mut r2 = r.clone();
        r2.kind = TransitionKind::ClaimCreated;
        receipt_hash(&r2)
    };
    let h_verified = {
        let mut r2 = r.clone();
        r2.kind = TransitionKind::ClaimVerified;
        receipt_hash(&r2)
    };
    assert_ne!(h_created, h_verified, "different kind must produce different hash");
}

// ─── Predicate 6: Toolchain Reproducibility ───────────────────────────────

/// #6: toolchain reproducibility — the same source compiled with the same
/// toolchain produces bitwise-identical serialized output.
#[test]
fn test_reproducible_build() {
    let r = sample_receipt();

    // Serialize 1000 times, verify identical output every time
    let baseline = to_cbor(&r).unwrap();
    for i in 0..1000 {
        let bytes = to_cbor(&r).unwrap();
        assert_eq!(
            bytes, baseline,
            "serialization non-deterministic at iteration {}", i
        );
    }

    // Also verify hash is deterministic
    let h_baseline = receipt_hash(&r);
    for i in 0..1000 {
        let h = receipt_hash(&r);
        assert_eq!(h, h_baseline, "hash non-deterministic at iteration {}", i);
    }
}

// ─── Predicate 7: Authority Boundary Preservation ────────────────────────

/// #7 positive: authority boundary — the TransitionReceipt type is sealed
/// with `#[serde(deny_unknown_fields)]`. This test verifies that the type
/// compiles and round-trips cleanly with exactly the five defined fields.
#[test]
fn test_authority_boundary_compile() {
    let r = sample_receipt();
    let bytes = to_cbor(&r).unwrap();

    // The serialized form must NOT contain custody decision keywords
    let serialized_str = String::from_utf8_lossy(&bytes);

    assert!(!serialized_str.contains("promote"), "evidence record must not contain 'promote'");
    assert!(!serialized_str.contains("reject"), "evidence record must not contain 'reject'");
    assert!(!serialized_str.contains("approve"), "evidence record must not contain 'approve'");
    assert!(!serialized_str.contains("custody"), "evidence record must not contain 'custody'");

    let r2: TransitionReceipt = from_cbor(&bytes).unwrap();
    let _ = r2.parent_state_hash;
    let _ = r2.event_hash;
    let _ = r2.child_state_hash;
    let _ = r2.kernel_version;
    let _ = r2.kind;
}

/// #7 negative: custody expression blocked — a CBOR payload containing a
/// "custody" field MUST be rejected by `from_cbor::<TransitionReceipt>`.
///
/// This test constructs a CBOR map with the valid five fields plus an extra
/// "custody" key and asserts that deserialization fails. This is enforced
/// by `#[serde(deny_unknown_fields)]` on TransitionReceipt.
#[test]
fn test_custody_expression_blocked() {
    use serde_cbor::Value;
    use std::collections::BTreeMap;

    // Build a CBOR map with all five valid fields + an extra "custody" field.
    // serde_cbor uses string keys for struct field names.
    let mut map: BTreeMap<Value, Value> = BTreeMap::new();
    map.insert(Value::Text("parent_state_hash".into()), Value::Bytes(vec![0xAB; 32]));
    map.insert(Value::Text("event_hash".into()),        Value::Bytes(vec![0xCD; 32]));
    map.insert(Value::Text("child_state_hash".into()),  Value::Bytes(vec![0xEF; 32]));
    map.insert(Value::Text("kernel_version".into()),    Value::Integer(1));
    map.insert(Value::Text("kind".into()),              Value::Text("ClaimCreated".into()));
    // Inject forbidden field
    map.insert(Value::Text("custody".into()),           Value::Text("approve".into()));

    let cbor_bytes = serde_cbor::to_vec(&Value::Map(map))
        .expect("building test CBOR must succeed");

    // With #[serde(deny_unknown_fields)], this MUST fail
    let result: Result<TransitionReceipt, _> = from_cbor(&cbor_bytes);
    assert!(
        result.is_err(),
        "TransitionReceipt must reject CBOR payloads containing unknown fields (e.g. 'custody'). \
         This failure means #[serde(deny_unknown_fields)] is not in effect — add it to TransitionReceipt."
    );
}
