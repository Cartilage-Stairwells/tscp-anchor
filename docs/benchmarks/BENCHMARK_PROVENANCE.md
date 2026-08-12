# Benchmark Provenance Index

This index registers evidence baselines captured for the TSCP project.
Each baseline is an immutable artifact identified by its evidence identity.
Corrections require a new evidence identity, not modification of an existing one.

---

## Registered Evidence Baselines

| Evidence Identity | Backend | Hardware | Status | Record SHA-256 | Git Tag |
|---|---|---|---|---|---|
| `firebird_74c6e5f` | AVX-512 CPU | Intel Ice Lake-SP (family=6, model=106) | Sealed | `0d9c2c8e...` | `benchmark/firebird_74c6e5f` |

---

## Evidence Identity Convention

Each evidence baseline receives a unique identifier of the form `firebird_<8hex>`.
The identifier is permanent once sealed. A superseded baseline retains its identity
and status; it is never renamed or overwritten.

## Comparison Architecture

```
CPU evidence                GPU evidence
firebird_74c6e5f            firebird_<future-id>
        |                           |
        +-----------+---------------+
                    |
                    v
          comparison artifact
          (separate claim, references both)
```

A CPU-GPU comparison is a **separate claim** that identifies both evidence
baselines. Neither baseline is the implicit denominator for the other.

## Layer Separation

| Layer | Description | Mutability |
|---|---|---|
| Evidence | Captured artifacts (manifest, results, hashes) | Immutable |
| Attestation | Documentation, Git tags, GitHub releases | Describes evidence; does not modify it |
| Interpretation | Performance analysis, comparisons, conclusions | Evolves freely |

## Related Documents

- [FIREBIRD_AVX512_BASELINE.md](./FIREBIRD_AVX512_BASELINE.md) — CPU AVX-512 evidence baseline attestation
