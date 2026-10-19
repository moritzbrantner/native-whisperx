#!/usr/bin/env python3
"""Score the fixed AMI #362 cohort only after native and oracle outputs are frozen.

Resource tier: pyannote.metrics 4.1 and scipy, in the pinned oracle environment.
This evaluator never runs a model or supplies speaker-count constraints.
"""

import argparse
from collections import defaultdict
import hashlib
import importlib.metadata
import json
import math
from pathlib import Path
import re
import statistics
import xml.etree.ElementTree as ET
import zipfile


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load_frozen(root):
    cohort_path = root / "cohort-before-prediction.json"
    cohort = json.loads(cohort_path.read_text())
    # These checks precede opening any held-out annotations.
    native = json.loads((root / "observed-predictions-frozen.json").read_text())
    oracle = json.loads((root / "oracle-predictions-frozen.json").read_text())
    for frozen in [native, oracle]:
        if frozen["cohortSha256"] != sha256(cohort_path):
            raise ValueError("frozen predictions do not match the selected cohort")
        if {run["meeting"] for run in frozen["runs"]} != {item["meeting"] for item in cohort["items"]}:
            raise ValueError("frozen predictions do not cover every selected meeting")
    for run in native["runs"]:
        if run["exitCode"] != 0:
            raise ValueError("native inference failed; retain the failure instead of scoring it as success")
        for name, expected in run["artifacts"].items():
            path = root / name if name.endswith("-report.json") else root / "native-observed-out" / name
            if sha256(path) != expected:
                raise ValueError(f"native artifact changed after freezing: {name}")
        required = f'{run["meeting"]}-observed-report.json'
        if required not in run["artifacts"]:
            raise ValueError("frozen native report is missing")
    for run in oracle["runs"]:
        if sha256(root / f'{run["meeting"]}-oracle-turns.json') != run["outputSha256"]:
            raise ValueError("oracle artifact changed after freezing")
    if sha256(root / "ami_public_manual_1.6.2.zip") != cohort["referenceArchiveSha256"]:
        raise ValueError("reference annotation archive differs from the selected release")
    for item in cohort["items"]:
        if sha256(root / f'{item["meeting"]}-000-120.wav') != item["clipSha256"]:
            raise ValueError("evaluation audio changed after selection")
    return cohort, native, oracle


def reference_turns(archive, meeting, start, end, kind):
    """Read independently annotated words or transcriber utterances for one window."""
    suffix = "words.xml" if kind == "words" else "segments.xml"
    prefix = "words/" if kind == "words" else "segments/"
    spans = []
    missing_times = 0
    selected = [name for name in archive.namelist() if name.startswith(prefix + meeting + ".") and name.endswith(suffix)]
    if not selected:
        raise ValueError(f"no {kind} annotations for {meeting}")
    for name in sorted(selected):
        speaker = Path(name).name.split(".")[1]
        document = ET.fromstring(archive.read(name))
        for element in document:
            tag = element.tag.split("}")[-1]
            if tag != ("w" if kind == "words" else "segment"):
                continue
            if element.attrib.get("punc") == "true":
                continue
            keys = ("starttime", "endtime") if kind == "words" else ("transcriber_start", "transcriber_end")
            if any(key not in element.attrib for key in keys):
                missing_times += 1
                continue
            a, b = (float(element.attrib[key]) for key in keys)
            if not math.isfinite(a) or not math.isfinite(b) or b < a:
                raise ValueError("invalid human annotation time range")
            if b <= a or b <= start or a >= end:
                continue
            spans.append({"speaker": speaker, "startSeconds": max(a, start) - start, "endSeconds": min(b, end) - start})
    return sorted(spans, key=lambda turn: (turn["startSeconds"], turn["endSeconds"], turn["speaker"])), missing_times


def annotation(turns, uri):
    from pyannote.core import Annotation, Segment
    result = Annotation(uri=uri)
    for index, turn in enumerate(turns):
        a, b = turn["startSeconds"], turn["endSeconds"]
        if not math.isfinite(a) or not math.isfinite(b) or b <= a:
            raise ValueError("invalid predicted activity interval")
        result[Segment(a, b), index] = turn["speaker"]
    return result.support()


