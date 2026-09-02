#!/usr/bin/env python3
"""Hermetic lifecycle tests for scripts/source-deps."""

from __future__ import annotations

import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import textwrap
import unittest


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
SOURCE_DEPS = REPOSITORY_ROOT / "scripts" / "source-deps"
SOURCE_DEPS_CONFIG = REPOSITORY_ROOT / ".coding-tooling.source-deps.json"
BACKUP = Path(".cargo/source-deps-registry.Cargo.lock")


class SourceDepsLifecycleTest(unittest.TestCase):
    def setUp(self) -> None:
        self.tempdir = tempfile.TemporaryDirectory()
        self.addCleanup(self.tempdir.cleanup)
        self.root = Path(self.tempdir.name)
        (self.root / "scripts").mkdir()
        shutil.copy2(SOURCE_DEPS, self.root / "scripts/source-deps")
        shutil.copy2(SOURCE_DEPS_CONFIG, self.root / SOURCE_DEPS_CONFIG.name)
        self.registry_lock = b"registry lock\nwith exact bytes\n"
        (self.root / "Cargo.lock").write_bytes(self.registry_lock)
        subprocess.run(
            ["git", "init", "--quiet"], cwd=self.root, check=True
        )

        self.bin_dir = self.root / "fake-bin"
        self.bin_dir.mkdir()
        self._write_executable(
            "coding-tooling",
            """
            #!/usr/bin/env bash
            set -euo pipefail
            printf '%s\\n' "$*" >> "$FAKE_TOOL_LOG"
            action="${2:-}"
            case "$action" in
              activate)
                mkdir -p .cargo
                printf '%s\\n' '[patch.crates-io]' > .cargo/config.toml
                if [[ "${FAKE_ACTIVATE_FAIL:-0}" == 1 ]]; then
                  exit 23
                fi
                ;;
              deactivate)
                if [[ "${FAKE_DEACTIVATE_FAIL:-0}" == 1 ]]; then
                  exit 24
                fi
                rm -f .cargo/config.toml
                ;;
              status) ;;
              *) exit 25 ;;
            esac
            """,
        )
        self._write_executable(
            "cargo",
            """
            #!/usr/bin/env bash
            set -euo pipefail
            printf '%s\\n' "$*" >> "$FAKE_CARGO_LOG"
            if [[ "${FAKE_CARGO_FAIL:-0}" == 1 ]]; then
              exit 26
            fi
            printf '%s\\n' 'source lock' > Cargo.lock
            """,
        )
        self.tool_log = self.root / "tool.log"
        self.cargo_log = self.root / "cargo.log"
        self.env = {
            **os.environ,
            "PATH": f"{self.bin_dir}:{os.environ['PATH']}",
            "FAKE_TOOL_LOG": str(self.tool_log),
            "FAKE_CARGO_LOG": str(self.cargo_log),
        }

    def _write_executable(self, name: str, contents: str) -> None:
        path = self.bin_dir / name
        path.write_text(textwrap.dedent(contents).lstrip(), encoding="utf-8")
        path.chmod(0o755)

    def run_source_deps(
        self, action: str, *, extra_env: dict[str, str] | None = None
    ) -> subprocess.CompletedProcess[str]:
        env = {**self.env, **(extra_env or {})}
        return subprocess.run(
            ["bash", "scripts/source-deps", action],
            cwd=self.root,
            env=env,
            text=True,
            capture_output=True,
            check=False,
        )

    def test_activate_is_idempotent_and_deactivate_restores_registry_lock(self) -> None:
        first = self.run_source_deps("activate")
        self.assertEqual(first.returncode, 0, first.stderr)
        self.assertEqual((self.root / BACKUP).read_bytes(), self.registry_lock)
        self.assertEqual((self.root / "Cargo.lock").read_text(), "source lock\n")
        self.assertTrue((self.root / ".cargo/config.toml").is_file())

        second = self.run_source_deps("activate")
        self.assertEqual(second.returncode, 0, second.stderr)
        self.assertEqual((self.root / BACKUP).read_bytes(), self.registry_lock)

        deactivate = self.run_source_deps("deactivate")
        self.assertEqual(deactivate.returncode, 0, deactivate.stderr)
        self.assertEqual((self.root / "Cargo.lock").read_bytes(), self.registry_lock)
        self.assertFalse((self.root / BACKUP).exists())
        self.assertFalse((self.root / ".cargo/config.toml").exists())

        packages = [
            patch["package"]
            for patch in json.loads(SOURCE_DEPS_CONFIG.read_text())["cargo"]["patches"]
        ]
        cargo_calls = self.cargo_log.read_text().splitlines()
        self.assertEqual(len(cargo_calls), 2)
        for call in cargo_calls:
            self.assertTrue(call.startswith("update "), call)
            for package in packages:
                self.assertIn(f"-p {package}", call)

    def test_cargo_failure_rolls_back_config_lock_and_backup(self) -> None:
        result = self.run_source_deps("activate", extra_env={"FAKE_CARGO_FAIL": "1"})

        self.assertNotEqual(result.returncode, 0)
        self.assertEqual((self.root / "Cargo.lock").read_bytes(), self.registry_lock)
        self.assertFalse((self.root / BACKUP).exists())
        self.assertFalse((self.root / ".cargo/config.toml").exists())
        self.assertEqual(
            self.tool_log.read_text().splitlines(),
            ["source-deps activate", "source-deps deactivate"],
        )

    def test_tool_failure_rolls_back_partial_activation(self) -> None:
        result = self.run_source_deps(
            "activate", extra_env={"FAKE_ACTIVATE_FAIL": "1"}
        )

        self.assertNotEqual(result.returncode, 0)
        self.assertEqual((self.root / "Cargo.lock").read_bytes(), self.registry_lock)
        self.assertFalse((self.root / BACKUP).exists())
        self.assertFalse((self.root / ".cargo/config.toml").exists())

    def test_failed_deactivation_preserves_recoverable_source_state(self) -> None:
        activate = self.run_source_deps("activate")
        self.assertEqual(activate.returncode, 0, activate.stderr)

        result = self.run_source_deps(
            "deactivate", extra_env={"FAKE_DEACTIVATE_FAIL": "1"}
        )

        self.assertNotEqual(result.returncode, 0)
        self.assertEqual((self.root / "Cargo.lock").read_text(), "source lock\n")
        self.assertEqual((self.root / BACKUP).read_bytes(), self.registry_lock)
        self.assertTrue((self.root / ".cargo/config.toml").is_file())


if __name__ == "__main__":
    unittest.main()
