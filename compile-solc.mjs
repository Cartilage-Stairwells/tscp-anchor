import fs from "fs";
import solc from "solc";

const source = fs.readFileSync("./contracts/TSCPAnchor.sol", "utf8");

const input = {
  language: "Solidity",
  sources: {
    "TSCPAnchor.sol": { content: source }
  },
  settings: {
    outputSelection: {
      "*": {
        "*": ["abi", "evm.bytecode.object"]
      }
    }
  }
};

const output = JSON.parse(solc.compile(JSON.stringify(input)));

if (output.errors) {
  for (const err of output.errors) console.error(err.formattedMessage);
  if (output.errors.some((err) => err.severity === "error")) process.exit(1);
}

const contract = output.contracts["TSCPAnchor.sol"]["TSCPAnchor"];

fs.mkdirSync("./artifacts/contracts/TSCPAnchor.sol", { recursive: true });

fs.writeFileSync(
  "./artifacts/contracts/TSCPAnchor.sol/TSCPAnchor.json",
  JSON.stringify({
    abi: contract.abi,
    bytecode: "0x" + contract.evm.bytecode.object
  }, null, 2)
);

console.log("OK: artifact written");
