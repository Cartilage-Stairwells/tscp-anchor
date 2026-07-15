# ARCHIVE_INDEX.md

This document is the master index for all repositories across **Triune-Oracle** (canonical production)
and **Cartilage-Stairwells** (archive / experimental lab).

## Hybrid Model

| Account | Role |
|---|---|
| **Triune-Oracle** | Canonical production: active protocol, verification, and tooling |
| **Cartilage-Stairwells** | Archive / lab: historical research, experiments, compliance anchors |

A new contributor can identify the authoritative code, understand historical lineage,
and verify security posture without requiring private context.

---

## Repository Index

| Repo | Account | Cluster | Status | Date Range | Canonical Replacement | Notes |
|:---|:---|:---|:---|:---|:---|:---|
| **avx512-butterfly** | Cartilage-Stairwells | COMPUTE | MIGRATED → T-O | 2025– | triune-kernel | AVX-512 NTT butterfly kernels, BabyBear field, IEP layer. Lab remains as historical compute origin. |
| **tscp-pl-phase1** | Cartilage-Stairwells | TSCP | ARCHIVE (compliance anchor) | Phase 1 freeze | None | 27 Lean 4 theorems, 0 sorry. `phase1-freeze` is read-only. Do not rebase or force-push. |
| **tscp-anchor** | Cartilage-Stairwells | TSCP | MIGRATED → T-O (canonical) | 2025– | Triune-Oracle/tscp-anchor | 1 commit ahead of T-O version. History contains 299MB packfile bloat from node_modules + target/. git-filter-repo cleanup pending. |
| **tscp-anchor** | Triune-Oracle | TSCP | MIGRATE (sync from C-S) | 2025– | ← C-S version is canonical | Keccak256 EVM commitment layer. Plonky3 ZK stack. |
| **tscp-canon** | Triune-Oracle | TSCP | MIGRATE | Active | None | Conformance fixtures and canonical test vectors. 32MB target/ bloat pending cleanup. |
| **tscp-crown-capsule** | Triune-Oracle | TSCP | REVIEW | Legacy | None | PLpgSQL, 3kb. Purpose unclear — needs architectural classification. |
| **toolintell** | Triune-Oracle | TSCP / TOOLS | REVIEW → triune-tools | Active June 2026 | None | Contains TSCP runtime artifacts, FRI/STARK analysis, mutation harnesses. Likely triune-tools or triune-execution. |
| **triune-swarm-engine** | Triune-Oracle | TRIUMVIRATE | MIGRATE | Active | None | Python multi-agent orchestrator. 1 open issue. Core operational layer. |
| **Triune-Oracle** | Triune-Oracle | TRIUMVIRATE | MIGRATE | Active | None | Public org profile repo. |
| **TtriumvirateMonitor-Mobile** | Triune-Oracle | TRIUMVIRATE | ARCHIVE | Inactive | None | TypeScript mobile monitor. |
| **Triune_Command_UI** | Triune-Oracle | TRIUMVIRATE | ARCHIVE | Inactive | None | JS command UI. |
| **Legio-Cognito** | Triune-Oracle | TRIUMVIRATE | ARCHIVE | Historical | None | 35MB images in Vault_of_Relics/. Historical blueprints. |
| **Triune--retrieval--node** | Triune-Oracle | TRIUMVIRATE | ARCHIVE | Inactive | None | Retrieval node. |
| **MirrorWatcherAI** | Triune-Oracle | TRIUMVIRATE | ARCHIVE | Inactive | None | Monitoring agent. |
| **Trumvirate-System-Memory-Merge-Protocol** | Triune-Oracle | TRIUMVIRATE | ARCHIVE | Legacy | None | Note: typo in name. Deprecated merge protocol. |
| **triumvirate-agent-framework** | Triune-Oracle | TRIUMVIRATE | ARCHIVE | Legacy | None | Prior agent framework. |
| **logos-agency-mvp-dashboard** | Triune-Oracle | LOGOS | REVIEW | Active | None | TypeScript. 1 open issue. Possible active deployment. |
| **Logos_Agency** | Triune-Oracle | LOGOS | ARCHIVE | Historical | None | Go-based agency framework. |
| **LogosTalisman** | Triune-Oracle | LOGOS | ARCHIVE | Legacy | None | |
| **logostalisman-presentation** | Triune-Oracle | LOGOS | ARCHIVE | Legacy | None | Presentation materials. |
| **logostalisman-marketing** | Triune-Oracle | LOGOS | ARCHIVE | Legacy | None | Marketing collateral. |
| **logos-talisman-fractal-ai** | Triune-Oracle | LOGOS | ARCHIVE | Legacy | None | Experimental fractal AI. |
| **Adamantine-Spine** | Triune-Oracle | MISC | ARCHIVE | Legacy | None | Next.js + Stripe/Prisma payout ledger. Internal name: catch-basin. No active deployment found. |
| **catch-basin** | Triune-Oracle | MISC | ARCHIVE | Legacy | None | Related to Adamantine-Spine. |
| **glyphicspore** | Triune-Oracle | MISC | ARCHIVE | Legacy | None | Neo4j graph viz layer for TriumvirateSwarm. |
| **sovereign-scroll-cycle** | Triune-Oracle | MISC | ARCHIVE | Legacy | None | |
| **integrated-pipeline-platform** | Triune-Oracle | MISC | ARCHIVE | Legacy | None | |
| **pinata-api-keys** | Triune-Oracle | SECURITY ⚠️ | DELETE after scrub | — | None | Credentials rotated. History scrub required before deletion. |
| **CulturalCodex** | Triune-Oracle | SECURITY ⚠️ | REVIEW | Historical | None | .env with Polygon key + Infura key. Both revoked. History scrub required. |
| **keys-and-rituals** | Triune-Oracle | SECURITY | REVIEW | — | None | ManusAI deployment spec for thecodexai.com. No live credentials found. |
| **nextjs-ai-chatbot** | Triune-Oracle | STARTERS | ARCHIVE/DELETE | Template | None | |
| **empathic-voice-interface-starter** | Triune-Oracle | STARTERS | ARCHIVE/DELETE | Template | None | |
| **empathic-voice-interface-launcher** | Triune-Oracle | STARTERS | ARCHIVE/DELETE | Template | None | |
| **nextjs-ai-chatbot-launcher** | Triune-Oracle | STARTERS | ARCHIVE/DELETE | Template | None | |
| **nextjs-ai-hackbot** | Triune-Oracle | STARTERS | ARCHIVE/DELETE | Template | None | |
| **vite-react** | Triune-Oracle | STARTERS | ARCHIVE/DELETE | Template | None | |
| *(~20 remaining misc repos)* | Triune-Oracle | MISC | ARCHIVE/DELETE | Various | None | Inactive, templates, or disposable. Review individually before deletion. |

