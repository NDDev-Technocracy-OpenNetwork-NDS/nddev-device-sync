# ADR 0013: estate hygiene and repository contracts

Status: accepted owner direction, 2026-10-22.

[ADR 0018](0018-control-and-diagnostic-profiles.md) keeps private engineering
policy and adoption evidence outside public product prose. This file records
the NDS repository contract. Historical locked sources retain their original
meaning; neither this amendment nor a source check changes running installations.

## Repository and delivery contracts

- The existing repository manifest declares its role and
  ecosystem profiles. `nddev-device-sync` is the system root
  (`role: system-root`, `system.id: nds`) and lists every member repository;
  all other repositories are components naming this repository as their
  system root. Generated `.nds/` material and adapter shims are produced
  only by the `nddev` generator and are never hand-edited.
- `dev` is the integration
  branch, `main` is the normal target, task branches carry task work and
  pull requests merge as merge commits.
- A short finite pull-request gate precedes a
  full validation on `main` pushes; `dev` pushes are not built by GitHub.
  Runtime repositories with native ecosystem profiles declare their contract
  in `.github/nddev-ci.toml` and use their selected public reusable implementation.
  The source root's `just check` validates its catalog, schemas and existing
  source fixtures. It is not a complete policy or product-conformance check.
- Native descriptors (`module.yaml`, package
  manifests) own component versions; release authority stays with the
  release manifest.

## Repository catalog corrections

- `web` is a React browser client per ADR 0010. The catalog `kind` enum
  gains `web-client` and the `web` entry uses it. `flutter-client` remains
  for `desktop` and `mobile`.
- The release sequence vocabulary is `alpha → beta → release` per standard
  08. Catalog `channel` and module `status` therefore use `release`, not
  `stable`. No module currently carries the old value; the change is a
  schema correction, not a status migration.
- `platforms` declares the runtime targets on which a module's artifacts
  actually run, including `web` for browser-delivered artifacts. A
  compile-time feature that merely permits a wasm build does not list a
  platform; a shipped browser artifact does.

## Structure variations recorded in repository manifests

Each adopting manifest records the following NDS-owned material as approved
variations referencing this decision: `.gds/` (owned by the GDS tool), `standarts/`
(the NDS corpus and its frozen directory name), `standarts.lock`,
`module.yaml`, `protocol.lock`, `toolchains.lock.json`,
`assembly.lock.json`, `provider.lock.json` (NDS contract locks) and
`contracts/` (the NDS contract corpus). These are deliberate product
contract locations, not structural defects.

## Version and lock discipline

- Nested crates and packages keep their own version tracks (for example
  the domain track, the SDK track and adapter IO track). Distinct tracks
  are intentional and are documented in each repository's manifest rather
  than forced into the product train.
- `standarts.lock` anchors move only to a released standards tag, never to
  arbitrary commits. A relock wave preserves the design system's
  `[amendments.visual]` section.
- Component pins (`protocol_commit`, dependency revisions) are intentional
  or they are drift. A divergent pin is either documented by the owning
  manifest or corrected; it is never left silent.

## Documentation honesty

Stale or misleading documentation is a defect and is corrected in the same
change that makes it stale. No repository knowingly commits an outdated
README, manifest description or generated-projection input. Planned
capabilities stay declared as planned.

References: [standard 08 release policy](../08-release-policy.md),
[standard 11 repository topology](../11-repository-topology.md),
[ADR 0010](0010-react-browser-presentation.md).
