#!/usr/bin/env python3
"""Structured diagnostics for the agent-loop skill."""

from __future__ import annotations

import argparse
import json
import math
import os
import re
import subprocess
import sys
import uuid
from collections import Counter
from datetime import UTC, datetime
from pathlib import Path
from typing import Any


SCHEMA_VERSION = 1
MAX_TEXT_LENGTH = 2000
VALID_ACTORS = {
    "master",
    "worker",
    "repair-worker",
    "prd-review-worker",
    "merge-conflict-worker",
}
VALID_EVENT_TYPES = {
    "run_started",
    "run_resumed",
    "poll_cycle_started",
    "queue_snapshot",
    "route_decision",
    "prd_review_started",
    "prd_review_completed",
    "slice_worker_spawned",
    "worker_report_received",
    "worker_blocked",
    "worker_failed",
    "ready_to_merge_detected",
    "checks_observed",
    "merge_attempted",
    "merge_completed",
    "feature_completed",
    "context_checkpoint_written",
    "diagnostic_summary_written",
    "diagnostics_failed",
    "run_stopped",
    "token_usage_estimated",
    "model_policy_selected",
    "model_policy_escalated",
    "model_policy_failed",
}
VALID_SNAPSHOT_MODES = {"compact", "full"}
VALID_ARTIFACT_KINDS = {
    "queue-snapshot",
    "worker-context-pack",
    "worker-prompt",
    "worker-report",
    "checkpoint",
    "status-comment",
    "summary",
    "issue-body",
    "prd-body",
    "slice-body",
    "other",
}
LIFECYCLE_REQUIRED_FIELDS = {
    "slice_worker_spawned": (("issue", "sliceIssue"), ("parentPrd", "parent"), ("branch",)),
    "worker_report_received": (("issue", "sliceIssue"), ("pr", "prs"), ("status",)),
    "ready_to_merge_detected": (("issue", "sliceIssue"), ("pr", "prs")),
    "checks_observed": (("issue", "sliceIssue"), ("pr", "prs")),
    "merge_attempted": (("issue", "sliceIssue"), ("pr", "prs")),
    "merge_completed": (("issue", "sliceIssue"), ("pr", "prs")),
    "worker_blocked": (("issue", "sliceIssue"), ("error", "blockedOn", "status")),
    "worker_failed": (("issue", "sliceIssue"), ("error", "failureCategory")),
    "feature_completed": (("issue", "parentPrd", "parent"),),
}
SENSITIVE_KEY_PATTERN = re.compile(
    r"(token|password|secret|api[_-]?key|authorization|cookie|credential|private[_-]?key|env)",
    re.IGNORECASE,
)
NONSECRET_TOKEN_KEYS = {
    "tokenEstimate",
    "tokenBudget",
    "estimatedTokens",
    "baselineEstimatedTokens",
    "estimatedSavingsTokens",
    "estimatedSavingsPercent",
    "estimatedTokensObserved",
    "estimatedTokensSaved",
    "tokenEstimatesLogged",
}
SENSITIVE_VALUE_PATTERN = re.compile(
    r"(gh[pousr]_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,}|sk-[A-Za-z0-9_-]{20,}|Bearer\s+[A-Za-z0-9._-]{20,})"
)


def utc_now() -> str:
    return datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def repo_slug(repo: str) -> str:
    return re.sub(r"[^A-Za-z0-9]+", "-", repo).strip("-").lower()


def diagnostics_root(base_dir: Path) -> Path:
    return base_dir / ".agent-loop" / "diagnostics"


def run_path(base_dir: Path, run_id: str) -> Path:
    return diagnostics_root(base_dir) / "runs" / f"{run_id}.jsonl"


def latest_path(base_dir: Path) -> Path:
    return diagnostics_root(base_dir) / "latest.json"


def report_path(base_dir: Path, run_id: str, suffix: str = "summary") -> Path:
    return diagnostics_root(base_dir) / "reports" / f"{run_id}-{suffix}.md"


def ensure_dirs(base_dir: Path) -> None:
    root = diagnostics_root(base_dir)
    (root / "runs").mkdir(parents=True, exist_ok=True)
    (root / "reports").mkdir(parents=True, exist_ok=True)


def count_events(path: Path) -> int:
    if not path.exists():
        return 0
    return sum(1 for line in path.read_text(encoding="utf-8").splitlines() if line.strip())


def estimate_tokens(text: str, method: str = "auto") -> dict[str, Any]:
    if method not in {"auto", "char-div-4", "tiktoken"}:
        raise ValueError(f"invalid token estimate method: {method}")
    if method in {"auto", "tiktoken"}:
        try:
            import tiktoken  # type: ignore

            encoder = tiktoken.get_encoding("cl100k_base")
            return {
                "method": "tiktoken",
                "chars": len(text),
                "estimatedTokens": len(encoder.encode(text)),
                "confidence": "medium",
            }
        except Exception:
            if method == "tiktoken":
                raise ValueError("tiktoken is not available")
    return {
        "method": "char-div-4",
        "chars": len(text),
        "estimatedTokens": math.ceil(len(text) / 4),
        "confidence": "medium",
    }


