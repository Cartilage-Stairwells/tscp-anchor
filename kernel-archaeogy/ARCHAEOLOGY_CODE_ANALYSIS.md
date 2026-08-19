# TSCP DELTA Archive — Technical Architecture, Contracts & Code Analysis

**Author:** Codebase Archaeology Task Force  
**Target Directory:** `/app/conversations/6a8516d2ee9e9ad0fb8ad3ba/tscp_delta/`  
**Date:** August 19, 2026  
**Status:** COMPLETE & GOVERNING  

---

## Executive Summary

This archaeological code analysis examines the codebase of the **TSCP DELTA archive**. The archive contains two distinct operational layers:
1. **The Normative Core Specification & Conformance Engine (`TSCP-CANON-001 v1.0`)**: A rigorous, deterministic JSON canonicalization engine implemented in TypeScript (`canonical.ts`), verified by a test harness (`canon_conformance.test.ts`, `run_conformance.ts`), and defined by an independent oracle manifest (`manifest.json`).
2. **The Multi-Agent Handoff & Governance Integration Layer (`TRIUNE Handoff Module`)**: A state-encapsulation and tRPC routing framework (`types.ts`, `services.ts`, `router.ts`, `index.ts`) that serializes agent execution runs into immutable "Handoff Capsules" and "Ritual Capsules" for multi-agent synthesis across heterogeneous LLMs (DeepSeek, Claude, ChatGPT).

Additionally, the archive contains **Python reconstruction utilities** (`extract_relay.py`, `extract_tscp.py`, `extract_bundle.py`) designed to unpack, recover, and re-establish system environments from raw archive dumps and base64 payloads, along with deployment configurations (`package.json`, `ecosystem.config.js`, `docker-compose.lite.yml`, `.env.delta_10_3.template`).

---

## 1. Analysis of TypeScript Codebase (7 Files)

### 1.1 `canonical.ts`
* **What It Does**:  
  Reference TypeScript implementation of **TSCP-CANON-001 v1.0** (JSON Canonicalization Standard). Converts raw JSON input strings into canonical, deterministic UTF-8 byte arrays (`Uint8Array`). It enforces structural validation, Unicode NFC normalization, duplicate key collision detection, numeric integer policy, and deterministic key sorting entirely within the function body prior to serialization.

* **Key Types & Interfaces Defined**:
  * `type JsonValue`: Union type representing valid JSON nodes (`null | boolean | string | RawNumber | JsonValue[] | { [key: string]: JsonValue }`).
  * `interface RawNumber`: Internal tagged object `{ __rawNumeric: true; text: string }` capturing exact numeric source text prior to `JSON.parse` coercion.
  * `function isRawNumber(v: unknown): v is RawNumber`: Type guard for tagged raw numbers.
  * `function isPlainObject(v: unknown): v is Record<string, unknown>`: Type guard distinguishing JSON objects from arrays/numbers.
  * `function canonicalize(rawInput: string): Uint8Array`: The primary public entry point for canonical byte generation.
  * **Error Taxonomy Referenced**: Emits `CanonicalizationError` (from `./errors`) with spec codes:
    * `TSCP-CANON-FLOAT-PROHIBITED` (Stage: `NUMERIC`): Rejects any numeric token containing `.` or `e`/`E`.
    * `TSCP-CANON-TOPLEVEL-NONMAP` (Stage: `VALIDATION`): Rejects non-object top-level JSON structures.
    * `TSCP-CANON-NON-STRING-KEY` (Stage: `VALIDATION`): Rejects non-string dictionary keys.
    * `TSCP-CANON-KEY-COLLISION` (Stage: `NORMALIZATION`): Rejects objects where keys collapse to the same string after NFC normalization.

* **Dependencies & Imports**:
  * **Imports**: `./errors` (`CanonicalizationError`).
  * **Globals**: `TextEncoder`, `JSON.parse`, `JSON.stringify`, `Object.keys`, `Object.entries`.
  * **Dependents**: `canon_conformance.test.ts`, `run_conformance.ts`.

* **Provenance**:  
  **Original Implementation.** Normative, authoritative reference code for TSCP-CANON-001.

