# Development

The root selects public component source. Each component owns its build,
dependencies and acceptance commands.

```sh
git clone --recurse-submodules https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync.git
cd nddev-device-sync
just setup
just check
```

`just setup` prepares isolated Python tooling. `just check` validates the catalog,
schemas, exact Gitlinks and documentation presence without compiling components.
`just test` exercises the tooling, including isolated real Git and Cargo fixtures;
it needs the pinned Rust toolchain and `just` available on the test host. CI runs
both commands and verifies recursive public checkout.

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
.venv/bin/python scripts/check-contracts.py /path/to/component/contracts/module.json
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
