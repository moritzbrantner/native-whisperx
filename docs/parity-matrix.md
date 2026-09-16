# Native WhisperX capability and parity matrix

This document records current product capability, ownership, and evidence. It is
not the normative version source.

- **Verified Compatibility Baseline:** read from
  `tests/parity/whisperx-version.json` (currently 3.8.6).
- **Upstream Target:** latest released WhisperX (3.8.6 as of 2026-09-16).
- **Architecture:** composition-only; see `docs/architecture.md` and ADR 0015.

The matrix intentionally keeps the repository's stable four-status contract:
`rust-native complete`, `blocked`, `reference-only`, and
`intentionally unsupported`. `blocked` does not necessarily mean the native
implementation is missing: where stated below it means implementation and
deterministic gates are present but a declared real-resource/runtime acceptance
run is still required. Native extensions such as Q8 are identified in the notes
instead of inventing a second status vocabulary.

| Area | Evidence / boundary | Status | Notes |
| --- | --- | --- | --- |
| Feature: `native` | workspace/default/no-default tests, Clippy, native feature matrix, local ASR/alignment parity fixtures | `rust-native complete` | `audio-analysis` transcription/alignment providers are canonical; Native WhisperX composes them |
| Feature: `translation` | translation feature matrix plus configured OPUS-MT/Marian model-backed coverage | `rust-native complete` | product translation planning/policy is permanent here; the browser consumer has triggered reusable WebGPU translation ownership in `nlp-stack`, while Rust Marian/OPUS-MT execution, SentencePiece/vocabulary glue, weight loading, and provider caching remain explicitly transitional Native implementation debt pending a separate source migration; built-in Whisper translation without an explicit native translation model is intentionally not emulated |
| Feature: `cuda` | CUDA-aware contracts, retained model-backed evidence, 30s/3m/10m benchmark ladder | `rust-native complete` | lower-level Candle/audio runtime owns execution |
| Feature: `media-decode` | media feature matrix, selected-media contracts, opt-in FFmpeg evidence | `rust-native complete` | finite audio/video input only; selected/default audio is transcribed |
| Feature: `diarization` | feature matrix and explicit native diarization evidence | `rust-native complete` | reusable speaker/diarization mechanics stay in `audio-analysis` |
| Feature: `onnx-diarization` | ONNX feature matrix and bundle/contract tests | `rust-native complete` | explicit ONNX resource path |
| Feature: `pyannote-diarization` | feature matrix, bundle validation, preflight, exact/ranged/embedding gating fixtures | `blocked` | implementation is present; #207 still requires a fresh hosted exact-source licensed/CUDA acceptance run |
| Feature: `silero-vad` | feature matrix and full-resource VAD fixture | `rust-native complete` | provider implementation stays in `audio-analysis` |
| Feature: `pyannote-vad` | feature matrix, automatic-selection/preflight tests, resource fixtures | `rust-native complete` | automatic selection remains caller-cache/prepared-resource based |
| Feature: `whisperx-compat` | compatibility CLI/config and parity harness tests | `reference-only` | non-default Python oracle/reference tooling; explicit product provider remains temporarily until #252 |
| Multiple finite inputs and wildcard expansion | deterministic CLI/input/output collision coverage; Input-Local Output preserved | `rust-native complete` | no cross-input output ambiguity |
| Whisper model aliases and explicit Hugging Face IDs | mapping tests plus representative real-resource runs | `rust-native complete` | advertised aliases canonicalize to `openai/whisper-*`; explicit repositories pass through |
| English ASR | gating English fixtures | `rust-native complete` | native Candle Whisper path |
| Multilingual ASR | `small-de-no-align-cache` plus explicit multilingual regression coverage | `rust-native complete` | multilingual requests use the stable autoregressive KV-cache path |
| Model cache resolution | deterministic model-dir/cache resolution tests | `rust-native complete` | caller-owned cache/model roots |
| `--model-cache-only` | deterministic no-download resolution behavior | `rust-native complete` | hard no-download guarantee |
| Device selection | config/mapping tests plus retained CPU/CUDA evidence | `rust-native complete` | no silent device fallback |
| Device index | mapping and non-default CUDA device smoke | `rust-native complete` | multi-device lists intentionally remain one-process-per-device |
| Float compute types | `auto`, fp16/float16, fp32/float32 mapping tests | `rust-native complete` | provider-owned execution semantics |
| Q8/Int8 | dedicated exact-int8 CPU ASR-only evidence contract | `rust-native complete` | explicit non-default Native WhisperX extension; not broad quantized WhisperX parity |
| Other quantized WhisperX aliases | explicit rejection with supported-value guidance | `intentionally unsupported` | no false aliasing to Q8 |
| Batch size | semantic chunk batching and benchmark diagnostics | `rust-native complete` | active-row vs autoregressive selection remains request-dependent |
| Initial prompt | request-scoped native decode configuration | `rust-native complete` | covered by mapping/regression tests |
| Explicit token suppression | validated before model setup and mapped into native decode | `rust-native complete` | `-1` retains model defaults |
| Numeral suppression | request-scoped native decode configuration | `rust-native complete` | no Python fallback |
| Previous-text conditioning | sequential/autoregressive execution preserves request history | `rust-native complete` | reusable sessions retain no transcript state across requests |
| Temperature schedule | native sampling/fallback schedule validation and mapping | `rust-native complete` | model-backed probe remains resource-gated evidence, not missing implementation |
| Compression-ratio threshold | native fallback-control mapping | `rust-native complete` | validated before model setup |
| Log-probability threshold | native fallback-control mapping | `rust-native complete` | validated before model setup |
| No-speech threshold | native fallback-control mapping | `rust-native complete` | validated before model setup |
| Beam size | native decode mapping and invalid-value rejection | `rust-native complete` | request-scoped |
| Best-of | native sampling mapping and invalid-combination rejection | `rust-native complete` | request-scoped |
| Patience and length penalty | native beam-score controls | `rust-native complete` | rejected when semantically incompatible |
| Decoder threads | native runtime control and validation | `rust-native complete` | zero/invalid values fail before model setup |
| Faster-whisper hotwords | rejected explicitly | `intentionally unsupported` | not emulated |
| Separate WhisperX `--fp16` flag emulation | compute-type selection is the native contract | `intentionally unsupported` | no redundant Python-shaped boolean |
| Python logging/progress flags | native product exposes structured progress/diagnostics | `intentionally unsupported` | Python logging semantics are not copied |
| Energy VAD | deterministic native workflow tests | `rust-native complete` | default non-diarized automatic choice |
| Silero VAD | deterministic plus resource-backed fixture evidence | `rust-native complete` | explicit capability |
| Pyannote VAD | deterministic selection/preflight plus resource fixture evidence | `rust-native complete` | provider-owned bundle validation |
| Automatic native diarization selection | deterministic selection chooses pyannote VAD plus community diarization and fails closed on absent resources | `blocked` | implementation is complete; #207 is the final full-resource acceptance gate |
| Pyannote exact speaker bounds | gating fixture `diarization-shrek-retold-3m-pyannote-exact-reference` | `blocked` | implementation complete; fresh hosted licensed/CUDA evidence required by #207 |
| Pyannote ranged speaker bounds | gating fixture `diarization-shrek-retold-3m-pyannote-range-reference` | `blocked` | implementation complete; fresh hosted licensed/CUDA evidence required by #207 |
| Pyannote speaker embeddings | gating fixture validates count, dimension, finiteness, normalization, and stable cluster association | `blocked` | raw vectors are neither serialized nor numerically compared; #207 remains the acceptance gate |
| Alignment enabled by default | deterministic mapping and model alias coverage | `rust-native complete` | default wav2vec2 alignment |
| `--no-align` | CLI/config and multilingual no-align regression coverage | `rust-native complete` | WhisperX no-timestamps/zero-context window contract preserved |
| Alignment interpolation | config/output fixture coverage | `rust-native complete` | canonical lower-level aligner owns mechanics |
| Character alignment | output/contract coverage | `rust-native complete` | opt-in character projection |
| Finite media/container decode | default media feature plus real FFmpeg smoke evidence | `rust-native complete` | broad live/container streaming parity remains out of scope |
| Post-ASR translation | configured OPUS-MT/Marian path with timed result coverage | `rust-native complete` | source transcript timing remains authoritative; Rust execution remains transitional while reusable browser translation is now owned by `nlp-stack` |
| WhisperX JSON output | semantic fixture comparison | `rust-native complete` | product-owned compatibility rendering |
| Native JSON output | versioned product report/transcript contract tests | `rust-native complete` | canonical Native WhisperX projection |
| TXT output | deterministic output tests | `rust-native complete` | format-only projection |
| SRT output | deterministic timed-text fixture coverage | `rust-native complete` | canonical lower-level timed-text renderer |
| WebVTT output | deterministic timed-text fixture coverage | `rust-native complete` | canonical lower-level timed-text renderer |
| TSV output | deterministic output tests | `rust-native complete` | header/tab normalization covered |
| AUD output | deterministic product output coverage | `rust-native complete` | explicit Native WhisperX output format |
| Subtitle width/count controls | timed-output fixtures | `rust-native complete` | semantic timing remains the gate |
| Highlighted subtitles | timed-output fixtures | `rust-native complete` | exact byte drift may remain non-gating where semantic timing passes |
| Python WhisperX as parity oracle | preflight, golden generation, comparison, benchmark reference | `reference-only` | retained long term under non-default compatibility/parity tooling |
| Python WhisperX as normal product provider | explicit non-default provider only; never a silent fallback | `blocked` | #252 removes/deprecates this temporary product-runtime branch after #195 closes |
| Browser-local WebGPU transcription | Pages consumes pinned `audio-analysis` browser adapter; static contract/deployment checks and a fail-closed `/acceptance/` harness exist | `blocked` | implementation is present; #272 still requires a real deployed WebGPU transcription/export acceptance record |
| Browser alignment | capability is explicitly reported unavailable | `intentionally unsupported` | no browser approximation |
| Browser diarization | capability is explicitly reported unavailable | `intentionally unsupported` | no browser approximation |
| Browser post-ASR translation | Pages consumes a pinned `nlp-stack` WebGPU translation adapter after browser ASR; source timing is retained, source word/char alignment is not falsely projected, translated Native JSON/TXT/SRT/WebVTT projections and fail-closed acceptance metadata exist | `blocked` | structural implementation is present under #286; a real deployed WebGPU/model-backed translated run is still required before claiming runtime acceptance |
| Live-input WhisperX parity | near-live native product exists but direct WhisperX live parity is outside the PRD | `intentionally unsupported` | do not expand the parity program to invent a live WhisperX contract |

## Remaining acceptance boundaries

`tests/parity/full-resource-fixtures.json` is the authoritative full-resource
manifest. `.github/workflows/pyannote-parity-gate.yml` activates the exact source
dependency graph, preflights the full manifest, and runs exact bounds, ranged
bounds, and speaker-embedding cases independently. A missing, skipped,
cancelled, or resource-incomplete run does not satisfy #207.

The Rust-native CUDA benchmark ladder is a separate performance evidence track:
30-second, 3-minute, and 10-minute `large-v3-turbo` cases require provenance,
warm-up, and repeated measured iterations. Q8 evidence is also separate because
Q8 is a Native WhisperX extension rather than shorthand for broad WhisperX
quantized-compute parity.

No native workflow silently falls back to Python. Until #252, callers may still
explicitly select the external WhisperX provider under non-default
`whisperx-compat`; after that migration, Python remains only the oracle used by
parity, golden generation, preflight, and comparison tooling.

Static Pages validation proves artifact/deployment correctness only. The merged
`/acceptance/` harness makes real browser evidence reproducible but does not
replace it: #272 tracks transcription runtime acceptance and #286 tracks the
optional post-ASR translation runtime acceptance.
