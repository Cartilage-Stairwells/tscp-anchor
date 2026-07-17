import fs from "fs";
import path from "path";

const VALID_TRANSITIONS = ["draft", "active", "DRAFT", "ACTIVE"];

const filePath = process.argv[2];
if (!filePath) {
  console.error("Usage: node scripts/ratify-tscp.mjs <path-to-artifact.json>");
  process.exit(1);
}

const raw = fs.readFileSync(filePath, "utf8");
const obj = JSON.parse(raw);

const currentStatus = obj.status || obj.tscp_header?.status;
if (!VALID_TRANSITIONS.includes(currentStatus)) {
  console.error(`REJECT: Cannot ratify from status '${currentStatus}'`);
  process.exit(1);
}

if (obj.status) obj.status = "RATIFIED";
if (obj.tscp_header?.status) obj.tscp_header.status = "RATIFIED";
obj.modified_at = new Date().toISOString();
obj.ratified_at = new Date().toISOString();

fs.writeFileSync(filePath, JSON.stringify(obj, null, 2));
console.log(`RATIFIED: ${filePath}`);
console.log("Running anchor...");

const { execSync } = await import("child_process");
execSync(`node anchor-sepolia.mjs ${filePath}`, { stdio: "inherit" });
