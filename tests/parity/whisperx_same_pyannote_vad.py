#!/usr/bin/env python3
"""Run WhisperX with a pinned pyannote VAD model.

The first, dependency-free stage re-executes this script with the Python
interpreter beside the configured WhisperX executable. The second stage swaps
only WhisperX's VAD provider; transcription, alignment, diarization, and output
remain owned by the installed, version-checked WhisperX command.
"""

from __future__ import annotations

import os
import sys
from pathlib import Path


def pop_option(name: str, *, required: bool = True) -> str | None:
    try:
        index = sys.argv.index(name)
    except ValueError:
        if required:
            raise SystemExit(f"missing required wrapper option: {name}") from None
        return None
    try:
        value = sys.argv[index + 1]
    except IndexError:
        raise SystemExit(f"missing value for wrapper option: {name}") from None
    del sys.argv[index : index + 2]
    return value


def reexec_with_whisperx_python() -> None:
    wrapped_command = pop_option("--wrapped-command")
    if wrapped_command is None:
        raise SystemExit("missing wrapped WhisperX command")
    python = Path(wrapped_command).resolve().with_name("python")
    if not python.is_file():
        raise SystemExit(f"WhisperX Python interpreter does not exist: {python}")
    os.execve(
        python,
        [str(python), str(Path(__file__).resolve()), *sys.argv[1:]],
        os.environ.copy(),
    )


if "--wrapped-command" in sys.argv:
    reexec_with_whisperx_python()

model_id = pop_option("--pyannote-vad-model")
revision = pop_option("--pyannote-vad-revision")

import numpy as np
import torch
from pyannote.audio import Inference, Model
from whisperx import asr as whisperx_asr
from whisperx.__main__ import cli
from whisperx.vads.pyannote import Pyannote as StockPyannote
from whisperx.vads.vad import Vad


class PinnedPyannote(Vad):
    """WhisperX VAD adapter using one immutable pyannote model revision."""

    def __init__(self, device: torch.device, token: str | None = None, **kwargs: object) -> None:
        super().__init__(float(kwargs["vad_onset"]))
        model = Model.from_pretrained(
            model_id,
            revision=revision,
            token=token or os.environ.get("HF_TOKEN"),
        )
        if model is None:
            raise RuntimeError(f"could not load {model_id}@{revision}")
        model.to(device)
        self.inference = Inference(
            model,
            device=device,
            pre_aggregation_hook=lambda scores: np.max(scores, axis=-1, keepdims=True),
        )

    def __call__(self, audio: object, **kwargs: object) -> object:
        return self.inference(audio)

    preprocess_audio = staticmethod(StockPyannote.preprocess_audio)
    merge_chunks = staticmethod(StockPyannote.merge_chunks)


whisperx_asr.Pyannote = PinnedPyannote
cli()
