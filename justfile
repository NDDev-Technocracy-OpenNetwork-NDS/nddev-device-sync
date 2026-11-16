set shell := ["bash", "-euo", "pipefail", "-c"]

# Lists the source-root commands.
default:
    @just --list

# Initialize the exact public source selection.
checkout:
    git -c credential.helper= -c protocol.file.allow=never submodule update --init --recursive --jobs 2

# Preserve argument boundaries; each component owns its checks and build logic.
[positional-arguments]
component id *args:
    .venv/bin/python scripts/component.py "$@"

# Prepares the selected local Cargo cache bindings.
cargo-cache-setup:
    python3 scripts/cargo-cache.py setup

# Checks the existing Cargo cache bindings.
cargo-cache-check:
    python3 scripts/cargo-cache.py check

# Checks owned source contracts and documentation without compilation.
check: catalog-check documentation-check
    just --fmt --check

# Validates the source catalog, Gitlinks and owned schemas.
catalog-check:
    .venv/bin/python scripts/check-contracts.py

# Exercises real Cargo fixtures and credential-free recursive source checkout.
test:
    .venv/bin/python -m unittest discover -s scripts -p 'test_*.py'
    just public-checkout-check

# Check the product entry points without compiling components.
documentation-check:
    test -s docs/architecture.md
    test -s docs/integration.md
    test -s docs/development.md
    test -s docs/status.md
    test -s LICENSE

# Isolated structural tooling; runtime repositories retain their own gates.
setup:
    python3 -m venv .venv
    .venv/bin/python -m pip install -r requirements-check.txt

# Requires initialized recursive source to match every selected Gitlink.
[group("source")]
public-checkout-check: checkout
    .venv/bin/python scripts/check-contracts.py --initialized

# Runs the source root's complete existing quality contract.
verify: check test
