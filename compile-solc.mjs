import fs from "fs";
import solc from "solc";

// EXC-005 remediation (2026-09-07): compile BOTH contracts — the verifier
// was previously outside the build scope, leaving its compilation state
// untracked. EXC-004 remediation: solc dependency pinned at 0.8.24 in
// package.json (was: unpinned dependency absent entirely; hardhat config
// mismatched at 0.8.19 vs ^0.8.24 pragmas).

const CONTRACTS = ["TSCPAnchor.sol", "TSCPFriVerifier.sol"];

const sources = {};
for (const name of CONTRACTS) {
  sources[name] = { content: fs.readFileSync(`./contracts/${name}`, "utf8") };
}

const input = {
  language: "Solidity",
  sources,
  settings: {
    outputSelection: {
      "*": {
        "*": ["abi", "evm.bytecode.object"]
      }
    }
  }
};

const output = JSON.parse(solc.compile(JSON.stringify(input)));

let hadError = false;
if (output.errors) {
  for (const err of output.errors) console.error(err.formattedMessage);
  hadError = output.errors.some((err) => err.severity === "error");
}
if (hadError) process.exit(1);

for (const name of CONTRACTS) {
  for (const contractName of Object.keys(output.contracts[name])) {
    const contract = output.contracts[name][contractName];
    const outDir = `./artifacts/contracts/${name}`;
    fs.mkdirSync(outDir, { recursive: true });
    fs.writeFileSync(
      `${outDir}/${contractName}.json`,
      JSON.stringify(
        {
          abi: contract.abi,
          bytecode: "0x" + contract.evm.bytecode.object
        },
        null,
        2
      )
    );
    console.log(`OK: ${name}:${contractName} artifact written`);
  }
}
