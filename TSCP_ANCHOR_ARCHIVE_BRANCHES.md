# TSCP Anchor Historical Branch Archive

## Purpose

This document records historical branch preservation after
history cleanup.

The archive namespace preserves engineering provenance without
reintroducing historical material into the canonical custody line.

## Canonical Branch

master

The master branch represents the post-history-cleanup canonical lineage.

## Historical Archive Namespace

refs/heads/archive/*

## Archived Branches

| Branch | Commit | Purpose |
|---|---|---|
| archive/m8-polyir-lowering | faa08cfa | PolyIR lowering, verifier evolution, build contract history |
| archive/next-dev | 23b6e174 | Development transition lineage |
| archive/phase2-instrumentation | 2ccd0103 | Admission control and instrumentation history |
| archive/post-freeze | 23b6e174 | Verifier freeze transition |
| archive/proof-layer-clean | b99ffa0f | Proof and provenance verification lineage |
| archive/tscp-v0.6.1-dogfood | fcf8476d | Release validation and dogfood evidence |

## Archive Rules

- Historical reference only.
- Not merged into canonical lineage.
- Preserve commit identity.
- No force pushes.
- Changes require explicit archive decision.

