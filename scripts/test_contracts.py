"""Regression cases for catalog/descriptor drift hidden by JSON syntax checks."""
import copy
import configparser
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest

from jsonschema import ValidationError

spec = importlib.util.spec_from_file_location('contracts', Path(__file__).with_name('check-contracts.py'))
contracts = importlib.util.module_from_spec(spec)
spec.loader.exec_module(contracts)


class Contracts(unittest.TestCase):
    def test_native_browser_and_descriptor_boundaries(self):
        value = {'id': 'clipboard', 'version': '0.0.1-alpha.3', 'api_version': 1,
                 'kind': 'tool', 'platforms': ['android', 'ios', 'web'],
                 'capabilities': ['share_clipboard'], 'permissions': ['clipboard'],
                 'dependencies': []}
        contracts.descriptor(value)
        for patch in [
            {'id': 'bad/id'}, {'api_version': 0}, {'api_version': 65536},
            {'platforms': ['android', 'android']}, {'platforms': ['invented']},
            {'capabilities': ['invented']}, {'permissions': ['root']},
            {'version': '   '}, {'dependencies': [{'id': 'core', 'minimum_api': 0}]},
            {'dependencies': [{'id': 'clipboard', 'minimum_api': 1}]},
            {'dependencies': [{'id': 'core', 'minimum_api': 1}, {'id': 'core', 'minimum_api': 2}]},
        ]:
            with self.subTest(patch=patch), self.assertRaises((ValueError, ValidationError)):
                contracts.descriptor(value | patch)

    def test_browser_control_is_a_distinct_capability_and_permission(self):
        contracts.descriptor({'id': 'browser-use', 'version': '0.0.1-alpha.1', 'api_version': 1,
                              'kind': 'integration', 'platforms': ['linux'],
                              'capabilities': ['control_browser'], 'permissions': ['browser_control'],
                              'dependencies': []})
        with self.assertRaises(ValidationError):
            contracts.descriptor({'id': 'browser-use', 'version': '0.0.1-alpha.1', 'api_version': 1,
                                  'kind': 'integration', 'platforms': ['linux'],
                                  'capabilities': ['control_browser'], 'permissions': ['browser'],
                                  'dependencies': []})

    def test_catalog_rejects_ambiguous_assembly(self):
        original = contracts.load(contracts.ROOT / 'contracts/module-catalog.json')
        for mutate in [
            lambda d: d['components'].append(d['components'][0]),
            lambda d: d['components'][0].update(repository='nddev-unrelated'),
            lambda d: d['components'][0].update(platforms=['all', 'linux']),
            lambda d: d['components'][0].update(status='implemented'),
            lambda d: d['components'][0].update(path='tools/core'),
            lambda d: d['components'][0].update(path='../core'),
            lambda d: d['components'][0].update(path=None),
            lambda d: next(x for x in d['components'] if x['id'] == 'estate').update(path='modules/estate'),
        ]:
            value = copy.deepcopy(original)
            mutate(value)
            with self.assertRaises((ValueError, ValidationError)):
                contracts.catalog(value)


class Assembly(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory(prefix='nds-assembly-')
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        self.data = contracts.load(contracts.ROOT / 'contracts/module-catalog.json')
        self.data['components'] = [x for x in self.data['components']
                                   if x['id'] in {'core', 'clipboard', 'estate'}]
        self.git('init', '--quiet')
        sections = []
        for item in self.data['components']:
            if item['path'] is None:
                continue
            sections.append(f'[submodule "{item["id"]}"]\npath = {item["path"]}\n'
                            f'url = https://github.com/{self.data["organization"]}/{item["repository"]}.git\n')
            self.git('update-index', '--add', '--cacheinfo', f'160000,{"a" * 40},{item["path"]}')
        self.modules = self.root / '.gitmodules'
        self.modules.write_text('\n'.join(sections))

    def git(self, *args):
        return subprocess.check_output(['git', *args], cwd=self.root)

    def test_uninitialized_children_validate_without_network_or_private_access(self):
        contracts.catalog(self.data)
        contracts.assembly(self.root, self.data)
        self.assertFalse((self.root / 'modules/core').exists())

    def test_wrong_remote_extra_private_link_and_duplicate_section_are_rejected(self):
        original = self.modules.read_text()
        for value in [original.replace('github.com/', 'example.invalid/'),
                      original + '[submodule "estate"]\npath = modules/estate\nurl = private\n',
                      original + '[submodule "core"]\npath = modules/core\n']:
            with self.subTest(value=value):
                self.modules.write_text(value)
                with self.assertRaises((ValueError, configparser.Error)):
                    contracts.assembly(self.root, self.data)

    def test_missing_gitlink_and_copied_component_source_are_rejected(self):
        self.git('update-index', '--force-remove', 'modules/core')
        with self.assertRaises(ValueError):
            contracts.assembly(self.root, self.data)
        self.git('update-index', '--add', '--cacheinfo', f'160000,{"a" * 40},modules/core')
        blob = subprocess.check_output(['git', 'hash-object', '-w', '--stdin'],
                                       input=b'owned synthetic source', cwd=self.root).decode().strip()
        self.git('update-index', '--add', '--cacheinfo', f'100644,{blob},modules/copied.rs')
        with self.assertRaises(ValueError):
            contracts.assembly(self.root, self.data)

    def test_symlinked_gitmodules_is_rejected(self):
        content = self.modules.read_text()
        self.modules.unlink()
        (self.root / 'other').write_text(content)
        self.modules.symlink_to('other')
        with self.assertRaises(ValueError):
            contracts.assembly(self.root, self.data)
