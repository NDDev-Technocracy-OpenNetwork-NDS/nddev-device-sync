"""Regression cases for catalog/descriptor drift hidden by JSON syntax checks."""
import copy
import importlib.util
from pathlib import Path
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
            lambda d: d['modules'].append(d['modules'][0]),
            lambda d: d['modules'][0].update(repository='nddev-unrelated'),
            lambda d: d['modules'][0].update(platforms=['all', 'linux']),
            lambda d: d['modules'][0].update(status='implemented'),
        ]:
            value = copy.deepcopy(original)
            mutate(value)
            with self.assertRaises((ValueError, ValidationError)):
                contracts.catalog(value)
