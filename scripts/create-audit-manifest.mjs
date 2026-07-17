import fs from "fs";
import crypto from "crypto";
import { validateReceipt } from "./receipt-schema.mjs";

function sha256_file(path) {
  return crypto.createHash("sha256").update(fs.readFileSync(path)).digest("hex");
}

function utc_now_seconds() {
  return new Date().toISOString().replace(/\.\d{3}Z$/, "Z");
}

const args = process.argv.slice(2);
const files = args.filter(function (arg) {
  return !arg.startsWith("--");
});

const out_arg = args.find(function (arg) {
  return arg.startsWith("--out=");
});

const auditor_arg = args.find(function (arg) {
  return arg.startsWith("--auditor=");
});

const out = out_arg ? out_arg.replace("--out=", "") : "audit-manifest.json";
const auditor = auditor_arg ? auditor_arg.replace("--auditor=", "") : "Triune-Oracle";

if (files.length === 0) {
  console.error("Usage: node scripts/create-audit-manifest.mjs <receipt...> [--out=audit-manifest.json] [--auditor=Triune-Oracle]");
  process.exit(2);
}

const receipts = files.map(function (file) {
  const raw = JSON.parse(fs.readFileSync(file, "utf8"));
  const receipt = validateReceipt(raw);

  return {
    file: file,
    tscp_id: receipt.tscp_id,
    chain_id: receipt.chain_id,
    tx_hash: receipt.tx_hash,
    batch_hash: receipt.batch_hash,
    sha256: sha256_file(file)
  };
});

const manifest = {
  tscp_id: "TSCP-AUDIT-MANIFEST-0001",
  version: "1.0.0",
  audit_type: "anchor_receipt_manifest",
  auditor: auditor,
  audited_at_utc: utc_now_seconds(),
  receipt_count: receipts.length,
  receipts: receipts
};

fs.writeFileSync(out, JSON.stringify(manifest, null, 2) + "\n");
console.log("audit manifest written: " + out);
console.log("receipt_count: " + receipts.length);
