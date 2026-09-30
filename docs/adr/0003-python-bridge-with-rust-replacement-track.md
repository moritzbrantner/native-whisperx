# Allow Python delegation with a Rust replacement track

Superseded for product runtime delegation by ADR 0016 after the #195 native
acceptance gate closed. The historical decision follows.

Python WhisperX and pyannote delegation is acceptable for parity while native
Rust features mature. Each delegated feature must have a planned Rust
replacement path and correctness plus runtime/resource benchmarks before the
Rust path replaces delegation as the default.
