#!/usr/bin/env python3
from pathlib import Path
import runpy
import sys

SCRIPT = Path("/home/moenarch/.codex/skills/setup-agent-loop-skills/scripts/sync_agent_loop_labels.py")

if not SCRIPT.is_file():
    raise SystemExit(f"Missing agent-loop label sync script: {SCRIPT}")

sys.argv[0] = str(SCRIPT)
runpy.run_path(str(SCRIPT), run_name="__main__")
