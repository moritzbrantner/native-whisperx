#!/usr/bin/env python3
"""GitHub queue helpers for the agent-loop skill.

This script owns mechanical GitHub state changes only. The Codex master thread
keeps product judgment and routing decisions in the skill instructions.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


MATT_LABELS = {
    "bug": ("D73A4A", "Something isn't working"),
    "enhancement": ("A2EEEF", "New feature or request"),
    "needs-triage": ("D4C5F9", "Needs review before assignment"),
    "needs-info": ("D876E3", "Waiting on reporter for more information"),
    "ready-for-agent": ("0E8A16", "Fully specified, ready for an AFK agent"),
    "ready-for-human": ("FBCA04", "Requires human implementation"),
    "wontfix": ("ffffff", "This will not be worked on"),
    "prd": ("6F42C1", "Product requirements document ready for workflow routing"),
}

LOOP_LABELS = {
    "agent-loop:claimed": ("1D76DB", "Claimed by the agent-loop master"),
    "agent-loop:in-progress": ("0E8A16", "Work is active in an agent-loop worker"),
    "agent-loop:blocked": ("D93F0B", "Blocked on human input or external access"),
    "agent-loop:ready-to-merge": ("5319E7", "Worker reports the PR is ready to merge"),
    "agent-loop:merged": ("8250DF", "Associated PR has been merged"),
    "agent-loop:done": ("0E8A16", "Agent-loop work is complete"),
    "agent-loop:failed": ("B60205", "Automation failed and needs review"),
}

ALL_LABELS = {**MATT_LABELS, **LOOP_LABELS}
OBSOLETE_READY_LABEL = "agent-ready"
CANONICAL_READY_LABEL = "ready-for-agent"

TERMINAL_OR_BLOCKING = {
    "agent-loop:blocked",
    "agent-loop:failed",
    "agent-loop:done",
    "agent-loop:merged",
}

ACTIVE_LABELS = {
    "agent-loop:claimed",
    "agent-loop:in-progress",
    "agent-loop:ready-to-merge",
}


@dataclass(frozen=True)
class Issue:
    number: int
    title: str
    url: str
    labels: set[str]
    body: str
    updated_at: str | None = None


@dataclass(frozen=True)
class LabeledItem:
    kind: str
    number: int
    title: str


@dataclass(frozen=True)
class QueueValidation:
    issue: Issue
    reason: str
    parent: int | None = None


def run_gh(args: list[str], input_text: str | None = None, check: bool = True) -> str:
    proc = subprocess.run(
        ["gh", *args],
        input=input_text,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if check and proc.returncode != 0:
        sys.stderr.write(proc.stderr)
        raise SystemExit(proc.returncode)
    return proc.stdout


def issue_from_json(raw: dict[str, Any]) -> Issue:
    return Issue(
        number=int(raw["number"]),
        title=raw.get("title") or "",
        url=raw.get("url") or "",
        labels={label["name"] for label in raw.get("labels", [])},
        body=raw.get("body") or "",
        updated_at=raw.get("updatedAt"),
    )


def list_issues(repo: str, state: str = "open", limit: int = 100) -> list[Issue]:
    out = run_gh(
        [
            "issue",
            "list",
            "--repo",
            repo,
            "--state",
            state,
            "--limit",
            str(limit),
            "--json",
            "number,title,url,labels,body,updatedAt",
        ]
    )
    return [issue_from_json(item) for item in json.loads(out)]


def get_issue(repo: str, number: int) -> Issue:
    out = run_gh(
        [
            "issue",
            "view",
            str(number),
            "--repo",
            repo,
            "--json",
            "number,title,url,labels,body,updatedAt",
        ]
    )
    return issue_from_json(json.loads(out))


def load_labels(repo: str) -> dict[str, dict[str, str]]:
    out = run_gh(
        [
            "label",
            "list",
            "--repo",
            repo,
            "--limit",
            "500",
            "--json",
            "name,color,description",
        ]
    )
    return {
        item["name"]: {
            "color": item.get("color") or "",
            "description": item.get("description") or "",
        }
        for item in json.loads(out)
    }


def list_labeled_items(repo: str, kind: str, label: str) -> list[LabeledItem]:
    out = run_gh(
        [
            kind,
            "list",
            "--repo",
            repo,
            "--state",
            "all",
            "--label",
            label,
            "--limit",
            "500",
            "--json",
            "number,title",
        ],
        check=False,
    )
    if not out.strip():
        return []
    return [
        LabeledItem(kind=kind, number=int(item["number"]), title=item.get("title") or "")
        for item in json.loads(out)
    ]


def emit(issues: list[Issue]) -> None:
    payload = [
        {
            "number": issue.number,
            "title": issue.title,
            "url": issue.url,
            "labels": sorted(issue.labels),
            **metadata_for_issue(issue),
            "updatedAt": issue.updated_at,
        }
        for issue in issues
    ]
    print(json.dumps(payload, indent=2, sort_keys=True))


def emit_validations(validations: list[QueueValidation]) -> None:
    payload = [
        {
            "number": validation.issue.number,
            "title": validation.issue.title,
            "url": validation.issue.url,
            "labels": sorted(validation.issue.labels),
            "parent": validation.parent,
            "reason": validation.reason,
            "updatedAt": validation.issue.updated_at,
        }
        for validation in validations
    ]
    print(json.dumps(payload, indent=2, sort_keys=True))


def has_any(issue: Issue, labels: set[str]) -> bool:
    return bool(issue.labels.intersection(labels))


def is_raw(issue: Issue) -> bool:
    if has_any(issue, TERMINAL_OR_BLOCKING):
        return False
    if not issue.labels:
        return True
    return "needs-triage" in issue.labels or "needs-info" in issue.labels


def is_prd(issue: Issue) -> bool:
    return (
        "prd" in issue.labels
        and "ready-for-agent" in issue.labels
        and not has_any(issue, TERMINAL_OR_BLOCKING | {"agent-loop:claimed", "agent-loop:in-progress"})
        and prd_validation_reason(issue) is None
    )


def is_slice(issue: Issue, issues_by_number: dict[int, Issue]) -> bool:
    return (
        "ready-for-agent" in issue.labels
        and "prd" not in issue.labels
        and not has_any(issue, TERMINAL_OR_BLOCKING | {"agent-loop:claimed", "agent-loop:in-progress"})
        and slice_validation_reason(issue, issues_by_number) is None
    )


def is_active(issue: Issue) -> bool:
    return has_any(issue, ACTIVE_LABELS) and not has_any(issue, {"agent-loop:done", "agent-loop:merged"})


def is_ready_to_merge(issue: Issue) -> bool:
    return (
        "agent-loop:ready-to-merge" in issue.labels
        and "agent-loop:merged" not in issue.labels
        and "agent-loop:done" not in issue.labels
    )


def add_labels(repo: str, issue: int, labels: list[str]) -> None:
    for label in labels:
        run_gh(["issue", "edit", str(issue), "--repo", repo, "--add-label", label])


def remove_labels(repo: str, issue: int, labels: list[str]) -> None:
    for label in labels:
        run_gh(["issue", "edit", str(issue), "--repo", repo, "--remove-label", label], check=False)


def comment(repo: str, issue: int, body: str) -> None:
    run_gh(["issue", "comment", str(issue), "--repo", repo, "--body-file", "-"], input_text=body)


def read_text(path: str) -> str:
    return Path(path).read_text(encoding="utf-8")


def extract_issue_numbers(text: str) -> set[int]:
    return {int(match) for match in re.findall(r"(?<![A-Za-z0-9_-])#(\d+)", text)}


def parse_scalar(value: str) -> Any:
    value = value.strip().strip('"').strip("'")
    if re.fullmatch(r"#?\d+", value):
        return int(value.removeprefix("#"))
    return value


def parse_frontmatter(body: str) -> dict[str, Any]:
    lines = body.splitlines()
    if not lines or lines[0].strip() != "---":
        return {}
    try:
        end = next(index for index, line in enumerate(lines[1:], start=1) if line.strip() == "---")
    except StopIteration:
        return {}
    metadata: dict[str, Any] = {}
    current_key: str | None = None
    for offset, raw_line in enumerate(lines[1:end], start=1):
        if not raw_line.strip() or raw_line.lstrip().startswith("#"):
            continue
        if raw_line.startswith((" ", "\t")) and current_key:
            stripped = raw_line.strip()
            if stripped.startswith("- "):
                metadata.setdefault(current_key, [])
                if isinstance(metadata[current_key], list):
                    metadata[current_key].append(parse_scalar(stripped[2:]))
                continue
            if ":" in stripped and isinstance(metadata.get(current_key), dict):
                key, value = stripped.split(":", 1)
                metadata[current_key][key.strip()] = parse_scalar(value)
                continue
        if ":" not in raw_line:
            current_key = None
            continue
        key, value = raw_line.split(":", 1)
        current_key = key.strip()
        if value.strip():
            metadata[current_key] = parse_scalar(value)
        else:
            next_nonempty = ""
            for lookahead in lines[offset + 1 : end]:
                if lookahead.strip():
                    next_nonempty = lookahead.strip()
                    break
            metadata[current_key] = [] if next_nonempty.startswith("- ") else {}
    return metadata


def list_value(value: Any) -> list[str]:
    if value is None:
        return []
    if isinstance(value, list):
        return [str(item) for item in value if str(item)]
    if isinstance(value, str) and value:
        return [value]
    return []


def parent_from_value(value: Any) -> int | None:
    if isinstance(value, int):
        return value
    if isinstance(value, str):
        match = re.search(r"#?(\d+)", value)
        if match:
            return int(match.group(1))
    return None


def metadata_for_issue(issue: Issue) -> dict[str, Any]:
    frontmatter = parse_frontmatter(issue.body)
    parent = parent_from_value(frontmatter.get("parent"))
    if parent is None:
        parent = extract_parent_issue_number_from_sections(issue.body)
    expected_write_scope = list_value(frontmatter.get("writeScope") or frontmatter.get("expectedWriteScope"))
    if not expected_write_scope:
        expected_write_scope = extract_list_section(issue.body, {"expected write scope", "write scope"})
    verification = list_value(frontmatter.get("verification"))
    if not verification:
        verification = extract_list_section(issue.body, {"verification", "checks"})
    context_docs = list_value(frontmatter.get("contextDocs") or frontmatter.get("context"))
    if not context_docs:
        context_docs = extract_list_section(issue.body, {"context docs", "required reading"})
    concurrency_group = frontmatter.get("concurrencyGroup") or frontmatter.get("concurrency")
    if concurrency_group is None:
        concurrency_group = extract_single_section_value(issue.body, {"concurrency group"})
    token_budget = frontmatter.get("tokenBudget")
    return {
        "parent": parent,
        "concurrencyGroup": str(concurrency_group) if concurrency_group else None,
        "expectedWriteScope": expected_write_scope,
        "verification": verification,
        "contextDocs": context_docs,
        "tokenBudget": token_budget if isinstance(token_budget, dict) else {},
    }


def extract_parent_issue_number(text: str) -> int | None:
    parent = parent_from_value(parse_frontmatter(text).get("parent"))
    if parent is not None:
        return parent
    return extract_parent_issue_number_from_sections(text)


def extract_parent_issue_number_from_sections(text: str) -> int | None:
    parent_line = re.search(
        r"(?im)^\s*Parent(?:\s+(?:PRD|issue))?\s*:\s*#(\d+)\b",
        text,
    )
    if parent_line:
        return int(parent_line.group(1))

    lines = text.splitlines()
    for index, line in enumerate(lines):
        if re.match(r"^\s*##+\s+Parent\s*$", line, re.IGNORECASE):
            for candidate in lines[index + 1 : index + 8]:
                match = re.search(r"(?<![A-Za-z0-9_-])#(\d+)\b", candidate)
                if match:
                    return int(match.group(1))
                if candidate.strip().startswith("##"):
                    break
    return None


def heading_block(body: str, names: set[str]) -> list[str]:
    normalized_names = {name.lower() for name in names}
    lines = body.splitlines()
    for index, line in enumerate(lines):
        match = re.match(r"^\s*##+\s+(.+?)\s*$", line)
        if not match or match.group(1).strip().lower() not in normalized_names:
            continue
        block: list[str] = []
        for candidate in lines[index + 1 :]:
            if candidate.strip().startswith("##"):
                break
            block.append(candidate)
        return block
    return []


def extract_single_section_value(body: str, names: set[str]) -> str | None:
    for name in names:
        pattern = rf"(?im)^\s*{re.escape(name)}\s*:\s*(.+?)\s*$"
        match = re.search(pattern, body)
        if match:
            return match.group(1).strip()
    for line in heading_block(body, names):
        value = line.strip().removeprefix("-").strip()
        if value:
            return value
    return None


def extract_list_section(body: str, names: set[str]) -> list[str]:
    values = []
    for line in heading_block(body, names):
        value = line.strip().removeprefix("-").strip()
        if value:
            values.append(value)
    return values


def has_heading(body: str, names: set[str]) -> bool:
    normalized_names = {name.lower() for name in names}
    for line in body.splitlines():
        match = re.match(r"^\s*##+\s+(.+?)\s*$", line)
        if match and match.group(1).strip().lower() in normalized_names:
            return True
    return False


def prd_validation_reason(issue: Issue) -> str | None:
    if "prd" not in issue.labels or "ready-for-agent" not in issue.labels:
        return None
    if not has_heading(issue.body, {"acceptance criteria", "acceptance criterias"}):
        return "missing-acceptance-criteria"
    if not has_heading(issue.body, {"out of scope", "out-of-scope"}):
        return "missing-out-of-scope"
    if not has_heading(issue.body, {"problem statement", "summary", "scope", "solution"}):
        return "missing-problem-or-summary"
    return None


def slice_validation_reason(issue: Issue, issues_by_number: dict[int, Issue]) -> str | None:
    if "ready-for-agent" not in issue.labels or "prd" in issue.labels:
        return None
    parent = extract_parent_issue_number(issue.body)
    if parent is None:
        return "missing-parent-prd"
    parent_issue = issues_by_number.get(parent)
    if parent_issue is None:
        return "parent-prd-not-found"
    if "prd" not in parent_issue.labels:
        return "parent-issue-not-prd"
    return None


def invalid_prds(issues: list[Issue]) -> list[QueueValidation]:
    validations = []
    for issue in issues:
        if (
            "prd" in issue.labels
            and "ready-for-agent" in issue.labels
            and not has_any(issue, TERMINAL_OR_BLOCKING | {"agent-loop:claimed", "agent-loop:in-progress"})
        ):
            reason = prd_validation_reason(issue)
            if reason is not None:
                validations.append(QueueValidation(issue=issue, reason=reason))
    return validations


def invalid_slices(issues: list[Issue]) -> list[QueueValidation]:
    issues_by_number = {issue.number: issue for issue in issues}
    validations = []
    for issue in issues:
        if (
            "ready-for-agent" in issue.labels
            and "prd" not in issue.labels
            and not has_any(issue, TERMINAL_OR_BLOCKING | {"agent-loop:claimed", "agent-loop:in-progress"})
        ):
            parent = extract_parent_issue_number(issue.body)
            reason = slice_validation_reason(issue, issues_by_number)
            if reason is not None:
                validations.append(QueueValidation(issue=issue, reason=reason, parent=parent))
    return validations


def migrate_obsolete_ready_label(repo: str, labels: dict[str, dict[str, str]]) -> None:
    has_old = OBSOLETE_READY_LABEL in labels
    has_new = CANONICAL_READY_LABEL in labels
    color, description = ALL_LABELS[CANONICAL_READY_LABEL]

    if has_old and not has_new:
        run_gh(
            [
                "label",
                "edit",
                OBSOLETE_READY_LABEL,
                "--repo",
                repo,
                "--name",
                CANONICAL_READY_LABEL,
                "--color",
                color,
                "--description",
                description,
            ]
        )
        return

    if has_old and has_new:
        for item in [
            *list_labeled_items(repo, "issue", OBSOLETE_READY_LABEL),
            *list_labeled_items(repo, "pr", OBSOLETE_READY_LABEL),
        ]:
            run_gh([item.kind, "edit", str(item.number), "--repo", repo, "--add-label", CANONICAL_READY_LABEL])
            run_gh([item.kind, "edit", str(item.number), "--repo", repo, "--remove-label", OBSOLETE_READY_LABEL])
        run_gh(["label", "delete", OBSOLETE_READY_LABEL, "--repo", repo, "--yes"])


def ensure_labels(args: argparse.Namespace) -> None:
    migrate_obsolete_ready_label(args.repo, load_labels(args.repo))
    for name, (color, description) in ALL_LABELS.items():
        run_gh(
            [
                "label",
                "create",
                name,
                "--repo",
                args.repo,
                "--color",
                color,
                "--description",
                description,
                "--force",
            ]
        )
    print(json.dumps({"ok": True, "labels": sorted(ALL_LABELS)}, indent=2))


def list_raw(args: argparse.Namespace) -> None:
    emit([issue for issue in list_issues(args.repo, limit=args.limit) if is_raw(issue)])


def list_prds(args: argparse.Namespace) -> None:
    emit([issue for issue in list_issues(args.repo, limit=args.limit) if is_prd(issue)])


def list_slices(args: argparse.Namespace) -> None:
    issues = list_issues(args.repo, limit=args.limit)
    issues_by_number = {issue.number: issue for issue in issues}
    emit([issue for issue in issues if is_slice(issue, issues_by_number)])


def list_invalid_prds(args: argparse.Namespace) -> None:
    emit_validations(invalid_prds(list_issues(args.repo, limit=args.limit)))


def list_invalid_slices(args: argparse.Namespace) -> None:
    emit_validations(invalid_slices(list_issues(args.repo, limit=args.limit)))


def list_active(args: argparse.Namespace) -> None:
    emit([issue for issue in list_issues(args.repo, limit=args.limit) if is_active(issue)])


def list_ready_to_merge(args: argparse.Namespace) -> None:
    emit([issue for issue in list_issues(args.repo, limit=args.limit) if is_ready_to_merge(issue)])


def claim(args: argparse.Namespace) -> None:
    labels = ["agent-loop:claimed"]
    if args.kind == "slice":
        issues = list_issues(args.repo, limit=args.limit)
        issue = next((item for item in issues if item.number == args.issue), None) or get_issue(args.repo, args.issue)
        issues_by_number = {item.number: item for item in issues}
        issues_by_number.setdefault(issue.number, issue)
        reason = slice_validation_reason(issue, issues_by_number)
        if reason is not None:
            print(
                json.dumps(
                    {
                        "ok": False,
                        "issue": args.issue,
                        "reason": reason,
                        "parent": extract_parent_issue_number(issue.body),
                    },
                    indent=2,
                    sort_keys=True,
                ),
                file=sys.stderr,
            )
            raise SystemExit(1)
        labels.append("agent-loop:in-progress")
    add_labels(args.repo, args.issue, labels)
    comment(
        args.repo,
        args.issue,
        f"""## Agent Loop Claim

