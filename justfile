set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# Public exact Gitlinks only; no private governance or Estate credentials needed.
checkout:
    git submodule update --init --recursive --jobs 2

# Preserve argument boundaries; each component owns its checks and build logic.
[positional-arguments]
component id *args:
    .venv/bin/python scripts/component.py "$@"

cargo-cache-setup:
    python3 scripts/cargo-cache.py setup

cargo-cache-check:
    python3 scripts/cargo-cache.py check

check: standards-check catalog-check

catalog-check:
    .venv/bin/python scripts/check-contracts.py
    .venv/bin/python -m unittest discover -s scripts -p 'test_*.py'

# Optional prepared native GDS CLI; no private Estate or device access.
gds-check:
    gds --json validate repository
    gds --json validate gitlinks

standards-check: catalog-check
    test -s standarts/README.md
    test -s LICENSE

# Isolated structural tooling; runtime repositories retain their own gates.
setup:
    python3 -m venv .venv
    .venv/bin/python -m pip install -r requirements-check.txt
