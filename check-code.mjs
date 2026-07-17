import { ethers } from "ethers";
import * as dotenv from "dotenv";
dotenv.config();

const provider = new ethers.JsonRpcProvider(process.env.SEPOLIA_RPC_URL);
const code = await provider.getCode(process.env.SEPOLIA_CONTRACT_ADDRESS);

console.log("Contract Bytecode Length:", code.length);
if (code === "0x") {
    console.error("Result: No code found at this address. You may have deployed to the wrong network or address.");
} else {
    console.log("Result: Contract is deployed and active.");
}
