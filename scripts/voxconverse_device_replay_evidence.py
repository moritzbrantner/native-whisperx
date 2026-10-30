#!/usr/bin/env python3
"""Score a frozen device replay (e.g. CUDA) of the VoxConverse primary slice.

The replay reuses the selected cohort and the checked-in CPU evidence as the
baseline. Every replay report is validated before any held-out RTTM is read.
"""
import argparse
import csv
import hashlib
import json
from pathlib import Path
import re
import subprocess
import wave

from real_conversation_evidence import score
from voxconverse_diarization_evidence import reference_turns

RUN_LINE = re.compile(r"^([a-z]{5}) exit=(\d+) wall=([0-9.]+)$")


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load_replay(run_dir, corpus_dir, baseline):
    ids = baseline["selection"]["recordings"]
    for item in baseline["inputs"]["recordings"]:
        if sha256(corpus_dir / (item["id"] + ".wav")) != item["sha256"]:
            raise ValueError("replay audio differs from the frozen cohort")
    runs = {}
    for line in (run_dir / "runs.txt").read_text().splitlines():
        match = RUN_LINE.match(line.strip())
        if not match:
            raise ValueError("invalid replay run record")
        runs[match[1]] = {"exitCode": int(match[2]), "wallSeconds": float(match[3])}
    if set(runs) != set(ids):
        raise ValueError("replay must retain every selected case exactly once")
    reports = {}
    for ident in ids:
        if runs[ident]["exitCode"] != 0:
            raise ValueError("failed inference cannot be scored as tentative success")
        observation = json.loads((run_dir / (ident + "-report.json")).read_text())["diarization"]
        if observation["countStatus"] != "tentativeAutomatic" or "countConstraints" in observation:
            raise ValueError("replay must use unconstrained automatic counts")
        expected = baseline["cases"][ids.index(ident)]["native"]["observation"]
        for key in ["modelId", "sourceRevision", "artifactSetSha256"]:
            if observation[key] != expected[key]:
                raise ValueError("replay and baseline model identities differ")
        reports[ident] = observation
    return runs, reports


def peak_gpu(run_dir, ident):
    rows = [int(row[0]) for row in csv.reader((run_dir / (ident + "-gpu.csv")).read_text().splitlines()) if row]
    return {"idleMiB": rows[0], "peakMiB": max(rows), "peakAboveIdleMiB": max(rows) - rows[0]} if rows else None


def evaluate(run_dir, corpus_dir, annotations, baseline_path, device):
    baseline = json.loads(baseline_path.read_text())
    runs, reports = load_replay(run_dir, corpus_dir, baseline)
    revision = subprocess.check_output(["git", "-C", str(annotations), "rev-parse", "HEAD"], text=True).strip()
    if revision != baseline["annotationRevision"]:
        raise ValueError("annotation source differs from the selected immutable revision")
    cases = []
    for base in baseline["cases"]:
        ident = base["id"]
        reference = reference_turns(annotations / "test" / (ident + ".rttm"), ident)
        count = len({turn["speaker"] for turn in reference})
        with wave.open(str(corpus_dir / (ident + ".wav"))) as audio:
            duration = audio.getnframes() / audio.getframerate()
        observation = reports[ident]
        metrics = score(reference, reference, observation["turns"], ident, duration)
        metrics["rttmDER"] = metrics.pop("wordsOnlyDER")
        metrics["turnBoundaries"] = metrics.pop("utteranceBoundaries")
        cpu_turns = base["native"]["observation"]["turns"]
        agreement = score(cpu_turns, cpu_turns, observation["turns"], ident, duration)["wordsOnlyDER"]
        log = (run_dir / (ident + "-native.log")).read_text()
        rss = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", log)
        cases.append({
            "id": ident, "durationSeconds": duration, "referenceSpeakerCount": count,
            "observation": observation, "countCorrect": observation["speakerCount"] == count,
            "cpuSpeakerCount": base["native"]["observation"]["speakerCount"], "metrics": metrics,
            "cpuBaselineDisagreement": agreement, "wallSeconds": runs[ident]["wallSeconds"],
            "cpuWallSeconds": base["native"]["wallSeconds"], "maxRssKiB": int(rss[1]) if rss else None,
            "gpuMemory": peak_gpu(run_dir, ident),
        })
    return {"schemaVersion": 1, "corpus": baseline["corpus"], "license": baseline["license"],
            "device": device, "annotationRevision": revision, "baseline": str(baseline_path),
            "nativeBinarySha256": (run_dir / "binary.sha256").read_text().split()[0],
            "protocol": "Same frozen cohort, model bundles and unconstrained automatic workflow as the CPU "
                        "baseline, changing only the ASR/alignment device. Replay reports are validated before "
                        "held-out RTTM is read. cpuBaselineDisagreement is permutation-invariant DER of the "
                        "replay turns against the CPU native turns (0/250 ms collars); it is device agreement, "
                        "not accuracy. Wall time covers the whole workflow; GPU memory is whole-device usage "
                        "sampled every 200 ms, reported above the idle desktop sample.",
            "summary": {"scoredCases": len(cases), "correctCounts": sum(case["countCorrect"] for case in cases),
                        "countsMatchCpu": sum(case["observation"]["speakerCount"] == case["cpuSpeakerCount"]
                                              for case in cases)},
            "cases": cases}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", required=True, type=Path)
    parser.add_argument("--corpus-dir", required=True, type=Path)
    parser.add_argument("--annotations", required=True, type=Path)
    parser.add_argument("--baseline", required=True, type=Path)
    parser.add_argument("--device", required=True)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    result = evaluate(args.run_dir, args.corpus_dir, args.annotations, args.baseline, args.device)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, allow_nan=False) + "\n")
    print(json.dumps(result["summary"]))


if __name__ == "__main__":
    main()
