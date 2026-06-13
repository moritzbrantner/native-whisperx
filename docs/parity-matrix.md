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
| Multiple input files | `<INPUT>...` | `native` | `--basename` is rejected with multiple inputs to avoid output collisions. |
| Transcription task | `--task transcribe` | `native` | Native ASR is the default workflow path. |
| Translation task | `--task translate` | `delegated` | Native rejects translate until implemented; Python WhisperX is the compatibility path. |
| Model selection | `--model` | `native` | Native ASR supports Whisper aliases such as `tiny.en`, `small`, and `large`, plus Hugging Face repo IDs with Candle-compatible files. |
| Model cache | `--model_dir`, cache-only behavior | `native/delegated` | Native ASR and native alignment use `--model-dir` / `--model-cache-only`; external WhisperX still receives the same flags. Wrapper coverage exists through the ignored `SMOKE_ROOT` native ASR cache smoke. |
| Language | `--language` | `native` | Already represented in ASR config. |
| Device | `--device` | `native` | CPU/CUDA selection exists, with feature-gated CUDA. |
| Device index | `--device_index` | `delegated` | Native rejects for now. |
| Compute type | `--compute_type` | `delegated` | Currently meaningful for Python WhisperX. |
| Batch size | `--batch_size` | `delegated` | Native still has sequential/semantic batch config; Python WhisperX receives the faster-whisper batch size. |
| Logging/progress | `--verbose`, `--log-level`, `--print_progress` | `delegated` | Forwarded to Python WhisperX when using the external provider. |
| VAD method | `--vad_method` | `native/delegated` | `energy` and feature-gated `silero` are native; `pyannote` remains delegated. |
| VAD thresholds/chunking | `--vad_onset`, `--vad_offset`, `--chunk_size` | `native/delegated` | Native Silero uses `vad_onset` and `chunk_size` according to WhisperX/Silero behavior; `vad_offset` remains pyannote/delegated semantics for exact parity. |
| Native VAD model wiring | `--vad-model-bundle`, `--vad-model-file`, `--vad-input-name`, `--vad-output-name` | `native` | Native extension for local/offline Silero ONNX execution. |
| Alignment enablement | default alignment and `--no_align` | `native` | Native alignment is enabled by default and can be disabled with `--no-align` / `--no_align`. |
| Alignment model | `--align_model` | `native` | `--align-model` / `--align_model` maps aliases such as `WAV2VEC2_ASR_BASE_960H` to supported Hugging Face wav2vec2 IDs. |
| Interpolation | `--interpolate_method` | `native` | Supports `nearest`, `linear`, and `ignore`. |
| Character alignments | `--return_char_alignments` | `native` | Optional char timings are written as `segments[].chars` and kept in native JSON contracts. |
| Diarization | `--diarize` | `native/delegated` | Native is heuristic/ONNX when features are enabled; pyannote-compatible behavior is delegated. |
| Diarization model | `--diarize_model` | `delegated` | Forwarded to Python WhisperX for pyannote-compatible behavior. |
| Hugging Face token | `--hf_token` | `delegated` | Forwarded to Python WhisperX. |
| Speaker bounds | `--min_speakers`, `--max_speakers` | `native` | Existing config supports bounds. |
| Speaker embeddings | `--speaker_embeddings` | `delegated` | Native validates this behind diarization but does not produce pyannote-compatible embeddings yet. |
| Decode controls | temperature, beam/best-of, patience, penalties, suppression, prompts, fp16, thresholds, threads | `delegated` | Native rejects for now instead of silently ignoring them. |
| Subtitle controls | `--max_line_width`, `--max_line_count`, `--highlight_words`, `--segment_resolution` | `native` | Basic layout/highlight support exists; exact WhisperX sentence layout remains planned. |
| Output formats | `--output_format` | `native` | Supports `all`, `json`, `native-json`, `srt`, `vtt`, `txt`, `tsv`, and `aud`. `json` defaults to WhisperX JSON; `native-json` is explicit. |
| Output directory | `--output_dir` | `native` | Existing output config supports directories. |
| Short aliases | `-o`, `-f`, `-P` | `native` | `-o` maps output dir, `-f` maps format, and `-P` prints Rust runtime/version text. Clap provides normal version handling separately. |
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
