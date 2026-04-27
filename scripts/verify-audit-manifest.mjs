import fs from "fs";
import crypto from "crypto";
import { validateReceipt } from "./receipt-schema.mjs";

function sha256_file(path) {
  return crypto.createHash("sha256").update(fs.readFileSync(path)).digest("hex");
}

const manifest_file = process.argv[2];

if (!manifest_file) {
  console.error("Usage: node scripts/verify-audit-manifest.mjs <audit-manifest.json>");
  process.exit(2);
}

let manifest;

try {
  manifest = JSON.parse(fs.readFileSync(manifest_file, "utf8"));
} catch (err) {
  console.error("INVALID: cannot read manifest: " + err.message);
  process.exit(1);
}

if (manifest.tscp_id !== "TSCP-AUDIT-MANIFEST-0001") {
  console.error("INVALID: wrong tscp_id");
  process.exit(1);
}

if (!Array.isArray(manifest.receipts)) {
  console.error("INVALID: receipts must be an array");
  process.exit(1);
}

if (manifest.receipt_count !== manifest.receipts.length) {
  console.error("INVALID: receipt_count mismatch");
  process.exit(1);
}

let invalid = false;

for (const entry of manifest.receipts) {
  try {
    const raw = JSON.parse(fs.readFileSync(entry.file, "utf8"));
    const receipt = validateReceipt(raw);
    const digest = sha256_file(entry.file);

    if (digest !== entry.sha256) {
      console.error(entry.file + " -> INVALID sha256 mismatch");
      invalid = true;
      continue;
    }

    if (receipt.tscp_id !== entry.tscp_id) {
      console.error(entry.file + " -> INVALID tscp_id mismatch");
      invalid = true;
      continue;
    }

    if (receipt.chain_id !== entry.chain_id) {
      console.error(entry.file + " -> INVALID chain_id mismatch");
      invalid = true;
      continue;
    }

    if (receipt.tx_hash !== entry.tx_hash) {
      console.error(entry.file + " -> INVALID tx_hash mismatch");
      invalid = true;
      continue;
    }

    if (receipt.batch_hash !== entry.batch_hash) {
      console.error(entry.file + " -> INVALID batch_hash mismatch");
      invalid = true;
      continue;
    }

    console.log(entry.tscp_id + " -> AUDITED");
  } catch (err) {
    console.error(entry.file + " -> INVALID " + err.message);
    invalid = true;
  }
}

if (invalid) process.exit(1);

console.log(manifest.tscp_id + " -> VALID [" + manifest.receipts.length + "/" + manifest.receipts.length + " receipts]");
process.exit(0);
