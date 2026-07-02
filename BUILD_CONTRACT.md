# TSCP Build Contract v1

**Purpose**: Guarantee identical, reproducible verification artifacts across Termux, proot, CI, and desktop.

## Core Principle
> Determinism first, convenience second.

The build surface is *explicit*, never inferred from environment.

## Modes

### 1. default (core)
- **Command**: `./build.sh`
- **Guarantees**:
    - `CARGO_BUILD_JOBS=1`
    - `RUSTFLAGS="-C codegen-units=1 -C opt-level=0"`
    - Excludes: `tscp-wasm-smoke`, `prover-server`
    - Packages: `tscp-polyir-verification`, `oracle-layer`, `commitment`, `tscp-cli`, `tscp-kernel`
- **Output**: `tscp-core-seal-<timestamp>-core.tar.gz`
- **Use**: Proof-grade builds, CI, Termux

### 2. --full
- **Command**: `./build.sh --full`
- **Guarantees**:
    - `CARGO_BUILD_JOBS=4` (override allowed)
    - No excludes, full `--workspace`
- **Output**: `tscp-core-seal-<timestamp>-full.tar.gz`
- **Use**: Developer expansion, proot/desktop only
