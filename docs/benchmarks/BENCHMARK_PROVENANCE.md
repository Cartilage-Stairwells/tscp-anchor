# Benchmark Provenance Index

This index registers evidence baselines captured for the TSCP project.
Each baseline is an immutable artifact identified by its evidence identity.
Corrections require a new evidence identity, not modification of an existing one.

---

## Registered Evidence Baselines

| Evidence Identity | Backend | Hardware | Status | Record SHA-256 | Git Tag / Branch |
|---|---|---|---|---|---|
| `firebird_74c6e5f` | AVX-512 CPU | Intel Ice Lake-SP (family=6, model=106) | Sealed | `0d9c2c8e...` | `benchmark/firebird_74c6e5f` |
| `experiment_a_afea62bc` | AVX-512 CPU (kernel) | AMD Zen 5 (family=191, model=2) | Registered | `afea62bc...` | `evidence/experiment-a` (zksha-rx-reviewer-access) |

---

## Evidence Identity Convention

Each evidence baseline receives a unique identifier. The firebird series uses
`firebird_<8hex>` for end-to-end proving pipeline baselines. The experiment series
uses `experiment_<letter>_<8hex>` for isolated kernel-level measurements.

The identifier is permanent once sealed or registered. A superseded baseline
retains its identity and status; it is never renamed or overwritten.

### Cross-Repository Evidence

`experiment_a_afea62bc` is a cross-repository evidence baseline. Its evidence
artifacts reside in `Cartilage-Stairwells/zksha-rx-reviewer-access` on the
`evidence/experiment-a` branch (commit `8df0c247`). This index registers the
identity and cross-references the source repository; it does not duplicate the
evidence artifacts.

## Comparison Architecture

```
CPU evidence (end-to-end)       CPU evidence (kernel-level)
firebird_74c6e5f               experiment_a_afea62bc
        |                               |
        +---------------+---------------+
                        |
                        v
              comparison artifact
              (separate claim, references both)
```

A comparison between end-to-end and kernel-level evidence is a **separate claim**
that identifies both evidence baselines. Neither baseline is the implicit
denominator for the other.

### Methodology Distinction

`firebird_74c6e5f` measures end-to-end proving pipeline performance with
`target-cpu=icelake-server`.

`experiment_a_afea62bc` measures isolated kernel butterfly performance with
`target-cpu=x86-64` (prevents compiler auto-vectorization in scalar baseline).

The different scalar baselines produce different speedup ratios. Both are valid
under their respective methodologies. See the cross-reference document for details.

## Layer Separation

| Layer | Description | Mutability |
|---|---|---|
| Evidence | Captured artifacts (manifest, results, hashes) | Immutable |
| Attestation | Documentation, Git tags, GitHub releases | Describes evidence; does not modify it |
| Interpretation | Performance analysis, comparisons, conclusions | Evolves freely |

## Related Documents

- [FIREBIRD_AVX512_BASELINE.md](./FIREBIRD_AVX512_BASELINE.md) — CPU AVX-512 end-to-end evidence baseline attestation
- [EXPERIMENT_A_EVIDENCE_REFERENCE.md](./EXPERIMENT_A_EVIDENCE_REFERENCE.md) — Experiment A kernel-level cross-repository evidence reference