def estimate_token_savings(current_text: str, baseline_text: str | None = None, method: str = "auto") -> dict[str, Any]:
    current = estimate_tokens(current_text, method)
    payload: dict[str, Any] = {
        "method": current["method"],
        "chars": current["chars"],
        "estimatedTokens": current["estimatedTokens"],
        "confidence": "medium" if baseline_text is not None else "low",
    }
    if baseline_text is None:
        return payload
    baseline = estimate_tokens(baseline_text, current["method"])
    savings = int(baseline["estimatedTokens"]) - int(current["estimatedTokens"])
    baseline_tokens = int(baseline["estimatedTokens"])
    payload.update(
        {
            "baselineChars": baseline["chars"],
            "baselineEstimatedTokens": baseline_tokens,
            "estimatedSavingsTokens": savings,
            "estimatedSavingsPercent": round((savings / baseline_tokens) * 100, 2) if baseline_tokens else 0,
            "confidence": "high",
        }
    )
    return payload


def token_estimate_from_event(event: dict[str, Any]) -> dict[str, Any] | None:
    estimate = event.get("tokenEstimate")
    if isinstance(estimate, dict) and isinstance(estimate.get("estimatedTokens"), int):
        return estimate
    return None


def token_totals_from_events(events: list[dict[str, Any]]) -> dict[str, Any]:
    totals = {
        "tokenEstimatesLogged": 0,
        "estimatedTokensObserved": 0,
        "baselineEstimatedTokens": 0,
        "estimatedTokensSaved": 0,
        "estimatedSavingsPercent": 0,
    }
    by_kind: dict[str, dict[str, Any]] = {}
    for event in events:
        estimate = token_estimate_from_event(event)
        if estimate is None:
            continue
        kind = str(event.get("artifactKind") or "other")
        bucket = by_kind.setdefault(
            kind,
            {
                "count": 0,
                "estimatedTokensObserved": 0,
                "baselineEstimatedTokens": 0,
                "estimatedTokensSaved": 0,
                "confidence": str(estimate.get("confidence") or "medium"),
            },
        )
        current = int(estimate.get("estimatedTokens") or 0)
        baseline = int(estimate.get("baselineEstimatedTokens") or 0)
        saved = int(estimate.get("estimatedSavingsTokens") or max(baseline - current, 0))
        totals["tokenEstimatesLogged"] += 1
        totals["estimatedTokensObserved"] += current
        totals["baselineEstimatedTokens"] += baseline
        totals["estimatedTokensSaved"] += saved
        bucket["count"] += 1
        bucket["estimatedTokensObserved"] += current
        bucket["baselineEstimatedTokens"] += baseline
        bucket["estimatedTokensSaved"] += saved
    baseline_total = int(totals["baselineEstimatedTokens"])
    if baseline_total:
        totals["estimatedSavingsPercent"] = round((int(totals["estimatedTokensSaved"]) / baseline_total) * 100, 2)
    totals["byArtifactKind"] = by_kind
    return totals


def model_policy_totals_from_events(events: list[dict[str, Any]]) -> dict[str, Any]:
    policy_events = [
        event
        for event in events
        if event.get("eventType") in {"model_policy_selected", "model_policy_escalated", "model_policy_failed"}
        or event.get("taskClass")
        or event.get("agentProfile")
        or event.get("model")
    ]
    by_task = Counter(str(event.get("taskClass") or "unspecified") for event in policy_events)
    by_model = Counter(str(event.get("model") or "unspecified") for event in policy_events if event.get("model"))
    by_profile = Counter(str(event.get("agentProfile") or "unspecified") for event in policy_events if event.get("agentProfile"))
    escalations = Counter(
        f"{event.get('modelPolicyEscalatedFrom')} -> {event.get('model')}"
        for event in policy_events
        if event.get("modelPolicyEscalatedFrom") and event.get("model")
    )
    failures = Counter(
        str((event.get("error") or {}).get("kind") or event.get("failureCategory") or event.get("outcome") or "unspecified")
        for event in policy_events
        if event.get("eventType") == "model_policy_failed"
    )
    low_confidence = Counter(
        str(event.get("agentProfile") or "unspecified")
        for event in policy_events
        if str(event.get("confidence") or "").lower() == "low" or event.get("outcome") in {"malformed-output", "low-confidence"}
    )
    return {
        "events": len(policy_events),
        "byTaskClass": dict(by_task),
        "byModel": dict(by_model),
        "byAgentProfile": dict(by_profile),
        "escalations": dict(escalations),
        "failures": dict(failures),
        "lowConfidenceOrMalformed": dict(low_confidence),
    }


