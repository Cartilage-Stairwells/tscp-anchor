import { ethers } from "ethers";
import * as dotenv from "dotenv";
dotenv.config();

const provider = new ethers.JsonRpcProvider(process.env.SEPOLIA_RPC_URL);
const wallet = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY, provider);

console.log("Checking wallet address:", wallet.address);
const balance = await provider.getBalance(wallet.address);
console.log("Current balance (in Wei):", balance.toString());
console.log("Current balance (in ETH):", ethers.formatEther(balance));