* **Invariants & Assertions Enforced**:
  1. *Top-Level Object Assertion*: Input MUST parse to a plain JSON object (map); top-level arrays, strings, or numbers throw `TSCP-CANON-TOPLEVEL-NONMAP`.
  2. *Integer-Only Policy*: Floating-point numbers are strictly prohibited. Any literal containing `.` or exponent notation (`e`/`E`) throws `TSCP-CANON-FLOAT-PROHIBITED`.
  3. *NFC String Normalization*: All strings and object keys are normalized to Unicode Normalization Form C (`s.normalize("NFC")`).
  4. *Key Collision Prevention*: Object keys that collide after NFC normalization trigger `TSCP-CANON-KEY-COLLISION`.
  5. *Lexicographical Key Sorting*: Object keys are sorted alphabetically before string encoding.
  6. *Whitespace Elimination*: Output canonical JSON bytes contain zero unnecessary formatting whitespace.

---

### 1.2 `services.ts`
* **What It Does**:  
  Domain service layer for generating, transforming, storing, and synthesizing agent handoff capsules and ritual prompt payloads. Bridges database state records (`triuneRuns`, `triuneSteps`) with higher-level TSCP specification placeholders.

* **Key Functions & Transformations Defined**:
  * `generateCapsule(runId: number): Promise<TriuneHandoffCapsule>`: Assembles a complete handoff capsule containing run metadata, ordered execution steps, a `tscpSpec` placeholder, and an ISO timestamp.
  * `toLiteCapsule(full: TriuneHandoffCapsule): TriuneHandoffCapsuleLite`: Condenses full capsule into summary stats (`summary`, `lastStepType`).
  * `toRitualCapsule(full: TriuneHandoffCapsule): TriuneRitualCapsule`: Formats capsule data into an agent prompt/ritual payload with `mission`, `phase`, `anchors` (step type list), `ask`, and `context`.
  * `storeRitualCapsule(...)`: Persists ritual data and capsule JSON into the `ritualCapsules` table.
  * `fetchLatestCapsuleForRun(...)`: Retrieves the most recent ritual capsule for a given run ID.
  * `recordHandoffDelivery(...)`: Logs responses from downstream receiver models in `handoffDeliveries`.
  * `synthesizeHandoff(...)`: Merges responses across multiple receivers into a unified consensus recommendation string.
  * `maybeGenerateHandoffCapsule(...)`: Auto-triggers capsule creation if `options.force` is set, if `lastStep.type` is `'SYMBOLIC'` or `'MONETIZATION'`, or if run age exceeds `durationThreshold` (default: 1 hour).

* **Dependencies & Imports**:
  * **Imports**: `./types` (`TriuneHandoffCapsule`, `TriuneRitualCapsule`, `TriuneHandoffCapsuleLite`), `./db/schema` (`ritualCapsules`, `handoffDeliveries`, `triuneRuns`, `triuneSteps`), `drizzle-orm` (`eq`, `desc`).
  * **Dependents**: `router.ts`.

* **Provenance**:  
  **Later Reconstruction / Integration Layer.** Connects execution step runs with handoff capsule data structures. Uses a stubbed database client (`const db: any = {}`).

* **Invariants & Assertions Enforced**:
  * Mandatory capsule trigger on specific step classifications (`SYMBOLIC`, `MONETIZATION`).
  * Step ordering guaranteed via `orderBy(triuneSteps.stepIndex)`.
  * Safe fallback to step type `'NONE'` when step array is empty.

---

### 1.3 `router.ts`
* **What It Does**:  
  Defines tRPC API routers (`handoffRouter`, `handoffUiRouter`, and root `appRouter`) using `@trpc/server` and Zod validation. Provides RPC procedures for broadcasting handoff prompts to external AI receivers, generating/fetching capsules, and running synthesis procedures.

* **Key Routers & Endpoints Defined**:
  * `handoffRouter.broadcast`: Mutation procedure that fetches the latest ritual capsule for `runId` and simulates dispatching handoff prompts across a list of target receivers (e.g., DeepSeek, Claude, ChatGPT), recording each delivery.
  * `handoffUiRouter.generateCapsule`: Mutation procedure to generate and store a new ritual capsule.
  * `handoffUiRouter.getLatestCapsule`: Query procedure fetching latest ritual capsule.
  * `handoffUiRouter.synthesize`: Mutation procedure aggregating model responses for a given `ritualId`.
  * `AppRouter`: Type export `export type AppRouter = typeof appRouter;`.

