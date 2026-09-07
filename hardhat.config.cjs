// Consolidated hardhat config (EXC-004 remediation, 2026-09-07).
// This repository previously carried TWO configs: hardhat.config.cjs
// (effective; solidity 0.8.24) and a stale hardhat.config.js (solidity
// 0.8.19 — cannot compile the contracts' ^0.8.24 pragmas; see EXC-004).
// The stale duplicate is deleted; this file is the single config.
// Plugins are required directly — @nomicfoundation/hardhat-toolbox@7.0.0
// is an empty stub with no dependencies and installs nothing.
require("@nomicfoundation/hardhat-ethers");
require("@nomicfoundation/hardhat-chai-matchers");

module.exports = {
  solidity: "0.8.24",
  networks: {
    sepolia: {
      url: process.env.SEPOLIA_RPC || "",
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : []
    }
  }
};
