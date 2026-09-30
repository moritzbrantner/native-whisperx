# Python WhisperX is an explicit oracle only

Status: accepted, 2026-10-01. Completes ADR 0015's Python retirement and supersedes
ADR 0003's allowance for normal product runtime delegation.

## Context

The native acceptance gate #195 and its final CUDA/parity dependency #207 are
closed. Every in-scope product capability is native or recorded as intentionally
unsupported in the capability matrix. Python no longer needs to be an execution
escape hatch for normal Workflow Composition.

## Decision

Normal CLI transcription and public finite workflow entrypoints reject the
retired external provider before accessing resources or starting work, including
when `whisperx-compat` is enabled. No native error recommends a Python product
fallback. Existing native-only live workflows retain that boundary.

Python WhisperX remains the reference oracle for the Parity Harness. The explicit
`run_whisperx_oracle` interface and dedicated comparison, preflight, golden, and
benchmark commands may execute it only with non-default `whisperx-compat`.

Oracle settings use `WhisperxOracleConfig`. The old type name is a deprecated
alias, and legacy serialized provider/settings values remain readable during
the pre-1.0 migration. Reading those values never authorizes product execution.
The CLI accepts the hidden retired provider spelling only to return migration
guidance. See `docs/python-oracle-migration.md`.

## Consequences

Product callers use native configuration and public product workflows. Maintainer
oracle callers select the explicit Parity Harness interface. Offline tests can
prove the execution boundary with fake external commands; resource-backed CUDA
and model evidence remain separately recorded and are not inferred from these
tests. Translation ownership debt #254 remains unchanged by this migration.
