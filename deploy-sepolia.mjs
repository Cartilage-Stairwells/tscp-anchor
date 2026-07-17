import "dotenv/config";
import { ethers } from "ethers";
import fs from "fs";

const provider = new ethers.JsonRpcProvider(process.env.SEPOLIA_RPC_URL);
const wallet = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY, provider);

const artifact = JSON.parse(
  fs.readFileSync("./artifacts/contracts/TSCPAnchor.sol/TSCPAnchor.json", "utf8")
);

const factory = new ethers.ContractFactory(artifact.abi, artifact.bytecode, wallet);
console.log("Deploying to Sepolia...");
const contract = await factory.deploy();
await contract.waitForDeployment();
const address = await contract.getAddress();

console.log("Deployed to Sepolia:", address);
console.log("Add to .env: SEPOLIA_CONTRACT_ADDRESS=" + address);
