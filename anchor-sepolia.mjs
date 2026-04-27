import "dotenv/config";
import { ethers } from "ethers";
import fs from "fs";
import path from "path";

const provider = new ethers.JsonRpcProvider(process.env.SEPOLIA_RPC_URL);
const wallet = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY, provider);

const artifact = JSON.parse(
  fs.readFileSync("./artifacts/contracts/TSCPAnchor.sol/TSCPAnchor.json", "utf8")
);
const anchor = new ethers.Contract(
  process.env.SEPOLIA_CONTRACT_ADDRESS,
  artifact.abi,
  wallet
);

const targetFile = process.argv[2] || "./data.json";
const file = fs.readFileSync(targetFile, "utf8");
const parsed = JSON.parse(file);
const batchHash = ethers.keccak256(ethers.toUtf8Bytes(file));

console.log("Payload file:", targetFile);
console.log("Batch hash:", batchHash);

const alreadyAnchored = await anchor.isAnchored(batchHash);
if (alreadyAnchored) {
  console.log("SKIP: already anchored on Sepolia");
  process.exit(0);
}

console.log("Anchoring to Sepolia...");
const tx = await anchor.commit(batchHash);
console.log("Tx submitted:", tx.hash);
console.log("Etherscan: https://sepolia.etherscan.io/tx/" + tx.hash);
const receipt = await tx.wait();
console.log("Confirmed in block:", receipt.blockNumber);

fs.mkdirSync("./anchor-receipts", { recursive: true });
const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
const receiptData = {
  tscp_id: parsed.tscp_id || path.basename(targetFile),
  batch_hash: batchHash,
  contract_address: process.env.SEPOLIA_CONTRACT_ADDRESS,
  tx_hash: tx.hash,
  block: receipt.blockNumber,
  network: "sepolia",
  etherscan: "https://sepolia.etherscan.io/tx/" + tx.hash,
  anchored_at: new Date().toISOString()
};
const receiptPath = `./anchor-receipts/${timestamp}-sepolia-anchor.json`;
fs.writeFileSync(receiptPath, JSON.stringify(receiptData, null, 2));
console.log("Receipt written:", receiptPath);