def token_totals_by_field(events: list[dict[str, Any]], field: str) -> dict[str, dict[str, Any]]:
    totals: dict[str, dict[str, Any]] = {}
    for event in events:
        estimate = token_estimate_from_event(event)
        if estimate is None:
            continue
        key = str(event.get(field) or "unspecified")
        bucket = totals.setdefault(
            key,
            {
                "count": 0,
                "estimatedTokensObserved": 0,
                "baselineEstimatedTokens": 0,
                "estimatedTokensSaved": 0,
                "retryTokenEstimate": 0,
            },
        )
        current = int(estimate.get("estimatedTokens") or 0)
        baseline = int(estimate.get("baselineEstimatedTokens") or 0)
        saved = int(estimate.get("estimatedSavingsTokens") or max(baseline - current, 0))
        bucket["count"] += 1
        bucket["estimatedTokensObserved"] += current
        bucket["baselineEstimatedTokens"] += baseline
        bucket["estimatedTokensSaved"] += saved
        if event.get("modelPolicyEscalatedFrom"):
            bucket["retryTokenEstimate"] += current
    return totals


def contains_secret(value: str) -> bool:
    return bool(SENSITIVE_VALUE_PATTERN.search(value))


def sanitize(value: Any, key: str | None = None) -> Any:
    if key and key not in NONSECRET_TOKEN_KEYS and SENSITIVE_KEY_PATTERN.search(key):
        return {"redacted": True, "redactionReason": "possible secret or credential"}
    if isinstance(value, dict):
        return {str(k): sanitize(v, str(k)) for k, v in value.items()}
    if isinstance(value, list):
        return [sanitize(item) for item in value]
    if isinstance(value, str):
        if contains_secret(value):
            return {"redacted": True, "redactionReason": "possible secret or credential"}
        if len(value) > MAX_TEXT_LENGTH:
            return value[:MAX_TEXT_LENGTH] + "...[truncated]"
        return value
    return value


def normalize_decision(value: Any) -> Any:
    if isinstance(value, dict):
        normalized = dict(value)
        if "kind" not in normalized:
            for candidate in ("action", "decision", "status", "outcome"):
                if candidate in normalized and normalized[candidate]:
                    normalized["kind"] = str(normalized[candidate])
                    break
        normalized.setdefault("kind", "unspecified")
        normalized.setdefault("reason", "")
        return normalized
    if isinstance(value, str):
        return {"kind": value, "reason": ""}
    return value


def normalize_event_payload(payload: dict[str, Any]) -> dict[str, Any]:
    normalized = dict(payload)
    if "decision" in normalized:
        normalized["decision"] = normalize_decision(normalized["decision"])
    return normalized