* **Dependencies & Imports**:
  * **Imports**: `@trpc/server` (`initTRPC`), `zod` (`z`), `./services` (`* as services`).
  * **Dependents**: External frontend/API consumers.

* **Provenance**:  
  **Later Reconstruction / Integration Layer.** Uses tRPC for multi-agent dispatch and UI communication.

* **Invariants & Assertions Enforced**:
  * Zod runtime validation on input parameters (`runId` as number, `receivers` as string array, `ritualId` as number).
  * Enforces existence of ritual capsule prior to broadcasting (`if (!ritual[0]) throw new Error('No ritual capsule found for this run.')`).

---

### 1.4 `types.ts`
* **What It Does**:  
  Defines Zod schemas and TypeScript types for the core TRIUNE run models and TSCP handoff capsule artifacts.

* **Key Schemas & Types Defined**:
  * `TriuneStepSchema`: Zod schema for single execution step (`id`, `runId`, `stepIndex`, `type`, `content`, `createdAt`).
  * `TriuneRunSchema`: Zod schema for run metadata (`id`, `name`, `status`, `createdAt`, `updatedAt`).
  * `TriuneHandoffCapsuleSchema` / `TriuneHandoffCapsule`: Represents full state snapshot (`run`, `steps`, `tscpSpec`, `timestamp`).
  * `TriuneHandoffCapsuleLiteSchema` / `TriuneHandoffCapsuleLite`: Summary format (`runId`, `summary`, `lastStepType`, `timestamp`).
  * `TriuneRitualCapsuleSchema` / `TriuneRitualCapsule`: Prompt construction object (`mission`, `phase`, `anchors`, `ask`, `context`).

* **Dependencies & Imports**:
  * **Imports**: `zod` (`z`).
  * **Dependents**: `services.ts`, `router.ts`.

* **Provenance**:  
  **Reconstruction / Integration Layer.** Defines data contracts between agent runs and protocol handoffs.

* **Invariants & Assertions Enforced**:
  * Enforces strict primitive types and structured objects across all agent state transfers.

---

### 1.5 `index.ts`
* **What It Does**:  
  Database schema definition using Drizzle ORM (`pg-core`). Defines PostgreSQL tables for core runs/steps and handoff capsule deliveries.

* **Key Tables & Schemas Defined**:
  * `triuneRuns`: Table `triune_runs` (`id` serial PK, `name`, `status`, `createdAt`, `updatedAt`).
  * `triuneSteps`: Table `triune_steps` (`id` serial PK, `runId` FK, `stepIndex`, `type`, `content` jsonb, `createdAt`). Index on `(runId, stepIndex)`.
  * `ritualCapsules`: Table `ritual_capsules` (`id` serial PK, `runId` FK, `ritualData` jsonb, `capsuleData` jsonb, `createdAt`).
  * `handoffDeliveries`: Table `handoff_deliveries` (`id` serial PK, `ritualId` FK, `receiver` text, `response` text, `metadata` jsonb, `createdAt`). Index on `(ritualId, receiver)`.

* **Dependencies & Imports**:
  * **Imports**: `drizzle-orm/pg-core`.
  * **Dependents**: `services.ts`.

* **Provenance**:  
  **Reconstruction / Persistence Layer.**

* **Invariants & Assertions Enforced**:
  * Foreign key constraints (`triuneSteps.runId` -> `triuneRuns.id`, `ritualCapsules.runId` -> `triuneRuns.id`, `handoffDeliveries.ritualId` -> `ritualCapsules.id`).
  * Not-null constraints on primary identifier and content columns.

---

### 1.6 `canon_conformance.test.ts`
* **What It Does**:  
  Jest test suite that executes the **TSCP-CANON-001 v1.0 conformance corpus** against the implementation in `canonical.ts`. Reads test case descriptions from `manifest.json`, loads input fixture files, invokes `canonicalize()`, and verifies SHA-256 hashes or error rejections.

