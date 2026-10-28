# ADR 0019: complete public source hierarchy and two private contexts

Status: accepted owner direction, 2026-10-28.

The owner selects one main public `nddev-device-sync` source root containing
all independent public repositories in the NDS organization as exact Git
submodules. GitHub repositories remain flat; modules are under `modules/` and
small tools under `tools/`. Histories, implementation, tests, dependency locks
and component release versions remain with their existing repositories.

Only `nddev-device-sync-standards` and `nddev-device-sync-estate` are private.
The first owns private NDDev engineering standards selection and adoption;
the second owns real deployment configuration and operational evidence.
Neither is cloned recursively, pinned as a public Gitlink, required to build
public source or included in an application artifact's source roles. Public
NDS architecture, module/API contracts, ADRs and build instructions remain
self-contained. Private governance does not become a second public policy
compiler. Existing published locks and public history are not rewritten.

Catalog version 3 adds explicit visibility, the two private context kinds and
the public organization profile as source-only `modules/organization`. That
repository owns the introduction and assets; it is not an executable module.
All other source is public OSS under its retained license notices. Repository
membership does not assert runtime readiness or extend device authority.

`just checkout` uses native recursive Git submodule checkout. Component recipe
dispatch uses each child's own justfile, requires its selected source identity
and never copies build logic into a new global manager. The unprofiled Cargo
helper scopes generated configuration inside the main source root. A selected
Cleaner profile retains its existing placement authority.

Adoption preserves old sibling checkouts, every registered worktree and dirty
change. Public children are initialized in the main root with independent Git
metadata; existing common object stores are not moved. No automatic GDS
enrollment, source deletion, daemon restart, dependency lock rewrite, deployment
or runtime rollout follows. Generic nested GDS synchronization, combined app
qualification and later full module implementations retain their own gates.
