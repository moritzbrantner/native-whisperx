#!/usr/bin/env python3
from pathlib import Path
import runpy
import sys

SCRIPT = Path("/home/moenarch/.codex/skills/agent-loop/scripts/agent_loop_diagnostics.py")

if not SCRIPT.is_file():
    raise SystemExit(f"Missing agent-loop helper script: {SCRIPT}")

sys.argv[0] = str(SCRIPT)
runpy.run_path(str(SCRIPT), run_name="__main__")
