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
    def test_application_manifest_binds_sources_and_distinct_data_formats(self):
        value = {'schema': 1, 'application': 'nds', 'version': '1.0.0-alpha.2',
                 'platform': 'linux', 'architecture': 'x86_64', 'channel': 'alpha',
                 'sources': [{'role': 'desktop', 'repository': 'https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-desktop',
                              'commit': 'a' * 40}],
                 'data': {'vault_journal': 8, 'sync_wire': 3, 'accounts_fence': 1},
                 'artifact': {'url': 'https://example.invalid/app.bin', 'bytes': 17,
                              'sha256': 'b' * 64, 'app_source': 'a' * 40, 'native_source': 'a' * 40}}
        contracts.app_release(value)
        # Distinct compiled package roles can pin distinct commits at one URI.
        shared_repository = copy.deepcopy(value)
        shared_repository['sources'].append(value['sources'][0] | {'role': 'adapter-io', 'commit': 'c' * 40})
        contracts.app_release(shared_repository)
        version_schema = contracts.load(contracts.ROOT / 'contracts/app-release.schema.json')['properties']['version']
        from jsonschema import Draft202012Validator
        version_validator = Draft202012Validator(version_schema)
        for version in ['1.0.0-alpha.1', '1.0.0-alpha.4294967295']:
            version_validator.validate(version)
        for version in ['1.0.0-alpha.0', '1.0.0-alpha.01', '1.0.0-alpha.4294967296', '1.0.0-alpha.9999999999', '1.0.0-alpha.1\n']:
            with self.subTest(version=version), self.assertRaises(ValidationError):
                version_validator.validate(version)
        for mutate in [
            lambda d: d.update(version='1.0.0-alpha.4294967296'),
            lambda d: d.update(channel='release'),
            lambda d: d.update(platform='windows'),
            lambda d: d['sources'].append(d['sources'][0] | {'commit': 'c' * 40}),
            lambda d: d['artifact'].update(native_source='c' * 40),
            lambda d: d['artifact'].update(bytes=1073741825),
            lambda d: d['artifact'].update(url='file:///tmp/app'),
            lambda d: d['data'].update(schema=3),
            lambda d: d.update(publisher={'issuer': 'candidate-selected'}),
        ]:
            changed = copy.deepcopy(value)
            mutate(changed)
            with self.subTest(changed=changed), self.assertRaises((ValueError, ValidationError)):
                contracts.app_release(changed)

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
            lambda d: d['components'][0].update(visibility='private'),
            lambda d: d['components'][0].update(kind='engineering-governance'),
            lambda d: next(x for x in d['components'] if x['id'] == 'estate').update(path='modules/estate'),
            lambda d: next(x for x in d['components'] if x['id'] == 'standards').update(visibility='public'),
            lambda d: next(x for x in d['components'] if x['id'] == 'organization').update(repository='nddev-device-sync-organization'),
        ]:
            value = copy.deepcopy(original)
            mutate(value)
            with self.assertRaises((ValueError, ValidationError)):
                contracts.catalog(value)

    def test_private_contexts_are_not_public_application_sources(self):
        data = contracts.load(contracts.ROOT / 'contracts/module-catalog.json')
        self.assertEqual({item['id'] for item in data['components'] if item['visibility'] == 'private'},
                         {'estate', 'standards'})
        value = {'schema': 1, 'application': 'nds', 'version': '1.0.0-alpha.2',
                 'platform': 'linux', 'architecture': 'x86_64', 'channel': 'alpha',
                 'sources': [{'role': 'desktop', 'repository': 'https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-desktop', 'commit': 'a' * 40}],
                 'data': {'vault_journal': 8, 'sync_wire': 3, 'accounts_fence': 1},
                 'artifact': {'url': 'https://example.invalid/app.bin', 'bytes': 17,
                              'sha256': 'b' * 64, 'app_source': 'a' * 40, 'native_source': 'a' * 40}}
        for item in data['components']:
            if item['visibility'] == 'private':
                changed = copy.deepcopy(value)
                changed['sources'].append({'role': item['id'], 'repository': f'https://github.com/{data["organization"]}/{item["repository"]}', 'commit': 'c' * 40})
                with self.assertRaises(ValueError):
                    contracts.app_release(changed)


class Assembly(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory(prefix='nds-assembly-')
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        self.data = contracts.load(contracts.ROOT / 'contracts/module-catalog.json')
        self.data['components'] = [x for x in self.data['components']
                                   if x['id'] in {'core', 'clipboard', 'estate', 'standards'}]
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
