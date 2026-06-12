# WhisperX 3.8.6 Parity Matrix

This matrix pins the first parity target to Python WhisperX 3.8.6, published on
PyPI on 2026-05-25. Update the baseline intentionally when adopting a newer
WhisperX release.

## Status Vocabulary

| Status | Meaning |
| --- | --- |
| `native` | Covered by the Rust workflow path. |
| `delegated` | Covered by calling Python WhisperX or a Python dependency. |
| `planned` | In scope but not implemented yet. |
| `failing` | Required by the parity contract and currently not passing. |

## CLI Surface

| Area | WhisperX 3.8.6 surface | native-whisperx status | Notes |
| --- | --- | --- | --- |
| Transcription task | `--task transcribe` | `native` | Native ASR is the default workflow path. |
| Translation task | `--task translate` | `planned` | In scope, lower priority, expected to depend on a future Rust translation crate. |
| Model selection | `--model` | `native` | Model IDs are accepted; automatic Hugging Face resolution is planned. |
| Model cache | `--model_dir`, cache-only behavior | `planned` | Alignment model resolution supports `--model_dir` and cache-only; broader ASR model resolution remains planned. |
| Language | `--language` | `native` | Already represented in ASR config. |
| Device | `--device` | `native` | CPU/CUDA selection exists, with feature-gated CUDA. |
| Compute type | `--compute_type` | `delegated` | Currently meaningful for Python WhisperX. |
| Batch size | `--batch_size` | `delegated` | Native batching exists under different config; CLI alias coverage is needed. |
| VAD method | `--vad_method` | `planned` | Current native VAD is RMS-based. |
| VAD thresholds | `--vad_onset`, `--vad_offset` | `planned` | Current native options use RMS threshold/frame/hop settings. |
| Alignment enablement | default alignment and `--no_align` | `native` | Native alignment is enabled by default and can be disabled with `--no-align` / `--no_align`. |
| Alignment model | `--align_model` | `native` | `--align-model` / `--align_model` maps aliases such as `WAV2VEC2_ASR_BASE_960H` to supported Hugging Face wav2vec2 IDs. |
| Interpolation | `--interpolate_method` | `native` | Supports `nearest`, `linear`, and `ignore`. |
| Character alignments | `--return_char_alignments` | `native` | Optional char timings are written as `segments[].chars` and kept in native JSON contracts. |
| Diarization | `--diarize` | `delegated` | Pyannote-compatible behavior is required for parity. |
| Diarization model | `--diarize_model` | `delegated` | Native config needs an alias and model resolver. |
| Speaker bounds | `--min_speakers`, `--max_speakers` | `native` | Existing config supports bounds. |
| Speaker embeddings | `--speaker_embeddings` | `planned` | Compare and serialize once diarization parity is stronger. |
| Output formats | `--output_format` | `native` | `json` defaults to WhisperX JSON; `native-json` is explicit. |
| Output directory | `--output_dir` | `native` | Existing output config supports directories. |
| Python-compatible top-level invocation | `whisperx input ...` shape | `native` | Top-level input invocation is normalized to the native `transcribe` command. |

## Diff Defaults

Structured parity diffs use these defaults unless a fixture overrides them:

- Segment boundary tolerance: 0.100 seconds.
- Word boundary tolerance: 0.050 seconds.
- Speaker comparison: permutation-aware.
- Confidence and probability fields: recorded but non-gating.

## Fixture Policy

Regular CI should use only offline core checks and tiny checked-in media
fixtures. Python WhisperX runs, Hugging Face downloads, and larger benchmark
media should be manual, scheduled, or explicitly opted in.