def boundary_evidence(reference, predicted, mapping, tolerance=0.25):
    """One-to-one utterance boundary agreement, separate from words-only DER."""
    import numpy as np
    from scipy.optimize import linear_sum_assignment
    expected, observed = defaultdict(list), defaultdict(list)
    for turn in reference:
        for edge in ["startSeconds", "endSeconds"]:
            expected[(turn["speaker"], edge)].append(turn[edge])
    for turn in predicted:
        for edge in ["startSeconds", "endSeconds"]:
            observed[(mapping.get(turn["speaker"], "unmapped:" + turn["speaker"]), edge)].append(turn[edge])
    errors = []
    matched = 0
    for key in expected.keys() | observed.keys():
        a, b = expected[key], observed[key]
        if not a or not b:
            continue
        distances = np.abs(np.array(a)[:, None] - np.array(b)[None, :])
        rows, columns = linear_sum_assignment(np.where(distances <= tolerance, distances, 1e6))
        for row, column in zip(rows, columns):
            if distances[row, column] <= tolerance:
                matched += 1
                errors.append(float(distances[row, column]))
    n_reference = sum(map(len, expected.values()))
    n_predicted = sum(map(len, observed.values()))
    return {"toleranceSeconds": tolerance, "referenceBoundaries": n_reference, "predictedBoundaries": n_predicted,
            "matched": matched, "missed": n_reference - matched, "extra": n_predicted - matched,
            "precision": matched / n_predicted if n_predicted else None,
            "recall": matched / n_reference if n_reference else None,
            "medianMatchedAbsoluteErrorSeconds": statistics.median(errors) if errors else None,
            "maximumMatchedAbsoluteErrorSeconds": max(errors) if errors else None}


def score(reference, utterances, predicted, uri, duration_seconds):
    from pyannote.core import Segment, Timeline
    from pyannote.metrics.diarization import DiarizationErrorRate
    ref, hyp = annotation(reference, uri), annotation(predicted, uri)
    mapper = DiarizationErrorRate(collar=0.0, skip_overlap=False)
    uem = Timeline([Segment(0.0, duration_seconds)])
    mapping = mapper.optimal_mapping(ref, hyp, uem=uem)
    mapped = hyp.rename_labels(mapping=mapping)
    mapped_labels = set(mapped.labels())
    short, missed_short, reentries, missed_reentries = 0, 0, 0, 0
    previous_end = {}
    for turn in utterances:
        span = Segment(turn["startSeconds"], turn["endSeconds"])
        coverage = mapped.label_timeline(turn["speaker"]).crop(span).duration() if turn["speaker"] in mapped_labels else 0.0
        if span.duration <= 1.0:
            short += 1
            missed_short += coverage == 0.0
        if turn["speaker"] in previous_end and span.start - previous_end[turn["speaker"]] >= 5.0:
            reentries += 1
            missed_reentries += coverage == 0.0
        previous_end[turn["speaker"]] = max(previous_end.get(turn["speaker"], 0.0), span.end)
    return {"labelMapping": mapping,
            "wordsOnlyDER": {str(collar): DiarizationErrorRate(collar=collar, skip_overlap=False)(ref, hyp, uem=uem, detailed=True) for collar in [0.0, 0.25]},
            "referenceOverlapSeconds": ref.get_overlap().duration(), "predictedOverlapSeconds": hyp.get_overlap().duration(),
            "utteranceBoundaries": boundary_evidence(utterances, predicted, mapping),
            "shortUtteranceDefinitionSeconds": 1.0, "shortReferenceUtterances": short, "fullyMissedShortUtterances": missed_short,
            "longGapDefinitionSeconds": 5.0, "referenceLongGapReentries": reentries, "fullyMissedLongGapReentries": missed_reentries}


