# WhisperX 3.8.6 Parity Matrix

This matrix pins the first parity target to Python WhisperX 3.8.6, published on
PyPI on 2026-05-25. Update the baseline intentionally when adopting a newer
WhisperX release.

## Status Vocabulary

The matrix separates the broad WhisperX Parity contract from the stricter
Rust-Native Parity program. `delegated` remains acceptable for compatibility
tracking, but it does not satisfy Rust-Native Parity.

| Status | Meaning |
| --- | --- |
| `native` | Covered by the Rust workflow path. |
| `delegated` | Covered by calling Python WhisperX or a Python dependency. |
| `planned` | In scope but not implemented yet. |
| `failing` | Required by the parity contract and currently not passing. |

Final Rust-Native Parity summaries should use these surface statuses:

| Status | Meaning |
| --- | --- |
| `rust-native complete` | The Rust/native path owns the user-visible WhisperX behavior and required fixtures or benchmarks pass. |
| `rust-native partial` | A Rust/native path exists, but behavior, fixture coverage, or performance evidence is incomplete. |
| `blocked` | The surface is in scope but cannot be completed without a documented dependency, model, or runtime capability. |
| `reference-only` | Python WhisperX is used only as the oracle/golden source for that surface. |

## CLI Surface

| Area | WhisperX 3.8.6 surface | native-whisperx status | Notes |
| --- | --- | --- | --- |
| Multiple input files | `<INPUT>...` | `native` | `--basename` is rejected with multiple inputs to avoid output collisions. |
| Transcription task | `--task transcribe` | `native` | Native ASR is the default workflow path. |
| Translation task | `--task translate` | `planned/delegated` | Native translation is blocked by published upstream APIs; use `--provider external-whisperx` for WhisperX translation parity today. |
| Translation model | `--translation-model`, `--translation-bundle`, source/target language, max tokens | `blocked by upstream crate` | The planned native family is Helsinki-NLP OPUS-MT/Marian; the current clean crates.io graph reports an explicit runtime error for this path. |
| Model selection | `--model` | `native` | Native ASR supports Whisper aliases such as `tiny.en`, `small`, and `large`, plus Hugging Face repo IDs with Candle-compatible files. |
| Model cache | `--model_dir`, cache-only behavior | `native/delegated` | Native ASR and alignment use `--model-dir` / `--model-cache-only`; external WhisperX still receives the same flags. Wrapper coverage exists through the ignored `SMOKE_ROOT` native ASR cache smoke. |
| Language | `--language` | `native` | Already represented in ASR config. English-only native Whisper aliases such as `tiny.en` also provide an `en` language hint when no explicit language is supplied. |
| Device | `--device` | `native` | CUDA is enabled by default for native builds; CPU remains available through `--device cpu`. |
| Device index | `--device_index` | `delegated` | Native rejects for now. |
| Compute type | `--compute_type` | `delegated` | Currently meaningful for Python WhisperX. |
| Batch size | `--batch_size` | `delegated` | Native still has sequential/semantic batch config; Python WhisperX receives the faster-whisper batch size. |
| Logging/progress | `--verbose`, `--log-level`, `--print_progress` | `delegated` | Forwarded to Python WhisperX when using the external provider. |
| VAD method | `--vad_method` | `native/delegated` | `energy` and feature-gated `silero` are native; `pyannote` remains delegated. |
| VAD thresholds/chunking | `--vad_onset`, `--vad_offset`, `--chunk_size` | `native/delegated` | Native Silero uses `vad_onset` and `chunk_size` according to WhisperX/Silero behavior. `vad_offset` is accepted for compatibility, but WhisperX Silero merge does not use it. |
| Native VAD model wiring | `--vad-model-bundle`, `--vad-model-file`, `--vad-input-name`, `--vad-output-name` | `native` | Native extension for local/offline Silero ONNX execution; full-resource parity compares merged VAD chunks, not raw probabilities. |
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
| Subtitle controls | `--max_line_width`, `--max_line_count`, `--highlight_words`, `--segment_resolution sentence\|chunk` | `native` | `sentence` is the default and `segment` is accepted only as a legacy native alias. SRT/VTT cue splitting follows WhisperX 3.8.6 writer behavior for word-timed subtitles. |
| Output formats | `--output_format` | `native` | Supports `all`, `json`, `native-json`, `srt`, `vtt`, `txt`, `tsv`, and `aud`. Text-like outputs are compared byte-for-byte in local parity fixtures; `json` defaults to WhisperX JSON and is compared semantically. |
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

The Rust-Native Parity benchmark ladder is described in
`tests/parity/rust-native-bench-fixtures.json`. It references local
Shrek-derived clips at 30 seconds, 3 minutes, and 10 minutes, all generated
under the smoke root rather than committed to the repository.

The local fixture harness supports gating and non-gating cases. Gating cases
must pass transcript comparison, required diagnostics, expected JSON checks, and
expected output-file comparisons. Non-gating cases are reported but do not fail
the suite, which keeps full-resource Silero and diarization measurements visible
while native behavior is still converging.

For `tests/parity/asr-fixtures.json`, the core English cache fixtures now gate
native timing parity against WhisperX 3.8.6: `tiny-en-no-align-cache`,
`small-en-no-align-cache`, and `tiny-language-detection` gate segment timing;
`tiny-en-aligned-cache` gates segment and word timing; and
`tiny-en-char-alignments` gates segment timing, word timing, and character
count. Output writer fixtures `tiny-output-subtitles-wrap` and
`tiny-output-segment-resolution-chunk` gate byte-for-byte SRT/VTT output
goldens, and `tiny-output-all-defaults` gates TXT/VTT/SRT/TSV byte-for-byte
goldens plus semantic WhisperX transcript JSON comparison.

Timing mismatch reports include native and WhisperX start/end values, absolute
start/end deltas, and the active tolerance. Remaining report-only ASR expansion
cases include `small-de-no-align-cache`, `tiny-en-alignment-alias-cache`, the
translation fixture, and `tiny-output-subtitles-highlight`.
`tiny-output-subtitles-highlight` remains report-only because highlighted SRT/VTT
cue boundaries are byte-level outputs derived from exact word cue milliseconds,
even when the underlying word timings pass the 0.050s tolerance.
