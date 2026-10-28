# ADR 0016: one NDS source root with independent modules and tools

Status: accepted owner direction, 2026-10-25.

The owner selects `nddev-device-sync` as the main NDS monorepo, with independent
component repositories composed through Git submodules. GitHub stays flat;
the checkout hierarchy is `modules/<id>` and `tools/<id>`.

Clipboard, sysinfo, updater and cleaner are the four confirmed small system
tools. Their catalog kind is `tool`, matching the existing runtime descriptor
kind. GDS, RDS and the other public product components belong under `modules/`.
Technical roles such as shared-core, contracts, server and client remain visible.
Cache and browser-use are accepted component boundaries with separate owners;
their source registration does not assert implementation or activation.

Catalog version 2 uses `components` and declares each component's path. Native
`.gitmodules` entries identify the public provider repositories, and indexed
Gitlinks pin exact commits. Central checks reject mismatched identities, paths,
missing/extra Gitlinks, copied implementation and private estate inclusion.
Component source histories, dependency locks and release tracks remain owned
by their repositories. Existing published standards locks and tags are immutable;
this decision governs the new central source snapshot.

The private estate remains a separate control-plane checkout with `path: null`.
It retains system membership without publishing its source revision or requiring
private access for public checkout/CI. The synthetic estate example is public.

The repository manifest records `modules/` and `tools/` as intentional
package grouping roots. This is an owner-selected layout variation, not an
exception permitting application implementation in the system root. Generated
instruction projections are produced by their recorded generator.

Physical relocation is a separate checked adoption of this declared layout.
Preserve dirty work, Vault, stash, nested repository identities and live services;
an in-use checkout is retained until it can move safely. Public assembly checks
are structural and do not qualify runtime behavior, deployment or an alpha release.
