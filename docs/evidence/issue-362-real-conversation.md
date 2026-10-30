# Real-conversation evidence for #362 — initial AMI cohort

The first fixed cohort **does not pass speaker-count acceptance** and does not
cover the required two-speaker scenario. The subsequent [natural-conversation evaluation](issue-362-natural-conversations.md)
adds the approved two-/three-speaker slice while retaining these failures. The observations below
are genuine model-backed native runs and an independent human-reference
comparison, not evidence of shipped diarization reliability.

## Inputs and holdout protocol

The cohort was selected before examining reference counts: the first 120 seconds
of all four non-scenario EN2002 recordings in AMI's Full-corpus-ASR test partition.
AMI publishes the signals and manual annotations under CC BY 4.0 and documents
participant consent. Attribution: AMI Meeting Corpus; manual annotations v1.6.2.

- [AMI signals, annotations and license](https://groups.inf.ed.ac.uk/ami/download/)
- [Official partition](https://groups.inf.ed.ac.uk/ami/corpus/datasets.shtml)
- [Consent documentation](https://groups.inf.ed.ac.uk/ami/corpus/ethicsandconsent.shtml)
- [Selected sources, windows and hashes](../../tests/parity/ami-real-conversation-cohort.json)
- [Complete machine-readable observations and errors](../../tests/parity/evidence/ami-real-conversation-initial.json)

Native predictions and the separately pinned provider-oracle predictions were
frozen and hashed before opening held-out annotations. No min/max/exact speaker
bounds were supplied. No Speaker Library was used and no draft identities were
saved. Human annotations remain distinct from the provider oracle.

## Observed results

| Recording, 0–120 s | Human count | Native count | Pinned oracle count | Native DER, 250 ms collar | Oracle DER, 250 ms collar |
| --- | --- | --- | --- | --- | --- |
| EN2002a | 4 | 3 | 3 | 21.90% | 22.43% |
| EN2002b | 4 | 2 | 2 | 24.91% | 26.83% |
| EN2002c | 3 | 6 | 6 | 55.59% | 54.31% |
| EN2002d | 4 | 2 | 2 | 50.81% | 50.43% |

All four tentative estimates remain in the denominator: **0/4 counts correct**.
The three-speaker EN2002c window is a concrete automatic-count quality failure;
there is no two-speaker human-reference window in this initial cohort. Additional
appropriately scoped held-out inputs and generic quality correction remain open.
Do not silently constrain or relabel these results to match the annotations.

The evaluator uses manually timed lexical words for counts and permutation-
invariant DER, with both 0 ms and 250 ms collars and overlap scored. Human
transcriber utterances provide separate boundary, short-turn and long-gap reentry
observations. These protocols have different temporal granularity; the boundary
measure is descriptive utterance-boundary agreement, not a claimed accuracy gate.
Untimed annotation elements are counted at meeting-file scope. Native/oracle
agreement is not treated as human ground truth. A lexical/time spot check confirmed
that the reference and ASR clocks correspond; this is not a simple input mismatch.

The machine-readable evidence retains raw anonymous turns, model identity, bundle
identity, label mappings, errors, boundary/overlap/short-turn/reentry measurements,
wall time and process peak RSS. Runtime values were collected under shared host
load and do not establish a throughput comparison.

## Exact resources and reporting

- Native reporting source: `55b565d9eca3d73f6883d13aeec1ffcb0295db26`.
- Native binary SHA-256: `26b09503bb207cacc22860fdfdd54124d6628c1f0b339a4fdc7545d0406978d6`.
- Audio Analysis: `cacb23f725b86e0610404fb6a5537f43b12481f7`.
- Foundation: `9a162dfd44d3cae7f88ad993de79defc0ac94cd6`.
- pyannote community-1 source: `3533c8cf8e369892e6b79ff1bf80f7b0286a54ee`.
- Converted diarization artifact set: `0a12189874dace9b590af9b09ef4637552006130716db57c35d72f984b36c577`.
- VAD source: pyannote segmentation-3.0 `e66f3d3b9eb0873085418a7b813d3b369bf160bb`.
- Native ONNX Runtime: 1.27.0, selected through the documented `ORT_DYLIB_PATH` prerequisite.
- Explicit reference oracle: pyannote.audio 4.0.4, torch/torchaudio 2.8.0, CPU;
  pyannote.metrics 4.1. Oracle telemetry and Hugging Face network access were disabled.

The new optional `diarization` report field retains the raw model turns and count.
The current integration reports no calibrated count uncertainty, so unconstrained
counts are `tentativeAutomatic` with `countUncertainty: "notReported"`. Turn scores
are not converted to a count confidence. User-selected bounds and missing count-
selection facts are represented separately. Zero-count observations are retained.

For each selected clip the real CLI command was:

```bash
ORT_DYLIB_PATH=/path/to/libonnxruntime.so native-whisperx transcribe CLIP.wav \
  --model small --language en --model-cache-only --compute-type float32 --batch-size 1 \
  --diarize --diarize-model pyannote/speaker-diarization-community-1 \
  --diarization-model-bundle /path/to/approved-community-bundle \
  --vad-method pyannote --vad-model-bundle /path/to/approved-vad-bundle \
  --no-speaker-library --no-save-draft-speakers \
  --format json --output-dir native-observed-out --report MEETING-observed-report.json
```

The corpus WAVs were normalized with `ffmpeg -i SOURCE.wav -t 120 -ac 1 -ar 16000
-c:a pcm_s16le CLIP.wav`; source and clip hashes are in the cohort manifest.
The oracle used the same checkpoint revision and full clips with no count bounds.
The resource-only evaluator is `scripts/real_conversation_evidence.py`; it checks
both prediction freezes and artifact hashes before reading human annotations.

## Failure separation and remaining work

A missing diarization model and a checksum-consistent, explicitly synthetic invalid
ONNX fixture both returned nonzero with no successful report. The latter reached
ONNX Runtime and reported a protobuf model-load failure. Unsupported/missing setup
has separate existing CLI coverage. These are failures, not tentative success.

Focused report tests, full workspace Clippy/tests, no-default-feature tests, and
four offline human-reference parser/holdout checks pass. The reporting-only change
left all four transcript JSON hashes identical to the pre-reporting runs. Remaining
work is the failed three-speaker automatic count, a genuine two-speaker reference
case, and completion of the generic quality obligations in Audio Analysis #152.