* **Key Types Defined**:
  * `interface ManifestCase`: Represents test specification (`id`, `status`, `input_file`, `expected_sha256`, `expected_output_file`, `error_code`, `error_stage`).
  * `interface Manifest`: Test manifest container (`manifest_schema_version`, `canon_version`, `test_cases`).

* **Dependencies & Imports**:
  * **Imports**: `fs`, `path`, `crypto` (`createHash`), `../src/canonical` (`canonicalize`), `../src/errors` (`CanonicalizationError`).
  * **Dependents**: Jest test framework runner.

* **Provenance**:  
  **Original Implementation.** Official conformance test suite for TSCP-CANON-001.

* **Invariants & Assertions Enforced**:
  * *REJECT cases*: Asserts that `canonicalize(rawInput)` throws a `CanonicalizationError` matching exact `error_code` and `error_stage`.
  * *SUCCESS cases*: Asserts computed SHA-256 hex digest equals `expected_sha256`, and binary bytes match `expected_output_file`.

---

### 1.7 `run_conformance.ts`
* **What It Does**:  
  Standalone Node.js CLI test execution runner. Runs the entire `manifest.json` conformance suite without requiring external test frameworks (like Jest). Prints formatted pass/fail lines and exits with code 0 or 1.

* **Key Structures Defined**:
  * Standalone CLI loop tracking `failures` and `total` count.

* **Dependencies & Imports**:
  * **Imports**: `fs`, `path`, `crypto`, `../src/canonical`, `../src/errors`.
  * **Dependents**: CI/CD scripts and manual CLI execution.

* **Provenance**:  
  **Original Utility.** Command-line runner for specification verification.

* **Invariants & Assertions Enforced**:
  * Enforces exact match on `CanonicalizationError` codes (`code` and `stage`) for REJECT cases.
  * Enforces exact byte-for-byte SHA-256 digest matching for SUCCESS cases.
  * Process exit code equals 0 if all cases pass, 1 if any fail.

---

## 2. Analysis of Python Utilities (3 Files)

### 2.1 `extract_relay.py`
* **What It Does**:  
  Extraction script that parses raw text/JSON payloads from pasted content (`/home/ubuntu/upload/pasted_content.txt`), extracts metadata into `canonical_source.json`, and programmatically recreates backend code (`server_enhanced.js`) and monitoring code (`triune_monitor.py`).

* **Dependencies & Imports**:
  * **Imports**: `json`, `os`, `re`.
  * **Outputs**: `/home/ubuntu/TSCP_DELTA_10_3/metadata/canonical_source.json`, `backend/server_enhanced.js`, `monitor/triune_monitor.py`.

* **Provenance**:  
  **Reconstruction Utility.** Written to recover runtime scripts from archived log dumps.

* **Invariants & Assertions Enforced**:
  * Handles JSON parsing errors via fallback regex matching (`r'\{.*\}'`).
  * Programmatically generates Express/WebSocket server with SQLite event tracking.

---

### 2.2 `extract_tscp.py`
* **What It Does**:  
  Parses raw input files to locate and salvage `{"TSCP": ...}` protocol metadata blocks. Uses regex pattern matching to reconstruct truncated JSON structures and writes `tscp_metadata.json` and a bash deployment script `deploy_delta_10_3.sh`.

* **Dependencies & Imports**:
  * **Imports**: `json`, `os`, `re`.
  * **Outputs**: `/home/ubuntu/TSCP_DELTA_10_3/metadata/tscp_metadata.json`, `scripts/deploy_delta_10_3.sh`.

* **Provenance**:  
  **Reconstruction Utility.**

* **Invariants & Assertions Enforced**:
  * Robust fallback mechanism for truncated JSON: extracts `metadata`, `version`, or `phase` fields via regex when JSON syntax is incomplete.

---

### 2.3 `extract_bundle.py`
* **What It Does**:  
  Unpacks structured multi-directory application bundles from archived JSON payloads. Iterates over bundle sections (`artifacts`, `audit`, `agents`, `metadata`, `proposal`, `LEGIO_ACTIVATION_WORKFLOW.md`) and decodes base64-encoded file strings into physical workspace files.

