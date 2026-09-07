const { expect } = require("chai");
const { ethers } = require("hardhat");

// EXC-001 regression test (2026-09-07): the TSCPFriVerifier placeholder
// verification functions must be FAIL-CLOSED. Predicate:
//  (1) with productionMode=false (default), verifyProof reverts (scaffold
//      behavior unchanged);
//  (2) with productionMode=true, a fabricated (well-formed) proof is
//      REJECTED via revert — never a vacuous pass;
//  (3) the trace is not recorded as verified.
describe("EXC-001: fail-closed verifier placeholders", function () {
  let verifier, owner, prover;
  const traceCommitment = ethers.ZeroHash;

  async function deploy() {
    const C = await ethers.getContractFactory("TSCPFriVerifier");
    const v = await C.deploy();
    await v.waitForDeployment();
    return v;
  }

  // well-formed FriProof (structurally valid ABI encoding; fabricated content)
  const fabricatedProof = {
    quotientCommitment: ethers.ZeroHash,
    foldings: [],
    queryResponses: [],
    powNonce: 0,
  };

  beforeEach(async function () {
    [owner, prover] = await ethers.getSigners();
    verifier = await deploy();
    await verifier.authorizeProver(prover.address);
  });

  it("reverts in scaffold mode (productionMode=false) — unchanged behavior", async function () {
    await expect(
      verifier.connect(prover).verifyProof(traceCommitment, fabricatedProof)
    ).to.be.revertedWith("TSCP: not in production mode - verification functions are placeholders");
  });

  it("rejects a fabricated proof in production mode via fail-closed placeholders", async function () {
    await verifier.setProductionMode(true);
    await expect(
      verifier.connect(prover).verifyProof(traceCommitment, fabricatedProof)
    ).to.be.revertedWith("TSCP: verification not implemented (scaffold)");
  });

  it("does not record the trace as verified after the rejected attempt", async function () {
    await verifier.setProductionMode(true);
    await expect(
      verifier.connect(prover).verifyProof(traceCommitment, fabricatedProof)
    ).to.be.reverted;
    expect(await verifier.isTraceVerified(traceCommitment)).to.equal(false);
  });

  it("productionMode defaults to false", async function () {
    expect(await verifier.productionMode()).to.equal(false);
  });
});
