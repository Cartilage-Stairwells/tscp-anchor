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

const dir = process.argv[2] || "./tscp-docs";
const files = fs.readdirSync(dir).filter(f => f.endsWith(".json"));
const manifest = [];

for (const filename of files) {
  const filepath = path.join(dir, filename);
  const file = fs.readFileSync(filepath, "utf8");
  const batchHash = ethers.keccak256(ethers.toUtf8Bytes(file));

  const alreadyAnchored = await anchor.isAnchored(batchHash);
  if (alreadyAnchored) {
    console.log(`SKIP: ${filename}`);
    manifest.push({ filename, hash: batchHash, block: "existing" });
    continue;
  }

  const tx = await anchor.commit(batchHash);
  const receipt = await tx.wait();
  console.log(`ANCHORED: ${filename} → block ${receipt.blockNumber}`);
  manifest.push({ filename, hash: batchHash, block: receipt.blockNumber });
}

fs.writeFileSync("./manifest.json", JSON.stringify(manifest, null, 2));
console.log("Manifest written.");
