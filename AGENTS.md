# Agent Instructions

## Required Reading

- `CONTEXT.md`
- `docs/architecture.md`
- Relevant ADRs under `docs/adr/`
- `docs/source-development.md` for unreleased cross-repository dependency work
- `docs/agents/source-first-ticket-rule.md` when an older issue mentions publication or forbids source overrides
- The assigned GitHub issue or PRD
- Nearby tests for the behavior being changed

## Working Rules

- Preserve existing parity terminology from `CONTEXT.md`.
- Treat `native-whisperx` as permanently composition-only. If a capability would still make sense in a Rust transcription application without Native WhisperX product semantics, implement it in its canonical upstream owner and consume it here through source mode when needed.
- Do not use this repository as a proving ground for reusable provider/runtime implementations. ADR 0015 supersedes ADR 0006's temporary implement-here-first exception.
- Keep `NativeWhisperxConfig`, product reports, progress events, outcomes, and programmatically meaningful errors product-owned. Do not newly expose upstream execution/media/provider DTOs through public re-exports or embedded response/error types unless they are canonical cross-project domain contracts such as `TranscriptionContract`.
- Provider choices and Cargo features exposed by Native WhisperX must represent user-visible capabilities or meaningful tradeoffs, not incidental backend technology.
- Keep Automatic Workflow Selection narrow and capability-specific. Do not introduce a generic planner, scoring engine, capability graph, or provider-negotiation framework without an explicit architecture decision.
- Python WhisperX runtime delegation is transitional. Do not add new silent or automatic Python fallback; Python remains acceptable as parity oracle/reference/golden source.
- Track the latest WhisperX release as the Upstream Target while keeping deterministic gating on the newest Verified Compatibility Baseline until reconciliation passes.
- Prefer small, behavior-focused changes.
- Do not revert unrelated dirty files.
- For bugs, reproduce the failing behavior before fixing where practical.
- Run the narrowest meaningful check first, then broader checks before handoff when feasible.
- Use source development mode when a Native WhisperX task needs unreleased audio, NLP, or foundation changes.
- Source development is local-workspace owned: prepare the required sibling repository/worktree at the exact pinned revision before activating source mode. Do not add private-repository tokens or authenticated Git fallback to make hosted CI reproduce that workspace.
- Do not publish crates or start a release train merely to unblock feature work.
- Repository-level source-development policy supersedes older ticket wording that made publication a prerequisite for implementation; reconcile stale ticket constraints instead of following them.
- Keep a normal task to Native WhisperX plus at most two upstream repositories unless broader migration scope was explicitly assigned.
- Do not create a new crate without an independent-versioning reason.
- Registry-only dependency verification is a release gate, not a prerequisite for source-mode implementation evidence.

## Agent skills

This repository is configured for the Matt Pocock workflow skills and the long-running agent-loop workflow.

Use GitHub Issues as the durable source of truth for planning, triage, implementation slices, and agent-loop state.

Read these repo-local instructions before routing or implementing substantial work:

- Issue tracker: `docs/agents/issue-tracker.md`
- Triage labels: `docs/agents/triage-labels.md`
- Domain context: `docs/agents/domain.md`
- Planning workflow: `docs/agents/planning-workflow.md`
- TDD workflow: `docs/agents/tdd-workflow.md`

### Planning workflow

Substantial new work should be planned into GitHub PRD issues instead of implemented directly. See `docs/agents/planning-workflow.md`.

Substantial or TDD-oriented implementation work should follow `docs/agents/tdd-workflow.md`.
