# Repository topology

`nddev-device-sync` is the single main NDS source repository. It owns standards,
assembly metadata and the component catalog. Independent public component
repositories are exact Git submodules under `modules/<id>`; the four small
system tools (clipboard, sysinfo, updater, cleaner) live under `tools/<id>`
and have catalog kind `tool`. GDS and RDS are full modules. Existing technical
roles such as shared-core, server and client remain explicit in the catalog.
Each runtime module owns its implementation, tests, dependency locks and release
boundary. `nddev-device-sync-estate` is the private configuration and operational
evidence repository. `nddev-device-sync-estate-example` is the public synthetic
self-hosting example, consuming the generic server through an exact source
Gitlink. Public modules never require private estate access for builds or CI.
The private estate has no Gitlink in the public root. Organization profile
repositories are outside the product assembly. GitHub repository names remain
flat; source hierarchy is represented by native Git submodules.

The catalog owns component identity, role and path. `.gitmodules` must match it
exactly, and every public entry must have one indexed Gitlink. Git owns the exact
commit, so the catalog does not duplicate source revisions. CI checks this
without recursively cloning components or requesting private credentials.
Initialize selected components explicitly; child dependency locks remain their
runtime/build authority. A source snapshot does not qualify an alpha release.

Preserve existing dirty and in-use checkouts. Physical adoption must reconcile
Git identity, local work, nested Gitlinks and GDS ownership before retiring an old
location. Do not create competing active roots or move a running agent's cwd.

Create a module repository when an accepted behavior needs that boundary.
Catalog reservations do not authorize empty repositories or inactive services.

A module contains AGENTS.md, LICENSE, a short README, standarts.lock, module.yaml,
a justfile and its applicable tests. Runtime dependency locks are committed and
checked in CI. A schema-only repository instead pins its validation/generation
tooling; it does not invent a Cargo workspace just to have a Cargo.lock.

Repository metadata declares packaging and ownership. Runtime descriptors
declare API version, capabilities, permissions, platforms and dependencies.
Their relationship is explicit and validated, with no conflicting handwritten
projections. Modules consume immutable source releases, never private estate
files or another repository's mutable branch.

Generated clients and design-token projections record their canonical source
and regenerate without manual edits. The release/check path verifies drift.
Keep operational addresses, accounts, secrets and evidence outside public source.