def evaluate(root):
    cohort, native, oracle = load_frozen(root)
    native_runs = {run["meeting"]: run for run in native["runs"]}
    oracle_runs = {run["meeting"]: run for run in oracle["runs"]}
    cases = []
    with zipfile.ZipFile(root / "ami_public_manual_1.6.2.zip") as archive:
        for item in cohort["items"]:
            meeting = item["meeting"]
            reference, unknown_words = reference_turns(archive, meeting, item["startSeconds"], item["endSeconds"], "words")
            utterances, unknown_utterances = reference_turns(archive, meeting, item["startSeconds"], item["endSeconds"], "utterances")
            truth = len({turn["speaker"] for turn in reference})
            report = json.loads((root / f"{meeting}-observed-report.json").read_text())
            observed = report["diarization"]
            baseline = json.loads((root / f"{meeting}-oracle-turns.json").read_text())
            if observed["countStatus"] != "tentativeAutomatic" or "countConstraints" in observed:
                raise ValueError("primary cohort must use unconstrained automatic speaker counts")
            if observed["speakerCount"] != len({turn["speaker"] for turn in observed["turns"]}):
                raise ValueError("reported count differs from the actual observed clusters")
            if (observed["modelId"], observed["sourceRevision"]) != (baseline["modelId"], baseline["sourceRevision"]):
                raise ValueError("native and oracle model identities differ")
            log = (root / f"{meeting}-observed-native.log").read_text()
            rss = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", log)
            cases.append({"meeting": meeting, "windowSeconds": [item["startSeconds"], item["endSeconds"]],
                "primaryTwoOrThreeSpeakerScenario": truth in [2, 3], "referenceSpeakerCount": truth,
                "untimedWordElementsInMeetingFiles": unknown_words, "untimedUtteranceElementsInMeetingFiles": unknown_utterances,
                "native": {"observation": observed, "countCorrect": observed["speakerCount"] == truth,
                    "metrics": score(reference, utterances, observed["turns"], meeting, item["endSeconds"] - item["startSeconds"]),
                    "wallSeconds": native_runs[meeting]["wallSeconds"], "maxRssKiB": int(rss[1]) if rss else None},
                "providerOracle": {"speakerCount": baseline["speakerCount"], "countCorrect": baseline["speakerCount"] == truth,
                    "metrics": score(reference, utterances, baseline["turns"], meeting, item["endSeconds"] - item["startSeconds"]), "wallSeconds": oracle_runs[meeting]["wallSeconds"]}})
    counts = sorted({case["referenceSpeakerCount"] for case in cases})
    return {"schemaVersion": 1, "license": cohort["license"], "attribution": "AMI Meeting Corpus, manual annotations v1.6.2",
        "source": cohort["licenseSource"], "referenceArchiveSha256": cohort["referenceArchiveSha256"],
        "cohortSha256": native["cohortSha256"], "nativeSource": native["nativeSource"], "nativeBinarySha256": native["nativeBinarySha256"],
        "audioSource": native["audioSource"], "foundationSource": native["foundationSource"],
        "oraclePackages": oracle["packages"], "metricPackage": importlib.metadata.version("pyannote.metrics"),
        "referenceProtocol": "AMI manually timed words for DER and counts; transcriber utterances for secondary boundary/reentry evidence. Both DER collars score overlap. No model output is human ground truth.",
        "summary": {"selectedCases": len(cases), "scoredCases": len(cases), "observedHumanSpeakerCounts": counts,
            "coversBothTwoAndThreeSpeakers": 2 in counts and 3 in counts,
            "nativeCorrectCounts": sum(case["native"]["countCorrect"] for case in cases),
            "oracleCorrectCounts": sum(case["providerOracle"]["countCorrect"] for case in cases),
            "tentativeCasesRetained": sum(case["native"]["observation"]["countStatus"] == "tentativeAutomatic" for case in cases)},
        "cases": cases}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--corpus-dir", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = evaluate(args.corpus_dir)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, allow_nan=False) + "\n")
    print(json.dumps(result["summary"]))


if __name__ == "__main__":
    main()
