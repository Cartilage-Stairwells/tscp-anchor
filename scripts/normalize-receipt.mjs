import fs from "fs";
import crypto from "crypto";
import { validateReceipt } from "./receipt-schema.mjs";

function sha256_file(path) {
  const data = fs.readFileSync(path);
  return crypto.createHash("sha256").update(data).digest("hex");
}

export function normalizeReceipt(raw) {
  if (!raw) {
    throw new Error("normalizeReceipt: missing receipt object");
  }

  const source_file =
    raw.source && raw.source.file
      ? raw.source.file
      : "tscp/impl/TSCP-IMPL-ANCHOR-0001.json";

  const artifact_hash =
    raw.artifact_hash ??
    raw.artifacthash ??
    (raw.source && raw.source.sha256) ??
    sha256_file(source_file);

  const normalized = {
    tscp_id: String(raw.tscp_id ?? raw.tscpid),
    version: String(raw.version ?? "1.0.0"),
    network: String(raw.network),
    chain_id: Number(raw.chain_id ?? raw.chainid ?? (String(raw.network).includes("sepolia") ? 11155111 : 31337)),
    contract_address: String(raw.contract_address ?? raw.contractaddress),
    tx_hash: String(raw.tx_hash ?? raw.txhash),
    block_number: Number(raw.block_number ?? raw.blocknumber ?? raw.block),
    batch_hash: String(raw.batch_hash ?? raw.batchhash),
    artifact_hash: String(artifact_hash),
    timestamp_utc: String(raw.timestamp_utc ?? raw.timestamputc ?? raw.anchored_at).replace(/\.\d{3}Z$/, "Z"),
    anchor_mode: String(raw.anchor_mode ?? raw.anchormode ?? "single"),
    source: {
      file: String(source_file),
      sha256: String(artifact_hash)
    }
  };

  return validateReceipt(normalized);
}

const is_main = import.meta.url === new URL(process.argv[1], "file://").href;

if (is_main) {
  const input = process.argv[2];

  if (!input) {
    console.error("Usage: node scripts/normalize-receipt.mjs <receipt.json>");
    process.exit(2);
  }

  const raw = JSON.parse(fs.readFileSync(input, "utf8"));
  const normalized = normalizeReceipt(raw);

  fs.writeFileSync(input, JSON.stringify(normalized, null, 2) + "\n");
  console.log("normalized: " + input);
}