---

## Canonical Triune-Oracle Target Structure

```
Triune-Oracle/
  triune-protocol       ← tscp-pl-phase1 content (frozen spec)
  triune-verifier       ← serialization seal + verification package
  triune-kernel         ← avx512-butterfly (NTT compute core)
  triune-execution      ← tscp-anchor (cleaned, history-rewritten)
  triune-benchmarks     ← π corpus, FRI/STARK analysis artifacts
  triune-tools          ← toolintell + utilities
  triune-documentation  ← architecture docs, dossier, this index

Cartilage-Stairwells/
  avx512-butterfly      ← lab origin (retained, read-only)
  tscp-pl-phase1        ← compliance anchor (phase1-freeze, permanent)
  tscp-anchor           ← canonical pre-migration state
```

---

## Pending Actions

| Action | Target | Priority |
|---|---|---|
| git-filter-repo bloat cleanup | tscp-anchor (both accounts) | HIGH |
| git-filter-repo bloat cleanup | tscp-canon | HIGH |
| History scrub + delete | pinata-api-keys | HIGH |
| History scrub | CulturalCodex (.env) | HIGH |
| GPG key setup + enforce signing | All active repos | HIGH |
| Add LICENSE, CHANGELOG, CONTRIBUTING | TSCP cluster (4 repos) | HIGH |
| Deploy CI verification gates | tscp-anchor, tscp-canon | HIGH |
| Classify and migrate | toolintell | MEDIUM |
| Classify | tscp-crown-capsule | MEDIUM |
| Archive or delete | ~30 inactive repos | LOW |

---

*Date: 2026-07-14*
*Maintained by Triune-Oracle*
