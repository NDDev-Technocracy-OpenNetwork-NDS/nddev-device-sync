set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# Initialize the exact public source selection.
checkout:
    git submodule update --init --recursive --jobs 2

# Preserve argument boundaries; each component owns its checks and build logic.
[positional-arguments]
component id *args:
    .tools/bin/nds-source component "$@"

cargo-cache-setup:
    python3 scripts/cargo-cache.py setup

cargo-cache-check:
    python3 scripts/cargo-cache.py check

check: source-check catalog-check documentation-check
    just --fmt --check

source-check:
    cd automation && cargo fmt --all -- --check
    cd automation && cargo deny --locked check licenses sources
    cd automation && cargo shear --locked --offline

catalog-check:
    .tools/bin/nds-source check

# Tooling acceptance includes real Cargo compilation in isolated fixtures.
test:
    cd automation && cargo nextest run --locked --profile ci
    cd automation && cargo test --locked --doc
    cd automation && cargo clippy --locked --all-targets -- -D warnings
    python3 -m unittest discover -s scripts -p 'test_*.py'

# Run only on an execution host; source machines use its verified artifact.
build-tools:
    cd automation && cargo run --locked -- --root .. install-tool

verify: check test
    cd automation && cargo build --locked --release

# The full check publishes the exact executable and its source/binary receipt.
source-artifact:
    mkdir -p .nddev-reports/source-tool
    cp .tools/bin/nds-source .nddev-reports/source-tool/
    .tools/bin/nds-source source-info > .nddev-reports/source-tool/source-info.json
    git rev-parse HEAD > .nddev-reports/source-tool/source-commit.txt
    cp automation/LICENSE automation/NOTICE .nddev-reports/source-tool/
    cd .nddev-reports/source-tool && sha256sum nds-source source-info.json source-commit.txt LICENSE NOTICE > SHA256SUMS

# Check the product entry points without compiling components.
documentation-check:
    test -s docs/architecture.md
    test -s docs/integration.md
    test -s docs/development.md
    test -s docs/status.md
    test -s LICENSE

# Download dependencies before the offline source check. This does not build.
setup:
    cd automation && cargo fetch --locked
