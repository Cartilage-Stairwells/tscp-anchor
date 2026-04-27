import "dotenv/config";
import { ethers } from "ethers";
import fs from "fs";

const provider = new ethers.JsonRpcProvider(process.env.LOCAL_RPC_URL);
const signer = await provider.getSigner(0);

const artifact = JSON.parse(
  fs.readFileSync("./artifacts/contracts/TSCPAnchor.sol/TSCPAnchor.json", "utf8")
);

const factory = new ethers.ContractFactory(artifact.abi, artifact.bytecode, signer);
const contract = await factory.deploy();
await contract.waitForDeployment();
const address = await contract.getAddress();

console.log("Deployed to local:", address);
console.log("Add to .env: ANCHOR_CONTRACT_ADDRESS=" + address);
