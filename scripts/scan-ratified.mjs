import fs from "fs";
import path from "path";
import { execSync } from "child_process";
import crypto from "crypto";

const TARGET_DIRS = ["./tscp", "./payloads"];
const RECEIPT_DIR = "./anchor-receipts";

function walk(dir) {
  let results = [];
  if (!fs.existsSync(dir)) return results;

  for (const file of fs.readdirSync(dir)) {
    const full = path.join(dir, file);
    const stat = fs.statSync(full);

    if (stat.isDirectory()) results = results.concat(walk(full));
    else if (file.endsWith(".json") || file.endsWith(".tscp")) results.push(full);
  }

  return results;
}

function sha256(raw) {
  return crypto.createHash("sha256").update(raw).digest("hex");
}

function isRatified(obj) {
  return obj?.status === "RATIFIED" || obj?.tscp_header?.status === "RATIFIED";
}

function alreadyAnchored(hash) {
  if (!fs.existsSync(RECEIPT_DIR)) return false;

  for (const file of fs.readdirSync(RECEIPT_DIR)) {
    try {
      const receipt = JSON.parse(fs.readFileSync(path.join(RECEIPT_DIR, file), "utf8"));
      if (receipt.source_hash === hash || receipt.batch_hash === hash) return true;
    } catch {}
  }

  return false;
}

fs.mkdirSync(RECEIPT_DIR, { recursive: true });

const files = TARGET_DIRS.flatMap(walk);
console.log(`Scanning ${files.length} candidate files...`);

for (const file of files) {
  try {
    const raw = fs.readFileSync(file, "utf8");
    const obj = JSON.parse(raw);

if (obj.anchor?.tx_hash) {
  console.log(`SKIP: already anchored ${file}`);
  continue;
}


    if (!isRatified(obj)) continue;

    const sourceHash = sha256(raw);

    if (alreadyAnchored(sourceHash)) {
      console.log(`SKIP already anchored: ${file}`);
      continue;
    }

    console.log(`ANCHOR ${file}`);
    execSync(`node anchor-agent.mjs ${file}`, {
      stdio: "inherit",
      
    });
  } catch (err) {
    console.warn(`SKIP invalid/unreadable: ${file}`);
  }
}
