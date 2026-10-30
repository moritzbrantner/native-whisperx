# Natural two-/three-speaker consumer acceptance

The actual Native WhisperX CLI now reports its observed count, anonymous timed
speaker turns, provider/checkpoint/bundle identity and the provider's uncertainty
limitations through `--report`. Automatic model counts are explicitly tentative;
no unsupported probability or verified identity is fabricated. Missing resources
and execution failures retain their existing fail-closed behavior.

The approved sustained-conversation slice produced correct unconstrained counts
on four independently annotated recordings, with both count strata represented.
This is bounded evidence: the retained AMI and initial holdout failures demonstrate
that these models do not infer every real-world count correctly. No provider,
threshold, fallback or hidden count constraint was changed to obtain these results.

## Frozen inputs and independent truth

Attribution: VoxConverse test corpus, CC BY 4.0; official independently verified
RTTM annotations at revision `24bf60be297701cd7e4ef18550c6d390c1b87365`.
[Corpus and annotation methodology](https://github.com/joonson/voxconverse).
Audio came from the official Oxford test WAV archive. The machine-readable files
retain exact selected IDs, full recording hashes, archive provenance and immutable
selection text; audio and transcript text are not committed.

An independent reference curator selected two recordings per count stratum by
SHA256 order, requiring each participant to have at least three annotated turns
and ten seconds of speech in a full 60–300 second recording. This prospectively
selects sustained alternating conversations rather than incidental background
voices. Previously exposed cases were excluded. The count mapping and RTTM turns
were withheld from inference and scoring until native and oracle predictions were
frozen. The curator necessarily consulted count/activity labels for stratification;
this is not a claim of entirely label-free cohort selection.

Each native run used the normal decode → VAD → ASR → alignment → diarization →
output workflow with Small English ASR, CPU float32, the approved community-1
bundle, no speaker library, no saved draft identities and **no min/max/exact
speaker bound**. Each task started once. All automatic outputs remain tentative
and stay in the evaluation denominator. The evaluator validates frozen hashes and
complete case coverage before opening reference turns.

## Observed primary slice

| Case | Human count | Native count | Oracle count | Native DER at 250 ms | Oracle DER at 250 ms |
| --- | --- | --- | --- | --- | --- |
| iabca | 3 | 3 | 3 | 13.20% | 13.35% |
| ltgmz | 2 | 2 | 2 | 7.30% | 8.47% |
| tkhgs | 2 | 2 | 2 | 0.82% | 0.84% |
| wcxfk | 3 | 3 | 3 | 14.88% | 13.61% |

[Full observations, boundaries, overlap, short-turn and reentry diagnostics](../../tests/parity/evidence/voxconverse-natural-conversations.json).
Anonymous label assignment is permutation invariant. Both zero- and 250-ms collar
DER are recorded with overlapping speech scored. The 250-ms collar is an explicit
measurement convention, not an invented product accuracy threshold. Oracle
agreement is diagnostic and does not replace independent human truth.

## All prior cases retained

The [initial AMI cohort](issue-362-real-conversation.md) remains **0/4** correct
counts. A [known-count diagnostic VoxConverse cohort](../../tests/parity/evidence/voxconverse-diagnostic-conversations.json)
is **4/4**; it is explicitly not blind primary evidence. The
[initial held-out VoxConverse cohort](../../tests/parity/evidence/voxconverse-initial-holdout.json)
is **3/4**. Its failing `vylyk` recording has three annotated speakers but both
native and the pinned oracle infer one; two voices contribute only about 1.2
seconds each. It is retained rather than relabeled or removed. Across all these
cohorts and the primary slice, **11/16** counts are correct. This aggregate is a
description of deliberately different cohorts, not a population accuracy estimate.

## Exact source, runtime and replay

Initial four-case inference used Native `0a4c012` and Audio Analysis
`c8d89467f86119a6064ba363738eba5dd9862cd7`, with the frozen binary SHA recorded in
the evidence. Pyannote community-1 source is
`3533c8cf8e369892e6b79ff1bf80f7b0286a54ee`, converted artifact set
`0a12189874dace9b590af9b09ef4637552006130716db57c35d72f984b36c577`.
ONNX Runtime was 1.30.0. The comparison oracle was pyannote.audio 4.0.4 with
Torch/Torchaudio 2.8, CPU, four threads, seed zero and the same source checkpoint.
No Python is used by the native product.

[Current-source replay](../../tests/parity/evidence/voxconverse-current-source-replay.json)
reran `iabca` and `ltgmz` on Native
`89ab7e09f6fb869ce05883e2c6da75e2463d30ae`, Audio Analysis
`ec42fcfc1fa3a0d0b9dccdfcf5506ababe287fc1` and foundation
`9a162dfd44d3cae7f88ad993de79defc0ac94cd6`, including the ZIP 9 backport graph.
Counts remained 3 and 2. These are source-development integrations, with no registry
publication or version bump.

Native primary whole-workflow wall times were 210–323 seconds; per-case peak RSS
and task counts are recorded. Oracle times cover diarization only, so they are not
comparable throughput figures. Inputs, all tentative outcomes and model limitations
remain visible. Existing AMI and `vylyk` cases provide minimal reproducible failures
for upstream #152; matching oracle failures do not identify a Rust algorithm defect.

Reproduce scoring after acquiring the licensed official audio and freezing native
and oracle runs:

```bash
python scripts/voxconverse_diarization_evidence.py \
  --corpus-dir /path/to/frozen-cohort \
  --annotations /path/to/voxconverse-at-exact-revision \
  --output /path/to/evidence.json
```

Guard/parser tests are ordinary offline tests. Model-backed corpus jobs remain
resource-tier work, outside ordinary PR CI. Shared convention sourceRevision:
`6cc05789dab39cee6758b288f83fb48b4ee6dbdb`.
