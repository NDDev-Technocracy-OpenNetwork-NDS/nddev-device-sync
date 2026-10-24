# Repository topology

The central repository owns standards, assembly metadata and the module catalog.
Each runtime module owns its implementation, tests, dependency locks and release
boundary. `nddev-device-sync-estate` is the private configuration and operational
evidence repository. `nddev-device-sync-estate-example` is the public synthetic
self-hosting example, consuming the generic server through an exact source
Gitlink. Public modules never require private estate access for builds or CI.

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