**Kind:** {args.kind}
**Action:** Claimed for autonomous processing.
""",
    )
    print(json.dumps({"ok": True, "issue": args.issue, "labels": labels}))


def block(args: argparse.Namespace) -> None:
    reason = read_text(args.reason_file)
    add_labels(args.repo, args.issue, ["agent-loop:blocked"])
    remove_labels(args.repo, args.issue, ["agent-loop:claimed", "agent-loop:in-progress", "agent-loop:ready-to-merge"])
    comment(args.repo, args.issue, reason)
    print(json.dumps({"ok": True, "issue": args.issue, "label": "agent-loop:blocked"}))


def mark_in_progress(args: argparse.Namespace) -> None:
    add_labels(args.repo, args.issue, ["agent-loop:in-progress"])
    print(json.dumps({"ok": True, "issue": args.issue, "label": "agent-loop:in-progress"}))


def mark_ready_to_merge(args: argparse.Namespace) -> None:
    add_labels(args.repo, args.issue, ["agent-loop:ready-to-merge"])
    comment(
        args.repo,
        args.issue,
        f"""## Agent Loop Ready To Merge

**PR:** #{args.pr}
**Worker status:** ready-to-merge
""",
    )
    print(json.dumps({"ok": True, "issue": args.issue, "pr": args.pr}))


def mark_done(args: argparse.Namespace) -> None:
    add_labels(args.repo, args.issue, ["agent-loop:merged", "agent-loop:done"])
    remove_labels(
        args.repo,
        args.issue,
        ["agent-loop:claimed", "agent-loop:in-progress", "agent-loop:ready-to-merge"],
    )
    comment(
        args.repo,
        args.issue,
        f"""## Agent Loop Done

