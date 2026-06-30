# audio-analysis-transcription

Rust-native and compatibility transcription orchestration for `video-analysis`.

Native Candle Whisper ASR is available behind the `candle` feature. CUDA device
selection is available behind `cuda`, and local model bundle validation is
available behind `model-bundles`. Native path decoding defaults to direct
samples or readable WAV files. With the explicit `audio-io` feature, non-WAV
paths use FFmpeg-backed `audio-analysis-io` decode and then normalize/resample
through the same native 16 kHz mono boundary. This opt-in media decode is not
WhisperX parity and is not part of default tests.

Native Whisper tries model timestamp tokens automatically when the tokenizer
defines Whisper timestamp metadata. If timestamp decoding does not produce
bounded text segments, it falls back to chunk/window segment timing. Native
Whisper segment times are emitted as global transcript times. Timestamp-token
segments also include approximate projected word timings based on
character-weighted distribution inside each segment. wav2vec2/CTC alignment
can run through the same native transcription pipeline: `alignment.enabled=true`
with no bundle uses deterministic transcript timing alignment, while
`alignment.enabled=true` with a supported local wav2vec2 bundle uses Candle
wav2vec2 CTC alignment.

Native Candle Whisper also supports Whisper's built-in translate-to-English
task with `provider.task="translate"`. This is GPU-backed when built with
`cuda` and requested with `provider.device="cuda"`. Translation uses the
Whisper decoder task token; it is not OPUS-MT/Marian post-ASR machine
translation. Because translated text is not source-language text,
`alignment.enabled=true` is rejected for native translation requests.
Diarization remains allowed because speaker assignment can use segment timing.

The external WhisperX command provider remains compatibility and parity tooling.
It keeps Python-based execution explicit for callers that still need WhisperX
decoding, batched ASR, alignment, or pyannote-backed diarization outside the
default Rust path. It is also the current path for video/container inputs.

Speaker diarization contracts are owned by `audio-analysis-speakers`.
Transcription owns pipeline orchestration and adapts the speaker-owned
`SpeakerDiarizationOptions` into its flattened `diarization` request JSON.
Native deterministic diarization execution is still available only behind
`diarization` as a heuristic spectral baseline, not a pyannote replacement or
production speaker recognition model. Transcript Speaker Assignment runs after
alignment when both options are enabled, so the native diarization seam can use
aligned word timings, or segment timings when word timings are absent, as
speech-span hints. When no transcript timing is available it falls back to the
energy-VAD baseline. `min_speakers` and `max_speakers` are validated by the
speaker-owned contract, reported in diagnostics, and applied as native
unknown-speaker clustering constraints. Known/enrolled speaker IDs are
preserved; bounds only affect generated unknown speaker labels.
An ONNX speaker embedding provider is available only when explicitly configured
with `diarization.speakerEmbeddingModelBundle` and the crate is built with
`diarization,onnx`. It feeds the existing `WindowedSpeakerDiarizer`; heuristic
diarization remains the default. ONNX diagnostics include
`diarizationRuntime=onnx`, `speakerEmbeddingProvider=onnx`,
`speakerEmbeddingDimension=N`, and `diarizationBaseline=false`. When speaker
bounds are requested, diagnostics include
`diarizationSpeakerBoundsApplied=true`; if the requested minimum cannot be
reached because too few usable speech spans exist, diagnostics include
`diarizationSpeakerBoundsSaturated=true`.

CTC alignment validates local wav2vec2 bundle files, config, tokenizer
vocabulary, and preprocessor metadata. Tokenizer discovery accepts
`tokenizer.json` and `vocab.json`; CTC blank resolution can use `pad_token_id`
when tokenizer metadata does not name a pad token. Supported local
`Wav2Vec2ForCTC` `model.safetensors` bundles execute through a private Candle
implementation and feed native CTC trellis/backtracking. Unsupported
architectures, stable-layer-norm configs, inconsistent positional-convolution
weight-norm tensors, or other unsupported safetensors layouts return typed
errors instead of falling back to deterministic timing. The Debug-only
`audio.transcription.alignmentBundlePlan` operation inspects local wav2vec2
bundle layout metadata without model inference and reports architecture,
stable-layer-norm status, positional-convolution layout, feature-extractor
norm, encoder layer count, missing tensor keys, and unsupported reasons.

Candle Whisper batch options are deterministic rather than concurrent in this
phase. `max_batch_size=0` is rejected, chunk order and global timing are
preserved, and diagnostics report `chunkCount`, `batchChunks`, `maxBatchSize`,
`batchCount`, and `batchExecution=candle-whisper-sequential`. This is semantic
batch grouping, not throughput parity or tensor-batched model execution.

Transcript contracts, normalization, caption formatting, and WhisperX JSON
import remain owned by `text-transcripts`.

## Package Operations

- `audio.transcription.transcribe`: run real transcription through the selected
  provider. Native Candle Whisper uses local WAV input or direct samples and can
  run `task="translate"` for Whisper translate-to-English; the external
  WhisperX provider remains available for compatibility.
