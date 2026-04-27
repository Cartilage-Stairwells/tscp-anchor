import "dotenv/config";
import { ethers } from "ethers";
import fs from "fs";

const provider = new ethers.JsonRpcProvider(process.env.LOCAL_RPC_URL);
const signer = await provider.getSigner(0);

// 👇 dynamic payload input (this is the fix)
const filePath = process.env.TSCP_PAYLOAD_FILE || "./data.json";
const file = fs.readFileSync(filePath, "utf8");

const batchHash = ethers.keccak256(
  ethers.toUtf8Bytes(file)
);

console.log("Payload file:", filePath);
console.log("Batch hash:", batchHash);

// TODO: plug into contract call if needed
