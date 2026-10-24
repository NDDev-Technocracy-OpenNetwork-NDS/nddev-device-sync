"""Actual isolated Cargo/configuration acceptance; no operator checkout changes."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

SOURCE = Path(__file__).with_name("cargo-cache.py")


class CargoCache(unittest.TestCase):
    def setUp(self):
        # A caller may itself run inside the shared-cache runner. The fixture
        # must exercise its own configuration, never populate that caller's lane.
        self.environment = {key: value for key, value in os.environ.items()
                            if key not in {"CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR", "CARGO_BUILD_BUILD_DIR", "NDS_CACHE_FRONTENDS", "NDS_CLEANER_CONFIG"}}
        self.directory = tempfile.TemporaryDirectory(prefix="nds-cache-check-")
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.scope = self.root / "nds-checkouts"
        self.project = self.scope / "nddev-device-sync"
        (self.project / "scripts").mkdir(parents=True)
        (self.project / "contracts").mkdir()
        shutil.copyfile(SOURCE, self.project / "scripts/cargo-cache.py")
        (self.project / "rust-toolchain.toml").write_text('[toolchain]\nchannel="1.99.1"\n')
        names = ["nddev-device-sync-core", "nddev-device-sync-server"]
        (self.project / "contracts/module-catalog.json").write_text(json.dumps({
            "product": "nddev-device-sync", "organization": "NDDev-Technocracy-OpenNetwork-NDS",
            "modules": [{"repository": name} for name in names],
        }))
        for name in ["nddev-device-sync", *names]:
            directory = self.scope / name
            directory.mkdir(exist_ok=True)
            self.command(["git", "init", "--quiet"], directory)
            self.command(["git", "remote", "add", "origin",
                          f"https://github.com/NDDev-Technocracy-OpenNetwork-NDS/{name}.git"], directory)

    def command(self, args, cwd=None, check=True):
        return subprocess.run(args, cwd=cwd or self.project, capture_output=True, text=True,
                              timeout=60, check=check, env=dict(self.environment, GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull))

    def helper(self, *args, cwd=None, check=True):
        return self.command([sys.executable, str(self.project / "scripts/cargo-cache.py"), *args], cwd, check)

    def setup_cache(self):
        return json.loads(self.helper("setup", "--cache-root", str(self.root / "cache")).stdout)

    def test_setup_is_idempotent_and_preserves_unmanaged_configuration(self):
        result = self.setup_cache()
        config = Path(result["configuration"])
        before = config.read_bytes(), config.stat().st_mtime_ns
        self.assertEqual(self.setup_cache(), result)
        self.assertEqual((config.read_bytes(), config.stat().st_mtime_ns), before)
        config.write_bytes(before[0] + b'\n[net]\noffline=true\n')
        extended = config.read_bytes()
        failed = self.helper("setup", check=False)
        self.assertNotEqual(failed.returncode, 0)
        self.assertEqual(config.read_bytes(), extended)
        config.write_text('[build]\njobs=2\n')
        failed = self.helper("setup", check=False)
        self.assertNotEqual(failed.returncode, 0)
        self.assertEqual(config.read_text(), '[build]\njobs=2\n')

    def test_configuration_cannot_cover_an_unregistered_directory(self):
        (self.scope / "unrelated-project").mkdir()
        failed = self.helper("setup", check=False)
        self.assertNotEqual(failed.returncode, 0)
        self.assertFalse((self.scope / ".cargo/config.toml").exists())

    def test_actual_workspaces_share_artifacts_and_allow_isolated_targets(self):
        core = self.scope / "nddev-device-sync-core"
        server = self.scope / "nddev-device-sync-server"
        common = self.project / "common"
        (common / "src").mkdir(parents=True)
        (common / "Cargo.toml").write_text('[package]\nname="nds_cache_common"\nversion="0.0.0"\nedition="2024"\n')
        (common / "src/lib.rs").write_text('pub fn value() -> u8 { 7 }\n')
        for directory, name, dependency in [(core, "cache_core", "../nddev-device-sync/common"), (server, "cache_server", "../nddev-device-sync/common")]:
            (directory / "src").mkdir(exist_ok=True)
            (directory / "Cargo.toml").write_text(f'[package]\nname="{name}"\nversion="0.0.0"\nedition="2024"\n[dependencies]\nnds_cache_common={{path="{dependency}"}}\n')
            (directory / "src/lib.rs").write_text('pub fn value() -> u8 { nds_cache_common::value() }\n')
        configured = self.setup_cache()
        for directory in [core, server]:
            metadata = json.loads(self.helper("run", "--", "cargo", "metadata", "--offline", "--no-deps", "--format-version=1", cwd=directory).stdout)
            self.assertEqual(metadata["target_directory"], configured["target_directory"])
        receipts = []
        for directory in [core, server]:
            result = self.helper("run", "--", "cargo", "check", "--offline", "--message-format=json", cwd=directory)
            artifacts = [json.loads(line) for line in result.stdout.splitlines()]
            receipts.append(next(row for row in artifacts if row.get("reason") == "compiler-artifact" and row["target"]["name"] == "nds_cache_common"))
        self.assertFalse(receipts[0]["fresh"])
        self.assertTrue(receipts[1]["fresh"])
        self.assertEqual(receipts[0]["filenames"], receipts[1]["filenames"])
        for variable in ["CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR"]:
            with self.subTest(variable=variable):
                isolated_target = self.root / variable
                isolated_build = self.root / (variable + "-build")
                self.environment.pop("CARGO_TARGET_DIR", None)
                self.environment.pop("CARGO_BUILD_TARGET_DIR", None)
                self.environment[variable] = str(isolated_target)
                self.environment["CARGO_BUILD_BUILD_DIR"] = str(isolated_build)
                metadata = json.loads(self.helper("run", "--", "cargo", "metadata", "--offline", "--no-deps", "--format-version=1", cwd=server).stdout)
                self.assertEqual(metadata["target_directory"], str(isolated_target))
                self.assertEqual(metadata["build_directory"], str(isolated_build))


if __name__ == "__main__":
    unittest.main()
