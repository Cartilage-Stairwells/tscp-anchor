# tscp-anchor

TSCP artifact anchoring agent. Hashes JSON documents via keccak256 and commits proofs to a local EVM chain via `TSCPAnchor.sol`.

## Usage
```bash
node anchor-agent.mjs ./path/to/artifact.json
node anchor-batch.mjs ./tscp-docs/
After that the repo is production-ready. Summary of what's live at `Triune-Oracle/tscp-anchor`:

- `TSCPAnchor.sol` — immutable on-chain hash registry
- `anchor-agent.mjs` — single file anchorer with CLI arg support
- `anchor-batch.mjs` — directory batch anchorer with manifest output
- `manifest.json` — current artifact registry with hashes + block numbers
- `.gitignore` — clean tracking

Ready to transfer to `sacred-asymmetry` or move on to the next track?
