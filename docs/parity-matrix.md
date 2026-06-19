# WhisperX 3.8.6 Parity Matrix

This matrix pins the first parity target to Python WhisperX 3.8.6, published on
PyPI on 2026-05-25. Update the baseline intentionally when adopting a newer
WhisperX release.

## Final Status Vocabulary

The matrix separates the broad WhisperX Parity contract from the stricter
Rust-Native Parity program. Delegated behavior remains acceptable for
compatibility tracking, but it does not satisfy Rust-Native Parity. The final
gate requires every row to be one of these end-state statuses:

| Status | Meaning |
| --- | --- |
| `rust-native complete` | The Rust/native path owns the user-visible WhisperX behavior and required fixtures or benchmarks pass. |
| `blocked` | The surface is in scope but cannot be completed without a documented dependency, model, or runtime capability. |
| `reference-only` | Python WhisperX is used only as the oracle/golden source for that surface. |
| `intentionally unsupported` | The surface is Python/faster-whisper-specific or outside the native contract; native mode rejects it with an explicit reason. |

## CLI Surface

| Area | WhisperX 3.8.6 surface | native-whisperx status | Notes |
| --- | --- | --- | --- |
| Multiple input files | `<INPUT>...` | `rust-native complete` | `--basename` is rejected with multiple inputs to avoid output collisions. |
| Transcription task | `--task transcribe` | `rust-native complete` | Native ASR is the default workflow path and the local ASR fixture suite gates the covered cache/timing cases. |
| Translation task | `--task translate` | `rust-native complete` | Native translation uses post-ASR Helsinki-NLP OPUS-MT/Marian segment translation when `--translation-model` or `--translation-bundle` is supplied; built-in Whisper translation without a native translation model remains rejected with an explicit fallback to `external-whisperx`. |
| Translation model | `--translation-model`, `--translation-bundle`, source/target language, max tokens | `rust-native complete` | `Helsinki-NLP/opus-mt-de-en` runs through the native Marian path and uses the existing Hugging Face cache rules. |
| Model selection | `--model` | `rust-native complete` | Native ASR supports Whisper aliases such as `tiny.en`, `small`, and `large`, plus Hugging Face repo IDs with Candle-compatible files. |
| Model cache | `--model_dir`, cache-only behavior | `rust-native complete` | Native ASR, alignment, and translation use `--model-dir` / `--model-cache-only`; external WhisperX still receives the same flags when selected explicitly. |
| Language | `--language` | `rust-native complete` | English-only native Whisper aliases such as `tiny.en` provide an `en` language hint when no explicit language is supplied. |
| Device | `--device` | `rust-native complete` | CUDA is enabled by default for native builds; CPU remains available through `--device cpu`. |
| Device index | `--device_index` | `blocked` | Native rejects with a reason because the Candle device resolver currently selects the default device for the requested backend. |
| Compute type | `--compute_type` | `blocked` | Native rejects with a reason because the Candle Whisper provider does not expose a compute-type or quantization selector. |
| Batch size | `--batch_size` | `rust-native complete` | Native maps the user control to `max_batch_size` for semantic chunk batching; benchmark diagnostics report chunk and batch execution. |
| Logging/progress | `--verbose`, `--log-level`, `--print_progress` | `intentionally unsupported` | These are Python WhisperX logging controls. Native mode keeps diagnostics in structured reports instead of emulating Python logging/progress output. |
| VAD method | `--vad_method` | `rust-native complete` | `energy`, feature-gated `silero`, and feature-gated local-ONNX `pyannote` are native. External WhisperX still handles delegated runs only when explicitly selected. |
| VAD thresholds/chunking | `--vad_onset`, `--vad_offset`, `--chunk_size` | `rust-native complete` | Native Silero uses `vad_onset` and `chunk_size` according to WhisperX/Silero behavior. Native pyannote uses `vad_onset`, `vad_offset`, and `chunk_size` for hysteresis and merged speech chunks. |
| Native VAD model wiring | `--vad-model-bundle`, `--vad-model-file`, `--vad-input-name`, `--vad-output-name` | `rust-native complete` | Native extension for local/offline Silero and pyannote ONNX execution; full-resource parity compares merged VAD chunks, not raw probabilities. |
| Alignment enablement | default alignment and `--no_align` | `rust-native complete` | Native alignment is enabled by default and can be disabled with `--no-align` / `--no_align`. |
| Alignment model | `--align_model` | `rust-native complete` | `--align-model` / `--align_model` maps aliases such as `WAV2VEC2_ASR_BASE_960H` to supported Hugging Face wav2vec2 IDs. |
| Interpolation | `--interpolate_method` | `rust-native complete` | Supports `nearest`, `linear`, and `ignore`. |
| Character alignments | `--return_char_alignments` | `rust-native complete` | Optional char timings are written as `segments[].chars` and kept in native JSON contracts. |
| Diarization | `--diarize` | `rust-native complete` | Native pyannote community behavior is available with an explicit local bundle and is covered by the full-resource parity gate. |
| Diarization model | `--diarize_model` | `rust-native complete` | Native accepts pyannote model IDs only with `--diarization-model-bundle`; other model IDs are rejected with explicit reasons. |
| Hugging Face token | `--hf_token` | `intentionally unsupported` | Native mode requires local bundles/cache material instead of runtime token access; the flag is forwarded only by the explicit external WhisperX provider. |
| Speaker bounds | `--min_speakers`, `--max_speakers` | `rust-native complete` | Existing config supports bounds and full-resource manifests exercise two-speaker cases. |
| Speaker embeddings | `--speaker_embeddings` | `rust-native complete` | Native accepts this only for pyannote diarization with an explicit local bundle; other native requests are rejected. |
| Decode controls | temperature, beam/best-of, patience, penalties, suppression, prompts, fp16, thresholds, threads | `blocked` | Native accepts default-equivalent zero-temperature greedy decode and `condition_on_previous_text=false`; behavior-changing controls fail with per-flag reasons until upstream Candle Whisper exposes sampling, beam search, prompt seeding, logit filtering, threshold metrics, precision, and thread-count APIs. |
| Subtitle controls | `--max_line_width`, `--max_line_count`, `--highlight_words`, `--segment_resolution sentence\|chunk` | `rust-native complete` | `sentence` is the default and `segment` is accepted only as a legacy native alias. SRT/VTT cue splitting follows WhisperX 3.8.6 writer behavior for word-timed subtitles. |
| Output formats | `--output_format` | `rust-native complete` | Supports `all`, `json`, `native-json`, `srt`, `vtt`, `txt`, `tsv`, and `aud`. Text-like outputs are compared byte-for-byte in local parity fixtures; `json` defaults to WhisperX JSON and is compared semantically. |
| Output directory | `--output_dir` | `rust-native complete` | Existing output config supports directories. |
| Short aliases | `-o`, `-f`, `-P` | `rust-native complete` | `-o` maps output dir, `-f` maps format, and `-P` prints Rust runtime/version text. Clap provides normal version handling separately. |
| Python-compatible top-level invocation | `whisperx input ...` shape | `rust-native complete` | Top-level input invocation is normalized to the native `transcribe` command. |
| Full-resource parity gate | Silero, pyannote VAD, pyannote diarization, speaker embeddings | `blocked` | The `final-full-surface` workflow suite runs `tests/parity/full-resource-fixtures.json` with `--require-non-gating-passed`, but current local preflight is blocked by missing expected WhisperX goldens, `two-speaker.wav`, pyannote VAD `models/pyannote-vad/segmentation.onnx`, `HF_TOKEN`, and a checkout-local `.audio-tools/whisperx-src` pinned to the parity tag. |
| Rust-Native benchmark ladder | 30s, 3m, and 10m large-v3-turbo CUDA clips | `blocked` | The `final-full-surface` workflow suite runs `tests/parity/rust-native-bench-fixtures.json` against the WhisperX reference and fails unless every iteration reports `nativeFasterThanWhisperx=true`. Master validation on 2026-06-20 passed the 30s and 3m rungs but failed the 10m rung because native ASR is still sequential over chunks (`batchExecution=candle-whisper-sequential`), with 10m native ASR taking about 43.2-43.6s versus WhisperX total time around 21.0-22.4s. |

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
