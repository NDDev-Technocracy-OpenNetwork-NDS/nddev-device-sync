#!/usr/bin/env python3
"""Validate owned schemas/catalog; optionally validate actual runtime descriptors."""
import argparse
import configparser
import json
from pathlib import Path
import re
import subprocess

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]


def load(path):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError(f"duplicate JSON property: {key}")
            result[key] = value
        return result
    return json.loads(path.read_text(), object_pairs_hook=unique)


def validator(name):
    schema = load(ROOT / 'contracts' / name)
    Draft202012Validator.check_schema(schema)
    return Draft202012Validator(schema)


def catalog(data):
    validator('module-catalog.schema.json').validate(data)
    for field in ['id', 'repository']:
        values = [module[field] for module in data['components']]
        if len(set(values)) != len(values):
            raise ValueError(f'duplicate catalog {field}')
    for module in data['components']:
        if 'all' in module['platforms'] and len(module['platforms']) != 1:
            raise ValueError('all cannot be combined with named platforms')
        expected = ('nddev-opennetwork-design-system' if module['id'] == 'design-system'
                    else 'nddev-device-sync-' + module['id'])
        if module['repository'] != expected:
            raise ValueError('repository does not match its module boundary')
        expected_path = (None if module['id'] == 'estate' else
                         ('tools/' if module['kind'] == 'tool' else 'modules/') + module['id'])
        if module['path'] != expected_path:
            raise ValueError('component path does not match its type or privacy boundary')


def assembly(root, data):
    """Check native Git metadata without cloning children or accessing private estate."""
    path = root / '.gitmodules'
    if path.is_symlink() or not path.is_file():
        raise ValueError('assembly requires a regular .gitmodules file')
    config = configparser.RawConfigParser(strict=True)
    config.read_string(path.read_text())
    expected = {item['path']: item for item in data['components'] if item['path'] is not None}
    sections = {f'submodule "{item["id"]}"' for item in expected.values()}
    if config.defaults() or set(config.sections()) != sections:
        raise ValueError('Git submodules must match the public catalog exactly')
    for location, item in expected.items():
        section = config[f'submodule "{item["id"]}"']
        url = f'https://github.com/{data["organization"]}/{item["repository"]}.git'
        if dict(section) != {'path': location, 'url': url}:
            raise ValueError('submodule path or provider identity differs from the catalog')
    index = subprocess.check_output(['git', 'ls-files', '--stage', '-z'], cwd=root).decode()
    links = {}
    for entry in index.split('\0'):
        if not entry:
            continue
        metadata, location = entry.split('\t', 1)
        mode, commit, stage = metadata.split()
        if stage != '0':
            raise ValueError('assembly index contains an unresolved merge')
        if mode == '160000':
            if not re.fullmatch(r'[0-9a-f]{40}', commit) or commit == '0' * 40:
                raise ValueError('submodule has no exact Git commit')
            links[location] = commit
        elif location.startswith(('modules/', 'tools/')):
            raise ValueError('component implementation must remain in its own repository')
    if set(links) != set(expected):
        raise ValueError('indexed Gitlinks do not match the public catalog')


def descriptor(data):
    validator('module-manifest.json').validate(data)
    dependencies = [dependency['id'] for dependency in data['dependencies']]
    if data['id'] in dependencies or len(set(dependencies)) != len(dependencies):
        raise ValueError('self or duplicate module dependency')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('runtime_manifests', nargs='*', type=Path)
    args = parser.parse_args()
    data = load(ROOT / 'contracts/module-catalog.json')
    catalog(data)
    assembly(ROOT, data)
    validator('module-manifest.json')
    for path in args.runtime_manifests:
        descriptor(load(path))
    print(f'Catalog and schemas valid; {len(args.runtime_manifests)} runtime descriptors checked.')


if __name__ == '__main__':
    main()
