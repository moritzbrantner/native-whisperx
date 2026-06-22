#!/usr/bin/env python3
"""Model and reasoning selection helper for agent-loop work."""

from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from datetime import UTC, datetime
from pathlib import Path
from typing import Any


STRONG_MODEL = "gpt-5.5"
MINI_MODEL = "gpt-5.4-mini"
SPARK_MODEL = "gpt-5.3-codex-spark"


@dataclass(frozen=True)
class Policy:
    agent_profile: str
    model: str
    reasoning: str
    sandbox: str
    tier: str
    reason: str
    fallback: str | None = None


POLICIES: dict[str, Policy] = {
    "tiny-text": Policy(
        "tiny-text-summarizer",
        SPARK_MODEL,
        "low",
        "read-only",
        "spark",
        "tiny read-only text task can use Spark",
        "context-compressor",
    ),
    "queue-scan": Policy("queue-scanner", MINI_MODEL, "low", "read-only", "mini", "bounded read-only queue scan"),
    "context-compress": Policy(
        "context-compressor",
        MINI_MODEL,
        "low",
        "read-only",
        "mini",
        "bounded read-only context compression",
    ),
    "log-extract": Policy("log-extractor", MINI_MODEL, "medium", "read-only", "mini", "bounded read-only log extraction"),
    "prd-review": Policy("prd-reviewer", STRONG_MODEL, "medium", "read-only", "strong", "PRD clarity affects downstream slices"),
    "slice-implementation": Policy(
        "slice-implementer",
        STRONG_MODEL,
        "medium",
        "inherit",
        "strong",
        "mutating slice implementation requires strongest model",
    ),
    "failed-ci-repair": Policy("repair-worker", STRONG_MODEL, "high", "inherit", "strong", "repair requires precise code changes"),
    "merge-conflict-repair": Policy(
        "repair-worker",
        STRONG_MODEL,
        "high",
        "inherit",
        "strong",
        "merge conflict repair requires precise code changes",
    ),
    "merge-decision": Policy("master", STRONG_MODEL, "high", "current", "strong", "merge decisions stay with the master"),
}


def utc_now() -> str:
    return datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def default_run_state_path(run_id: str | None) -> Path | None:
    if not run_id:
        return None
    return Path(".agent-loop") / "model-policy" / f"{run_id}.json"


def read_json_object(raw: str | None) -> dict[str, Any]:
    if not raw:
        return {}
    value = json.loads(raw)
    if not isinstance(value, dict):
        raise ValueError("--metadata-json must be a JSON object")
    return value


def read_state(path: Path | None, run_id: str | None = None) -> dict[str, Any]:
    if path is None or not path.exists():
        return {"runId": run_id, "disabledModels": {}}
    state = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(state, dict):
        raise ValueError(f"model policy state is not an object: {path}")
    state.setdefault("runId", run_id)
    state.setdefault("disabledModels", {})
    return state


def write_state(path: Path, state: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(state, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def disabled_models(state: dict[str, Any]) -> set[str]:
    disabled = state.get("disabledModels")
    return set(disabled.keys()) if isinstance(disabled, dict) else set()


def strong_policy(reason: str, sandbox: str = "read-only") -> Policy:
    return Policy("master", STRONG_MODEL, "high", sandbox, "strong", reason)


def select_policy(task_class: str, metadata: dict[str, Any], disabled: set[str]) -> dict[str, Any]:
    if task_class not in POLICIES:
        raise ValueError(f"invalid task class: {task_class}")
    base = POLICIES[task_class]

    previous_failures = int(metadata.get("previousFailures") or 0)
    risk = str(metadata.get("risk") or "").lower()
    is_mutating = bool(metadata.get("isMutating", task_class in {"slice-implementation", "failed-ci-repair", "merge-conflict-repair"}))

    selected = base
    escalated_from: str | None = None
    fallback_used = False

    if task_class == "tiny-text" and SPARK_MODEL in disabled:
        selected = POLICIES["context-compress"]
        escalated_from = SPARK_MODEL
        fallback_used = True
        reason = "Spark is disabled for this run; using mini context compressor"
    elif task_class == "merge-decision":
        reason = base.reason
    elif is_mutating:
        selected = base if base.model == STRONG_MODEL else strong_policy("mutating work requires strongest model", "inherit")
        reason = selected.reason
    elif previous_failures > 0:
        selected = strong_policy("previous failure recorded; using strongest safe default")
        escalated_from = base.model if base.model != STRONG_MODEL else None
        fallback_used = escalated_from is not None
        reason = selected.reason
    elif risk == "high":
        selected = strong_policy("high-risk metadata; using strongest safe default")
        escalated_from = base.model if base.model != STRONG_MODEL else None
        fallback_used = escalated_from is not None
        reason = selected.reason
    elif task_class == "slice-implementation" and not metadata.get("expectedWriteScope"):
        selected = base
        reason = "missing expected write scope; already using strongest implementation model"
    else:
        reason = base.reason

    return {
        "taskClass": task_class,
        "agentProfile": selected.agent_profile,
        "model": selected.model,
        "modelReasoningEffort": selected.reasoning,
        "sandboxMode": selected.sandbox,
        "modelPolicyReason": reason,
        "modelPolicyTier": selected.tier,
        "modelPolicyEscalatedFrom": escalated_from,
        "modelPolicyFallbackUsed": fallback_used,
        "fallback": selected.fallback,
    }


def cmd_select(args: argparse.Namespace) -> None:
    metadata = read_json_object(args.metadata_json)
    state_path = Path(args.run_state) if args.run_state else default_run_state_path(args.run_id)
    state = read_state(state_path, args.run_id)
    result = select_policy(args.task_class, metadata, disabled_models(state))
    if args.repo:
        result["repo"] = args.repo
    if args.run_id:
        result["runId"] = args.run_id
    if args.issue is not None:
        result["issue"] = args.issue
    print(json.dumps(result, indent=2, sort_keys=True))


def cmd_disable_model(args: argparse.Namespace) -> None:
    state_path = Path(args.run_state)
    state = read_state(state_path)
    disabled = state.setdefault("disabledModels", {})
    disabled[args.model] = {"reason": args.reason, "timestamp": utc_now()}
    write_state(state_path, state)
    print(json.dumps({"ok": True, "path": str(state_path), "disabledModels": disabled}, indent=2, sort_keys=True))


def cmd_state(args: argparse.Namespace) -> None:
    state_path = Path(args.run_state)
    print(json.dumps(read_state(state_path), indent=2, sort_keys=True))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(required=True)

    select = sub.add_parser("select", help="Select an agent profile, model, and reasoning effort")
    select.add_argument("--task-class", required=True, choices=sorted(POLICIES))
    select.add_argument("--repo")
    select.add_argument("--run-id")
    select.add_argument("--issue", type=int)
    select.add_argument("--metadata-json", default="{}")
    select.add_argument("--run-state")
    select.set_defaults(func=cmd_select)

    disable = sub.add_parser("disable-model", help="Disable a model for one run state file")
    disable.add_argument("--run-state", required=True)
    disable.add_argument("--model", required=True)
    disable.add_argument("--reason", required=True)
    disable.set_defaults(func=cmd_disable_model)

    state = sub.add_parser("state", help="Print model policy run state")
    state.add_argument("--run-state", required=True)
    state.set_defaults(func=cmd_state)

    return parser


def main() -> None:
    args = build_parser().parse_args()
    try:
        args.func(args)
    except (OSError, ValueError, json.JSONDecodeError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error


if __name__ == "__main__":
    main()

