import { ethers } from "ethers";
import * as dotenv from "dotenv";
import fs from "fs"; // Needed to read the file
dotenv.config();

async function main() {
    const provider = new ethers.JsonRpcProvider(process.env.SEPOLIA_RPC_URL);
    const wallet = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY, provider);

    // 1. Load the ABI from your build artifact
    // ADJUST THE PATH BELOW if your file is in a different folder
    const artifactPath = "./artifacts/Anchor.json"; 
    const artifact = JSON.parse(fs.readFileSync(artifactPath, "utf8"));
    const abi = artifact.abi;

    const contract = new ethers.Contract(process.env.SEPOLIA_CONTRACT_ADDRESS, abi, wallet);

    console.log("Interacting with contract at:", contract.target);

    // 2. Now you can call the actual functions found in your code
    // Example: const result = await contract.getAnchor();
    // console.log("Result:", result);
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
