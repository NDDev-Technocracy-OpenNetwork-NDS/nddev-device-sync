# Repository topology

`nddev-device-sync` is the single main NDS source repository. It owns public product contracts,
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
Neither private Estate nor private `nddev-device-sync-standards` has a Gitlink
in the public root. Private Standards owns NDDev engineering governance; public
NDS product contracts remain self-contained. These are the only two private
organization repositories. The public organization profile `.github` is included
as source-only `modules/organization`; it is not a runtime module. GitHub repository names remain
flat; source hierarchy is represented by native Git submodules.

The catalog owns component identity, role and path. `.gitmodules` must match it
exactly, and every public entry must have one indexed Gitlink. Git owns the exact
commit, so the catalog does not duplicate source revisions. CI checks this
without recursively cloning components or requesting private credentials.
Use `just checkout` for the public recursive tree or initialize selected components explicitly; child dependency locks remain their
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

Catalog version 3 declares visibility and the non-runtime kinds
`organization-profile`, `engineering-governance` and `deployment-configuration`.
Private contexts have no source path or public Gitlink. Central checks reject
private contexts as application artifact source roles. Each actual runtime
module's descriptor remains the authority for executable capabilities; catalog
membership alone does not grant runtime admission.

`just component <id> <recipe> ...` delegates to the selected child's justfile.
The unprofiled Cargo helper places its configuration inside this source root,
covering only this tree; unrelated sibling projects are outside that scope.
The existing Cleaner profile, when selected, continues to own cache placement.
Neither checkout nor source dispatch rewrites that profile or creates another
GDS synchronization root. Generic GDS graph reconciliation and deployment remain
separate functional work.
