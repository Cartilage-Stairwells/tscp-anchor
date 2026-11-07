# Anchor payload contract and acceptance of the historical nine

**In force from 2026-11-07.** Applies to every anchor issued after this date.

## 1. Acceptance of the historical nine

The nine receipts filed before this contract are formally **UNBOUND** and will not be repaired,
re-pointed or special-cased. The cause is mechanical: the anchoring script hashed a file,
committed that hash on chain, then rewrote the file, so the hashed bytes no longer exist and the
commitment cannot be re-derived from disk. Retroactive repair would itself be an unbound claim.
UNBOUND is the correct verdict on these nine, not a defect in them.

## 2. Contract for new anchors

1. The bytes hashed are the bytes that remain published at the recorded path. No rewrite
   follows the anchor.
2. If the destination must later change, the change records the pre-anchor digest, the
   post-change digest and the transformation, in a new document. The receipt is not edited.
3. Every receipt declares the contract version, path, pre-anchor sha256, transaction hash and
   block number.
4. Binding is verified for receipts issued on or after the cutover. Earlier receipts are
   checked for classification, not for binding.

## 3. Gate policy

The gate asserts classification: every receipt is either bound-and-verified or explicitly
UNBOUND with a reason. A receipt that claims a binding its bytes do not support fails. Known
historical UNBOUND does not fail, because a permanently red gate on an explainable history
teaches readers to ignore the gate. The strict verifier is unchanged and still exits 4.
