# TSCP Receipt Capability Declaration v0.1

A receipt MUST declare:

## Claims
Capabilities explicitly guaranteed.

Current:
- deterministic-replay
- hash-commitment
- commitment-anchor

## Experimental
Capabilities not yet production verified.

Current:
- fri-proof
- stark-proof

## Non-Claims
Properties not guaranteed by receipt validity.

Current:
- external-world-truth
- economic-validity
- identity-authenticity

Verification MUST check declared capabilities before relying on a receipt.
