# REPRODUCTION RECORD

===================

Date:           2026-07-23

Verifier:       Sean Christopher Southwick / Cartilage-Stairwells

Environment:
                OS: Android Linux 4.19.191-ab1rck61v164bspP43
                Architecture: armv7l
                CPU: ARMv7 Processor rev 4
                Cores: 4
                Max frequency: 2001 MHz
                Rust: not installed
                Cargo: not installed

AVX-512:        no

Results:

  butterfly_portable:   not executed (Rust unavailable)
  property_tests:        not executed
  verify_constants:      not executed
  butterfly_xor:         not executed
  butterfly_no_std:      not executed
  m8_polyir_lowering:    not executed
  lean_proof:            not executed
  bench_suite:           not executed
  plonky3_bench:         not executed

Falsification tests:
                not executed

Deviations:
                Environment differs from x86_64 benchmark host.
                ARMv7 device; no AVX-512 capability.
                Rust toolchain unavailable.

Signature:
                Pending verifier attestation.