- `audio.transcription.importWhisperX`: import existing WhisperX JSON without
  running external tools.
- `audio.transcription.providers`: inspect available provider families.
- `audio.transcription.plan`: describe runtime setup without execution.
- `audio.transcription.modelPlan`: inspect ASR model requirements.
- `audio.transcription.vadPlan`: inspect deterministic VAD defaults.
- `audio.transcription.alignmentPlan`: inspect CTC alignment requirements.
- `audio.transcription.alignmentBundlePlan`: inspect local wav2vec2 bundle
  readiness without model inference.
- `audio.transcription.decodePlan`: explain source decode routing without
  opening files or running FFmpeg.
- `audio.transcription.diarizationPlan`: explain heuristic diarization status,
  speaker-owned assignment policies, and future model-backed provider
  directions.
- `describe`: inspect package metadata.

## Setup

For native Candle Whisper execution, provide a local model bundle containing:

- `config.json`
- `generation_config.json`
- `tokenizer.json`
- `preprocessor_config.json`
- `model.safetensors`

For native wav2vec2 CTC alignment, provide a local `Wav2Vec2ForCTC` bundle
containing:

- `config.json`
- `tokenizer.json` or `vocab.json`
- `preprocessor_config.json`
- `model.safetensors`

The real wav2vec2 smoke prints the alignment bundle layout report before
execution. Stable-layer-norm bundles remain `unsupported_runtime` until that
architecture path is implemented.

2026-06-10 validation update: `scripts/sync_model_bundles.sh` provisioned
`facebook/wav2vec2-base-960h` under the ignored smoke model root. The ignored
real-bundle alignment smoke passed with `vocab.json` tokenizer discovery and
reported `architecture="Wav2Vec2ForCTC"`,
`do_stable_layer_norm=false`, `positional_conv_layout="weight-norm"`,
`feature_extractor_norm="group"`, `encoder_layer_count=12`, no missing keys,
and no unsupported layout reasons. Native positional convolution reconstruction
now supports the observed per-kernel weight-norm `weight_g` layout used by this
bundle.

Use `candle,model-bundles` for CPU local smoke tests and add `cuda` for
CUDA-backed Whisper smoke tests. No runtime downloads are performed by this
crate.

Use `audio-io` only when native non-WAV media/container decode is explicitly
needed and local FFmpeg support is available:

```bash
RUN_NATIVE_MEDIA_DECODE_TESTS=1 \
TRANSCRIPTION_MEDIA_PATH=/path/to/video-or-audio-container \
cargo test -p moritzbrantner-audio-analysis-transcription \
  --features audio-io \
  native_media_decode_when_requested -- --ignored --nocapture
```

Use `diarization,onnx` only when caller-owned local ONNX speaker embeddings are
explicitly configured. Add `model-bundles` when the path is a model-runtime
bundle manifest:

```json
{
  "diarization": {
    "enabled": true,
    "speakerEmbeddingModelBundle": "/path/to/onnx-speaker-model",
    "speakerEmbeddingDimension": 192,
    "speakerEmbeddingSampleRate": 16000
  }
}
```

The ONNX path accepts a direct `.onnx` file, a directory containing
`model.onnx`, or a model-runtime manifest when `model-bundles` is enabled.
It does not resample in this tranche; request audio must already match the
configured speaker embedding sample rate. Speaker model input shape is detected
from ONNX metadata: waveform inputs use `[B,S]` or `[B,1,S]`, while feature
inputs such as `[B,T,80]` use the shared speaker fbank/log-mel CPU
preprocessor.

Ignored local transcription ONNX diarization smoke:

```bash
RUN_NATIVE_SPEAKER_MODEL_TESTS=1 \
ORT_DYLIB_PATH="$PWD/.audio-tools/whisperx-venv/lib/python3.11/site-packages/onnxruntime/capi/libonnxruntime.so.1.26.0" \
SPEAKER_EMBEDDING_MODEL_BUNDLE=/path/to/onnx-speaker-model \
SPEAKER_EMBEDDING_MODEL_FILE=model.onnx \
DIARIZATION_AUDIO_PATH=/path/to/meeting-16khz.wav \
SPEAKER_EMBEDDING_DIMENSION=192 \
cargo test -p moritzbrantner-audio-analysis-transcription \
  --features diarization,onnx,model-bundles \
  native_onnx_diarization_smoke_when_requested -- --ignored --nocapture
```

The smoke uses direct samples and mock ASR timing, so it validates only the
native transcription pipeline's explicit ONNX diarization path. It does not use
FFmpeg, Python, CUDA, pyannote, Hugging Face auth, network, or downloaded model
files.

2026-06-10 validation: the checked-in WebM fixture decoded successfully with
`audio-io` after the ignored smoke harness resolved workspace-root-relative
fixture paths. After local ONNX bundle provisioning, the ONNX diarization smoke
used direct-sample setup and mock ASR timing. Static ONNX metadata inspection
succeeded for the selected current `wespeaker-voxceleb-resnet34-LM` artifact:
f32 input `feats` `[B,T,80]` and f32 output `embs` `[B,256]`. Static graph
diagnostics reported default-domain ONNX ops only, opset 14, 110 nodes, and
75 initializers. The speaker adapter supports this feature-input category.

