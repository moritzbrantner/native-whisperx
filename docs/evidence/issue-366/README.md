# ZIP 9 without a Candle release

Both existing Candle families use ZIP 9.0.5 through immutable minimal source
backports of upstream [Candle #4041](https://github.com/huggingface/candle/pull/4041),
commit `70f72e9cfdbfc570846ca363ead2135394af3e9a`:

| Candle | Base | Backport |
| --- | --- | --- |
| 0.10.2 | upstream tag `0.10.2` | `b57f9eb7ccc6bf5cebf0c3822ed69573b06877a9` |
| 0.11.0 | upstream tag `0.11.0` | `375bf2c2a9de9b272bceb1d3ba349d7e024174f9` |

Each backport changes only the ZIP requirement, fallible NPZ/PTH filename reads,
and upstream serialization regression tests. Tensor/model execution and package
versions are unchanged. Audio Analysis owns its 0.10 workspace override; Cargo
requires Native WhisperX to repeat the override at its own application root.
The 0.11 override covers the transitional translation implementation.

Removed all three RUSTSEC-2026-0367/0368/0369 advisory exceptions and the obsolete
ZIP duplicate exemption. Git source policy allows only the Candle fork and
requires immutable revision specifications. Existing unrelated exceptions remain.

Verification:

- Both Candle versions: seven serialization tests, including UTF-8/CP437 filenames
  and invalid NPZ/PTH; all CPU core library/integration tests passed.
- Native exact source graph: full workspace tests, no-default-feature workspace
  tests, formatting and all-target Clippy with warnings denied passed.
- Registry consumer lock with both source patches: `cargo deny --workspace
  --all-features --locked check` passed (advisories, bans, licenses, sources).
- Real model-backed cache-only Marian test: pinned legacy PyTorch direct and
  English-pivot translation models passed through the patched archive reader.
  Three model weight checksums were verified; the two-case run took 28.85 s.
- Audio Analysis: source readiness and handoff passed. Its existing karaoke lint
  warnings remain unrelated.
- Shared convention source: `6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.

No crates, tags or releases were published. The immutable patches are a source
backport solution while upstream #4041/#4042 remain open, not a claim that an
upstream release is available. Replace these overrides with compatible published
Candle releases through a later dependency transaction, rerun the graph audit
and model compatibility checks, then remove the narrow Git allowlist. Releases
must separately verify their intended distribution graph.
