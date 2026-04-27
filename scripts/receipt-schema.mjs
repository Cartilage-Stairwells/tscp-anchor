import { z } from "zod";

const eth_address_regex = /^0x[a-fA-F0-9]{40}$/;
const eth_hash_regex = /^0x[a-fA-F0-9]{64}$/;
const hex64_regex = /^[a-fA-F0-9]{64}$/;
const timestamp_utc_regex = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$/;

export const receiptSchema = z.object({
  tscp_id: z.string().min(1),
  version: z.string().min(1),
  network: z.string().min(1),
  chain_id: z.number().int().positive(),
  contract_address: z.string().regex(eth_address_regex),
  tx_hash: z.string().regex(eth_hash_regex),
  block_number: z.number().int().nonnegative(),
  batch_hash: z.string().regex(eth_hash_regex),
  artifact_hash: z.string().regex(hex64_regex),
  timestamp_utc: z.string().regex(timestamp_utc_regex),
  anchor_mode: z.enum(["single", "batch"]),
  source: z.object({
    file: z.string().min(1),
    sha256: z.string().regex(hex64_regex)
  })
});

export function validateReceipt(receipt) {
  return receiptSchema.parse(receipt);
}
