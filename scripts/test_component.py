"""Native Git and just dispatch, entirely inside an owned disposable source tree."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

SOURCE = Path(__file__).resolve().parents[1]


class Component(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='nds-component-')
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        for folder in ('scripts', 'contracts', '.venv/bin'):
            (self.root / folder).mkdir(parents=True)
        for path in ('scripts/component.py', 'scripts/check-contracts.py',
                     'contracts/module-catalog.schema.json', 'justfile'):
            shutil.copyfile(SOURCE / path, self.root / path)
        (self.root / '.venv/bin/python').symlink_to(sys.executable)
        catalog = json.loads((SOURCE / 'contracts/module-catalog.json').read_text())
        catalog['components'] = [item for item in catalog['components'] if item['id'] in {'core', 'estate', 'standards'}]
        (self.root / 'contracts/module-catalog.json').write_text(json.dumps(catalog))
        self.git(self.root, 'init', '--quiet')
        self.child = self.root / 'modules/core'
        self.child.mkdir(parents=True)
        self.git(self.child, 'init', '--quiet')
        self.git(self.child, 'remote', 'add', 'origin', 'https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-core.git')
        (self.child / 'justfile').write_text('check:\n    exit 7\n')
        self.git(self.child, 'add', 'justfile')
        self.git(self.child, '-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
                 '-c', 'commit.gpgsign=false', 'commit', '-qm', 'fixture')
        self.commit = self.git(self.child, 'rev-parse', 'HEAD').strip()
        self.git(self.root, 'update-index', '--add', '--cacheinfo', f'160000,{self.commit},modules/core')
        (self.root / '.gitmodules').write_text('[submodule "core"]\npath = modules/core\n'
            'url = https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-core.git\n')

    def git(self, cwd, *args):
        return subprocess.check_output(['git', *args], cwd=cwd, text=True,
                                       env=dict(os.environ, GIT_CONFIG_GLOBAL=os.devnull, GIT_CONFIG_NOSYSTEM='1'))

    def call(self, identity='core', *args, environment=None):
        return subprocess.run([sys.executable, str(self.root / 'scripts/component.py'), identity, *args],
                              cwd=self.root, env=environment, text=True, capture_output=True, timeout=15)

    def test_real_just_recipe_preserves_arguments_cwd_and_exit(self):
        just = shutil.which('just')
        self.assertIsNotNone(just, 'the canonical check requires its pinned just on PATH')
        stub = self.root / 'bin'
        stub.mkdir()
        program = stub / 'just'
        program.write_text(f'#!{sys.executable}\nimport json,os,sys\n'
            'from pathlib import Path\nPath(os.environ["FIXTURE_OUTPUT"]).write_text(json.dumps([os.getcwd(), sys.argv[1:]]))\nsys.exit(7)\n')
        program.chmod(0o700)
        out = self.root / 'arguments.json'
        env = dict(os.environ, PATH=str(stub) + os.pathsep + os.environ['PATH'], FIXTURE_OUTPUT=str(out))
        args = ['check', 'two words', '$(never-run)', '; literal']
        result = subprocess.run([just, 'component', 'core', *args], cwd=self.root, env=env,
                                text=True, capture_output=True, timeout=15)
        self.assertEqual(result.returncode, 7, result.stderr)
        cwd, argv = json.loads(out.read_text())
        self.assertEqual(cwd, str(self.root))
        self.assertEqual(argv, ['--working-directory', str(self.child), '--justfile', str(self.child / 'justfile'), *args])

    def test_private_unknown_uninitialized_and_wrong_source_are_refused(self):
        for identity in ('estate', 'standards', 'missing'):
            self.assertNotEqual(self.call(identity).returncode, 0)
        self.git(self.child, 'remote', 'set-url', 'origin', 'https://example.invalid/foreign.git')
        self.assertNotEqual(self.call().returncode, 0)
        self.git(self.child, 'remote', 'set-url', 'origin', 'https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-core.git')
        self.git(self.root, 'update-index', '--cacheinfo', f'160000,{"a" * 40},modules/core')
        self.assertNotEqual(self.call().returncode, 0)
        shutil.rmtree(self.child)
        self.child.mkdir()
        self.assertNotEqual(self.call().returncode, 0)
