#!/usr/bin/env python3
"""Context pack and prompt helpers for agent-loop workers."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import agent_loop_diagnostics as diagnostics
import agent_loop_queue as queue


DEFAULT_PACK_TOKEN_BUDGET = 2000
VALID_TEMPLATES = {"slice", "docs-only", "test-only", "repair", "merge-conflict"}


@dataclass(frozen=True)
class IssueView:
    number: int
    title: str
    url: str
    labels: list[str]
    body: str
    comments: list[dict[str, Any]]
    updated_at: str | None = None


def run_gh(args: list[str]) -> str:
    proc = subprocess.run(["gh", *args], text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    if proc.returncode != 0:
        sys.stderr.write(proc.stderr)
        raise SystemExit(proc.returncode)
    return proc.stdout


def issue_from_json(raw: dict[str, Any]) -> IssueView:
    return IssueView(
        number=int(raw["number"]),
        title=raw.get("title") or "",
        url=raw.get("url") or "",
        labels=[label["name"] for label in raw.get("labels", [])],
        body=raw.get("body") or "",
        comments=raw.get("comments") or [],
        updated_at=raw.get("updatedAt"),
    )


def fetch_issue(repo: str, number: int) -> IssueView:
    output = run_gh(
        [
            "issue",
            "view",
            str(number),
            "--repo",
            repo,
            "--json",
            "number,title,url,labels,body,comments,updatedAt",
        ]
    )
    return issue_from_json(json.loads(output))


def issue_for_metadata(issue: IssueView) -> queue.Issue:
    return queue.Issue(
        number=issue.number,
        title=issue.title,
        url=issue.url,
        labels=set(issue.labels),
        body=issue.body,
        updated_at=issue.updated_at,
    )


def section_text(body: str, names: set[str]) -> str:
    return "\n".join(queue.heading_block(body, names)).strip()


def list_or_section(metadata: dict[str, Any], key: str, body: str, headings: set[str]) -> list[str]:
    values = metadata.get(key)
    if isinstance(values, list):
        return [str(value) for value in values]
    if isinstance(values, str):
        return [values]
    return queue.extract_list_section(body, headings)


def short_text(text: str, max_chars: int = 1200) -> str:
    text = text.strip()
    if len(text) <= max_chars:
        return text
    return text[:max_chars].rstrip() + "\n...[truncated]"


def summarize_comments(comments: list[dict[str, Any]]) -> list[str]:
    summaries = []
    for comment in comments[-5:]:
        body = str(comment.get("body") or "")
        url = str(comment.get("url") or "")
        if "Human input needed" in body or "Agent Loop Blocker" in body or "Agent Loop Ready To Merge" in body:
            first_line = next((line.strip() for line in body.splitlines() if line.strip()), "comment")
            summaries.append(f"- {first_line} {url}".strip())
    return summaries


def render_list(items: list[str], empty: str = "None") -> str:
    if not items:
        return empty
    return "\n".join(f"- {item}" for item in items)


def build_pack_text(repo: str, parent: IssueView, slice_issue: IssueView) -> str:
    metadata = queue.metadata_for_issue(issue_for_metadata(slice_issue))
    parent_metadata = queue.metadata_for_issue(issue_for_metadata(parent))
    acceptance = section_text(slice_issue.body, {"acceptance criteria"}) or section_text(parent.body, {"acceptance criteria"})
    goal = section_text(slice_issue.body, {"goal", "summary", "what to build", "problem statement"}) or slice_issue.title
    write_scope = metadata.get("expectedWriteScope") or parent_metadata.get("expectedWriteScope") or []
    verification = metadata.get("verification") or parent_metadata.get("verification") or []
    context_docs = metadata.get("contextDocs") or parent_metadata.get("contextDocs") or ["AGENTS.md"]
    blockers = section_text(slice_issue.body, {"blockers", "blocked on"})
    comments = summarize_comments(slice_issue.comments)
    lines = [
        f"# Worker Context Pack: issue #{slice_issue.number}",
        "",
        f"Repository: {repo}",
        f"Parent PRD: #{parent.number} {parent.url}",
        f"Slice issue: #{slice_issue.number} {slice_issue.url}",
        f"Concurrency group: {metadata.get('concurrencyGroup') or 'unspecified'}",
        "",
        "## Goal",
        "",
        short_text(goal, 900),
        "",
        "## Acceptance Criteria",
        "",
        short_text(acceptance, 1200) if acceptance else "None listed",
        "",
        "## Expected Write Scope",
        "",
        render_list([str(item) for item in write_scope]),
        "",
        "## Verification",
        "",
        render_list([str(item) for item in verification]),
        "",
        "## Required Reading",
        "",
        render_list([str(item) for item in context_docs]),
        "",
        "## Blockers",
        "",
        short_text(blockers, 600) if blockers else "None",
        "",
        "## Recent Relevant Comments",
        "",
        "\n".join(comments) if comments else "None",
        "",
        "## Notes",
        "",
        "Keep implementation limited to this slice. Read more files only when the pack and required docs are insufficient.",
        "",
    ]
    return "\n".join(lines)


def trim_to_budget(text: str, token_budget: int, method: str) -> str:
    estimate = diagnostics.estimate_tokens(text, method)
    if int(estimate["estimatedTokens"]) <= token_budget:
        return text
    sections = text.split("\n## Recent Relevant Comments\n")
    if len(sections) == 2:
        text = sections[0] + "\n## Recent Relevant Comments\n\nOmitted to stay within token budget.\n"
    estimate = diagnostics.estimate_tokens(text, method)
    if int(estimate["estimatedTokens"]) <= token_budget:
        return text
    return text[: token_budget * 4].rstrip() + "\n...[truncated to token budget]\n"


def build_pack(repo: str, slice_number: int, token_method: str = "auto") -> tuple[str, dict[str, Any]]:
    slice_issue = fetch_issue(repo, slice_number)
    metadata = queue.metadata_for_issue(issue_for_metadata(slice_issue))
    parent_number = metadata.get("parent")
    if not isinstance(parent_number, int):
        raise ValueError(f"slice #{slice_number} does not have a parent PRD link")
    parent = fetch_issue(repo, parent_number)
    token_budget = metadata.get("tokenBudget") if isinstance(metadata.get("tokenBudget"), dict) else {}
    pack_budget = int(token_budget.get("contextPack") or DEFAULT_PACK_TOKEN_BUDGET)
    text = trim_to_budget(build_pack_text(repo, parent, slice_issue), pack_budget, token_method)
    generic_prompt = worker_prompt_reference_path().read_text(encoding="utf-8")
    baseline = parent.body + "\n\n" + slice_issue.body + "\n\n" + generic_prompt
    estimate = diagnostics.estimate_token_savings(text, baseline, token_method)
    return text, estimate


def worker_prompt_reference_path() -> Path:
    local_path = Path(__file__).resolve().parent.parent / "references" / "worker-prompts.md"
    if local_path.exists():
        return local_path
    skill_path = Path.home() / ".codex" / "skills" / "agent-loop" / "references" / "worker-prompts.md"
    if skill_path.exists():
        return skill_path
    raise FileNotFoundError("worker-prompts.md reference file not found")


def prompt_template(template: str) -> str:
    if template not in VALID_TEMPLATES:
        raise ValueError(f"invalid template: {template}")
    action = {
        "slice": "Implement the ready slice described by the context pack.",
        "docs-only": "Complete the documentation-only slice described by the context pack.",
        "test-only": "Complete the test or characterization slice described by the context pack.",
        "repair": "Repair the PR or branch described by the context pack.",
        "merge-conflict": "Resolve the merge conflict described by the context pack.",
    }[template]
    return f"""Use $implement for this agent-loop task.