**Merged PR:** #{args.pr}
**Status:** Complete
""",
    )
    print(json.dumps({"ok": True, "issue": args.issue, "pr": args.pr}))


def feature_status(args: argparse.Namespace) -> None:
    issues = list_issues(args.repo, state="all", limit=args.limit)
    children = [
        issue
        for issue in issues
        if issue.number != args.issue
        and extract_parent_issue_number(issue.body) == args.issue
        and "ready-for-agent" in issue.labels
    ]
    done = [issue for issue in children if "agent-loop:done" in issue.labels]
    blocked = [issue for issue in children if "agent-loop:blocked" in issue.labels]
    failed = [issue for issue in children if "agent-loop:failed" in issue.labels]
    payload = {
        "parent": args.issue,
        "childIssues": [issue.number for issue in children],
        "done": [issue.number for issue in done],
        "blocked": [issue.number for issue in blocked],
        "failed": [issue.number for issue in failed],
        "finished": bool(children) and len(done) == len(children) and not blocked and not failed,
    }
    print(json.dumps(payload, indent=2, sort_keys=True))


def validate_queue(args: argparse.Namespace) -> None:
    issues = list_issues(args.repo, limit=args.limit)
    invalid = [*invalid_prds(issues), *invalid_slices(issues)]
    payload = {
        "ok": not invalid,
        "invalid": [
            {
                "number": validation.issue.number,
                "title": validation.issue.title,
                "reason": validation.reason,
                "parent": validation.parent,
                "labels": sorted(validation.issue.labels),
                "url": validation.issue.url,
            }
            for validation in invalid
        ],
    }
    print(json.dumps(payload, indent=2, sort_keys=True))
    if invalid:
        raise SystemExit(1)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="GitHub queue helpers for the agent-loop skill")
    sub = parser.add_subparsers(required=True)

    def add_repo(p: argparse.ArgumentParser) -> None:
        p.add_argument("--repo", required=True, help="GitHub repository in OWNER/REPO form")

    def add_limit(p: argparse.ArgumentParser) -> None:
        p.add_argument("--limit", type=int, default=100, help="Maximum issues to fetch")

    p = sub.add_parser("ensure-labels", help="Create or update agent-loop labels")
    add_repo(p)
    p.set_defaults(func=ensure_labels)

    for name, func in [
        ("list-raw", list_raw),
        ("list-prds", list_prds),
        ("list-slices", list_slices),
        ("list-invalid-prds", list_invalid_prds),
        ("list-invalid-slices", list_invalid_slices),
        ("list-active", list_active),
        ("list-ready-to-merge", list_ready_to_merge),
    ]:
        p = sub.add_parser(name, help=f"{name.replace('-', ' ')} as JSON")
        add_repo(p)
        add_limit(p)
        p.set_defaults(func=func)

    p = sub.add_parser("claim", help="Claim an issue for loop processing")
    add_repo(p)
    add_limit(p)
    p.add_argument("--issue", type=int, required=True)
    p.add_argument("--kind", choices=["prd", "slice", "raw"], required=True)
    p.set_defaults(func=claim)

    p = sub.add_parser("block", help="Mark an issue blocked and post a blocker comment")
    add_repo(p)
    p.add_argument("--issue", type=int, required=True)
    p.add_argument("--reason-file", required=True)
    p.set_defaults(func=block)

    p = sub.add_parser("mark-in-progress", help="Mark an issue in progress")
    add_repo(p)
    p.add_argument("--issue", type=int, required=True)
    p.set_defaults(func=mark_in_progress)

    p = sub.add_parser("mark-ready-to-merge", help="Mark a slice issue ready to merge")
    add_repo(p)
    p.add_argument("--issue", type=int, required=True)
    p.add_argument("--pr", type=int, required=True)
    p.set_defaults(func=mark_ready_to_merge)

    p = sub.add_parser("mark-done", help="Mark a slice issue done after merge")
    add_repo(p)
    p.add_argument("--issue", type=int, required=True)
    p.add_argument("--pr", type=int, required=True)
    p.set_defaults(func=mark_done)

    p = sub.add_parser("feature-status", help="Summarize child slice state for a parent PRD")
    add_repo(p)
    add_limit(p)
    p.add_argument("--issue", type=int, required=True, help="Parent PRD issue number")
    p.set_defaults(func=feature_status)

    p = sub.add_parser("validate-queue", help="Report invalid ready PRDs or slices and exit non-zero when found")
    add_repo(p)
    add_limit(p)
    p.set_defaults(func=validate_queue)

    return parser


def main() -> None:
    parser = build_parser()
    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
