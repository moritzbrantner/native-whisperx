# HF_TOKEN and real pyannote in both interfaces

Native issue #403, parent #392. Both interfaces compose the approved community-1 implementation from audio-analysis PR #173, browser source revision `d79630b630307988b8cc4d8bf0a4c382e2db260d`. The native Rust graph remains pinned independently to its existing approved source graph; no provider implementation is duplicated in the product.

## Product behavior

The website runs selected stages in Transcribe → Align → Diarize → Translate order. It prepares resources before audio decode/ASR so missing access fails early, then passes the same decoded PCM to the actual community ONNX/WASM adapter. There is no spectral fallback. Counts are explicitly tentative automatic observations and uncertainty is not reported. The token is request-scoped and cleared after preparation; an older localStorage token is removed on startup. Only the verified resource bundle is retained in page memory.

The local app exposes a password field and Prepare pyannote models action when diarization is selected. The same upstream browser loader creates a verified, flat stored ZIP. A session-authenticated endpoint installs it inside the authorized workspace cache and delegates model/provenance/hash checks to the existing native VAD and diarization inspectors. One community bundle configures both providers. Credentials never reach the local server, settings, presets, reports or history. Setup can be cancelled and retried. Other native model cache requirements remain visible in preflight.

## Evidence

- Real Chromium website: actual browser decode, verified resources, ONNX Runtime Web 1.30 CPU WASM and Rust WASM produced one anonymous speaker on generated speech. ASR alone was a boundary fixture. Gated downloads used already licensed local bytes; no live Hugging Face credential authorization is claimed. Token clearing and migration from old storage passed. `website-real-pyannote.png` records this run.
- Real local app/browser: actual upstream resource preparation and actual authenticated binary HTTP upload installed all eight verified files, configured both model paths and cleared the input. Inspected every local POST to verify the HF token was absent. Desktop/mobile screenshots record this flow.
- Actual 34,143,397-byte ZIP transport: successful installation and cache reuse both returned the approved community model/source/artifact identity. An unauthorized request declaring that full body length received 403 before sending any body.
- Public CLI HTTP test rejects unauthorized setup and invalid resources. Bounded transport rejects paths, unexpected entries, duplicates, non-stored compression, excessive sizes and invalid model integrity. Unverified staging directories are removed.
- Desktop and 390px touch workflow tests: order, aligned timing propagation, four downloads, cancellation, failure/retry, translation guards, denied model access before decode, cleared token and no completed output on failure. Boundary doubles cover model-independent UI behavior; this is distinct from real inference evidence above.
- Real upstream conversation checks recovered 2 speakers/48 turns and 3 speakers/62 turns with the same Rust clustering core; see audio-analysis `docs/evidence/browser-community-diarization.md`. This selected sample is not a general accuracy estimate. Existing #362/#152 quality limitations remain open.

Validation: frontend formatting, strict types, 31 tests and production build; Rust workspace tests, no-default-feature tests, strict Clippy and formatting; product ownership boundary; cargo-deny; pinned website source/WASM build; static site contract; desktop/touch workflow checks. Shared policy sourceRevision `6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.