{action}

Context pack: __PACK_PATH__

Read budget:
- Read the context pack first.
- Read only the required docs listed in the pack.
- Inspect files under Expected Write Scope before broad exploration.
- Use `rg` before opening broad files.
- Do not paste long file contents or command output in the final report.

Work rules:
- Stay inside the slice scope.
- Do not revert unrelated edits.
- Run focused checks first, then final checks from the pack.
- Open or update the linked PR if implementation changes are made.
- Do not merge.

Return only this report shape plus concise notes when needed:

```json
{{
  "schemaVersion": 1,
  "workerKind": "{template}",
  "status": "ready-to-merge",
  "parentPrd": null,
  "sliceIssue": null,
  "pr": null,
  "branch": null,
  "concurrencyGroup": null,
  "startedAt": null,
  "finishedAt": null,
  "filesChanged": [],
  "checks": [],
  "risks": [],
  "blockedOn": [],
  "failureCategory": null,
  "reworkNeeded": false,
  "confidence": "medium"
}}
```
"""


def model_policy_header(args: argparse.Namespace) -> str:
    fields = [
        ("Agent profile", args.agent_profile),
        (
            "Model policy",
            " / ".join(
                value
                for value in [args.task_class, args.model, args.model_reasoning_effort]
                if value
            ),
        ),
        ("Escalation", args.escalation_rule),
    ]
    rendered = [f"{label}: {value}" for label, value in fields if value]
    if not rendered:
        return ""
    return "\n".join(rendered) + "\n\n"


def cmd_build_pack(args: argparse.Namespace) -> None:
    text, estimate = build_pack(args.repo, args.slice, args.method)
    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    path = out_dir / f"issue-{args.slice}-worker-pack.md"
    path.write_text(text, encoding="utf-8")
    print(json.dumps({"ok": True, "path": str(path), "tokenEstimate": estimate}, indent=2, sort_keys=True))


def cmd_render_prompt(args: argparse.Namespace) -> None:
    rendered = model_policy_header(args) + prompt_template(args.template).replace("__PACK_PATH__", args.pack)
    if args.output:
        Path(args.output).write_text(rendered, encoding="utf-8")
        print(json.dumps({"ok": True, "path": args.output}, indent=2, sort_keys=True))
    else:
        print(rendered, end="")


def cmd_estimate(args: argparse.Namespace) -> None:
    text = Path(args.path).read_text(encoding="utf-8")
    print(json.dumps({"ok": True, **diagnostics.estimate_tokens(text, args.method)}, indent=2, sort_keys=True))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(required=True)

    build_pack_cmd = sub.add_parser("build-pack", help="Build a local worker context pack")
    build_pack_cmd.add_argument("--repo", required=True)
    build_pack_cmd.add_argument("--slice", type=int, required=True)
    build_pack_cmd.add_argument("--out-dir", default=".agent-loop/context")
    build_pack_cmd.add_argument("--method", choices=["auto", "char-div-4", "tiktoken"], default="auto")
    build_pack_cmd.set_defaults(func=cmd_build_pack)

    render = sub.add_parser("render-prompt", help="Render a short worker prompt from a context pack")
    render.add_argument("--pack", required=True)
    render.add_argument("--template", choices=sorted(VALID_TEMPLATES), required=True)
    render.add_argument("--output")
    render.add_argument("--task-class")
    render.add_argument("--agent-profile")
    render.add_argument("--model")
    render.add_argument("--model-reasoning-effort")
    render.add_argument("--escalation-rule")
    render.set_defaults(func=cmd_render_prompt)

    estimate = sub.add_parser("estimate", help="Estimate tokens for a local file")
    estimate.add_argument("--path", required=True)
    estimate.add_argument("--method", choices=["auto", "char-div-4", "tiktoken"], default="auto")
    estimate.set_defaults(func=cmd_estimate)

    return parser


def main() -> None:
    args = build_parser().parse_args()
    try:
        args.func(args)
    except (OSError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error


if __name__ == "__main__":
    main()
