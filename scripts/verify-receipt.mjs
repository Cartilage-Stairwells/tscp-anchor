import fs from "fs";
import crypto from "crypto";
import { ethers } from "ethers";
import { validateReceipt } from "./receipt-schema.mjs";

function sha256_file(path) {
  const data = fs.readFileSync(path);
  return crypto.createHash("sha256").update(data).digest("hex");
}

async function verify_receipt(path, rpc_url) {
  let receipt;

  try {
    receipt = validateReceipt(JSON.parse(fs.readFileSync(path, "utf8")));
  } catch (err) {
    return {
      file: path,
      status: "INVALID",
      reason: "schema failure: " + err.message
    };
  }

  const computed_artifact_hash = sha256_file(receipt.source.file);

  if (computed_artifact_hash !== receipt.source.sha256) {
    return {
      file: path,
      tscp_id: receipt.tscp_id,
      status: "INVALID",
      reason: "source.sha256 mismatch"
    };
  }

  if (computed_artifact_hash !== receipt.artifact_hash) {
    return {
      file: path,
      tscp_id: receipt.tscp_id,
      status: "INVALID",
      reason: "artifact_hash mismatch"
    };
  }

  if (!rpc_url) {
    return {
      file: path,
      tscp_id: receipt.tscp_id,
      status: "VALID",
      mode: "offline"
    };
  }

  const provider = new ethers.JsonRpcProvider(rpc_url);
  const network = await provider.getNetwork();

  if (Number(network.chainId) !== receipt.chain_id) {
    return {
      file: path,
      tscp_id: receipt.tscp_id,
      status: "INVALID",
      reason:
        "chain_id mismatch: receipt=" +
        receipt.chain_id +
        ", rpc=" +
        network.chainId
    };
  }

  const tx = await provider.getTransactionReceipt(receipt.tx_hash);

  if (!tx) {
    return {
      file: path,
      tscp_id: receipt.tscp_id,
      status: "INDETERMINATE",
      reason: "tx not found"
    };
  }

  if (tx.to && tx.to.toLowerCase() !== receipt.contract_address.toLowerCase()) {
    return {
      file: path,
      tscp_id: receipt.tscp_id,
      status: "INVALID",
      reason: "contract address mismatch"
    };
  }

  const expected_hash = receipt.batch_hash.toLowerCase();

  const hash_found = tx.logs.some(function (log) {
    const topics = log.topics.map(function (t) {
      return t.toLowerCase();
    });
    const data = String(log.data).toLowerCase();

    return (
      topics.includes(expected_hash) ||
      data.includes(expected_hash.replace("0x", ""))
    );
  });

  if (!hash_found) {
    return {
      file: path,
      tscp_id: receipt.tscp_id,
      status: "INVALID",
      reason: "batch_hash not found in tx logs"
    };
  }

  return {
    file: path,
    tscp_id: receipt.tscp_id,
    status: "VALID",
    mode: "rpc"
  };
}

const files = process.argv.slice(2).filter(function (arg) {
  return !arg.startsWith("--");
});

const rpc_arg = process.argv.find(function (arg) {
  return arg.startsWith("--rpc=");
});

const rpc_url = rpc_arg ? rpc_arg.replace("--rpc=", "") : undefined;

if (files.length === 0) {
  console.error(
    "Usage: node scripts/verify-receipt.mjs <receipt...> [--rpc=<url>]"
  );
  process.exit(2);
}

let has_invalid = false;
let has_indeterminate = false;

for (const file of files) {
  const result = await verify_receipt(file, rpc_url);

  const label = result.tscp_id || file;
  const mode = result.mode ? " (" + result.mode + ")" : "";
  const reason = result.reason ? " — " + result.reason : "";

  console.log(label + " → " + result.status + mode + reason);

  if (result.status === "INVALID") has_invalid = true;
  if (result.status === "INDETERMINATE") has_indeterminate = true;
}

if (has_invalid) process.exit(1);
if (has_indeterminate) process.exit(3);
process.exit(0);
