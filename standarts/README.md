# nddev-device-sync standards

Start with the [NDS system standard](17-system-standard.md): stable product
clauses, canonical rules, accountable components and observable acceptance.
Its contract revision 1 organizes the first-alpha target; it does not change
the meaning of an existing wire version, package release or standards lock.

This directory is the normative product standard for `nddev-device-sync`.
The documents are written in English because code, APIs, events, schemas,
release metadata and operational identifiers use English. The product UI
supports English and Russian from the first user-facing release.

The standard is intentionally explicit about data ownership, trust boundaries,
release channels, observability and the recovery policy. An implementation may
add detail, but it must not silently weaken a rule here. Any change to a rule
requires an ADR in `standarts/decisions/` before code is changed.

Keep one canonical statement of each rule. Implementation details belong in
code, schemas and executable checks; historical alternatives stay in Git history.
The current engineering and identity baseline is recorded in
[ADR 0002](decisions/0002-autonomous-engineering-and-email-identity.md), with
the platform scope updated by
[ADR 0003](decisions/0003-dual-sign-in-and-platform-scope.md) and the three-method,
agent-first identity target recorded in
[ADR 0006](decisions/0006-agent-first-sign-in-and-vault-access.md).
[ADR 0007](decisions/0007-offline-browser-client.md) adds the accepted offline
browser scope and its implementation/qualification gates.
[ADR 0008](decisions/0008-nds-owned-screen-view.md) defines the NDS-owned screen
endpoint boundary while preserving existing native remote endpoint identities and policy.
[ADR 0009](decisions/0009-first-alpha-product-contract.md) defines the accepted
NDS 1 first-alpha outcomes and their cross-platform acceptance boundary.
[ADR 0010](decisions/0010-react-browser-presentation.md) changes the browser
presentation to React/TypeScript while preserving Rust behavior and offline data.
[ADR 0011](decisions/0011-company-membership-and-device-access.md) adds company
membership, invitations and administrative remote-screen access while preserving personal
data isolation.
[ADR 0012](decisions/0012-shared-cargo-cache.md) defines the shared local Cargo
artifact cache and safe build/acceptance path resolution.
[ADR 0015](decisions/0015-agent-browser-automation.md) defines NDS-owned agent
browser automation inside owner-granted agent tab groups.
[ADR 0017](decisions/0017-extensible-offline-kernel.md) defines the extensible
offline kernel, multiple personal/organization spaces, device placement,
full module ownership and independently versioned delivery. Its explicit
supersessions govern the target while earlier locks retain their original scope.
An edit here does not change the meaning of a module's existing release lock.
[ADR 0018](decisions/0018-control-and-diagnostic-profiles.md) adds trusted
fullauto/manual action profiles, standard/debug/off diagnostics, diagnostic
access roles and separately key-authenticated global administration. These
are product contracts; private engineering governance is not this corpus.

The [active alpha development plan](plans/alpha-development.md) owns the current
implementation order and completion scenarios. It does not replace these rules
or assert that planned behavior already works.

## Normative documents

1. [Product boundary](01-product.md)
2. [System architecture](02-architecture.md)
3. [Identity and vault](03-identity-and-vault.md)
4. [Sync and data](04-sync-and-data.md)
5. [Observability](05-observability.md)
6. [Infrastructure](06-infrastructure.md)
7. [Quality gates](07-quality.md)
8. [Release policy](08-release-policy.md)
9. [Internationalization and visual system](09-i18n-and-visual-system.md)
10. [Operations](10-operations.md)
11. [Repository topology](11-repository-topology.md)
12. [Product decisions](12-product-decisions.md)
13. [Engineering and agent standard](13-engineering-and-agent-standard.md)
14. [Technology standard](14-technology-standard.md)
15. [Observability and telemetry standard](15-observability-and-telemetry-standard.md)
16. [Development workflow and consistency](16-development-workflow.md)
17. [NDS system standard](17-system-standard.md)

## Status

The product starts in the `alpha` channel. These documents describe the
baseline for `1.0.0-alpha.<candidate>`; they are reviewed before the first public
artifact and again before moving to `beta`.

[ADR 0020](decisions/0020-system-standard-and-staged-adoption.md) records the
system-wide adoption method. Product rules are targets; source, component checks,
packaged artifacts and actual target behavior retain separate evidence. The
active plan governs changes through those levels one coherent cohort at a time.

The public visual token source is
`NDDev-Technocracy-OpenNetwork-NDS/nddev-opennetwork-design-system`, licensed under
AGPL-3.0-only.

The [complete source/privacy boundary](decisions/0019-complete-source-and-privacy-boundary.md)
keeps private NDDev engineering governance separate from these public NDS product
contracts. Public checkout and checks do not require private standards or Estate.
