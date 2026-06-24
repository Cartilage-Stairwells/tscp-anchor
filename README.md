# ZKP Privacy Token

ERC-20 token with Zero-Knowledge Proof gated transfers.  
Built with Circom 2 · SnarkJS · Hardhat · OpenZeppelin · Anvil.

---

## Architecture

```
circuits/
  SecretKnowledge.circom     Poseidon commitment + range check circuit

contracts/
  Verifier.sol               Auto-generated Groth16 verifier (from snarkjs)
  ZKPrivacyToken.sol         ERC-20 with ZK-gated transferWithProof()

scripts/
  compile-circuit.sh         circom → r1cs + wasm
  trusted-setup.sh           Powers of Tau + Groth16 zkey ceremony
  generate-proof.js          Witness → proof → calldata
  verify-proof.js            Local proof verification
  deploy.js                  Deploy Verifier + Token to any network

test/
  ZKPrivacyToken.test.js     Full-stack tests (circuit + contract)
```

---

## ZK Flow

```
Off-chain                          On-chain
─────────────────────────────────  ──────────────────────────────
secret, salt ──► Poseidon ──► commitment ──► registerCommitment()

proof = Groth16(
  private: secret, salt
  public:  commitment, minValue    verifier.verifyProof(proof, [commitment, minValue])
)                                           ↓
                                   token.transferWithProof(to, amount, nullifier, proof)
```

---

## Quick Start

### 1. Install

```bash
npm install
npm install -g circom snarkjs
```

### 2. Compile the circuit

```bash
npm run circuit:compile
```

### 3. Trusted setup (dev — not for production)

```bash
npm run circuit:setup
```

### 4. Export Solidity verifier

```bash
snarkjs zkey export solidityverifier \
  build/circuits/SecretKnowledge_final.zkey \
  contracts/Verifier.sol
```

### 5. Compile contracts

```bash
npm run compile
```

### 6. Start local Anvil testnet

```bash
npm run anvil
# In a new terminal:
```

### 7. Deploy locally

```bash
npm run deploy:local
```

### 8. Run full test suite

```bash
npm test
```

### 9. Generate a proof

```bash
# Edit secret/salt/minValue in scripts/generate-proof.js first
node scripts/generate-proof.js
# → build/circuits/calldata.json contains args for transferWithProof()
```

---

## Environment Variables

Create `.env`:

```env
# Required for Sepolia deployment
SEPOLIA_RPC_URL=https://sepolia.infura.io/v3/YOUR_KEY
DEPLOYER_PRIVATE_KEY=0x...

# Required for Etherscan verification
ETHERSCAN_API_KEY=...
```

---

## Security Notes

- **Trusted setup:** The dev ceremony in `trusted-setup.sh` is **not production safe**. Use a real Powers of Tau file from the Hermez or Semaphore ceremony for mainnet.
- **Nullifiers:** Each proof is single-use. Nullifiers are stored on-chain permanently.
- **Commitment binding:** The Poseidon hash binds secret + salt to the sender's address implicitly (stored per address). A compromised commitment reveals nothing about the underlying secret.
- **Circuit constraints:** The range check uses 64-bit LessThan. Secrets must be in `[0, 2^64)`.

---

## Sepolia Deployment

```bash
npm run deploy:sepolia
```

Etherscan verification runs automatically after 5 block confirmations.
