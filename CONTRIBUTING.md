# Contributing to TSCP

TSCP is an enterprise-grade deterministic cryptographic proof system. We enforce rigorous
engineering discipline and deterministic reproducibility.

## Development Setup

### Rust Toolchain
- **Channel**: Rust Nightly (see `rust-toolchain.toml` for pinned revision)
- Install: `rustup toolchain install nightly`

### Hardware (AVX-512)
Core performance kernels require x86_64 with `avx512f`, `avx512vl`, `avx512dq`.
```bash
RUSTFLAGS="-C target-cpu=native" cargo build
```

## Branching Model

- **`master`**: Stable verified release. All merges via PR, must pass full CI.
- **`feature/*`**: Active development.
- **`phase1-freeze`**: Read-only compliance anchor. Do not rebase or force-push.

## Commit Standards

Follow [Conventional Commits v1.0.0](https://www.conventionalcommits.org/):
`type(scope): description`

Types: `feat`, `fix`, `refactor`, `perf`, `test`, `proof`, `ci`, `docs`, `chore`
Scopes: `avx512`, `phase1`, `anchor`, `canon`, `spec`, `wasm`

### GPG Signing
All commits must be GPG-signed. Enforcement is being phased in.
```bash
git config --global commit.gpgsign true
git config --global user.signingkey <YOUR_KEY_ID>
```

## Code Standards

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Invariant Testing

All contributions must preserve the five TSCP invariants:

1. **Canonical Identity** — Equivalent structures produce identical canonical bytes
2. **Stable Identity** — Serialize/deserialize cycles preserve hashes
3. **Context Isolation** — Different TSCP domains produce different hashes
4. **Mutation Detection** — Changed artifacts produce changed identities
5. **Evidence Authority Boundary** — Evidence records cannot express custody decisions

## Issue Tracking

Open issues may represent deliberate audit trail entries. Do not close without evidence.
