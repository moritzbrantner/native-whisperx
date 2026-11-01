#!/usr/bin/env python3
"""Assemble a local Q8 Whisper bundle from public Hugging Face files.

The bundle is the five-file layout described in docs/model-bundles.md
("Q8 CPU bundle"). The sidecars come from the matching FP32 snapshot (for
example `openai/whisper-small`) and the weights from a Q8_0 GGUF with
Hugging Face tensor names (for example `oxide-lab/whisper-small-GGUF`,
`whisper-small-q8_0.gguf`). Published GGUFs may omit `general.file_type`;
this script copies the GGUF and records `general.file_type = 7` (Q8_0),
which the native loader requires.

Requires the `gguf` Python package (`pip install gguf`).
"""

import argparse
import shutil
import sys
from pathlib import Path

SIDECARS = (
    "config.json",
    "generation_config.json",
    "tokenizer.json",
    "preprocessor_config.json",
)
GGML_FTYPE_MOSTLY_Q8_0 = 7


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--snapshot", type=Path, required=True,
                        help="FP32 Hugging Face snapshot directory with the four JSON sidecars")
    parser.add_argument("--gguf", type=Path, required=True,
                        help="Q8_0 Whisper GGUF with Hugging Face tensor names")
    parser.add_argument("--output", type=Path, required=True, help="bundle directory to create")
    args = parser.parse_args()

    import gguf
    from gguf.scripts.gguf_new_metadata import MetadataDetails, copy_with_new_metadata

    reader = gguf.GGUFReader(args.gguf, "r")
    architecture = reader.get_field(gguf.Keys.General.ARCHITECTURE)
    if architecture is None or architecture.contents() != "whisper":
        print(f"{args.gguf}: general.architecture must be `whisper`", file=sys.stderr)
        return 1

    args.output.mkdir(parents=True, exist_ok=True)
    for name in SIDECARS:
        shutil.copyfile(args.snapshot / name, args.output / name)

    target = args.output / "model.q8_0.gguf"
    writer = gguf.GGUFWriter(target, arch="whisper", endianess=reader.endianess)
    new_metadata = {
        gguf.Keys.General.FILE_TYPE: MetadataDetails(
            gguf.GGUFValueType.UINT32, GGML_FTYPE_MOSTLY_Q8_0
        )
    }
    copy_with_new_metadata(reader, writer, new_metadata, remove_metadata=[])
    print(target)
    return 0


if __name__ == "__main__":
    sys.exit(main())
