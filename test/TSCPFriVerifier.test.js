const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("TSCPFriVerifier — Golf Suite", function () {
  let verifier, owner, prover, stranger;

  beforeEach(async function () {
    [owner, prover, stranger] = await ethers.getSigners();
    const TSCPFriVerifier = await ethers.getContractFactory("TSCPFriVerifier");
    verifier = await TSCPFriVerifier.deploy();
    await verifier.waitForDeployment();
  });

  it("deploys with SAFE OWSL status and 128 bits remaining", async function () {
    const summary = await verifier.getOWSLSummary();
    expect(summary.status).to.equal("SAFE");
    expect(summary.action).to.equal("COMMIT");
    expect(summary.bitsRemaining).to.equal(128n);
    expect(summary.permits).to.equal(true);
  });

  it("allows owner to authorize a prover", async function () {
    await expect(verifier.connect(owner).authorizeProver(prover.address))
      .to.not.be.reverted;
    expect(await verifier.authorizedProvers(prover.address)).to.equal(true);
  });

  it("rejects unauthorized prover verification", async function () {
    const trace = ethers.keccak256(ethers.toUtf8Bytes("golf-test-trace"));
    const proof = {
      quotientCommitment: ethers.randomBytes(32),
      foldings: [],
      queryResponses: [],
      powNonce: 0
    };
    await expect(
      verifier.connect(stranger).verifyProof(trace, proof)
    ).to.be.revertedWith("TSCP: not authorized");
  });

  it("rejects verification when OWSL is CRITICAL", async function () {
    await verifier.connect(owner).authorizeProver(prover.address);
    await verifier.connect(owner).updateOWSLStatus({
      timestamp: Math.floor(Date.now() / 1000),
      status: "CRITICAL",
      action: "HALT",
      round: 1,
      bitsConsumed: 0,
      bitsRemaining: 0,
      anomalies: [],
      frameCount: 0,
      windowStart: 0,
      windowEnd: 0,
      checksumValid: true
    });
    const trace = ethers.keccak256(ethers.toUtf8Bytes("golf-critical"));
    const proof = {
      quotientCommitment: ethers.randomBytes(32),
      foldings: [],
      queryResponses: [],
      powNonce: 0
    };
    await expect(
      verifier.connect(prover).verifyProof(trace, proof)
    ).to.be.revertedWith("TSCP: OWSL CRITICAL");
  });

  it("rejects duplicate trace verification (replay protection)", async function () {
    await verifier.connect(owner).authorizeProver(prover.address);
    const trace = ethers.keccak256(ethers.toUtf8Bytes("golf-replay"));
    const proof = {
      quotientCommitment: ethers.randomBytes(32),
      foldings: [[1, 2, 3]],
      queryResponses: [],
      powNonce: 0
    };
    await verifier.connect(prover).verifyProof(trace, proof);
    await expect(
      verifier.connect(prover).verifyProof(trace, proof)
    ).to.be.revertedWith("TSCP: trace already verified");
  });

  it("emits ProofVerified on successful scaffold verification", async function () {
    await verifier.connect(owner).authorizeProver(prover.address);
    const trace = ethers.keccak256(ethers.toUtf8Bytes("golf-emit"));
    const proof = {
      quotientCommitment: ethers.randomBytes(32),
      foldings: [[1]],
      queryResponses: [],
      powNonce: 0
    };
    await expect(verifier.connect(prover).verifyProof(trace, proof))
      .to.emit(verifier, "ProofVerified")
      .withArgs(trace, await ethers.provider.getBlockNumber() + 1, await time.latest());
  });
});
