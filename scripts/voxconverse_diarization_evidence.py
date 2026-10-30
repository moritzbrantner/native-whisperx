#!/usr/bin/env python3
"""Resource-tier VoxConverse evaluation; freeze native/oracle predictions before truth."""
import argparse
import hashlib
import json
import math
from pathlib import Path
import re
import subprocess
import wave

from real_conversation_evidence import score


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load_frozen(root):
    selection = json.loads((root / "selection-before-predictions.json").read_text())
    inputs = json.loads((root / "inputs-frozen.json").read_text())
    native = json.loads((root / "native-predictions-frozen.json").read_text())
    oracle = json.loads((root / "oracle-predictions-frozen.json").read_text())
    ids = selection["recordings"]
    if not ids or len(set(ids)) != len(ids) or any(not re.fullmatch(r"[a-z]{5}", value) for value in ids):
        raise ValueError("invalid or duplicate selected case IDs")
    expected = set(ids)
    for frozen, key in [(native, "id"), (oracle, "meeting")]:
        if frozen["cohortSha256"] != sha256(root / "selection-before-predictions.json"):
            raise ValueError("predictions do not match the selected cohort")
        if len(frozen["runs"]) != len(ids) or {run[key] for run in frozen["runs"]} != expected:
            raise ValueError("predictions must retain every selected case exactly once")
    if {item["id"] for item in inputs["recordings"]} != expected:
        raise ValueError("audio manifest does not cover the selected cohort")
    for item in inputs["recordings"]:
        if sha256(root / (item["id"] + ".wav")) != item["sha256"]:
            raise ValueError("evaluation audio changed after selection")
    reports, baselines = {}, {}
    for run in native["runs"]:
        ident = run["id"]
        if run["exitCode"] != 0:
            raise ValueError("failed inference cannot be scored as tentative success")
        path = root / (ident + "-report.json")
        if sha256(path) != run["reportSha256"]:
            raise ValueError("native report changed after freezing")
        observation = json.loads(path.read_text())["diarization"]
        count = observation["speakerCount"]
        if type(count) is not int or count < 0 or count != len({turn["speaker"] for turn in observation["turns"]}):
            raise ValueError("observed count must retain the actual anonymous clusters")
        if observation["countStatus"] != "tentativeAutomatic" or "countConstraints" in observation:
            raise ValueError("primary inference must use unconstrained automatic counts")
        if any(arg in run["command"] for arg in ["--min-speakers", "--max-speakers", "--num-speakers"]):
            raise ValueError("reference-count bounds are not permitted in primary inference")
        reports[ident] = observation
    for run in oracle["runs"]:
        ident = run["meeting"]
        path = root / (ident + "-oracle-turns.json")
        if sha256(path) != run["outputSha256"]:
            raise ValueError("oracle predictions changed after freezing")
        baseline = json.loads(path.read_text())
        if (reports[ident]["modelId"], reports[ident]["sourceRevision"]) != (baseline["modelId"], baseline["sourceRevision"]):
            raise ValueError("native and oracle checkpoint identities differ")
        baselines[ident] = baseline
    return selection, inputs, native, oracle, reports, baselines


def reference_turns(path, ident):
    turns = []
    for line in path.read_text().splitlines():
        if not line.strip():
            continue
        values = line.split()
        if len(values) != 10 or values[0] != "SPEAKER" or values[1] != ident:
            raise ValueError("invalid reference RTTM record")
        start, duration = float(values[3]), float(values[4])
        if not math.isfinite(start) or not math.isfinite(duration) or start < 0 or duration <= 0:
            raise ValueError("invalid reference RTTM timing")
        turns.append({"speaker": values[7], "startSeconds": start, "endSeconds": start + duration})
    if not turns:
        raise ValueError("reference contains no independently annotated activity")
    return sorted(turns, key=lambda turn: (turn["startSeconds"], turn["endSeconds"], turn["speaker"]))


def evaluate(root, annotations):
    # Verify every frozen artifact before consulting any held-out count/turn label.
    selection, inputs, native, oracle, reports, baselines = load_frozen(root)
    revision = subprocess.check_output(["git", "-C", str(annotations), "rev-parse", "HEAD"], text=True).strip()
    if revision != selection["annotationRevision"]:
        raise ValueError("annotation source differs from the selected immutable revision")
    cases = []
    for ident in selection["recordings"]:
        reference = reference_turns(annotations / "test" / (ident + ".rttm"), ident)
        count = len({turn["speaker"] for turn in reference})
        with wave.open(str(root / (ident + ".wav"))) as audio:
            duration = audio.getnframes() / audio.getframerate()
        observations = {}
        for kind, predicted, run in [
            ("native", reports[ident], next(run for run in native["runs"] if run["id"] == ident)),
            ("providerOracle", baselines[ident], next(run for run in oracle["runs"] if run["meeting"] == ident)),
        ]:
            metrics = score(reference, reference, predicted["turns"], ident, duration)
            metrics["rttmDER"] = metrics.pop("wordsOnlyDER")
            metrics["turnBoundaries"] = metrics.pop("utteranceBoundaries")
            observations[kind] = {"observation": predicted, "countCorrect": predicted["speakerCount"] == count,
                                  "metrics": metrics, "wallSeconds": run["wallSeconds"]}
        log = (root / (ident + "-native.log")).read_text()
        rss = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", log)
        observations["native"]["maxRssKiB"] = int(rss[1]) if rss else None
        observations["native"]["taskStarts"] = {task: len(re.findall(r"progress task start file=1 task=" + re.escape(task) + r"\b", log))
                                                for task in ["decode", "vad", "asr", "alignment", "diarization", "output"]}
        cases.append({"id": ident, "durationSeconds": duration, "referenceSpeakerCount": count, **observations})
    return {"schemaVersion": 1, "corpus": "VoxConverse test", "license": "CC-BY-4.0",
            "source": "https://github.com/joonson/voxconverse", "annotationRevision": revision,
            "selection": selection, "inputs": inputs, "nativeBinarySha256": native["binarySha256"],
            "oraclePackages": oracle["packages"], "protocol": "Official independently verified RTTM activity; label-permutation-invariant DER with 0/250 ms collars and overlap scored. Counts/turn labels held out from inference; no count bounds, alternate runs or tuning. All tentative outcomes remain scored. Native wall time includes the whole workflow; oracle wall time is diarization only, so these are not throughput comparisons.",
            "summary": {"selectedCases": len(cases), "scoredCases": len(cases),
                        "nativeCorrectCounts": sum(case["native"]["countCorrect"] for case in cases),
                        "oracleCorrectCounts": sum(case["providerOracle"]["countCorrect"] for case in cases),
                        "humanCounts": sorted({case["referenceSpeakerCount"] for case in cases})}, "cases": cases}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--corpus-dir", required=True, type=Path)
    parser.add_argument("--annotations", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    result = evaluate(args.corpus_dir, args.annotations)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, allow_nan=False) + "\n")
    print(json.dumps(result["summary"]))


if __name__ == "__main__":
    main()
