import fs from "fs";
import crypto from "crypto";
import { spawnSync } from "child_process";

function sha256_text(text) {
  return crypto.createHash("sha256").update(text).digest("hex");
}

function run(cmd, args) {
  const result = spawnSync(cmd, args, {
    encoding: "utf8",
    shell: false
  });

  return {
    cmd: [cmd].concat(args).join(" "),
    status: result.status,
    stdout: result.stdout || "",
    stderr: result.stderr || ""
  };
}

function utc_now_seconds() {
  return new Date().toISOString().replace(/\.\d{3}Z$/, "Z");
}

const mode = process.argv[2] || "offline";
const git_commit = run("git", ["rev-parse", "HEAD"]).stdout.trim();
const git_status = run("git", ["status", "--short"]).stdout.trim();
const node_version = run("node", ["-v"]).stdout.trim();
const npm_version = run("npm", ["-v"]).stdout.trim();

const verify =
  mode === "sepolia"
    ? run("npm", ["run", "verify:rpc:sepolia"])
    : run("npm", ["run", "verify:offline"]);

const passed = verify.status === 0;
const receipt_lines = verify.stdout
  .split("\n")
  .filter(function (line) {
    return line.includes("→");
  });

const audit = {
  tscp_id: "TSCP-AUDIT-0001",
  version: "1.0.0",
  audit_type: "receipt_verification",
  mode: mode,
  timestamp_utc: utc_now_seconds(),
  git_commit: git_commit,
  git_dirty: git_status.length > 0,
  node_version: node_version,
  npm_version: npm_version,
  command: verify.cmd,
  status: passed ? "PASS" : "FAIL",
  receipt_count: receipt_lines.length,
  stdout_sha256: sha256_text(verify.stdout),
  stderr_sha256: sha256_text(verify.stderr),
  stdout: verify.stdout,
  stderr: verify.stderr
};

const out =
  "audit-receipts/" +
  audit.timestamp_utc.replace(/:/g, "-") +
  "-" +
  mode +
  "-audit.json";

fs.writeFileSync(out, JSON.stringify(audit, null, 2) + "\n");

console.log("audit written: " + out);
console.log("status: " + audit.status);
console.log("receipt_count: " + audit.receipt_count);

if (!passed) process.exit(1);
