#!/usr/bin/env python3
"""Run a component's own just recipes at the root's selected public source."""
import argparse
import importlib.util
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def selected(root, identity):
    spec = importlib.util.spec_from_file_location('contracts', root / 'scripts/check-contracts.py')
    contracts = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(contracts)
    data = contracts.load(root / 'contracts/module-catalog.json')
    contracts.catalog(data)
    contracts.assembly(root, data)
    item = next((item for item in data['components'] if item['id'] == identity), None)
    if item is None or item['visibility'] != 'public':
        raise ValueError('select a public component from the catalog')
    child = root / item['path']
    if any(parent.is_symlink() for parent in (child, child.parent)) or not (child / '.git').exists():
        raise ValueError('initialize the selected public submodule with just checkout')

    def git(*arguments):
        return subprocess.check_output(['git', *arguments], cwd=child, text=True, timeout=30).strip()

    if Path(git('rev-parse', '--show-toplevel')).resolve() != child.resolve():
        raise ValueError('component does not own its Git checkout')
    remote = git('remote', 'get-url', 'origin').removesuffix('.git')
    remote = remote.removeprefix('https://').replace('git@github.com:', 'github.com/')
    if remote.lower() != f'github.com/{data["organization"]}/{item["repository"]}'.lower():
        raise ValueError('component provider identity differs from the catalog')
    indexed = subprocess.check_output(['git', 'ls-files', '--stage', '--', item['path']],
                                      cwd=root, text=True, timeout=30).split()[1]
    if git('rev-parse', 'HEAD') != indexed:
        raise ValueError('component HEAD differs from the selected Gitlink')
    if not (child / 'justfile').is_file():
        raise ValueError('this source-only component has no just recipes')
    return child


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('id')
    parser.add_argument('recipes', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    child = selected(ROOT, args.id)
    return subprocess.call(['just', '--working-directory', str(child),
                            '--justfile', str(child / 'justfile'), *args.recipes])


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, OSError, subprocess.SubprocessError) as error:
        sys.exit(f'NDS component: {error}')
