import { ethers } from "ethers";
import fs from "fs";

const provider = new ethers.JsonRpcProvider("http://127.0.0.1:8545");

const signer = await provider.getSigner(0);
const deployer = await signer.getAddress();

console.log("Deployer:", deployer);
console.log("Balance:", ethers.formatEther(await provider.getBalance(deployer)));

const artifact = JSON.parse(
  fs.readFileSync("./artifacts/contracts/TSCPAnchor.sol/TSCPAnchor.json", "utf8")
);

const factory = new ethers.ContractFactory(
  artifact.abi,
  artifact.bytecode,
  signer
);

const contract = await factory.deploy();
await contract.waitForDeployment();

console.log("TSCPAnchor deployed to:", await contract.getAddress());
