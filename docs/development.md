# Development

The root selects public component source. Each component owns its build,
dependencies and acceptance commands.

```sh
git clone --recurse-submodules https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync.git
cd nddev-device-sync
just check
```

`just check` validates the Rust source, catalog, schemas, exact Gitlinks and
documentation without compilation or network access. It requires the pinned
Rust tools and a verifier prepared from these exact sources. On an execution
host, run `just setup build-tools` once; `setup` downloads dependencies and
`build-tools` compiles and installs the verifier in this checkout's `.tools/`.

A source machine takes `nds-source-tool` from the successful full CI run of the
selected source commit. Check that the run belongs to this repository and commit,
verify `SHA256SUMS`, and compare `source-commit.txt`. Run the downloaded executable
with `--root /path/to/checkout install-tool`. It checks its embedded digest of
the Rust source, lock, configuration and schemas before installing. An older
verifier refuses changed inputs, and installation preserves unrelated files.
The artifact includes the executable, source/binary receipt and license notices.

`just test` exercises the compiled CLI through actual Git and just, plus the
existing isolated Cargo cache fixtures. `just verify` also builds the release
tool. These commands require an execution host with the pinned compiler, Python
and just; they never run on a source-only machine. CI also verifies recursive
public checkout. Component runtime acceptance stays with its owning repository.

## Working with components

`just checkout` initializes the exact recursive source selection. Selective
checkout is ordinary Git:

```sh
git submodule update --init -- modules/github tools/cleaner
just component github --list
```

`just component <id> <recipe> ...` preserves argument boundaries and refuses a
different repository or revision from the root's selected Gitlink. For work on a
different component revision, invoke its own justfile directly. The dispatcher
does not reset a working checkout.

Source Gitlinks and compiled dependency locks select different things. A newer
component main branch is not automatically a compatible application assembly.
Update the affected consumers together and verify the resulting package graph.
Issued releases and their dependency/provenance snapshots remain immutable.

## Contract tooling

`contracts/module-catalog.json` lists the public source components. Catalog
revision 4 omits non-product contexts and requires every entry to have a public
source path. It changes no runtime descriptor or application release version.

`contracts/module-manifest.json` validates the current compiled-in descriptors:

```sh
.tools/bin/nds-source check /path/to/component/contracts/module.json
```

`contracts/app-release.schema.json` describes application candidates. Source
roles may select different revisions at the same public repository. Structural
validation is separate from publisher verification, platform acceptance and
installation.

## Cargo artifacts

The existing helper remains available from a component under `modules/` or
`tools/`:

```sh
python3 ../../scripts/cargo-cache.py run -- just check
```

Its setup/check recipes are `just cargo-cache-setup` and `just cargo-cache-check`.
The helper checks source membership and selects the complete pinned compiler.
Explicit staging and component-specific provider preparation retain their own
contracts. See the component README before regenerating a prepared lock.
