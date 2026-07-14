# Verification Environment: tscp-serialization-v0.1

Reproducibility requires exact environment specification.

## Toolchain

| Component | Requirement |
|---|---|
| Rust channel | nightly (see rust-toolchain.toml for pinned hash) |
| Edition | 2021 |
| Target | x86_64-unknown-linux-gnu (primary) |
| WASM target | wasm32-unknown-unknown (smoke test) |

## Hardware

| Component | Requirement |
|---|---|
| Architecture | x86_64 |
| AVX-512 | Required for avx512-butterfly kernel tests |
| RAM | ≥ 8GB |

Note: GitHub Actions ubuntu-latest runners may not support AVX-512.
AVX-512 tests must be run on hardware with confirmed support.

## Verification Execution Commands

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --release
cargo audit
cargo build --target wasm32-unknown-unknown --release
```

---
*Date: 2026-07-14*