# Python WhisperX oracle migration

Python WhisperX is now only a parity oracle/reference/golden source. The native
acceptance gate #195/#207 is closed. Normal product transcription has no Python
execution provider or automatic/manual Python fallback, even when the optional
`whisperx-compat` feature is enabled.

## CLI callers

Remove `--provider external-whisperx`; use the default native provider or
`--provider native`. The retired spelling is hidden from help and returns an
early migration error without running Python, decoding input, or writing output.

Native transcription uses explicit native resources. In particular:

| Former delegated combination | Native migration or limitation |
| --- | --- |
| Built-in Whisper `--task translate` | Supply `--translation-model` or `--translation-bundle` for native post-ASR OPUS-MT translation. |
| Quantized WhisperX aliases such as `int8_float16` | Intentionally unsupported; exact `--compute-type int8` selects the documented native CPU Q8 ASR-only workflow. |
| `--hotwords` or WhisperX `--fp16` controls | Intentionally unsupported; native precision is selected through compute type and device. Remove unsupported controls. |
| Non-WAV finite media | Enable `media-decode` (enabled by default), or provide WAV/samples. |
| VAD, alignment, diarization, speaker bounds/embeddings | Use their native feature/resource configuration; missing resources fail explicitly. |

The complete capability and limitation inventory is
[`parity-matrix.md`](parity-matrix.md). Missing native resources do not trigger
Python delegation.

## Rust API callers

Product callers use `AsrProvider::Native` (the default) with `run`, controlled,
selected-media, or batch workflows. These interfaces reject
`AsrProvider::ExternalWhisperX` before work begins. Legacy serialized
`externalWhisperX` values remain readable so applications can detect and migrate
stored configurations; successful deserialization does not enable execution.

For explicit oracle evidence, replace the former `run(external_config)` call
with `run_whisperx_oracle(config)` and enable `whisperx-compat`. This interface
forces oracle selection and uses `config.asr.external_whisperx` settings. Use
`WhisperxOracleConfig` for those settings; `ExternalWhisperxConfig` is a deprecated
type alias. Native product configurations need no oracle settings.

The legacy field name and type alias remain for the pre-1.0 configuration
migration window. Product execution retirement is immediate; retaining those
serialization forms is not a second product architecture.

## Maintainer tooling

Dedicated `parity`, `parity-fixtures`, `parity-preflight`, `parity-goldens`, and
reference-enabled `parity-bench` commands retain explicit oracle access:

```bash
cargo run -p native-whisperx-cli --features whisperx-compat -- parity input.wav
```

Golden dry runs and no-resource help/version commands do not spawn Python.
Default builds remain Python-free. Enabling the feature does not enable Python
execution through normal transcription. See
[ADR 0016](adr/0016-python-whisperx-is-an-explicit-oracle-only.md).
