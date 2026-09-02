#!/usr/bin/env python3
"""Build a verified native bundle from pyannote community diarization.

The converter resolves only the exact pinned snapshot through a caller-provided
directory or the standard Hugging Face cache/authentication state. It never
accepts a token argument and never writes source paths, credentials, model
weights, or audio fixtures into the repository.
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

MODEL_ID = "pyannote/speaker-diarization-community-1"
REVISION = "3533c8cf8e369892e6b79ff1bf80f7b0286a54ee"
LICENSE = "CC-BY-4.0"
SOURCE_HASHES = {
    "config.yaml": "5ce2bfa9a938dc132cec1172592d65173cbb8f444ea1e4133f10f9391de155be",
    "README.md": "2db91f9265bd81f1653ff088b5bff22bf6aebebea03328513af65501643f8a31",
    "segmentation/pytorch_model.bin": "7ad24338d844fb95985486eb1a464e32d229f6d7a03c9abe60f978bacf3f816e",
    "embedding/pytorch_model.bin": "6f10ff60898a1d185fa22e1d11e0bfa8a92efec811f11bca48cb8cafebefd929",
    "embedding/README.md": "fa9e5105ae95edb231d841476cdb91eef4be0621c372ed4f7d3421294b5f8ad7",
    "plda/plda.npz": "9b77bcd840692710dd3496f62ecfeed8d8e5f002fd991b785079b244eab7d255",
    "plda/xvec_transform.npz": "325f1ce8e48f7e55e9c8aa47e05d2766b7c48c4b25b8de8dd751e7a4cc5fbe8f",
    "plda/README.md": "e1316dbbeb3261431478d48ceebbd4bba395c3587e7b80c254dbab00f1209d0a",
}
SAMPLE_RATE = 16_000
WINDOW_SECONDS = 10
WINDOW_SAMPLES = SAMPLE_RATE * WINDOW_SECONDS
SEGMENTATION_FRAMES = 589
LOCAL_SPEAKERS = 3
EMBEDDING_DIMENSION = 256
PLDA_DIMENSION = 128
ONNX_OPSET = 17
FIXTURE_SEED = 218
CLUSTERING = {
    "kind": "vbx",
    "threshold": 0.6,
    "fa": 0.07,
    "fb": 0.8,
    "maxIters": 20,
    "minActiveRatio": 0.2,
    "constrainedAssignment": True,
}


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
    return Path(
        snapshot_download(
            repo_id=MODEL_ID,
            revision=REVISION,
            allow_patterns=list(SOURCE_HASHES),
            token=True,
        )
    )


def validate_source(source: Path) -> dict[str, str]:
    hashes = {}
    for name, expected in SOURCE_HASHES.items():
        path = source / name
        if not path.is_file():
            raise RuntimeError(f"canonical source file is missing: {name}")
        hashes[name] = digest(path)
        if hashes[name] != expected:
            raise RuntimeError(
                f"source checksum mismatch for {name} at pinned revision {REVISION}"
            )
    return hashes


def package(name: str) -> str:
    try:
        return importlib.metadata.version(name)
    except importlib.metadata.PackageNotFoundError as error:
        raise RuntimeError(f"conversion dependency is not installed: {name}") from error


def runtime():
    try:
        import numpy
        import onnx
        import onnxruntime
        import torch
        import yaml
        from pyannote.audio import Pipeline
        from pyannote.audio.utils.powerset import Powerset
        from torchaudio.compliance import kaldi
    except ImportError as error:
        raise RuntimeError(
            "install torch, torchaudio, pyannote.audio, numpy, pyyaml, onnx, and "
            "onnxruntime in one Python environment"
        ) from error
    return numpy, onnx, onnxruntime, torch, yaml, Pipeline, Powerset, kaldi


def validate_pipeline_config(source: Path, yaml_module) -> None:
    config = yaml_module.safe_load((source / "config.yaml").read_text(encoding="utf-8"))
    if config.get("pipeline", {}).get("name") != "pyannote.audio.pipelines.SpeakerDiarization":
        raise RuntimeError("pinned source does not declare the SpeakerDiarization pipeline")
    params = config.get("params", {})
    if params.get("clustering") != {
        "threshold": CLUSTERING["threshold"],
        "Fa": CLUSTERING["fa"],
        "Fb": CLUSTERING["fb"],
    }:
        raise RuntimeError("pinned source VBx parameters do not match the native contract")
    pipeline = config.get("pipeline", {}).get("params", {})
    if pipeline.get("clustering") != "VBxClustering" or pipeline.get(
        "embedding_exclude_overlap"
    ) is not True:
        raise RuntimeError("pinned source does not declare the approved VBx pipeline")


def conversion_modules(torch, Powerset, kaldi, segmentation_model, embedding_model):
    class Segmentation(torch.nn.Module):
        def __init__(self):
            super().__init__()
            self.model = segmentation_model
            self.to_multilabel = Powerset(LOCAL_SPEAKERS, 2)

        def forward(self, waveform):
            return self.to_multilabel(self.model(waveform))

    class Embedding(torch.nn.Module):
        """Fixed-window Kaldi fbank plus the pinned pyannote WeSpeaker model."""

        def __init__(self):
            super().__init__()
            self.resnet = embedding_model.resnet
            self.register_buffer(
                "window",
                torch.hamming_window(
                    400, periodic=False, alpha=0.54, beta=0.46
                ),
            )
            mel, _ = kaldi.get_mel_banks(
                80, 512, 16_000.0, 20.0, 0.0, 100.0, -500.0, 1.0
            )
            self.register_buffer("mel", torch.nn.functional.pad(mel, (0, 1)))
            self.register_buffer("fft_window", torch.ones(512))

        def forward(self, waveform, masks):
            frames = waveform[0, 0].unfold(0, 400, 160)
            frames = frames - frames.mean(dim=1, keepdim=True)
            previous = torch.nn.functional.pad(
                frames.unsqueeze(0), (1, 0), mode="replicate"
            ).squeeze(0)[:, :-1]
            frames = (frames - 0.97 * previous) * self.window
            frames = torch.nn.functional.pad(frames, (0, 112))
            spectrum = torch.stft(
                frames,
                n_fft=512,
                hop_length=512,
                win_length=512,
                window=self.fft_window,
                center=False,
                return_complex=False,
            )
            spectrum = (spectrum * spectrum).sum(dim=-1).squeeze(-1)
            features = (
                torch.mm(spectrum, self.mel.T)
                .clamp_min(torch.finfo(torch.float32).eps)
                .log()
                .unsqueeze(0)
            )
            features = features - features.mean(dim=1, keepdim=True)
            return self.resnet(features, weights=masks)[1]

    return Segmentation().eval(), Embedding().eval()


def export_models(source: Path, output: Path, tolerance: float) -> tuple[dict, object]:
    numpy, onnx, onnxruntime, torch, yaml, Pipeline, Powerset, kaldi = runtime()
    validate_pipeline_config(source, yaml)
    pipeline = Pipeline.from_pretrained(str(source))
    segmentation_model = pipeline._segmentation.model.eval()
    embedding_model = pipeline._embedding.model_.eval()
    segmentation, embedding = conversion_modules(
        torch, Powerset, kaldi, segmentation_model, embedding_model
    )

    generator = torch.Generator(device="cpu").manual_seed(FIXTURE_SEED)
    waveform = torch.randn(
        (1, 1, WINDOW_SAMPLES), generator=generator, dtype=torch.float32
    )
    masks = (
        torch.rand((1, SEGMENTATION_FRAMES), generator=generator) > 0.35
    ).to(torch.float32)
    with torch.no_grad():
        expected_segmentation = segmentation(waveform).detach().cpu().numpy()
        expected_embedding = (
            embedding_model(waveform, weights=masks).detach().cpu().numpy()
        )

    if expected_segmentation.shape != (1, SEGMENTATION_FRAMES, LOCAL_SPEAKERS):
        raise RuntimeError(
            f"unexpected segmentation shape: {expected_segmentation.shape}"
        )
    if expected_embedding.shape != (1, EMBEDDING_DIMENSION):
        raise RuntimeError(f"unexpected embedding shape: {expected_embedding.shape}")

    segmentation_path = output / "segmentation.onnx"
    embedding_path = output / "embedding.onnx"
    torch.onnx.export(
        segmentation,
        waveform,
        segmentation_path,
        input_names=["waveform"],
        output_names=["segmentations"],
        opset_version=ONNX_OPSET,
        do_constant_folding=True,
    )
    torch.onnx.export(
        embedding,
        (waveform, masks),
        embedding_path,
        input_names=["waveform", "masks"],
        output_names=["embeddings"],
        opset_version=ONNX_OPSET,
        do_constant_folding=True,
    )
    onnx.checker.check_model(str(segmentation_path))
    onnx.checker.check_model(str(embedding_path))

    segmentation_session = onnxruntime.InferenceSession(
        str(segmentation_path), providers=["CPUExecutionProvider"]
    )
    embedding_session = onnxruntime.InferenceSession(
        str(embedding_path), providers=["CPUExecutionProvider"]
    )
    actual_segmentation = segmentation_session.run(
        ["segmentations"], {"waveform": waveform.numpy()}
    )[0]
    actual_embedding = embedding_session.run(
        ["embeddings"], {"waveform": waveform.numpy(), "masks": masks.numpy()}
    )[0]
    segmentation_difference = float(
        numpy.max(numpy.abs(expected_segmentation - actual_segmentation))
    )
    embedding_difference = float(
        numpy.max(numpy.abs(expected_embedding - actual_embedding))
    )
    for kind, difference in [
        ("segmentation", segmentation_difference),
        ("embedding", embedding_difference),
    ]:
        if not math.isfinite(difference) or difference > tolerance:
            raise RuntimeError(
                f"{kind} ONNX comparison difference {difference} exceeds {tolerance}"
            )
    return (
        {
            "segmentationMaxAbsoluteDifference": segmentation_difference,
            "embeddingMaxAbsoluteDifference": embedding_difference,
            "packages": {
                name: package(name)
                for name in [
                    "torch",
                    "torchaudio",
                    "pyannote.audio",
                    "numpy",
                    "onnx",
                    "onnxruntime",
                ]
            },
        },
        pipeline,
    )


def convert_plda(source: Path, output: Path) -> None:
    numpy, *_ = runtime()
    transform = numpy.load(source / "plda/xvec_transform.npz")
    plda = numpy.load(source / "plda/plda.npz")
    values = {
        "schemaVersion": 1,
        "inputDimension": EMBEDDING_DIMENSION,
        "outputDimension": PLDA_DIMENSION,
        "mean1": transform["mean1"].tolist(),
        "mean2": transform["mean2"].tolist(),
        "lda": transform["lda"].tolist(),
    }
    if (
        transform["mean1"].shape != (EMBEDDING_DIMENSION,)
        or transform["mean2"].shape != (PLDA_DIMENSION,)
        or transform["lda"].shape != (EMBEDDING_DIMENSION, PLDA_DIMENSION)
    ):
        raise RuntimeError("pinned PLDA transform dimensions are incompatible")
    (output / "plda_transform.json").write_text(
        json.dumps(values, separators=(",", ":")) + "\n", encoding="utf-8"
    )
    values = {
        "schemaVersion": 1,
        "dimension": PLDA_DIMENSION,
        "mean": plda["mu"].tolist(),
        "transform": plda["tr"].tolist(),
        "psi": plda["psi"].tolist(),
    }
    if (
        plda["mu"].shape != (PLDA_DIMENSION,)
        or plda["tr"].shape != (PLDA_DIMENSION, PLDA_DIMENSION)
        or plda["psi"].shape != (PLDA_DIMENSION,)
        or not numpy.isfinite(plda["psi"]).all()
        or not (plda["psi"] > 0).all()
    ):
        raise RuntimeError("pinned PLDA model dimensions or values are incompatible")
    (output / "plda_model.json").write_text(
        json.dumps(values, separators=(",", ":")) + "\n", encoding="utf-8"
    )
    (output / "clustering.json").write_text(
        json.dumps(CLUSTERING, separators=(",", ":")) + "\n", encoding="utf-8"
    )


def validate_two_speaker_fixture(pipeline, fixture: Path) -> dict:
    numpy, *_ = runtime()
    if not fixture.is_file():
        raise RuntimeError(f"two-speaker fixture does not exist: {fixture}")
    result = pipeline(str(fixture), num_speakers=2)
    annotation = result.speaker_diarization
    turns = list(annotation.itertracks(yield_label=True))
    speakers = {speaker for _, _, speaker in turns}
    embeddings_finite = bool(numpy.isfinite(result.speaker_embeddings).all())
    if len(speakers) != 2 or not turns or not embeddings_finite:
        raise RuntimeError(
            "pinned pipeline did not produce a complete finite two-speaker assignment"
        )
    return {
        "fixtureSha256": digest(fixture),
        "requestedSpeakers": 2,
        "assignedSpeakers": len(speakers),
        "turnCount": len(turns),
        "embeddingsFinite": embeddings_finite,
    }


def license_document() -> str:
    return "\n".join(
        [
            "# Model license",
            "",
            f"`{MODEL_ID}` at `{REVISION}` declares `CC-BY-4.0`.",
            "",
            "License reference: https://creativecommons.org/licenses/by/4.0/",
            "",
            "This file documents the license of the converted model artifacts; "
            "native-whisperx itself retains its repository license.",
            "",
        ]
    )


def provenance(manifest: dict) -> str:
    comparison = manifest["numericalComparison"]
    return "\n".join(
        [
            "# Native pyannote community diarization bundle provenance",
            "",
            f"- Source: `{MODEL_ID}`",
            f"- Pinned revision: `{REVISION}`",
            f"- License: `{LICENSE}`",
            "- Distribution: caller-owned checksum-addressed local cache snapshot",
            f"- Conversion command: `{manifest['conversion']['command']}`",
            f"- Python: `{manifest['conversion']['python']}`",
            f"- ONNX opset: `{manifest['conversion']['onnxOpset']}`",
            "- Segmentation contract: `waveform` [1, 1, 160000] -> "
            "`segmentations` [1, 589, 3]",
            "- Embedding contract: `waveform` [1, 1, 160000] + `masks` "
            "[1, 589] -> `embeddings` [1, 256]",
            f"- Segmentation max absolute difference: "
            f"{comparison['segmentationMaxAbsoluteDifference']}",
            f"- Embedding max absolute difference: "
            f"{comparison['embeddingMaxAbsoluteDifference']}",
            f"- Numerical tolerance: {comparison['tolerance']}",
            "- PLDA: 256-to-128 x-vector transform plus 128-dimensional model",
            "- Clustering: pinned VBx threshold/Fa/Fb configuration",
            "",
            "Source and generated SHA-256 values are retained in "
            "`pyannote_diarization_manifest.json`. No credentials, source weights, "
            "audio, or absolute source paths are stored in this bundle.",
            "",
            "Automatic remote hydration is intentionally out of scope. Verify and "
            "install this immutable local snapshot before cache-only execution.",
            "",
        ]
    )


def artifact_set_digest(files: dict[str, str]) -> str:
    canonical = json.dumps(files, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(canonical).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir", type=Path)
    parser.add_argument("--two-speaker-fixture", type=Path, required=True)
    cache_home = Path(os.environ.get("XDG_CACHE_HOME", Path.home() / ".cache"))
    parser.add_argument(
        "--output-root",
        type=Path,
        default=cache_home / "native-whisperx/model-bundles",
    )
    parser.add_argument("--tolerance", type=float, default=1e-4)
    args = parser.parse_args()
    if not math.isfinite(args.tolerance) or args.tolerance <= 0:
        parser.error("--tolerance must be finite and positive")

    source = source_snapshot(args.source_dir)
    input_hashes = validate_source(source)
    repository = (
        args.output_root / "models--pyannote--speaker-diarization-community-1"
    )
    snapshots = repository / "snapshots"
    snapshots.mkdir(parents=True, exist_ok=True)
    staging = Path(tempfile.mkdtemp(prefix=".community-1-", dir=snapshots))
    destination = None
    created_destination = False
    ref_staging = None
    try:
        comparison, pipeline = export_models(source, staging, args.tolerance)
        convert_plda(source, staging)
        end_to_end = validate_two_speaker_fixture(
            pipeline, args.two_speaker_fixture.resolve()
        )
        (staging / "LICENSE.md").write_text(license_document(), encoding="utf-8")
        manifest = {
            "schemaVersion": 1,
            "kind": "pyannote-diarization",
            "source": {
                "modelId": MODEL_ID,
                "revision": REVISION,
                "license": LICENSE,
            },
            "conversion": {
                "command": "python scripts/convert_pyannote_community.py "
                "--source-dir <standard-hugging-face-cache-snapshot> "
                "--two-speaker-fixture <local-two-speaker-wav>",
                "python": sys.version.split()[0],
                "packages": comparison.pop("packages"),
                "onnxOpset": ONNX_OPSET,
                "inputHashes": input_hashes,
            },
            "modelId": MODEL_ID,
            "sampleRate": SAMPLE_RATE,
            "labelFormat": "SPEAKER_{:02}",
            "segmentation": {
                "inputName": "waveform",
                "outputName": "segmentations",
                "durationSeconds": float(WINDOW_SECONDS),
                "stepRatio": 0.1,
                "powerset": True,
                "frames": SEGMENTATION_FRAMES,
                "localSpeakers": LOCAL_SPEAKERS,
            },
            "embedding": {
                "waveformInputName": "waveform",
                "maskInputName": "masks",
                "outputName": "embeddings",
                "dimension": EMBEDDING_DIMENSION,
                "maskFrames": SEGMENTATION_FRAMES,
            },
            "clustering": CLUSTERING,
            "numericalComparison": {
                "tolerance": args.tolerance,
                "fixtureSeed": FIXTURE_SEED,
                **comparison,
            },
            "endToEndComparison": end_to_end,
            "files": {},
        }
        provenance_path = staging / "MODEL_PROVENANCE.md"
        provenance_path.write_text(provenance(manifest), encoding="utf-8")
        manifest["files"] = {
            name: digest(staging / name)
            for name in [
                "segmentation.onnx",
                "embedding.onnx",
                "plda_transform.json",
                "plda_model.json",
                "clustering.json",
                "MODEL_PROVENANCE.md",
                "LICENSE.md",
            ]
        }
        bundle_digest = artifact_set_digest(manifest["files"])
        manifest["artifactSetSha256"] = bundle_digest
        (staging / "pyannote_diarization_manifest.json").write_text(
            json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )
        snapshot_name = f"sha256-{bundle_digest}"
        destination = snapshots / snapshot_name
        if destination.exists():
            raise RuntimeError(
                f"refusing to overwrite immutable bundle snapshot: {destination}"
            )
        staging.rename(destination)
        created_destination = True
        refs = repository / "refs"
        refs.mkdir(parents=True, exist_ok=True)
        ref_staging = refs / f".main-{os.getpid()}"
        ref_staging.write_text(snapshot_name + "\n", encoding="utf-8")
        ref_staging.replace(refs / "main")
    except Exception:
        shutil.rmtree(staging, ignore_errors=True)
        if created_destination and destination is not None:
            shutil.rmtree(destination, ignore_errors=True)
        if ref_staging is not None:
            ref_staging.unlink(missing_ok=True)
        raise
    print(
        json.dumps(
            {
                "kind": "pyannote-diarization",
                "sourceRevision": REVISION,
                "artifactSetSha256": bundle_digest,
                "snapshot": snapshot_name,
                "outputRoot": str(args.output_root),
            }
        )
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except RuntimeError as error:
        print(f"conversion failed: {error}", file=sys.stderr)
        raise SystemExit(2)
