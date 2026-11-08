# Issue #428: speech evidence and music-only hallucinations

Owner decision (2026-11-03, option A): go beyond WhisperX parity so that
music-only audio yields no fabricated words, while genuine speech, including
quiet speech and speech over music, is kept and uncertainty stays explicit.

## What changed

Audio Analysis ([#233](https://github.com/moritzbrantner/audio-analysis/issues/233),
PR [#234](https://github.com/moritzbrantner/audio-analysis/pull/234)) owns the
reusable evidence and decoder mechanics:

- A window whose every fallback attempt exceeds the compression-ratio limit is
  withheld (`compressionRejectedWindows`), also for one-temperature schedules.
- The pipeline passes the VAD observation to ASR as `AsrRequest::speech_evidence`
  (`modelSpeechCandidate`, `energyActivityOnly`, `notDetected` for the
  full-source fallback, `unmeasured`).
- The opt-in `CandleWhisperNoSpeechGate::RequireModelVadSpeech` keeps the
  faster-whisper log-probability override only for model-VAD speech candidates.
  Elsewhere a window above the no-speech threshold is withheld even when its
  average log probability passes. When the VAD found no spans, the
  full-source safety net decodes only the first temperature attempt, so
  sampling at higher temperatures cannot produce text that merely passes the
  thresholds.
- `windowDecodeEvidence` records each window's span, no-speech probability,
  average log probability, compression ratio, temperatures and outcome.

Native WhisperX composes the policy:

- Native ASR requests `RequireModelVadSpeech` with the existing thresholds
  (no-speech 0.6, log probability -1.0, compression 2.4). No new threshold was
  calibrated from these clips.
- Automatic Workflow Selection without diarization prefers a cached Silero VAD
  model that loads with the available ONNX Runtime
  (`<--model-dir>/silero-vad/silero_vad.onnx`, then the Torch Hub cache) and
  otherwise keeps energy VAD. Nothing is downloaded.
- The report's `speechEvidence` exposes the VAD observation, compression
  rejections and no-speech rejection. WhisperX-compatible transcript JSON is
  unchanged. None of these values labels audio as music or silence.

## Real-audio acceptance (2026-11-08)

Native CLI, CPU, `--model small --language en --compute-type float32
--batch-size 1 --no-align --model-cache-only` with the cached
`openai/whisper-small` (no downloads), 16 kHz mono WAV, ONNX Runtime 1.26 via
`ORT_DYLIB_PATH` for Silero and pyannote. Builds:

- **main**: Native WhisperX `47e03ef` with Audio Analysis `d7c7802`.
- **#234 draft**: Audio Analysis `477c6da` (compression rejection only) plus
  per-window evidence. For speech, silence and noise it behaved like main: no
  compression rejection occurred.
- **fix**: Audio Analysis `e47b6f5` with this branch. Rows marked † ran on the
  intermediate `c647109`, which lacked only the first-attempt safety net.
  That net changes only inputs where the VAD found no spans, and all of those
  were re-run on `e47b6f5`.

Inputs:

- Music only: the CC BY *Blender 3.6 LTS - Reel*
  (https://www.youtube.com/watch?v=BnY2JdBrvaI), 84.8 s.
- Speech over music: the CC BY *Blender Conference 2026 Recap*
  (https://www.youtube.com/watch?v=sO_lhW8av3U), 148.8 s.
- Speech: the 25 s smoke clip `native-transcription-smoke.wav`. Its speech is
  in 5.7-14.6 s, taken from the recap.
- Quiet speech: the same clip at -30 dB.
- Speech over music, 0 dB and +6 dB SNR: speech 5.4-14.8 s of the smoke clip
  mixed at 12 s into the first 30 s of the reel (ffmpeg `amix`).
- Silence: 30 s digital silence. Noise: 30 s pink noise (amplitude 0.3).

| Input | Build / VAD | Transcript | Evidence |
| --- | --- | --- | --- |
| Music only | main, auto (energy) | `Oh!` (0-30 s), `No, no, no, no, no.` (60-82.5 s) | |
| Music only | main, Silero | `Oh!`, `Hello, hello, …` loop | Silero found no spans; full-source fallback |
| Music only | main, pyannote | `L?`, `F?`, `O?`, `T?` (11 segments, 69-82 s) | avg log prob -2.18 after all temperatures |
| Music only | #234 draft, energy | `Oh!`, `No, no, no, no, no.` | `Oh!`: no-speech 0.770, log prob -0.688 |
| Music only | #234 draft, Silero | `Oh!` | loop withheld (compression 22.4) |
| Music only | #234 draft, pyannote | unchanged `L?`/`F?` | `modelSpeechCandidate` |
| Music only | † auto (Silero), all temperatures | empty (WAV) / `Space, Space` (the 24 kbps Opus smoke copy, 60-84.8 s) | the high-temperature sample passed the thresholds |
| **Music only** | **fix, auto (Silero)** | **empty, both WAV and Opus** | `notDetected`; windows: noSpeechRejected (0.770), noSpeechRejected (0.762), compressionRejected (greedy loop, 19.7); schedule `0` |
| Music only | † `--vad-method energy` | `No, no, no, no, no.` (60-82.5 s) | `Oh!` withheld; last window no-speech 0.500 < 0.6 |
| Speech | #234 draft, energy | full sentence, 0-25 s | no-speech 0.022 |
| Speech | † auto (Silero) | same words, 5.67-14.42 s | `modelSpeechCandidate`, no-speech 0.005 |
| Quiet speech (-30 dB) | #234 draft, energy | full sentence | energy `notDetected`, fallback no-speech 0.027 |
| Quiet speech (-30 dB) | fix, `--vad-method energy` | full sentence, unchanged | energy `notDetected`; greedy safety net, no-speech 0.027 |
| Quiet speech (-30 dB) | † auto (Silero) | same words, 5.70-14.33 s | `modelSpeechCandidate` |
| Quiet speech (-45 dB) | fix, auto (Silero) | `vendor is just such a great tool …` (`Blender` misheard at -45 dB), 5.99-14.38 s | `modelSpeechCandidate`, no-speech 0.090 |
| Speech over music 0 dB | main, auto (energy) | `Glenda is just such a great tool …` | `Glenda` is the model's misrecognition of `Blender` at 0 dB |
| Speech over music 0 dB | fix, auto (Silero) | same words, 12.32-21.27 s | no-speech 0.011 |
| Speech over music 0 dB | † `--vad-method energy` | same words, 0-30 s | no-speech 0.062 |
| Speech over music +6 dB | main / † Silero / † energy | full correct sentence in all three | no-speech 0.006 (Silero) / 0.055 (energy) |
| Conference recap | #234 draft, energy | 7 segments; ends `And we're coming, coming out` ×3 (138-146.6 s) | last window no-speech 0.585 |
| Conference recap | fix, auto (Silero) | every spoken passage (8 segments, 25.7-111.2 s), with small wording changes from the different chunking (`Our engineers are` for `For software engineers it's`; `default key` for `default kick`); the 138-146.6 s phrase is not transcribed | `modelSpeechCandidate` |
| Silence | #234 draft, energy | `you` | no-speech 0.938, log prob -0.392 |
| Silence | fix, auto (Silero) | empty | noSpeechRejected |
| Pink noise | #234 draft / fix, auto (Silero) | empty / empty | noSpeechRejected (0.946) |

The recap's 138-146.6 s tail is vocal music, not verified speech. `small`
decodes `And we're coming, coming out` ×3 there and `large-v3` decodes
`10 with 10 with … now`. Both have a no-speech probability of about 0.58.
Silero proposes no speech there. With this change the phrase is not
transcribed. That is accepted as unverified rather than claimed as a correct
rejection.

### Cost

No inference pass or model is added. Silero VAD took 0.09-0.56 s per input.
Energy VAD took under 0.01 s. Peak RSS stayed about 1.9 GB on every build.
Generated decoder tokens per input:

| Input | Before (main or #234 draft, energy) | Fix (auto, Silero) |
| --- | --- | --- |
| Music only | 2348 tokens, 9 min 00 s | 531 tokens, 1 min 04 s |
| Speech | 62 | 34 † |
| Conference recap | 234 | 119 |
| Speech over music, 0 dB | 50 | 33 |

On the music-only clip, sampling stopped and the time fell from 9 minutes to
1 minute. Before, the last window sampled five more temperatures with
`best_of` 5. Now Silero finds no spans and the full-source safety net decodes
each window once. Wall times were measured on a shared 16-core host and some
runs overlapped compilation. The token counts are the comparable cost measure.

### Opt-in regression

`native_asr_music_only_reel_does_not_fabricate_speech` in
`crates/native-whisperx-cli/tests/native_asr_cache_smoke.rs` passed against
this branch with `SMOKE_ROOT` holding the 85 s reel
(`audio/blender-3.6-lts-reel-0-85s.opus`), the cached `small` model, the Torch
Hub Silero model and `ORT_DYLIB_PATH` pointing to ONNX Runtime 1.26:

```sh
SMOKE_ROOT=/path/to/smoke ORT_DYLIB_PATH=/path/to/libonnxruntime.so \
  cargo test --release --locked -p native-whisperx-cli \
  --test native_asr_cache_smoke \
  native_asr_music_only_reel_does_not_fabricate_speech -- --ignored --exact
```

With the intermediate `c647109` (all temperatures) the same test failed on
`Space, Space`. With `e47b6f5` it passed in 70.7 s. All eight opt-in tests in
that file passed on this branch, including the 40 s reel chronology, the
English recap without `--language` and the Q8 CPU smoke. The Q8 test used
`NATIVE_WHISPERX_Q8_BUNDLE` and `NATIVE_WHISPERX_Q8_WAV`.

## Known limits

- **Energy-only environments.** Without a loadable Silero model, for example
  with no Torch Hub or model-dir copy or ONNX Runtime older than 1.24,
  automatic selection keeps energy VAD. The reel then still yields
  `No, no, no, no, no.`: that window's no-speech probability (0.500) is below
  the 0.6 threshold, and RMS energy is not speech evidence.
- **Pyannote.** pyannote VAD, the automatic choice with `--diarize`, proposes
  parts of the reel as speech, so the faster-whisper rule applies and
  low-confidence `L?`/`F?` fragments remain.
- **Fixtures not tested.** No independently annotated singing or
  ambiguous-speech fixture was available. Real singing is unverified.
- **Consumer path.** YouTube Corpus ingest, store and retrieval was not
  re-run here.
