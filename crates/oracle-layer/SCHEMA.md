# Golden Vector Schema Lock — v1

**Protocol:** TSCP-PL v1.963
**Lineage:** LogosTalisman-GoldenVector-v1
**Gate:** Gate 5 — Orion Boundary Artifact
**Schema:** v1 (LOCKED)

## Conformance Checklist

1. `schema_version == "v1"`
2. `prime == "0x78000001"` and `prime_decimal == "2013265921"`
3. `montgomery_r == "2^32"`
4. All field arithmetic vectors satisfy mod P arithmetic
5. All butterfly vectors satisfy `a' = a + w*b`, `b' = a - w*b`
6. All NTT stage traces match DIT algorithm
7. `roundtrip_verified == true` and `naive_dft_match == true`
