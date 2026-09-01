# External Review Outreach Materials

## 1. Plonky3 GitHub Discussion Post

**Forum:** https://github.com/Plonky3/plonky3/discussions
**Title:** Formal verification of NTT + custody verification protocol built on Plonky3 — looking for feedback

**Body:**

I've built a custody verification protocol (TSCP) on top of Plonky3 — using BabyBear, Poseidon2, and MerkleTreeMmcs — and I'm looking for feedback from the Plonky3 community. I want to be upfront about what's proven, what's implemented, and what's still open.

**What it does:** TSCP separates artifact identity, observation, computational binding, independent verification, and promotion into explicit custody stages. The core principle: a declared hash is an assertion, not a verification.

**What I've built on Plonky3:**
- FRI commit/query/verify in the oracle layer using Plonky3's BatchMerkle MMCS (working prover + verifier with proper Fiat-Shamir, Merkle verification, fold consistency)
- Sumcheck protocol with self-checking prover (prover verifies its own proof before emitting). **Note: sumcheck verifier is not yet implemented.**
- AVX-512 NTT optimization (9.15x speedup over scalar baseline, 16-lane SIMD)
- On-chain anchoring (Sepolia testnet) via TSCPAnchor contract
- DEEP-ALI quotient computation (but not yet connected to the FRI prover)

**Formal verification (Lean 4):**
- 83 theorems proving NTT correctness over BabyBear
- Mathematical layer (Montgomery multiplication, NTT butterfly, stage decomposition) fully proven
- 3 open machine-refinement axioms (AVX-512 intrinsics = scalar ops), backed by 104 differential test vectors
- Categorical verification backbone (Kernel/Universe/Bridge) proving verification properties compose across systems — composition theorem is mechanically proven
- 2 engineering axioms (NTT bridge certificate, end-to-end preservation) backed by empirical evidence

**What I know is missing (being transparent):**
- The sumcheck verifier is not implemented
- The DEEP-ALI quotient is computed but not connected to the real FRI prover
- The on-chain FRI verifier is a scaffold
- Proof serialization omits Merkle openings and uses debug-format encoding
- **Most importantly: the proof system currently proves properties about trace columns, but does not yet cryptographically bind an external artifact to the computation output.** The protocol defines this binding (Bound(A, C, W) => A = Canonicalize(Output(C))) as a requirement, but the implementation does not yet establish it. This is the central open problem.

**Adversarial audit:** I ran an ARCHER methodology audit (36 findings across the full stack: 23 fixed, 13 documented as open engineering boundaries). Findings clustered at system boundaries — configuration defaults, input validation, placeholder code. No defects found in the FRI/Merkle/Montgomery cryptographic core. Full details in the commit history.

I'd appreciate feedback on:
1. The formal verification approach — is the axiom structure (math proven, machine open) reasonable, or should I be doing something different?
2. The artifact binding gap — has anyone addressed the problem of binding a ZK proof to an external artifact (not just proving trace properties)?
3. The custody pipeline concept — does separating identity, resolution, and provenance binding fill a gap you've seen?
4. Any issues with how I'm using Plonky3's primitives

All code is open source: github.com/Cartilage-Stairwells/tscp-anchor and github.com/Cartilage-Stairwells/zksha-rx

---

## 2. Outreach to ZKSecurity (email/message template)

**To:** contact@zksecurity.xyz (or their Lean 4 ZK verification team)
**Subject:** Lean 4 formal verification of NTT for FRI-based proof systems — review request

**Body:**

Hi,

I've been following ZKSecurity's work on Clean (the Lean 4 formal verification DSL for ZK circuits) and I think there may be overlap with a project I've been working on.

I've built a formal verification of NTT correctness over the BabyBear field in Lean 4 — 83 theorems covering Montgomery multiplication, butterfly operations, and stage decomposition. The mathematical layer is fully proven; the machine refinement layer (AVX-512 intrinsics) has 3 open axioms backed by 104 test vectors.

I've also built a categorical verification backbone (Kernel/Universe/Bridge) that proves verification properties compose across ZK proof systems, with an implementation on Plonky3.

I'm a solo developer with no institutional affiliation, and I'm looking for an independent review of the formal verification work. Would you or someone on your team be willing to take a look? Even a brief assessment of whether the approach is sound would be enormously valuable.

The code is at github.com/Cartilage-Stairwells/zksha-rx (Lean 4 proofs) and github.com/Cartilage-Stairwells/tscp-anchor (implementation).

I'm also preparing a paper for IACR ePrint and would be happy to share the draft.

Thanks for your time,
Sean Southwick

---

## 3. Outreach to LambdaClass (email/message template)

**To:** LambdaClass (via their blog contact or GitHub)
**Subject:** Lean 4 formal verification of ZK NTT — review request

**Body:**

Hi,

I read your post "If It Compiles, It Is Correct" on using Lean 4 for ZK systems and found it very relevant to work I've been doing.

I've formalized NTT correctness over the BabyBear field (Plonky3's field) in Lean 4 — 83 theorems covering the mathematical layer (Montgomery multiplication, butterfly, stage decomposition) with a categorical backbone proving verification property composition. The machine refinement layer (AVX-512) has 3 open axioms.

I'm looking for independent review of the formal verification approach. Would anyone at LambdaClass be open to taking a look?

Code: github.com/Cartilage-Stairwells/zksha-rx
Paper draft available on request.

Thanks,
Sean Southwick

---

## 4. IACR ePrint Submission Notes

**URL:** https://eprint.iacr.org/submit

**Process:**
- IACR ePrint is a preprint server — papers are posted by authors, no peer review required
- Need an IACR account (free to create)
- Submission is a PDF + metadata (title, authors, abstract)
- Papers appear within 1-2 business days
- Once posted, the paper is publicly visible and citable

**Before submitting:**
- Convert the paper draft (Markdown) to PDF (LaTeX recommended, but not required)
- Add proper author information (can use pseudonym if concerned about privacy, but real names are preferred)
- Ensure references are complete (some refs in the draft need URLs/dates filled in)
- Consider adding an "independent review" note in the acknowledgments

**After submitting:**
- Post the ePrint link to:
  - Plonky3 GitHub Discussions
  - Reddit r/crypto and r/zkProofs
  - Twitter/X (if Sean has or creates an account)
  - The IACR Crypto Discord (if it exists)

---

## 5. Outreach Strategy

### Who to contact first (lowest friction, highest likely response)

1. **Plonky3 GitHub Discussions** — free, public, community sees it. Post first.
2. **IACR ePrint** — submit the paper. This makes all other outreach more credible ("I have a paper on ePrint").
3. **ZKSecurity** — they do Lean 4 ZK formal verification for a living. Cold email with the paper link.
4. **LambdaClass** — they've written about Lean 4 for ZK. Similar cold email.

### Who to contact after paper is on ePrint

5. **Polygon/Plonky3 team directly** — now with a citable paper, reach out about the custody verification layer
6. **Ethereum Foundation ESP** — reference the paper in the grant application
7. **Open Philanthropy** — reference the paper in the EOI

### Realistic expectations

- Plonky3 GitHub: expect 0-3 responses, mostly from community members not core devs
- IACR ePrint: no direct feedback, but establishes a citable artifact
- ZKSecurity: may or may not respond — they're a company, not academics. But if they do, their review would be high-quality
- LambdaClass: similar — may not respond, but if they do, they understand the Lean 4 + ZK intersection

**The key insight:** one external review is enough to transform "self-verified" into "independently reviewed." It doesn't need to be a comprehensive audit — even a brief assessment that the approach is sound (or that specific issues exist) adds enormous credibility.
