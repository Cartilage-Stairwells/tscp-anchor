import "dotenv/config";
import { ethers } from "ethers";
import fs from "fs";

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

const batchHash = ethers.keccak256(ethers.toUtf8Bytes(file));
console.log("Batch hash:", batchHash);

const alreadyAnchored = await anchor.isAnchored(batchHash);
if (alreadyAnchored) {
  console.log("SKIP: already anchored");
  process.exit(0);
}

const tx = await anchor.commit(batchHash);
console.log("Commit tx:", tx.hash);
const receipt = await tx.wait();
console.log("Anchored in block:", receipt.blockNumber);
