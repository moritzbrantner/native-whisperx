# ADR 0017: Separate observed speaker counts from count certainty

## Status

Accepted as the reporting implementation of #362's approved tentative-count outcome.

## Decision

A completed native diarization result exposes a product-owned `diarization`
observation in `NativeWhisperxReport`, including the model's distinct-label count,
raw timestamped turns, provider/model identity, source revision, and bundle checksum
when reported. Raw turns retain overlap and returning labels. This report does
not store voice embeddings or claim verified speaker identities.

The current provider integration does not report calibrated speaker-count
uncertainty. Report `countUncertainty: "notReported"`; do not interpret turn scores
as count confidence or invent a percentage. An explicitly unconstrained result is
`countStatus: "tentativeAutomatic"`: it is the best available model estimate, not
a verified count or a claim that the provider measured a particular probability
of ambiguity. User-selected bounds produce `constrained` with their limits.
Missing selection evidence remains `unspecified` rather than being guessed.
A completed zero-turn result retains its observed count of zero, so evaluators
can score it instead of excluding it.

Unavailable resources and provider execution errors remain workflow failures;
they are not converted to tentative successful observations. The existing
`--report <PATH>` output exposes these facts through the real CLI. Default
WhisperX JSON and the Transcription Progress Stream retain their existing
contracts. Older report JSON without the optional observation remains readable.

Human annotations and a pinned provider oracle remain distinct evaluation inputs.
Neither supplies hidden speaker bounds to the primary native workflow. Tentative,
incorrect, and zero-count model outputs stay in the evaluation denominator.
