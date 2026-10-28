# nddev-device-sync

NDS is the public, self-hostable NDDev OpenNetwork distributed user system for
devices, servers, functional modules and tools. Its common kernel owns identity,
security, connections, synchronization, module lifetimes, updates and observation.
Rust owns the domain and system behavior. The
accepted presentation stack is Flutter for desktop/mobile and React with strict
TypeScript for web, in English and Russian. The product keeps
the NDDev OpenNetwork name and [nddev.ai](https://nddev.ai) attribution.

License: AGPL-3.0-only. See [LICENSE](LICENSE) and [TRADEMARKS.md](TRADEMARKS.md).
Release journals live in [GitHub Releases](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync/releases).

This is the main NDS source monorepo. It owns the
[public product contracts](standarts/README.md), [component catalog](contracts/module-catalog.json)
and exact Git submodules for the independent component repositories:

```text
nddev-device-sync/
  modules/       core, protocol, server, agent, clients, Vault, GDS, RDS, organization, ...
  tools/         clipboard, sysinfo, updater, cleaner
  contracts/     catalog and assembly validation
  standarts/     NDS product contracts
```

GitHub repositories keep their flat organization names, histories and releases.
The [catalog](contracts/module-catalog.json) gives every component its exact path;
`.gitmodules` and Gitlinks own the source snapshot. Get the public tree with:

```sh
git clone --recurse-submodules https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync.git
cd nddev-device-sync
just setup
just component gds check
```

For an existing clean root, `just checkout` initializes the exact public source
recursively. Selective checkout remains native Git, for example
`git submodule update --init -- modules/gds tools/cleaner`. `just component <id>
<recipe> ...` runs the component's own justfile, preserving argument boundaries
and refusing a different provider or HEAD than the selected Gitlink. Use a child
worktree directly for feature development; central dispatch never resets it.
The organization introduction is source-only at `modules/organization`, with no
runtime descriptor or executor.

Every component retains its history, releases, checks and dependency locks.
Root Gitlinks select source for development; application assembly locks select
compiled dependencies, including distinct roles at the same repository URI.
A component's passing CI or source membership does not qualify the combined app.

The [engineering contract](standarts/13-engineering-and-agent-standard.md)
governs autonomous delivery, minimal code and state, real verification and
scope control. [ADR 0002](standarts/decisions/0002-autonomous-engineering-and-email-identity.md)
records the engineering baseline; [ADR 0006](standarts/decisions/0006-agent-first-sign-in-and-vault-access.md)
records email OTP, GitHub and passkey sign-in with automatic vault access and
native agent capabilities. The coordinated implementation includes
server-managed owner keys, encrypted replica catch-up, paginated vault records,
explicit conflict resolution and signed client diagnostics. Advertised sign-in
methods depend on the instance configuration. A compiled platform is not proof
of runtime qualification; each runtime repository owns that evidence.
Existing module locks and published alpha tags keep their original meaning.

Use the installed NDS development profile to share Cargo artifacts outside Git.
The central helper remains available: from a component under `modules/` or
`tools/`, use `python3 ../../scripts/cargo-cache.py run -- just check` to select
the complete pinned Rust toolchain. Existing sibling checkouts retain their
previous helper path until an explicit, checked relocation.
[ADR 0012](standarts/decisions/0012-shared-cargo-cache.md)
defines cache isolation, artifact paths and safe cleanup.

Exactly two organization repositories are private and kept outside recursive
public checkout: `nddev-device-sync-standards` owns private NDDev engineering
standards selection/adoption, and `nddev-device-sync-estate` owns actual deployment
configuration and operational evidence. All other repositories are public OSS.
NDS-specific architecture, API contracts and necessary build instructions remain
public under `standarts/`; private governance is not a public build dependency. The public
[estate example](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-estate-example)
contains synthetic values and consumes the server's existing Compose contract
through an exact Gitlink. It does not expose company infrastructure.

[ADR 0017](standarts/decisions/0017-extensible-offline-kernel.md) records the
owner-selected extensible offline architecture: personal and multiple
organization spaces, unified device profiles and workload placement, native NDS
state synchronization, full module ownership and independently versioned delivery.
The [architecture](standarts/02-architecture.md) assigns all component roles
and the migration boundaries. Official alpha modules use the contract intended
for later user-developed modules. These targets supersede the earlier
compiled-in-only/adapter-only product limit; current implementations and
immutable release locks require deliberate adoption and runtime qualification.

The [active development plan](standarts/plans/alpha-development.md) sequences
the remaining implementation and names a real completion scenario for each
stage. It starts with personal/organization identity, devices and fullauto/manual
profiles, with a correlated diagnostic path early in the work. Product rules
remain owned by the architecture and diagnostic contracts, not the plan.

[ADR 0009](standarts/decisions/0009-first-alpha-product-contract.md) records the
accepted NDS 1 first-alpha outcomes and acceptance criteria. It includes module
setup after sign-in, seven-day offline authorization, bidirectional GDS sync,
cross-device viewing, NDS application updates and common observability. These
targets are not a claim that the deployed composition already implements them.
[ADR 0011](standarts/decisions/0011-company-membership-and-device-access.md) adds
companies and invited users with administrative RDS access; ADR 0017 extends
membership to multiple organization spaces while personal Vault and GDS data
remain owner-only. Server and client sources implement company,
membership and invitation workflows. Runtime repositories own supported-target
qualification and migration acceptance for existing installations.

The catalog describes repository boundaries and declared platforms, not a
release bill of materials or a runtime capability response. The accepted
[offline browser scope](standarts/decisions/0007-offline-browser-client.md)
is implemented by the [web assembly](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-web),
which uses React/TanStack/shadcn presentation under
[ADR 0010](standarts/decisions/0010-react-browser-presentation.md) and the shared
Rust/WASM composition. Desktop and mobile retain Flutter. Browser
adapters provide encrypted local credentials, a durable replica and telemetry
queue, while the server remains authoritative for committed state. The web
repository owns artifact provenance and browser qualification; an alpha catalog
entry does not assert deployment or support for untested browsers.

Run `just setup` once to install isolated, pinned structural tooling, then
`just check` for the central repository's structural checks. Runtime
repositories own their implementation and acceptance checks. Passing central
checks is not evidence that the application or observability pipeline works.

To check an actual compiled-in descriptor against the central contract, run
`.venv/bin/python scripts/check-contracts.py /path/to/module/contracts/module.json`.
This validates schema and dependency declarations; runtime activation, dependency
graph compatibility and OS behavior remain the runtime owner’s responsibility.
The descriptor schema includes the `web` platform accepted by ADR 0007.
Earlier native-only schema snapshots cannot validate browser descriptors;
their immutable release and source locks remain unchanged.

Catalog version 3 records visibility and distinguishes the two private contexts
and the organization introduction from runtime components. It extends the
version 2 source layout without changing a runtime descriptor API or reinterpreting
issued tags/locks. [ADR 0019](standarts/decisions/0019-complete-source-and-privacy-boundary.md)
records the organization-wide boundary. Existing sibling checkouts and their
worktrees remain preserved; no source command enrolls devices, deploys a service
or registers a competing GDS workspace.