def update_latest_metadata(base_dir: Path, repo: str, run_id: str, event: dict[str, Any]) -> None:
    latest = latest_path(base_dir)
    existing: dict[str, Any] = {}
    if latest.exists():
        try:
            existing = read_json(latest)
        except (OSError, json.JSONDecodeError):
            existing = {}

    run_file = run_path(base_dir, run_id)
    try:
        token_totals = token_totals_from_events(read_events(run_file))
    except (OSError, ValueError):
        token_totals = token_totals_from_events([event])
    report = event.get("reportPath") if event.get("eventType") == "diagnostic_summary_written" else existing.get("latestReportPath")
    failures = int(existing.get("diagnosticsFailures") or 0)
    if event.get("eventType") == "diagnostics_failed":
        failures += 1

    metadata = {
        "ok": True,
        "runId": run_id,
        "repo": repo,
        "path": str(run_file),
        "startedAt": existing.get("startedAt") or event.get("timestamp"),
        "updatedAt": event.get("timestamp"),
        "resumed": bool(existing.get("resumed", False)),
        "eventsLogged": count_events(run_file),
        "diagnosticsFailures": failures,
        "lastCycle": event.get("cycle", existing.get("lastCycle")),
        "lastEventId": event.get("eventId"),
        "lastEventType": event.get("eventType"),
        "lastStatus": event.get("status") or event.get("outcome") or existing.get("lastStatus"),
        "latestReportPath": report,
        "tokenEstimatesLogged": token_totals["tokenEstimatesLogged"],
        "estimatedTokensObserved": token_totals["estimatedTokensObserved"],
        "baselineEstimatedTokens": token_totals["baselineEstimatedTokens"],
        "estimatedTokensSaved": token_totals["estimatedTokensSaved"],
        "estimatedSavingsPercent": token_totals["estimatedSavingsPercent"],
    }
    latest.write_text(json.dumps(metadata, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def append_event(base_dir: Path, repo: str, run_id: str, actor: str, event_type: str, payload: dict[str, Any]) -> dict[str, Any]:
    if actor not in VALID_ACTORS:
        raise ValueError(f"invalid actor: {actor}")
    if event_type not in VALID_EVENT_TYPES:
        raise ValueError(f"invalid event type: {event_type}")
    ensure_dirs(base_dir)
    event = {
        "schemaVersion": SCHEMA_VERSION,
        "runId": run_id,
        "eventId": str(uuid.uuid4()),
        "timestamp": utc_now(),
        "repo": repo,
        "actor": actor,
        "eventType": event_type,
    }
    event.update(normalize_event_payload(sanitize(payload)))
    path = run_path(base_dir, run_id)
    with path.open("a", encoding="utf-8") as file:
        file.write(json.dumps(event, sort_keys=True, separators=(",", ":")) + "\n")
    update_latest_metadata(base_dir, repo, run_id, event)
    return event


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_events(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        raise FileNotFoundError(f"run file not found: {path}")
    events = []
    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        if not line.strip():
            continue
        try:
            events.append(json.loads(line))
        except json.JSONDecodeError as error:
            raise ValueError(f"invalid JSONL at {path}:{line_number}: {error}") from error
    return events


def run_command(args: list[str], cwd: Path) -> tuple[int, str, str]:
    proc = subprocess.run(args, cwd=cwd, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    return proc.returncode, proc.stdout, proc.stderr


def run_queue_command(base_dir: Path, subcommand: str, repo: str) -> list[dict[str, Any]]:
    queue_script = Path(__file__).with_name("agent_loop_queue.py")
    code, stdout, stderr = run_command(
        [sys.executable, str(queue_script), subcommand, "--repo", repo],
        cwd=base_dir,
    )
    if code != 0:
        raise RuntimeError(f"{subcommand} failed: {stderr.strip()}")
    return json.loads(stdout)


def compact_queue_item(item: dict[str, Any]) -> dict[str, Any]:
    fields = [
        "number",
        "title",
        "labels",
        "url",
        "parent",
        "concurrencyGroup",
        "expectedWriteScope",
        "verification",
        "contextDocs",
        "updatedAt",
        "reason",
    ]
    return {field: item[field] for field in fields if item.get(field) not in (None, "", [], {})}


def compact_queues(queues: dict[str, list[dict[str, Any]]]) -> dict[str, list[dict[str, Any]]]:
    return {name: [compact_queue_item(item) for item in items] for name, items in queues.items()}


def stable_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def load_payload(raw_json: str) -> dict[str, Any]:
    try:
        payload = json.loads(raw_json)
    except json.JSONDecodeError as error:
        raise ValueError(f"malformed --json payload: {error}") from error
    if not isinstance(payload, dict):
        raise ValueError("--json payload must be a JSON object")
    return payload


def cmd_start_run(args: argparse.Namespace) -> None:
    base_dir = Path.cwd()
    ensure_dirs(base_dir)
    latest = latest_path(base_dir)
    if args.resume and latest.exists():
        existing = read_json(latest)
        if existing.get("repo") == args.repo and run_path(base_dir, existing["runId"]).exists():
            append_event(base_dir, args.repo, existing["runId"], "master", "run_resumed", {})
            resumed = read_json(latest_path(base_dir))
            resumed["resumed"] = True
            latest_path(base_dir).write_text(json.dumps(resumed, indent=2, sort_keys=True) + "\n", encoding="utf-8")
            print(json.dumps({"ok": True, "runId": existing["runId"], "path": existing["path"], "resumed": True}, indent=2))
            return
    run_id = f"{datetime.now(UTC).strftime('%Y%m%d-%H%M%S')}-{repo_slug(args.repo)}"
    path = run_path(base_dir, run_id)
    metadata = {
        "ok": True,
        "runId": run_id,
        "repo": args.repo,
        "path": str(path),
        "startedAt": utc_now(),
        "resumed": False,
    }
    latest.write_text(json.dumps(metadata, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    append_event(base_dir, args.repo, run_id, "master", "run_started", {"diagnosticsPath": str(path)})
    print(json.dumps({"ok": True, "runId": run_id, "path": str(path), "resumed": False}, indent=2))


def cmd_log(args: argparse.Namespace) -> None:
    payload = load_payload(args.json)
    event = append_event(Path.cwd(), args.repo, args.run_id, args.actor, args.event_type, payload)
    print(json.dumps({"ok": True, "eventId": event["eventId"]}, indent=2))


def cmd_snapshot(args: argparse.Namespace) -> None:
    base_dir = Path.cwd()
    queues = {
        "raw": run_queue_command(base_dir, "list-raw", args.repo),
        "prds": run_queue_command(base_dir, "list-prds", args.repo),
        "slices": run_queue_command(base_dir, "list-slices", args.repo),
        "invalidPrds": run_queue_command(base_dir, "list-invalid-prds", args.repo),
        "invalidSlices": run_queue_command(base_dir, "list-invalid-slices", args.repo),
        "active": run_queue_command(base_dir, "list-active", args.repo),
        "readyToMerge": run_queue_command(base_dir, "list-ready-to-merge", args.repo),
    }
    selected_queues = compact_queues(queues) if args.mode == "compact" else queues
    full_payload = {
        "cycle": args.cycle,
        "snapshotMode": "full",
        "queues": queues,
        "counts": {
            "rawIssues": len(queues["raw"]),
            "readyPrds": len(queues["prds"]),
            "readySlices": len(queues["slices"]),
            "invalidPrds": len(queues["invalidPrds"]),
            "invalidSlices": len(queues["invalidSlices"]),
            "activeWorkers": len(queues["active"]),
            "readyToMerge": len(queues["readyToMerge"]),
        },
    }
    payload = {
        "cycle": args.cycle,
        "snapshotMode": args.mode,
        "artifactKind": "queue-snapshot",
        "artifactId": f"cycle-{args.cycle}",
        "queues": selected_queues,
        "counts": full_payload["counts"],
    }
    payload["tokenEstimate"] = estimate_token_savings(
        stable_json(payload),
        stable_json(full_payload) if args.mode == "compact" else None,
        args.token_method,
    )
    event = append_event(base_dir, args.repo, args.run_id, "master", "queue_snapshot", payload)
    print(json.dumps({"ok": True, "eventId": event["eventId"], "counts": payload["counts"]}, indent=2))


def command_failures(events: list[dict[str, Any]]) -> list[str]:
    failures = []
    for event in events:
        for command in event.get("commands", []) or event.get("commandsRun", []):
            result = command.get("result")
            exit_code = command.get("exitCode")
            if is_no_required_checks_command(command, event):
                continue
            if result == "failed" or (isinstance(exit_code, int) and exit_code != 0):
                failures.append(f"{command.get('command', '<unknown>')} -> {result or exit_code}")
    return failures


def is_no_required_checks_command(command: dict[str, Any], event: dict[str, Any]) -> bool:
    command_text = str(command.get("command") or "")
    summary = str(command.get("summary") or command.get("result") or "").lower()
    outcome = str(event.get("outcome") or "").lower()
    return (
        "gh pr checks" in command_text
        and "--required" in command_text
        and (
            "no checks reported" in summary
            or "no required checks" in summary
            or outcome in {"no-required-checks", "no-required-checks-reported", "no-github-checks-reported"}
        )
    )


def no_required_checks_count(events: list[dict[str, Any]]) -> int:
    count = 0
    for event in events:
        for command in event.get("commands", []) or event.get("commandsRun", []):
            if is_no_required_checks_command(command, event):
                count += 1
    return count


def selected_summary_filter(args: argparse.Namespace) -> tuple[str, Any] | None:
    selected = [
        ("since-event", args.since_event),
        ("cycle", args.cycle),
        ("last", args.last),
    ]
    active = [(name, value) for name, value in selected if value is not None]
    if len(active) > 1:
        raise ValueError("summarize accepts only one of --since-event, --cycle, or --last")
    return active[0] if active else None


def filter_events(events: list[dict[str, Any]], args: argparse.Namespace) -> tuple[list[dict[str, Any]], str, str]:
    selected = selected_summary_filter(args)
    if selected is None:
        return events, "all events", "summary"
    kind, value = selected
    if kind == "since-event":
        for index, event in enumerate(events):
            if event.get("eventId") == value:
                return events[index + 1 :], f"events after {value}", f"summary-since-{str(value)[:8]}"
        raise ValueError(f"--since-event did not match any eventId: {value}")
    if kind == "cycle":
        return [event for event in events if event.get("cycle") == value], f"cycle {value}", f"summary-cycle-{value}"
    if kind == "last":
        if int(value) <= 0:
            raise ValueError("--last must be greater than zero")
        return events[-int(value) :], f"last {value} events", f"summary-last-{value}"
    return events, "all events", "summary"


def has_any_field(event: dict[str, Any], names: tuple[str, ...]) -> bool:
    return any(event.get(name) not in (None, "", []) for name in names)


def lifecycle_missing_counts(events: list[dict[str, Any]]) -> Counter[str]:
    missing: Counter[str] = Counter()
    for event in events:
        event_type = event.get("eventType")
        for names in LIFECYCLE_REQUIRED_FIELDS.get(event_type, ()):
            if not has_any_field(event, names):
                missing[f"{event_type}:{'/'.join(names)}"] += 1
    return missing


def parse_timestamp(value: Any) -> datetime | None:
    if not isinstance(value, str) or not value:
        return None
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return None


def worker_duration_ms(event: dict[str, Any]) -> int | None:
    duration = event.get("durationMs")
    if isinstance(duration, int):
        return duration
    started = parse_timestamp(event.get("startedAt"))
    finished = parse_timestamp(event.get("finishedAt"))
    if started is None or finished is None or finished < started:
        return None
    return round((finished - started).total_seconds() * 1000)


def improvement_candidates(events: list[dict[str, Any]], blockers: Counter[str], failures: Counter[str]) -> list[str]:
    candidates: list[str] = []
    if blockers:
        kind, count = blockers.most_common(1)[0]
        candidates.append(f"Review blocker handling for `{kind}` ({count} occurrence(s)).")
    if failures:
        kind, count = failures.most_common(1)[0]
        candidates.append(f"Improve worker or repair prompts around `{kind}` failures ({count} occurrence(s)).")
    missing = sum(1 for event in events if event.get("outcome") == "missing-diagnostics")
    if missing:
        candidates.append(f"Tighten worker report instructions; {missing} worker report(s) missed diagnostics JSON.")
    checkpoints = [event for event in events if event.get("eventType") == "context_checkpoint_written"]
    if checkpoints:
        candidates.append("Review master context usage and checkpoint cadence.")
    if not candidates:
        candidates.append("No obvious prompt or skill changes from this run yet.")
    return candidates


def cmd_summarize(args: argparse.Namespace) -> None:
    base_dir = Path.cwd()
    all_events = read_events(run_path(base_dir, args.run_id))
    events, summary_scope, suffix = filter_events(all_events, args)
    event_counts = Counter(event.get("eventType", "unknown") for event in events)
    statuses = Counter(str(event.get("status")) for event in events if event.get("status"))
    outcomes = Counter(str(event.get("outcome")) for event in events if event.get("outcome"))
    blockers = Counter(
        str((event.get("error") or {}).get("kind") or event.get("failureCategory") or event.get("status"))
        for event in events
        if event.get("eventType") in {"worker_blocked", "worker_failed"} or event.get("status") in {"blocked", "failed"}
    )
    blockers.pop("None", None)
    failures = Counter(
        str((event.get("error") or {}).get("kind") or event.get("failureCategory"))
        for event in events
        if event.get("eventType") in {"worker_failed", "checks_observed"} and ((event.get("error") or {}).get("kind") or event.get("failureCategory"))
    )
    durations = [
        duration
        for event in events
        if event.get("eventType") == "worker_report_received"
        for duration in [worker_duration_ms(event)]
        if duration is not None
    ]
    average_worker_duration = round(sum(durations) / len(durations)) if durations else None
    checkpoints = [
        (event.get("timestamp"), (event.get("context") or {}).get("checkpointPath") or event.get("checkpointPath"))
        for event in events
        if event.get("eventType") == "context_checkpoint_written"
    ]
    failures_list = command_failures(events)
    no_required_checks = no_required_checks_count(events)
    missing_lifecycle = lifecycle_missing_counts(events)
    token_totals = token_totals_from_events(events)
    model_policy_totals = model_policy_totals_from_events(events)
    candidates = improvement_candidates(events, blockers, failures)

    lines = [
        f"# Agent Loop Diagnostics Summary: {args.run_id}",
        "",
        f"- Scope: {summary_scope}",
        "",
        f"- Repo: `{args.repo}`",
        f"- Total events: {len(events)}",
        f"- Total cycles: {event_counts.get('poll_cycle_started', 0)}",
        f"- Worker spawns: {event_counts.get('slice_worker_spawned', 0)}",
        f"- Worker reports: {event_counts.get('worker_report_received', 0)}",
        f"- Blocked events: {event_counts.get('worker_blocked', 0)}",
        f"- Failed events: {event_counts.get('worker_failed', 0)}",
        f"- Merge completions: {event_counts.get('merge_completed', 0)}",
        f"- Average worker duration: {average_worker_duration if average_worker_duration is not None else 'unknown'} ms",
        "",
        "## Outcomes",
        "",
        *[f"- `{key}`: {value}" for key, value in sorted(outcomes.items())],
        *(["- None recorded"] if not outcomes else []),
        "",
        "## Statuses",
        "",
        *[f"- `{key}`: {value}" for key, value in sorted(statuses.items())],
        *(["- None recorded"] if not statuses else []),
        "",
        "## Most Common Blockers",
        "",
        *[f"- `{key}`: {value}" for key, value in blockers.most_common()],
        *(["- None recorded"] if not blockers else []),
        "",
        "## Failed Commands",
        "",
        *[f"- {failure}" for failure in failures_list],
        *(["- None recorded"] if not failures_list else []),
        "",
        "## No Required Checks",
        "",
        f"- Observations: {no_required_checks}",
        "",
        "## Diagnostics Completeness",
        "",
        *[f"- `{key}`: {value}" for key, value in sorted(missing_lifecycle.items())],
        *(["- No missing lifecycle fields recorded"] if not missing_lifecycle else []),
        "",
        "## Token Estimates",
        "",
        f"- Estimates logged: {token_totals['tokenEstimatesLogged']}",
        f"- Estimated tokens observed: {token_totals['estimatedTokensObserved']}",
        f"- Baseline estimated tokens: {token_totals['baselineEstimatedTokens']}",
        f"- Estimated savings: {token_totals['estimatedTokensSaved']} tokens",
        f"- Estimated savings percent: {token_totals['estimatedSavingsPercent']}%",
        "",
        "## Token Estimates By Artifact",
        "",
        *[
            f"- `{kind}`: count={data['count']}, current={data['estimatedTokensObserved']}, baseline={data['baselineEstimatedTokens']}, saved={data['estimatedTokensSaved']}, confidence={data['confidence']}"
            for kind, data in sorted(token_totals["byArtifactKind"].items())
        ],
        *(["- None recorded"] if not token_totals["byArtifactKind"] else []),
        "",
        "## Model Policy",
        "",
        f"- Model policy events: {model_policy_totals['events']}",
        "",
        "### By Task Class",
        "",
        *[f"- `{key}`: {value}" for key, value in sorted(model_policy_totals["byTaskClass"].items())],
        *(["- None recorded"] if not model_policy_totals["byTaskClass"] else []),
        "",
        "### By Model",
        "",
        *[f"- `{key}`: {value}" for key, value in sorted(model_policy_totals["byModel"].items())],
        *(["- None recorded"] if not model_policy_totals["byModel"] else []),
        "",
        "### Escalations",
        "",
        *[f"- `{key}`: {value}" for key, value in sorted(model_policy_totals["escalations"].items())],
        *(["- None recorded"] if not model_policy_totals["escalations"] else []),
        "",
        "### Failures",
        "",
        *[f"- `{key}`: {value}" for key, value in sorted(model_policy_totals["failures"].items())],
        *(["- None recorded"] if not model_policy_totals["failures"] else []),
        "",
        "### Low Confidence Or Malformed",
        "",
        *[f"- `{key}`: {value}" for key, value in sorted(model_policy_totals["lowConfidenceOrMalformed"].items())],
        *(["- None recorded"] if not model_policy_totals["lowConfidenceOrMalformed"] else []),
        "",
        "## Context Checkpoints",
        "",
        *[f"- {timestamp}: {path or 'path not recorded'}" for timestamp, path in checkpoints],
        *(["- None recorded"] if not checkpoints else []),
        "",
        "## Merge Throughput",
        "",
        f"- Merge attempts: {event_counts.get('merge_attempted', 0)}",
        f"- Merge completions: {event_counts.get('merge_completed', 0)}",
        f"- Ready-to-merge detections: {event_counts.get('ready_to_merge_detected', 0)}",
        "",
        "## Prompt Or Skill Improvement Candidates",
        "",
        *[f"- {candidate}" for candidate in candidates],
        "",
    ]
    path = report_path(base_dir, args.run_id, suffix=suffix)
    path.write_text("\n".join(lines), encoding="utf-8")
    append_event(
        base_dir,
        args.repo,
        args.run_id,
        "master",
        "diagnostic_summary_written",
        {"reportPath": str(path)},
    )
    print(json.dumps({"ok": True, "path": str(path)}, indent=2))


def cmd_estimate_tokens(args: argparse.Namespace) -> None:
    if args.artifact_kind not in VALID_ARTIFACT_KINDS:
        raise ValueError(f"invalid artifact kind: {args.artifact_kind}")
    text = Path(args.text_file).read_text(encoding="utf-8")
    baseline_text = Path(args.baseline_text_file).read_text(encoding="utf-8") if args.baseline_text_file else None
    token_estimate = estimate_token_savings(text, baseline_text, args.method)
    payload = {
        "artifactKind": args.artifact_kind,
        "artifactId": args.artifact_id,
        "tokenEstimate": token_estimate,
    }
    for key, value in {
        "taskClass": args.task_class,
        "agentProfile": args.agent_profile,
        "model": args.model,
        "modelReasoningEffort": args.model_reasoning_effort,
    }.items():
        if value:
            payload[key] = value
    event = append_event(Path.cwd(), args.repo, args.run_id, "master", "token_usage_estimated", payload)
    output = {"ok": True, "eventId": event["eventId"], **payload}
    print(json.dumps(output, indent=2, sort_keys=True))


def token_report_lines(events: list[dict[str, Any]], title: str) -> list[str]:
    totals = token_totals_from_events(events)
    by_model = token_totals_by_field(events, "model")
    by_task = token_totals_by_field(events, "taskClass")
    escalation_count = sum(1 for event in events if event.get("modelPolicyEscalatedFrom"))
    retry_token_estimate = sum(
        int((token_estimate_from_event(event) or {}).get("estimatedTokens") or 0)
        for event in events
        if event.get("modelPolicyEscalatedFrom")
    )
    net_savings = int(totals["estimatedTokensSaved"]) - retry_token_estimate
    lines = [
        f"# {title}",
        "",
        "## Estimated Savings",
        "",
        f"- Observed current tokens: {totals['estimatedTokensObserved']}",
        f"- Estimated baseline tokens: {totals['baselineEstimatedTokens']}",
        f"- Estimated savings: {totals['estimatedTokensSaved']}",
        f"- Escalation count: {escalation_count}",
        f"- Retry token estimate: {retry_token_estimate}",
        f"- Net estimated savings: {net_savings}",
        f"- Estimated savings percent: {totals['estimatedSavingsPercent']}%",
        "",
        "## By Artifact",
        "",
        "| Artifact kind | Count | Current | Baseline | Savings | Confidence |",
        "| --- | ---: | ---: | ---: | ---: | --- |",
    ]
    for kind, data in sorted(totals["byArtifactKind"].items()):
        lines.append(
            f"| {kind} | {data['count']} | {data['estimatedTokensObserved']} | {data['baselineEstimatedTokens']} | {data['estimatedTokensSaved']} | {data['confidence']} |"
        )
    if not totals["byArtifactKind"]:
        lines.append("| None | 0 | 0 | 0 | 0 | n/a |")
    lines.extend(
        [
            "",
            "## By Model",
            "",
            "| Model | Count | Current | Baseline | Savings | Retry estimate |",
            "| --- | ---: | ---: | ---: | ---: | ---: |",
        ]
    )
    for model, data in sorted(by_model.items()):
        lines.append(
            f"| {model} | {data['count']} | {data['estimatedTokensObserved']} | {data['baselineEstimatedTokens']} | {data['estimatedTokensSaved']} | {data['retryTokenEstimate']} |"
        )
    if not by_model:
        lines.append("| None | 0 | 0 | 0 | 0 | 0 |")
    lines.extend(
        [
            "",
            "## By Task Class",
            "",
            "| Task class | Count | Current | Baseline | Savings | Retry estimate |",
            "| --- | ---: | ---: | ---: | ---: | ---: |",
        ]
    )
    for task_class, data in sorted(by_task.items()):
        lines.append(
            f"| {task_class} | {data['count']} | {data['estimatedTokensObserved']} | {data['baselineEstimatedTokens']} | {data['estimatedTokensSaved']} | {data['retryTokenEstimate']} |"
        )
    if not by_task:
        lines.append("| None | 0 | 0 | 0 | 0 | 0 |")
    lines.extend(
        [
            "",
            "## Notes",
            "",
            "- Estimates use `char-div-4` unless `tiktoken` is available and selected.",
            "- Savings are guestimates unless both current and baseline text were measured.",
            "",
        ]
    )
    return lines


def read_run_dir_events(runs_dir: Path) -> list[dict[str, Any]]:
    events: list[dict[str, Any]] = []
    for path in sorted(runs_dir.glob("*.jsonl")):
        events.extend(read_events(path))
    return events


def cmd_token_report(args: argparse.Namespace) -> None:
    base_dir = Path.cwd()
    if bool(args.run_id) == bool(args.runs_dir):
        raise ValueError("token-report requires exactly one of --run-id or --runs-dir")
    if args.run_id:
        events = read_events(run_path(base_dir, args.run_id))
        path = report_path(base_dir, args.run_id, suffix="token-report")
        title = f"Agent Loop Token Report: {args.run_id}"
    else:
        runs_dir = Path(args.runs_dir)
        events = read_run_dir_events(runs_dir)
        ensure_dirs(base_dir)
        path = diagnostics_root(base_dir) / "reports" / "token-report-all.md"
        title = "Agent Loop Token Report: all runs"
    path.write_text("\n".join(token_report_lines(events, title)), encoding="utf-8")
    print(json.dumps({"ok": True, "path": str(path), "events": len(events)}, indent=2, sort_keys=True))


def cmd_latest(args: argparse.Namespace) -> None:
    path = latest_path(Path.cwd())
    if not path.exists():
        raise SystemExit("latest diagnostics file not found")
    print(path.read_text(encoding="utf-8"), end="")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(required=True)

    start = sub.add_parser("start-run", help="Create or resume a diagnostics run")
    start.add_argument("--repo", required=True)
    start.add_argument("--resume", action="store_true", help="Resume latest run for repo when possible")
    start.set_defaults(func=cmd_start_run)

    log = sub.add_parser("log", help="Append one diagnostics event")
    log.add_argument("--repo", required=True)
    log.add_argument("--run-id", required=True)
    log.add_argument("--event-type", required=True, choices=sorted(VALID_EVENT_TYPES))
    log.add_argument("--actor", required=True, choices=sorted(VALID_ACTORS))
    log.add_argument("--json", required=True, help="Additional event fields as a JSON object")
    log.set_defaults(func=cmd_log)

    snapshot = sub.add_parser("snapshot", help="Log a queue snapshot event")
    snapshot.add_argument("--repo", required=True)
    snapshot.add_argument("--run-id", required=True)
    snapshot.add_argument("--cycle", type=int, required=True)
    snapshot.add_argument("--mode", choices=sorted(VALID_SNAPSHOT_MODES), default="compact")
    snapshot.add_argument("--token-method", choices=["auto", "char-div-4", "tiktoken"], default="auto")
    snapshot.set_defaults(func=cmd_snapshot)

    summarize = sub.add_parser("summarize", help="Write a Markdown summary report")
    summarize.add_argument("--repo", required=True)
    summarize.add_argument("--run-id", required=True)
    summarize.add_argument("--since-event")
    summarize.add_argument("--cycle", type=int)
    summarize.add_argument("--last", type=int)
    summarize.set_defaults(func=cmd_summarize)

    estimate = sub.add_parser("estimate-tokens", help="Estimate and log token use for an artifact")
    estimate.add_argument("--repo", required=True)
    estimate.add_argument("--run-id", required=True)
    estimate.add_argument("--artifact-kind", required=True, choices=sorted(VALID_ARTIFACT_KINDS))
    estimate.add_argument("--artifact-id", required=True)
    estimate.add_argument("--text-file", required=True)
    estimate.add_argument("--baseline-text-file")
    estimate.add_argument("--method", choices=["auto", "char-div-4", "tiktoken"], default="auto")
    estimate.add_argument("--task-class")
    estimate.add_argument("--agent-profile")
    estimate.add_argument("--model")
    estimate.add_argument("--model-reasoning-effort")
    estimate.set_defaults(func=cmd_estimate_tokens)

    token_report = sub.add_parser("token-report", help="Write a token estimate report")
    token_report.add_argument("--repo", required=True)
    token_report.add_argument("--run-id")
    token_report.add_argument("--runs-dir")
    token_report.set_defaults(func=cmd_token_report)

    latest = sub.add_parser("latest", help="Print latest diagnostics metadata")
    latest.set_defaults(func=cmd_latest)

    return parser


def main() -> None:
    args = build_parser().parse_args()
    try:
        args.func(args)
    except (OSError, ValueError, RuntimeError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error


if __name__ == "__main__":
    main()
