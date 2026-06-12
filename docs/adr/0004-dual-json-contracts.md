# Expose WhisperX JSON and Native JSON separately

The default `json` output is WhisperX-compatible JSON because CLI parity is the
user-facing contract. The Rust transcript contract remains available as an
explicit `native-json` format so downstream Rust users can opt into the native
shape without making `json` ambiguous.