With `ORT_DYLIB_PATH` unset, file and memory diagnostic load modes timed out
via external `timeout 120s` exit `124` after `onnxSessionBuilder=begin` and
before `onnxSessionBuilder=ok`. Python ONNX Runtime 1.26.0 loaded the same
artifact. With `ORT_DYLIB_PATH` pointed at the local `.audio-tools/whisperx-venv`
`libonnxruntime.so.1.26.0`, the transcription ONNX diarization smoke passed:
direct samples and mock ASR timing reached `diarizationRuntime=onnx`,
`speakerEmbeddingProvider=onnx`, `speakerEmbeddingDimension=256`,
`diarizationBaseline=false`, and non-empty speaker assignments. ONNX
diarization remains window-by-window embedding, not throughput parity.
Classification: implicit ORT dynamic-library selection/setup blocker on this
host, fixed for local smokes by an explicit compatible dylib.

On the current RTX 3060 Ti smoke host, `/usr/local/cuda` points at CUDA 13.3
while the passing smoke uses a local CUDA 12.3 library shim at
`$SMOKE_ROOT/cuda12-libs`. The local smoke
bundle and fixture used there are:

- `$SMOKE_ROOT/whisper-tiny`
- `$SMOKE_ROOT/audio/native-transcription-smoke.wav`

Install and configure external compatibility tools outside the default test
flow only when using the WhisperX provider:

```bash
whisperx --help
ffmpeg -version
python -c 'import whisperx'
```

WhisperX diarization requires a Hugging Face token accepted by pyannote:

```bash
export HF_TOKEN=...
```

No default build or test downloads models, requires CUDA, or requires network
access. Default tests also do not require Python, WhisperX, Hugging Face tokens,
external model files, or FFmpeg.

Optional external WhisperX parity can be run manually when local tools and media
are configured:

```bash
RUN_WHISPERX_PARITY_TESTS=1 \
WHISPERX_COMMAND=whisperx \
WHISPERX_AUDIO_PATH=/path/to/audio.wav \
cargo test -p moritzbrantner-audio-analysis-transcription external_whisperx_parity_when_requested -- --ignored --nocapture
```

Set `WHISPERX_EXPECTED_JSON=/path/to/expected.json` to compare command output
against a known WhisperX JSON fixture. Optional parity-only overrides are
`WHISPERX_MODEL`, `WHISPERX_LANGUAGE`, `WHISPERX_DEVICE`, and
`WHISPERX_COMPUTE_TYPE`. Set `WHISPERX_DIARIZE=1` only when `HF_TOKEN` is
available.

2026-06-10 validation update: the broken global console entry point was bypassed
with an ignored local venv at `.audio-tools/whisperx-venv`. Its
`bin/whisperx --help` command worked, and non-diarization parity passed with
`WHISPERX_MODEL=tiny.en`, `WHISPERX_LANGUAGE=en`, `WHISPERX_DEVICE=cpu`, and
`WHISPERX_COMPUTE_TYPE=int8`. Token-gated diarization parity was not run because
`HF_TOKEN` was absent, so pyannote-backed diarization parity remains incomplete.

Token-gated continuation command shape:

```bash
test -n "$HF_TOKEN"

RUN_WHISPERX_PARITY_TESTS=1 \
WHISPERX_COMMAND="$PWD/.audio-tools/whisperx-venv/bin/whisperx" \
WHISPERX_MODEL="tiny.en" \
WHISPERX_LANGUAGE="en" \
WHISPERX_DEVICE="cpu" \
WHISPERX_COMPUTE_TYPE="int8" \
WHISPERX_DIARIZE=1 \
WHISPERX_AUDIO_PATH="$SMOKE_ROOT/audio/native-transcription-smoke.wav" \
HF_TOKEN="$HF_TOKEN" \
cargo test --test audio_transcription_native_contracts \
  external_whisperx_parity_when_requested -- --ignored --nocapture
```

2026-06-10 token-gated result: setup/auth failure. WhisperX reached pyannote
diarization, but Hugging Face rejected access to
`pyannote/speaker-diarization-community-1`. Initial non-secret error prefix:
`huggingface_hub.errors.GatedRepoError: 403 Client Error`; after refreshing the
token, the rerun failed with
`huggingface_hub.errors.HfHubHTTPError: 403 Forbidden` until the fine-grained
token allowed public gated repositories and access to the pyannote model.

2026-06-11 token-gated result: pass. WhisperX diarization parity completed with
`WHISPERX_DIARIZE=1`, `.audio-tools/whisperx-venv/bin/whisperx`, `tiny.en`,
CPU, int8, and `HF_TOKEN="$HF_TOKEN"`. Pyannote/Hugging Face access was valid
for this run. No Rust parser or contract bug was exposed, and no token value
was documented.
