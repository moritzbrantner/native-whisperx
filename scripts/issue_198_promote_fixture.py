#!/usr/bin/env python3
"""One-shot current-main forward port for issue #198 parity evidence."""

from __future__ import annotations

import json
from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(
            f"{path}: expected one replacement target, found {count}: {old[:120]!r}"
        )
    file.write_text(text.replace(old, new, 1))


def promote_fixture() -> None:
    path = Path("tests/parity/asr-fixtures.json")
    data = json.loads(path.read_text())
    fixtures = data["fixtures"]
    index = next(
        index
        for index, fixture in enumerate(fixtures)
        if fixture["name"] == "small-de-no-align-cache"
    )
    fixtures[index] = {
        "name": "small-de-no-align-cache",
        "gating": True,
        "input": "audio/native-transcription-smoke-de.wav",
        "comparison": {
            "text": True,
            "segmentText": True,
            "segmentCount": True,
            "segmentTiming": True,
            "wordTiming": False,
            "charCount": False,
            "speakerTurns": False,
            "vadSegments": True,
            "vadSegmentCount": True,
            "vadSegmentTiming": True,
        },
        "nativeAsr": {
            "modelId": "small",
            "maxBatchSize": 1,
            "decode": {"beamSize": 1},
        },
        "vad": {
            "method": "silero",
            "onset": 0.5,
            "offset": 0.363,
            "chunkSize": 30.0,
            "modelBundle": "models/silero-vad",
            "modelFile": "silero_vad.onnx",
        },
        "alignment": {"enabled": False},
        "whisperx": {
            "model": "small",
            "batchSize": 1,
            "extraArgs": ["--beam_size", "1"],
        },
        "language": "de",
        "requiredDiagnostics": [
            "asrModelSource=hugging-face-cache",
            "asrModelId=openai/whisper-small",
            "beamSize=1",
            "batchExecution=candle-whisper-autoregressive-kv-cache",
            "maxBatchSize=1",
            "timingMode=noTimestamps",
            "leadingContextSeconds=0",
            "trailingContextSeconds=0",
            "sileroVadThreshold=0.5",
        ],
    }
    path.write_text(json.dumps(data, indent=2) + "\n")


def update_parity_workflow() -> None:
    path = ".github/workflows/parity-fixtures.yml"
    replace_once(
        path,
        '            asr)\n              manifest="tests/parity/asr-fixtures.json"\n              features=""',
        '            asr)\n              manifest="tests/parity/asr-fixtures.json"\n              features="whisperx-compat,silero-vad"',
    )
    replace_once(
        path,
        '''          set +e
          cargo run -p native-whisperx-cli -- "${args[@]}" \\
            > "${{ steps.parity.outputs.preflight_report }}" \\
            2>> "${{ steps.parity.outputs.progress_log }}"''',
        '''          cargo_args=("run" "-p" "native-whisperx-cli")
          if [[ -n "${{ steps.parity.outputs.features }}" ]]; then
            cargo_args+=("--features" "${{ steps.parity.outputs.features }}")
          fi
          set +e
          cargo "${cargo_args[@]}" -- "${args[@]}" \\
            > "${{ steps.parity.outputs.preflight_report }}" \\
            2>> "${{ steps.parity.outputs.progress_log }}"''',
    )
    replace_once(
        path,
        '          cargo run -p native-whisperx-cli -- "${args[@]}"\n\n      - name: Run parity fixtures',
        '''          cargo_args=("run" "-p" "native-whisperx-cli")
          if [[ -n "${{ steps.parity.outputs.features }}" ]]; then
            cargo_args+=("--features" "${{ steps.parity.outputs.features }}")
          fi
          cargo "${cargo_args[@]}" -- "${args[@]}"

      - name: Run parity fixtures''',
    )


def update_cli_contracts() -> None:
    path = Path("crates/native-whisperx-cli/tests/cli_smoke.rs")
    text = path.read_text()
    old = '.stdout(predicate::str::contains("\\\"modelId\\\": \\\"tiny.en\\\""))'
    new = '''.stdout(predicate::str::contains(
            "\\\"modelId\\\": \\\"openai/whisper-tiny.en\\\"",
        ))'''
    if text.count(old) != 1:
        raise SystemExit("cli_smoke.rs: expected one raw tiny.en modelId assertion")
    text = text.replace(old, new, 1)

    workflow_old = '''    assert!(workflow.contains("manifest=\\\"tests/parity/full-resource-fixtures.json\\\""));
    assert!(workflow.contains("fixture_args+=(\\\"--require-non-gating-passed\\\")"));'''
    workflow_new = '''    assert!(workflow.contains("manifest=\\\"tests/parity/full-resource-fixtures.json\\\""));
    assert!(workflow.contains("features=\\\"whisperx-compat,silero-vad\\\""));
    assert!(workflow.contains("fixture_args+=(\\\"--require-non-gating-passed\\\")"));'''
    if text.count(workflow_old) != 1:
        raise SystemExit("cli_smoke.rs: expected one parity workflow assertion seam")
    text = text.replace(workflow_old, workflow_new, 1)

    needle = 'fixture.name == "small-de-no-align-cache"'
    position = text.index(needle)
    start_marker = "    assert!(parsed.fixtures.iter().any(|fixture| {"
    start = text.rfind(start_marker, 0, position)
    end_marker = "    }));"
    end = text.index(end_marker, position) + len(end_marker)
    if start < 0:
        raise SystemExit("cli_smoke.rs: German fixture assertion start not found")

    replacement = '''    assert!(parsed.fixtures.iter().any(|fixture| {
        fixture.name == "small-de-no-align-cache"
            && fixture.gating
            && fixture.vad.method == native_whisperx::VadMethod::Silero
            && fixture.vad.model_bundle.as_deref() == Some(Path::new("models/silero-vad"))
            && fixture.native_asr.max_batch_size == Some(1)
            && fixture.native_asr.decode.beam_size == Some(1)
            && fixture.whisperx.batch_size == Some(1)
            && fixture.whisperx.extra_args == ["--beam_size", "1"]
            && fixture.comparison.text
            && fixture.comparison.segment_text
            && fixture.comparison.segment_count
            && fixture.comparison.segment_timing
            && fixture.comparison.vad_segments
            && fixture.comparison.vad_segment_count
            && fixture.comparison.vad_segment_timing
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "asrModelSource=hugging-face-cache")
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "asrModelId=openai/whisper-small")
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "beamSize=1")
            && fixture.required_diagnostics.iter().any(|diagnostic| {
                diagnostic == "batchExecution=candle-whisper-autoregressive-kv-cache"
            })
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "maxBatchSize=1")
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "timingMode=noTimestamps")
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "leadingContextSeconds=0")
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "trailingContextSeconds=0")
            && fixture
                .required_diagnostics
                .iter()
                .any(|diagnostic| diagnostic == "sileroVadThreshold=0.5")
    }));'''
    text = text[:start] + replacement + text[end:]
    path.write_text(text)


if __name__ == "__main__":
    promote_fixture()
    update_parity_workflow()
    update_cli_contracts()
