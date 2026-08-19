# Aria's Second Independent Red-Team Review of the Admissibility Contract Specification

**Reviewer:** Aria (ChatGPT)
**Date:** August 18, 2026
**Posture:** Hostile / adversarial
**Subject:** ADMISSIBILITY_CONTRACT_SPEC v0.1

---

## Executive Disposition: HOLD → experimental implementation justified

The architecture has crossed an important threshold: the conceptual custody boundary is now explicit enough to test experimentally.

It has not yet crossed the stronger threshold where AdmittedEvidence can safely be treated as a security-bearing type under an unspecified implementation language and threat model.

The central reason: **A type distinction is not itself a custody guarantee.**

| Area | Disposition |
|:---|:---|
| Conceptual separation of evidence and authority | PASS |
| Evidence → admit() → AdmittedEvidence as intended flow | PASS at specification level |
| AdmittedEvidence inherently conferring authority | PASS — it need not, provided the contract stays narrow |
| Validation / Binding / Completeness decomposition | HOLD |
| Type-level enforcement | HOLD |
| Serialization / persistence | HOLD |
| Unsafe / FFI / reflection / generated code | HOLD |
| Hostile inputs | PASS as explicitly separate threat surface |
| Custody/specification relationship | PASS conceptually; HOLD mechanically |
| Sufficient for minimal experimental implementation | PASS, conditionally |
| Sufficient as implementation-independent security guarantee | FAIL |

---

## Key Findings

### AdmittedEvidence Interpretation (§2)

Four possible interpretations of what AdmittedEvidence means:
- **A:** "Evidence satisfied syntactic/structural requirements to enter internal evaluation domain" — CORRECT
- **B:** "Evidence is valid" — DANGEROUS
- **C:** "Evidence is true/correct" — NOT ESTABLISHED
- **D:** "Evidence is authorized to influence the system" — CATASTROPHIC (collapses admissibility into authority)

Spec must explicitly define AdmittedEvidence as A and reject B–D.

### Semantic Laundering Risk (§7, §24, §25)

The composition attack: V∧B∧C → AdmittedEvidence → "trusted" → "correct" → "authorized"

The spec needs explicit non-implications:
- AdmittedEvidence ⇏ Truth
- AdmittedEvidence ⇏ Correctness
- AdmittedEvidence ⇏ Authenticity
- AdmittedEvidence ⇏ Authority

### Forbidden Arrows (§24, §25)

- `AdmittedEvidence → Authority` — admission must not be an authority-producing operation
- `AdmittedEvidence → Truth` — admission must not establish truth

Correct architecture:
```
Evidence → Admission → AdmittedEvidence → Evaluation → Decision → Transition Contract → Custody State → Authority
```

### Stage Semantics (§4-6, §26)

- **Validation:** "Does evidence satisfy required structural/contractual validity conditions?" NOT "is evidence true"
- **Binding:** "Is evidence associated with the correct canonical specification/context?" NOT "does specification endorse the claim"
- **Completeness:** "Does evidence contain everything required by the admission contract relative to the specified completeness criterion?" NOT "nothing relevant is missing from reality"

### Implementation-Level Concerns (§8-18)

All HOLD: private constructor (language-dependent), serialization (second admission path), reflection, unsafe code, FFI, memory/layout, generated code/macros, type erasure, persistence boundaries.

### 15-Point Success Criteria for Experimental Validation (§29)

1. Evidence cannot directly enter evaluate()
2. AdmittedEvidence cannot be constructed externally
3. Every construction path is enumerated
4. Serialization cannot silently manufacture admission
5. Persistence cannot silently manufacture admission
6. FFI cannot silently manufacture admission
7. Unsafe facilities are explicitly inside/outside the TCB
8. Generated code cannot introduce an unauthorized constructor path
9. Type erasure cannot bypass re-admission
10. Validation establishes no stronger proposition than specified
11. Binding establishes no stronger proposition than specified
12. Completeness establishes no stronger proposition than specified
13. AdmittedEvidence contains no authority semantics
14. Authority appears only through the downstream custody transition
15. Specification meaning is preserved rather than inferred from implementation behavior

---

## Most Important Conclusion

> AdmittedEvidence is defensible as an admissibility fact, but only if the system refuses to interpret that fact as truth, correctness, authenticity, or authority.

> The most important engineering risk is not the obvious constructor. It is semantic laundering.

> The kernel succeeds if that chain is mechanically broken. The current specification does break it conceptually. The next question is whether a real language/runtime can be made to preserve that break.

~Aria