* **Dependencies & Imports**:
  * **Imports**: `json`, `os`, `base64`.
  * **Outputs**: Files written under `/home/ubuntu/TRIUNE_GENESIS_BLOCK_0/`.

* **Provenance**:  
  **Reconstruction Utility.**

* **Invariants & Assertions Enforced**:
  * Automatically detects base64 encoding via schema key check (`content_base64`) or string prefix heuristic (`'PT09'`) prior to decoding.

---

## 3. Analysis of Configuration & Environment Files (6 Files)

1. **`package.json`**:
   * **Role**: Package manifest for `triune-handoff-ts` (v1.0.0). Specifies module entry point (`index.js`).
2. **`manifest.json`**:
   * **Role**: Conformance manifest (schema v1.1, canon v1.0) generated by `reference_canon.py v1.1.0`. Defines 17 test cases (`001_key_sorting`, `002_whitespace_elimination`, `003_nested_and_array_order`, `004_unicode_nfc_normalization`, ..., `malformed_float`) with explicit input paths, output paths, expected SHA-256 hashes, and error rejection codes (`TSCP-CANON-FLOAT-PROHIBITED`, Stage: `NUMERIC`).
3. **`ecosystem.config.js`**:
   * **Role**: PM2 process deployment configuration for `TSCP_DELTA_10_3`. Defines three microservices:
     * `triune-backend`: Express Node.js server on port 5000.
     * `triune-ml-service`: Python ML service on port 8080.
     * `triune-monitor`: Python monitoring script with Slack webhook alerts.
4. **`docker-compose.lite.yml`**:
   * **Role**: Container orchestration file (v3.8) for Lite deployment mode. Configures `triune-backend` (Express + SQLite volume), `triune-frontend` (Nginx/HTTP on port 80), and `triune-monitor` with automatic container healthchecks.
5. **`.gitignore`**:
   * **Role**: Standard version control exclusion rules (`node_modules`, `dist`, `.env`, `*.log`).
6. **`.env.delta_10_3.template`**:
   * **Role**: Configuration template defining deployment parameters (`TRIUNE_API_KEY`, `JWT_SECRET`, database credentials, Redis configuration, `DEPLOYMENT_MODE=lite`).

---

## 4. Recurring Technical Primitives & Minimal Kernel Concepts

Across the code files, tests, scripts, and governing documents (`FOUNDING_DOCUMENT.md`), a clear set of recurring technical primitives emerges. These primitives form the core architecture of the **TSCP Evidence-to-Authority Kernel**.

```
┌──────────────────────────────────────────────────────────────────────────┐
│                   TSCP RECURRING TECHNICAL PRIMITIVES                     │
└──────────────────────────────────────────────────────────────────────────┘

  [ Raw Json Input ] ────► [ canonical.ts ] ────► [ Canonical UTF-8 Bytes ]
                                                         │
                                                         ▼
  [ Decision: Allow/Deny ] ◄── [ Kernel Evaluation ] ◄── [ SHA-256 Digest / Evidence ]
                                     ▲
                                     │
  [ Handoff Capsule ] ──────► [ Ritual Prompt ] ────► [ Multi-Agent Receivers ]
  (services.ts/types.ts)       (router.ts)            (Claude, DeepSeek, ChatGPT)
```

### 4.1 Recurring Primitives Found Across Files

1. **Deterministic Canonicalization (`canonicalize(raw) -> Uint8Array -> SHA-256`)**:
   * *Where it appears*: `canonical.ts`, `canon_conformance.test.ts`, `run_conformance.ts`, `manifest.json`.
   * *Role*: Forms the bedrock cryptographic primitive. Ensures that regardless of platform, language (TS/Python/Rust/Lean), or JSON parser, identical protocol objects produce identical byte representations and hashes. Enforces integer-only numeric policy and Unicode NFC normalization.

2. **Capsule Encapsulation & State Packaging (`TriuneHandoffCapsule` / `TriuneRitualCapsule`)**:
   * *Where it appears*: `types.ts`, `services.ts`, `router.ts`, `index.ts`.
   * *Role*: Encapsulates execution run context (`runId`, `steps`, `status`, `timestamp`) into immutable JSON payloads. Converts technical execution history into standardized "Ritual Capsules" designed for consumption by downstream agent models.

