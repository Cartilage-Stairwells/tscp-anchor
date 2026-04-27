import "dotenv/config";
import { ethers } from "ethers";
import fs from "fs";
import path from "path";

const provider = new ethers.JsonRpcProvider(process.env.LOCAL_RPC_URL);
const signer = await provider.getSigner(0);

const artifact = JSON.parse(
  fs.readFileSync("./artifacts/contracts/TSCPAnchor.sol/TSCPAnchor.json", "utf8")
);
const anchor = new ethers.Contract(
  process.env.ANCHOR_CONTRACT_ADDRESS,
  artifact.abi,
  signer
);

const targetFile = process.argv[2] || "./data.json";
const file = fs.readFileSync(targetFile, "utf8");
const parsed = JSON.parse(file);
const batchHash = ethers.keccak256(ethers.toUtf8Bytes(file));

console.log("Batch hash:", batchHash);

const alreadyAnchored = await anchor.isAnchored(batchHash);
if (alreadyAnchored) {
  console.log("SKIP: already anchored");
  process.exit(0);
}

const tx = await anchor.commit(batchHash);
const receipt = await tx.wait();
console.log("Anchored in block:", receipt.blockNumber);

fs.mkdirSync("./anchor-receipts", { recursive: true });
const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
const receiptData = {
  tscp_id: parsed.tscp_id || path.basename(targetFile),
  batch_hash: batchHash,
  contract_address: process.env.ANCHOR_CONTRACT_ADDRESS,
  tx_hash: tx.hash,
  block: receipt.blockNumber,
  network: process.env.LOCAL_RPC_URL,
  anchored_at: new Date().toISOString()
};
const receiptPath = `./anchor-receipts/${timestamp}-anchor.json`;
fs.writeFileSync(receiptPath, JSON.stringify(receiptData, null, 2));
console.log("Receipt written:", receiptPath);
