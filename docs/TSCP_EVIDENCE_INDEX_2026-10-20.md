# TSCP Evidence Index — 2026-10-20

This page identifies the current Cartilage-Stairwells evidence surfaces. Older
paper drafts, outreach drafts, and benchmark narratives remain in the repository
as historical working material and must not be read as superseding the current
claims statement.

## Current evidence surfaces

| Layer | Repository | Disposition |
|---|---|---|
| Canonical serialization | [tscp-canon](https://github.com/Cartilage-Stairwells/tscp-canon) | Public, Apache-2.0, 15/15 cross-runtime conformance receipt |
| Formal TSCP model | `tscp-anchor` | Public, scoped C1 model result |
| AVX-512 implementation and sealed captures | [avx512-butterfly](https://github.com/Cartilage-Stairwells/avx512-butterfly) | Access-controlled, current implementation evidence |
| Phase 1 legacy artifact | `tscp-pl-phase1` | Private, historical microbenchmarks and formal artifact; provenance and licensing unresolved |
| Serialization custody and rebinding | `tscp-zkptp` | Private, V1/V2 identity migration and custody records |
| Formal refinement artifact | `zksha-rx` | Private, provenance and licensing review; C3-ISA boundary open |
| Reviewer package | [zksha-rx-reviewer-access](https://github.com/Cartilage-Stairwells/zksha-rx-reviewer-access) | Public curated surface; mixed-source licensing boundary documented, no blanket license claim |
| P0 resolver baseline | [tscp-v04-p0-baseline](https://github.com/Cartilage-Stairwells/tscp-v04-p0-baseline) | Public, Apache-2.0, bounded resolver evidence |

## Claim boundary

The Phase 1 `9.15x` DIT and `4.58x` XOR figures are legacy kernel
microbenchmarks against their stated scalar baselines. They are not the current
C4 comparison and must not be combined with the later Firebird fused-pass results.
The current C4 performance claim is limited to the named hosts, pinned Plonky3
version, measured pass, and sealed captures documented by the claims statement.

Lean model results, SMT results, finite differential tests, and performance
measurements establish different things. None is a universal implementation
correspondence result unless that correspondence is separately proved.

## Outcome vocabulary

Evidence gates use **ADMITTED**, **REFUSED**, or **ADMITTED UNDER REBIND**. A
failed or incomplete obligation is retained as evidence about the boundary; it is
not converted into an unconditional success claim.

## Provenance boundary

Public visibility is not a license or an authorship determination. The reviewer
package and the private Phase 1/zksha-rx surfaces contain mixed technical
material whose file-level ancestry is still being resolved. Upstream-derived
components retain their applicable terms.