3. **Multi-Receiver Consensus & Broadcast (`broadcast` -> `recordHandoffDelivery` -> `synthesize`)**:
   * *Where it appears*: `router.ts`, `services.ts`, `index.ts`, `ecosystem.config.js`.
   * *Role*: Implements a fault-tolerant multi-agent pattern ("The Triumvirate"). Prompts are dispatched to multiple independent LLM receivers (e.g. Claude, ChatGPT, DeepSeek), individual responses are logged into `handoffDeliveries`, and a synthesis function merges them into a single recommendation.

4. **Resilient Extraction & Environment Recovery**:
   * *Where it appears*: `extract_relay.py`, `extract_tscp.py`, `extract_bundle.py`.
   * *Role*: Provides pattern-matching extraction and base64-decoding primitives to rebuild system components, metadata files, and server scripts from raw, unstructured, or truncated archive dumps.

---

### 4.2 The Minimal Kernel Model (Core Abstraction)

As articulated in `FOUNDING_DOCUMENT.md` and demonstrated in `canonical.ts`, the core TSCP system relies on a minimal set of fundamental concepts:

$$\text{Central Invariant: } \mathbf{\text{Evidence NEVER creates authority by itself.}}$$

The absolute minimal set of technical concepts required by the protocol is:

1. **Canonical Evidence**: Raw input payload normalized via NFC, key sorting, and integer restrictions, represented deterministically as canonical UTF-8 bytes and SHA-256 digests.
2. **Contract & Predicate**: Pure, deterministic rules governing state transitions without side effects (no network, no DB, no LLM during evaluation).
3. **Custody State**: The verified current state of authority or multi-agent execution (`triuneRuns`, `triuneSteps`).
4. **Proposed Transition**: A candidate state change submitted alongside canonical evidence.
5. **Deterministic Evaluation Function**:
   $$\text{evaluate}(\text{contract}, \text{evidence}, \text{predicate}, \text{current\_state}, \text{proposed\_transition}) \longrightarrow \text{Decision}$$
   Where $\text{Decision} \in \{\text{Allow}(\text{next\_state}), \text{Reject}(\text{reason}), \text{Hold}(\text{reason}), \text{Defer}(\text{reason})\}$.

---

## 5. Summary Matrix of Code Artifacts

| File Path | Primary Function | Language / Type | Provenance | Core Invariant / Assertion |
| :--- | :--- | :--- | :--- | :--- |
| `canonical.ts` | Canonicalization Engine | TypeScript (Core) | Original | Integer-only, NFC keys/strings, sorted keys, top-level map required. |
| `services.ts` | Handoff Capsule Service | TypeScript (Domain) | Reconstruction | Auto-trigger on `SYMBOLIC`/`MONETIZATION` steps or 1hr timeout. |
| `router.ts` | tRPC API Router | TypeScript (API) | Reconstruction | Zod schema validation; throws if ritual capsule is missing. |
| `types.ts` | Type Schemas | TypeScript (Types) | Reconstruction | Strict Zod runtime type enforcement for runs, steps, and capsules. |
| `index.ts` | DB Schema | TypeScript (ORMs) | Reconstruction | Foreign key referential integrity; compound indexes on step runs. |
| `canon_conformance.test.ts` | Conformance Test Suite | TypeScript (Tests) | Original | Rejection code/stage matching; exact SHA-256 digest comparison. |
| `run_conformance.ts` | CLI Conformance Runner | TypeScript (CLI) | Original | Identical to test suite; exits with code 0 on 100% pass, 1 on fail. |
| `extract_relay.py` | Relay Unpacker | Python (Tooling) | Reconstruction | Fallback regex JSON recovery; programmatically writes server code. |
| `extract_tscp.py` | Metadata Extractor | Python (Tooling) | Reconstruction | Regex recovery for truncated metadata (`version`, `phase`). |
| `extract_bundle.py` | Bundle File Extractor | Python (Tooling) | Reconstruction | Base64 heuristic decoding (`content_base64` or `'PT09'` prefix). |

---
*Analysis completed and saved to `/app/conversations/6a8516d2ee9e9ad0fb8ad3ba/tscp_delta/ARCHAEOLOGY_CODE_ANALYSIS.md`.*
