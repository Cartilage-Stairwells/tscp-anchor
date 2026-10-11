#!/usr/bin/env python3
"""Reference implementation of tscp-fri-proof-serialization-v1.

Independent, pure-stdlib decoder/encoder for the byte format specified in
tscp-fri-proof-serialization-v1-verification/CONTRACT.md. The contract is the
authority; this script exists to demonstrate that a second implementation,
in a second language, reproduces identical bytes and digests.

Usage:
  python3 scripts/fri_proof_v1_reference.py fixtures/phase3/fri_proof_v1_test_vector.bin
  python3 scripts/fri_proof_v1_reference.py <file> --expect-digest <hex>
"""
import hashlib
import struct
import sys

# BabyBear prime (p3-baby-bear 0.6.1, BabyBearParameters::PRIME = 0x78000001)
ORDER = 0x78000001
DIGEST_ELEMS = 8
MAGIC = b"tscp-fri"
VERSION = 1


class Reject(Exception):
    """Malformed input: the decoder fails closed."""


class Reader:
    def __init__(self, data):
        self.b = data
        self.p = 0

    def take(self, n):
        if self.p + n > len(self.b):
            raise Reject("truncated input")
        s = self.b[self.p:self.p + n]
        self.p += n
        return s

    def u16(self):
        return struct.unpack("<H", self.take(2))[0]

    def u32(self):
        return struct.unpack("<I", self.take(4))[0]

    def u64(self):
        return struct.unpack("<Q", self.take(8))[0]

    def elem(self):
        v = self.u32()
        if v >= ORDER:
            raise Reject(f"field value {v} >= ORDER {ORDER}")
        return v

    def digest(self):
        return [self.elem() for _ in range(DIGEST_ELEMS)]

    def bounded_len(self, item_width):
        n = self.u32()
        if n > (len(self.b) - self.p) // item_width:
            raise Reject(f"length prefix {n} exceeds remaining budget")
        return n


def decode_opening(r):
    leaf_index = r.u64()
    leaf_value = r.elem()
    auth_len = r.bounded_len(4 * DIGEST_ELEMS)
    auth = [r.digest() for _ in range(auth_len)]
    return {"leaf_index": leaf_index, "leaf_value": leaf_value, "auth": auth}


def decode(data):
    r = Reader(data)
    if r.take(8) != MAGIC:
        raise Reject("bad magic")
    if r.u16() != VERSION:
        raise Reject("unsupported version")
    final_value = r.elem()
    num_roots = r.bounded_len(4)
    if num_roots < 2:
        raise Reject(f"num_roots {num_roots} < 2")
    roots = []
    for _ in range(num_roots):
        nd = r.bounded_len(4 * DIGEST_ELEMS)
        if nd == 0 or (nd & (nd - 1)) != 0:
            raise Reject(f"digest count {nd} invalid")
        roots.append([r.digest() for _ in range(nd)])
    num_queries = r.bounded_len(8)
    if num_queries == 0:
        raise Reject("num_queries is 0")
    indices = [r.u64() for _ in range(num_queries)]
    queries = []
    for _ in range(num_queries):
        nrnd = r.bounded_len(8)
        if nrnd != num_roots - 1:
            raise Reject(f"round count {nrnd} != num_roots - 1 ({num_roots - 1})")
        queries.append([[decode_opening(r) for _ in range(3)] for _ in range(nrnd)])
    if r.p != len(data):
        raise Reject("trailing bytes")
    return {
        "final_value": final_value,
        "roots": roots,
        "indices": indices,
        "queries": queries,
    }


def encode(d):
    out = bytearray(MAGIC)
    out += struct.pack("<H", VERSION)
    out += struct.pack("<I", d["final_value"])
    out += struct.pack("<I", len(d["roots"]))
    for cap in d["roots"]:
        out += struct.pack("<I", len(cap))
        for dig in cap:
            out += b"".join(struct.pack("<I", e) for e in dig)
    out += struct.pack("<I", len(d["indices"]))
    for i in d["indices"]:
        out += struct.pack("<Q", i)
    for q in d["queries"]:
        out += struct.pack("<I", len(q))
        for rnd in q:
            for o in rnd:
                out += struct.pack("<Q", o["leaf_index"])
                out += struct.pack("<I", o["leaf_value"])
                out += struct.pack("<I", len(o["auth"]))
                for dig in o["auth"]:
                    out += b"".join(struct.pack("<I", e) for e in dig)
    return bytes(out)


def main():
    path = sys.argv[1]
    expect = None
    if len(sys.argv) > 3 and sys.argv[2] == "--expect-digest":
        expect = sys.argv[3]
    data = open(path, "rb").read()
    decoded = decode(data)
    reencoded = encode(decoded)
    if reencoded != data:
        print("FAIL: re-encoding is not byte-identical")
        sys.exit(1)
    digest = hashlib.sha256(data).hexdigest()
    n_open = sum(len(rnd) for q in decoded["queries"] for rnd in [q])
    n_openings = sum(len(rnd) for q in decoded["queries"] for rnd in [q]) * 3
    print("decoded OK:")
    print(f"  version        = {VERSION}")
    print(f"  final_value    = {decoded['final_value']}")
    print(f"  roots          = {len(decoded['roots'])} commitments")
    print(f"  queries        = {len(decoded['indices'])}")
    print(f"  fold rounds    = {len(decoded['queries'][0])}")
    print(f"  openings       = {n_openings} Merkle openings (all decoded)")
    print(f"  sha256         = {digest}")
    print("re-encode: byte-identical to input")
    if expect:
        if digest != expect:
            print(f"FAIL: digest mismatch (expected {expect})")
            sys.exit(1)
        print("digest matches expected value")


if __name__ == "__main__":
    main()
