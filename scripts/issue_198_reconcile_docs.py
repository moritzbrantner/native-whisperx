#!/usr/bin/env python3
"""One-shot documentation reconciliation for native-whisperx issue #198."""

from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one match, found {count}: {old[:120]!r}")
    file.write_text(text.replace(old, new, 1))


replace_once(
    "docs/model-bundles.md",
    """config.json
generation_config.json
tokenizer.json
preprocessor_config.json
model.safetensors
```

Cache-only example:""",
    """config.json
generation_config.json
tokenizer.json
preprocessor_config.json
model.safetensors
```

The native model registry maps `tiny`, `tiny.en`, `base`, `base.en`, `small`,
`small.en`, `medium`, `medium.en`, `large`, `large-v1`, `large-v2`, `large-v3`,
and `large-v3-turbo` to canonical `openai/whisper-*` repositories. Explicit
Hugging Face repository IDs pass through unchanged when they identify an
`owner/repository` pair with Candle-compatible files.

Cache-only example:""",
)

replace_once(
    "docs/parity-worklist.md",
    "| Model selection | native partial | local fixture harness | Starter suite covers `tiny.en` and `small`; add more aliases as local fixtures mature. |",
    "| Model selection | native complete | pure mapping tests plus real-resource gates | Pure request-mapping tests cover every advertised Whisper alias and preserve explicit Hugging Face repository IDs; real-resource evidence retains representative `tiny.en`, `small`, and `large-v3-turbo` execution. |",
)
replace_once(
    "docs/parity-worklist.md",
    "| Language | native partial | local fixture harness | Explicit English and English-only model alias inference are gating; `small-de-no-align-cache` gates German language/model-cache coverage but keeps transcript text, segment structure, and VAD structure report-only until non-English decode drift is resolved. |",
    "| Language | native complete | local fixture harness | Explicit English and English-only alias inference remain covered. Explicit multilingual runs use the stable autoregressive KV-cache decoder; `small-de-no-align-cache` gates German transcript text, segment structure, VAD structure, language, cache source, and canonical model diagnostics. |",
)
replace_once(
    "docs/parity-worklist.md",
    """`tiny-en-no-align-cache`, `small-en-no-align-cache`, and
`tiny-language-detection` gate segment timing. `small-de-no-align-cache` gates
German language and cache diagnostics only; current native decoding still emits
`Nativa Whisper X` plus an extra short `X` segment against the WhisperX 3.8.6
reference, so German transcript text, segment structure, and VAD structure stay
report-only rather than weakening the broader English ASR gates.""",
    """`tiny-en-no-align-cache`, `small-en-no-align-cache`, and
`tiny-language-detection` gate segment timing. `small-de-no-align-cache` gates
German transcript text, segment text/count/timing, VAD segment count/timing,
language, Hugging Face cache source, canonical `openai/whisper-small` model ID,
and the stable autoregressive KV-cache decode path against WhisperX 3.8.6.""",
)

replace_once(
    "docs/parity-matrix.md",
    "| Model selection | `--model` | `rust-native complete` | Native ASR supports Whisper aliases such as `tiny.en`, `small`, and `large`, plus Hugging Face repo IDs with Candle-compatible files. |",
    "| Model selection | `--model` | `rust-native complete` | Pure mapping tests cover `tiny`, `tiny.en`, `base`, `base.en`, `small`, `small.en`, `medium`, `medium.en`, `large`, `large-v1`, `large-v2`, `large-v3`, and `large-v3-turbo`; explicit Hugging Face repository IDs pass through unchanged. Real-resource evidence retains representative `tiny.en`, `small`, and `large-v3-turbo` execution. |",
)
replace_once(
    "docs/parity-matrix.md",
    "| Language | `--language` | `rust-native complete` | English-only native Whisper aliases such as `tiny.en` provide an `en` language hint when no explicit language is supplied. |",
    "| Language | `--language` | `rust-native complete` | English-only native Whisper aliases such as `tiny.en` provide an `en` language hint when no explicit language is supplied. Explicit multilingual requests select the stable autoregressive KV-cache decoder, and `small-de-no-align-cache` gates German text, segment, VAD, language, model, and cache parity. |",
)
replace_once(
    "docs/parity-matrix.md",
    """Timing mismatch reports include native and WhisperX start/end values, absolute
start/end deltas, and the active tolerance. Remaining report-only ASR expansion
cases include `small-de-no-align-cache`, `tiny-en-alignment-alias-cache`, the
translation fixture, and `tiny-output-subtitles-highlight`.""",
    """Timing mismatch reports include native and WhisperX start/end values, absolute
start/end deltas, and the active tolerance. `small-de-no-align-cache` now gates
German transcript, segment, and VAD parity; remaining report-only cases stay
tracked by their own fixture status rather than weakening this gate.""",
)

replace_once(
    "docs/parity.md",
    """English-only Whisper aliases such as `tiny.en` provide an `en` language hint
when no explicit language is supplied, which keeps the local language-detection
fixture aligned with WhisperX for English-only models. Native `--task translate""",
    """English-only Whisper aliases such as `tiny.en` provide an `en` language hint
when no explicit language is supplied, which keeps the local language-detection
fixture aligned with WhisperX for English-only models. Explicit multilingual
requests use the stable autoregressive KV-cache decode path; the German `small`
fixture gates transcript, segment, VAD, language, canonical model, and cache
parity against WhisperX 3.8.6. Native `--task translate""",
)
replace_once(
    "docs/parity.md",
    """and the char-alignment fixture gates segment timing, word timing, and character
count. Timing reports include native and WhisperX start/end values, absolute
deltas, and the configured tolerance for each mismatch. German ASR expansion,
alignment alias/cache behavior, translation, and
`tiny-output-subtitles-highlight` remain non-gating until independently
promoted.""",
    """and the char-alignment fixture gates segment timing, word timing, and character
count. The German `small` fixture gates transcript text, segment structure, VAD
structure, language, canonical model/cache diagnostics, and stable decoder
selection. Timing reports include native and WhisperX start/end values, absolute
deltas, and the configured tolerance for each mismatch. Other expansion cases
remain governed by their own fixture status until independently promoted.""",
)
