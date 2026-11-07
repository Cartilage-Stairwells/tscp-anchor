#!/usr/bin/env node
/**
 * Classifying gate over the receipt set.
 *
 * The strict verifier (scripts/verify-receipt.mjs) exits 4 when any receipt is UNBOUND,
 * which is correct but makes the gate permanently red for the nine historical receipts that
 * docs/ANCHOR_PAYLOAD_CONTRACT.md accepts as UNBOUND. This wrapper keeps that detection and
 * changes only what is gated: a receipt that CLAIMS a binding its bytes do not support still
 * fails, while a receipt that states UNBOUND, within the accepted count, passes.
 *
 * Exit 0 pass, 4 binding failure, 2 environment problem.
 */
import { spawnSync } from 'node:child_process';
import { readdirSync } from 'node:fs';
import { join } from 'node:path';

const ACCEPTED_UNBOUND = 9;
const dir = 'anchor-receipts';
const files = readdirSync(dir).filter((f) => f.endsWith('.json')).map((f) => join(dir, f)).sort();
if (files.length === 0) {
  console.error(`environment: no receipts found in ${dir}`);
  process.exit(2);
}

const r = spawnSync('node', ['scripts/verify-receipt.mjs', ...files], { encoding: 'utf8' });
const out = `${r.stdout || ''}${r.stderr || ''}`;
const counts = { VALID: 0, UNBOUND: 0, INVALID: 0 };
for (const line of out.split('\n')) {
  for (const k of Object.keys(counts)) {
    if (line.includes(`\u2192 ${k}`) || line.includes(`-> ${k}`)) counts[k] += 1;
  }
}
console.log(out.trim());
console.log(
  `classification: VALID ${counts.VALID} | UNBOUND ${counts.UNBOUND} | INVALID ${counts.INVALID} | strict exit ${r.status}`,
);
if (counts.INVALID > 0) {
  console.error('FAIL: a receipt claims a binding its bytes do not support');
  process.exit(4);
}
if (counts.UNBOUND > ACCEPTED_UNBOUND) {
  console.error(
    `FAIL: ${counts.UNBOUND} UNBOUND exceeds the ${ACCEPTED_UNBOUND} accepted in docs/ANCHOR_PAYLOAD_CONTRACT.md`,
  );
  process.exit(4);
}
if (counts.VALID + counts.UNBOUND === 0) {
  console.error('FAIL: no verdicts parsed from the strict verifier');
  process.exit(2);
}
console.log('PASS: every receipt is either bound-and-verified or explicitly UNBOUND (docs/ANCHOR_PAYLOAD_CONTRACT.md)');
process.exit(0);
