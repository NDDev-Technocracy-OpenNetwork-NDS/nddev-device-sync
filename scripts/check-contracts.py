#!/usr/bin/env python3
"""Validate owned schemas/catalog; optionally validate actual runtime descriptors."""
import argparse
import configparser
import json
from pathlib import Path
import re
import subprocess
from urllib.parse import urlsplit

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
        expected = {'design-system': 'nddev-opennetwork-design-system',
                    'organization': '.github'}.get(module['id'], 'nddev-device-sync-' + module['id'])
        if module['repository'] != expected:
            raise ValueError('repository does not match its module boundary')
        private_kinds = {'estate': 'deployment-configuration', 'standards': 'engineering-governance'}
        if module['id'] in private_kinds:
            if module['visibility'] != 'private' or module['kind'] != private_kinds[module['id']]:
                raise ValueError('private context has a different ownership boundary')
        elif module['visibility'] != 'public' or module['kind'] in private_kinds.values():
            raise ValueError('product components must remain public')
        if (module['id'] == 'organization') != (module['kind'] == 'organization-profile'):
            raise ValueError('organization profile has a different ownership boundary')
        expected_path = (None if module['visibility'] == 'private' else
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
    sections = {f'submodule "{item["path"]}"' for item in expected.values()}
    if config.defaults() or set(config.sections()) != sections:
        raise ValueError('Git submodules must match the public catalog exactly')
    for location, item in expected.items():
        section = config[f'submodule "{item["path"]}"']
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


def app_release(data, development_http=False):
    """Structural candidate checks, never publisher or runtime compatibility proof."""
    validator('app-release.schema.json').validate(data)
    candidate = int(data['version'].removeprefix('1.0.0-alpha.'))
    if candidate > 4294967295:
        raise ValueError('application candidate exceeds its unsigned 32-bit bound')
    sources = {item['role']: item for item in data['sources']}
    if len(sources) != len(data['sources']):
        raise ValueError('duplicate application source role')
    source_catalog = load(ROOT / 'contracts/module-catalog.json')
    private_sources = {(source_catalog['organization'].lower(), entry['repository'].lower())
                       for entry in source_catalog['components'] if entry['visibility'] == 'private'}
    for item in sources.values():
        repository = item['repository']
        owner, name = repository.removeprefix('https://github.com/').split('/')
        if owner in {'.', '..'} or name in {'.', '..'}:
            raise ValueError('invalid application source repository')
        if (owner.lower(), name.removesuffix('.git').lower()) in private_sources:
            raise ValueError('private context is not a public application source')
    desktop = 'https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-desktop'
    artifact = data['artifact']
    if sources.get('desktop', {}).get('repository') != desktop or sources.get('desktop', {}).get('commit') != artifact['app_source'] or artifact['native_source'] != artifact['app_source']:
        raise ValueError('Linux wrapper/native sources must match their Desktop source')
    url = urlsplit(artifact['url'])
    if url.username is not None or url.password is not None or url.fragment:
        raise ValueError('artifact URL must not contain credentials or a fragment')
    if url.scheme != 'https' and not (development_http and url.scheme == 'http' and
            url.hostname == '127.0.0.1' and url.port is not None and
            url.netloc == f'127.0.0.1:{url.port}' and 1024 <= url.port <= 65535):
        raise ValueError('HTTP is restricted to explicitly selected development loopback')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('runtime_manifests', nargs='*', type=Path)
    parser.add_argument('--app-release', type=Path,
                        help='validate a candidate before signing; does not verify a signature')
    parser.add_argument('--development-http', action='store_true', help='allow owned literal 127.0.0.1 HTTP qualification only')
    args = parser.parse_args()
    data = load(ROOT / 'contracts/module-catalog.json')
    catalog(data)
    assembly(ROOT, data)
    validator('module-manifest.json')
    validator('app-release.schema.json')
    if args.app_release is not None:
        app_release(load(args.app_release), args.development_http)
    for path in args.runtime_manifests:
        descriptor(load(path))
    print(f'Catalog and schemas valid; {len(args.runtime_manifests)} runtime descriptors checked.')


if __name__ == '__main__':
    main()
