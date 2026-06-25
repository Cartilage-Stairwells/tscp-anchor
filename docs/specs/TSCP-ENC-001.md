# TSCP Canonical Encoding v0.1

Rules:
- UTF-8 encoding
- Explicit versioning
- Deterministic field ordering
- Canonical numeric representation
- Hash computed over canonical bytes

Pipeline:

Object
↓
Canonical Bytes
↓
Hash
↓
Receipt
