set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

cargo-cache-setup:
    python3 scripts/cargo-cache.py setup

cargo-cache-check:
    python3 scripts/cargo-cache.py check

check: standards-check catalog-check

catalog-check:
    .venv/bin/python scripts/check-contracts.py
    .venv/bin/python -m unittest discover -s scripts -p 'test_*.py'

standards-check: catalog-check
    test -s standarts/README.md
    test -s LICENSE

# Isolated structural tooling; runtime repositories retain their own gates.
setup:
    python3 -m venv .venv
    .venv/bin/python -m pip install -r requirements-check.txt
