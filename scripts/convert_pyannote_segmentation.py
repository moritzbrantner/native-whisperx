#!/usr/bin/env python3
"""Create a checksum-addressed native ONNX bundle from pyannote/segmentation-3.0.

The source is resolved only with the standard Hugging Face credential/cache.
No credential, source weight, audio fixture, or absolute source path is written
to the resulting bundle.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.metadata
import json
import math
import os
import shutil
import sys
import tempfile
from pathlib import Path

MODEL_ID = "pyannote/segmentation-3.0"
REVISION = "e66f3d3b9eb0873085418a7b813d3b369bf160bb"
SOURCE_HASHES = {
    "pytorch_model.bin": "da85c29829d4002daedd676e012936488234d9255e65e86dfab9bec6b1729298",
    "config.yaml": "fa65a47a751602f04cc570135007d76859b69e8f9f1bfdf5878a5145980d4263",
    "README.md": "a37bc19811cc1a52a4c128c33207813b1558b4e49b050b03e814d0a96d14f05d",
    "LICENSE": "63a777ad4b3c7aed4b260b084d8fb49ec781c46c70c6b599ca5d2402ef7ebe50",
}
SAMPLE_RATE, WINDOW_SECONDS, ONNX_OPSET, FIXTURE_SEED = 16_000, 10, 17, 217
WINDOW_SAMPLES = SAMPLE_RATE * WINDOW_SECONDS


def digest(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def source_snapshot(source_dir: Path | None) -> Path:
    if source_dir:
        return source_dir.resolve()
    try:
        from huggingface_hub import snapshot_download
    except ImportError as error:
        raise RuntimeError("install huggingface_hub in the conversion environment") from error
    return Path(snapshot_download(
        repo_id=MODEL_ID,
        revision=REVISION,
        allow_patterns=list(SOURCE_HASHES),
        token=True,
    ))


def validate_source(source: Path) -> dict[str, str]:
    hashes = {}
    for name, expected in SOURCE_HASHES.items():
        path = source / name
        if not path.is_file():
            raise RuntimeError(f"canonical source file is missing: {name}")
        hashes[name] = digest(path)
        if hashes[name] != expected:
            raise RuntimeError(f"source checksum mismatch for {name} at pinned revision {REVISION}")
    return hashes


def runtime():
    try:
        import numpy
        import onnx
        import onnxruntime
        import torch
        from pyannote.audio import Model
    except ImportError as error:
        raise RuntimeError(
            "install torch, pyannote.audio, onnx, and onnxruntime in one Python environment"
        ) from error
    return numpy, onnx, onnxruntime, torch, Model


def package(name: str) -> str:
    try:
        return importlib.metadata.version(name)
    except importlib.metadata.PackageNotFoundError as error:
        raise RuntimeError(f"conversion dependency is not installed: {name}") from error


def export(source: Path, output: Path, tolerance: float) -> dict:
    numpy, onnx, onnxruntime, torch, Model = runtime()
    model = Model.from_pretrained(str(source)).eval()

    class Scores(torch.nn.Module):
        def __init__(self, wrapped):
            super().__init__()
            self.wrapped = wrapped

        def forward(self, waveform):
            scores = self.wrapped(waveform)
            return scores[0] if isinstance(scores, tuple) else scores

    scores = Scores(model).eval()
    waveform = torch.randn(
        (1, 1, WINDOW_SAMPLES),
        generator=torch.Generator(device="cpu").manual_seed(FIXTURE_SEED),
        dtype=torch.float32,
    )
    with torch.no_grad():
        torch_scores = scores(waveform).detach().cpu().numpy()
    if torch_scores.ndim != 3 or torch_scores.shape[0] != 1:
        raise RuntimeError(f"unexpected pyannote score shape: {tuple(torch_scores.shape)}")

    model_path = output / "segmentation.onnx"
    torch.onnx.export(
        scores, waveform, model_path, input_names=["waveform"], output_names=["scores"],
        opset_version=ONNX_OPSET, do_constant_folding=True,
    )
    onnx.checker.check_model(str(model_path))
    session = onnxruntime.InferenceSession(str(model_path), providers=["CPUExecutionProvider"])
    onnx_scores = session.run(["scores"], {"waveform": waveform.numpy()})[0]
    difference = float(numpy.max(numpy.abs(torch_scores - onnx_scores)))
    if not math.isfinite(difference) or difference > tolerance:
        raise RuntimeError(f"ONNX comparison difference {difference} exceeds tolerance {tolerance}")
    return {
        "frameCount": int(torch_scores.shape[1]),
        # The model emits powerset scores, while the upstream local speaker
        # contract is the three-speaker chunk limit from config.yaml.
        "localSpeakerCount": 3,
        "maxAbsoluteDifference": difference,
        "packages": {
            "torch": package("torch"), "pyannote.audio": package("pyannote.audio"),
            "onnx": package("onnx"), "onnxruntime": package("onnxruntime"),
        },
    }


def provenance(manifest: dict) -> str:
    source, conversion = manifest["source"], manifest["conversion"]
    contract, check = manifest["tensorContract"], manifest["numericalComparison"]
    return "\n".join([
        "# Native pyannote VAD bundle provenance", "",
        f"- Source model: `{source['modelId']}`",
        f"- Pinned revision: `{source['revision']}`",
        f"- License: `{source['license']}`",
        "- Source layout: standard Hugging Face cache snapshot",
        f"- Conversion command: `{conversion['command']}`",
        f"- Python: `{conversion['python']}`",
        f"- ONNX opset: `{conversion['onnxOpset']}`",
        "- Canonical source checksums:",
        *[f"  - `{name}`: `{checksum}`" for name, checksum in conversion["inputHashes"].items()],
        f"- Generated `segmentation.onnx` SHA-256: `{manifest['files']['segmentation.onnx']}`",
        f"- Tensor contract: `{contract['inputName']}` {contract['inputShape']} -> `{contract['outputName']}`",
        f"- Audio preprocessing: {contract['sampleRate']} Hz mono, {contract['windowSeconds']} second windows",
        f"- Numerical check: deterministic seed {check['fixtureSeed']}, max absolute difference {check['maxAbsoluteDifference']}, tolerance {check['tolerance']}",
        "",
        "Install this immutable snapshot beneath a local `--model-dir` cache root and use `--model-cache-only` for offline resolution.",
        "No source weights, credentials, audio fixtures, or absolute paths are stored in this document.",
        "",
    ])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir", type=Path, help="canonical standard-cache source snapshot")
    cache_home = Path(os.environ.get("XDG_CACHE_HOME", Path.home() / ".cache"))
    parser.add_argument("--output-root", type=Path, default=cache_home / "native-whisperx/model-bundles")
    parser.add_argument("--tolerance", type=float, default=1e-4)
    args = parser.parse_args()
    if not math.isfinite(args.tolerance) or args.tolerance <= 0:
        parser.error("--tolerance must be finite and positive")

    source = source_snapshot(args.source_dir)
    input_hashes = validate_source(source)
    repository = args.output_root / "models--pyannote--segmentation-3.0"
    snapshots = repository / "snapshots"
    destination = snapshots / REVISION
    if destination.exists():
        raise RuntimeError(f"refusing to overwrite immutable bundle snapshot: {destination}")
    snapshots.mkdir(parents=True, exist_ok=True)
    staging = Path(tempfile.mkdtemp(prefix="pyannote-vad-", dir=snapshots))
    try:
        result = export(source, staging, args.tolerance)
        manifest = {
            "schemaVersion": 1, "kind": "pyannote-vad",
            "source": {"modelId": MODEL_ID, "revision": REVISION, "license": "MIT"},
            "conversion": {
                "command": "python scripts/convert_pyannote_segmentation.py --source-dir <standard-hugging-face-cache-snapshot>",
                "python": sys.version.split()[0], "packages": result["packages"],
                "onnxOpset": ONNX_OPSET, "inputHashes": input_hashes,
            },
            "tensorContract": {
                "inputName": "waveform", "inputShape": [1, 1, WINDOW_SAMPLES],
                "outputName": "scores", "sampleRate": SAMPLE_RATE,
                "windowSeconds": float(WINDOW_SECONDS), "frameCount": result["frameCount"],
                "localSpeakerCount": result["localSpeakerCount"],
            },
            "numericalComparison": {
                "tolerance": args.tolerance, "fixtureSeed": FIXTURE_SEED,
                "maxAbsoluteDifference": result["maxAbsoluteDifference"],
            },
            "files": {"segmentation.onnx": digest(staging / "segmentation.onnx")},
        }
        provenance_path = staging / "MODEL_PROVENANCE.md"
        provenance_path.write_text(provenance(manifest), encoding="utf-8")
        manifest["files"]["MODEL_PROVENANCE.md"] = digest(provenance_path)
        (staging / "pyannote_vad_manifest.json").write_text(
            json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )
        staging.rename(destination)
        refs = repository / "refs"
        refs.mkdir(parents=True, exist_ok=True)
        (refs / "main").write_text(REVISION + "\n", encoding="utf-8")
    except Exception:
        shutil.rmtree(staging, ignore_errors=True)
        raise
    print(json.dumps({"kind": "pyannote-vad", "revision": REVISION, "outputRoot": str(args.output_root)}))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except RuntimeError as error:
        print(f"conversion failed: {error}", file=sys.stderr)
        raise SystemExit(2)
