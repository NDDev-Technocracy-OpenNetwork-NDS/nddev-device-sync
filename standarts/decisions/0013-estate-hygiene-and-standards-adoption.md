# ADR 0013: estate hygiene and organization standards adoption

Status: accepted owner direction, 2026-10-22.

NDS adopts the organization standards corpus alongside its own locked
standards. The product contract in `standarts/` remains the governing source
for NDS behavior; the organization corpus governs repository structure,
GitHub workflow, CI/CD shape and component versioning. Both apply; where a
topic is covered by only one corpus, that corpus rules.

## Adopted organization standards

- STD-0001 edition 3.0.0 governs repository structure. Every NDS repository
  carries `.nds/nddev.yaml` declaring the adopted edition, its role and its
  ecosystem profiles. `nddev-device-sync` is the system root
  (`role: system-root`, `system.id: nds`) and lists every member repository;
  all other repositories are components naming this repository as their
  system root. Generated `.nds/` material and adapter shims are produced
  only by the `nddev` generator and are never hand-edited.
- STD-0003 governs the already-practiced workflow: `dev` is the integration
  branch, `main` is the normal target, task branches carry task work and
  pull requests merge as merge commits.
- STD-0004 governs CI shape: a short finite pull-request gate precedes a
  full validation on `main` pushes; `dev` pushes are not built by GitHub.
  Runtime repositories with native ecosystem profiles declare their contract
  in `.github/nddev-ci.toml` and run it through the pinned std-0004 composite
  at the latest final std-0004 release tag; this standards root keeps its
  native `just` gate, which is already short and finite, and additionally
  runs `nddev check` in full main validation.
- STD-0008 governs versioning: native descriptors (`module.yaml`, package
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
variations referencing this decision: `.gds/` (owned by the GDS tool,
excluded from std-0001 tooling per its own boundary rule), `standarts/`
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
[ADR 0010](0010-react-browser-presentation.md),
[STD-0001](https://github.com/NDDev-Technocracy-Standarts/std-0001-github-repository-structure),
[STD-0004](https://github.com/NDDev-Technocracy-Standarts/std-0004-ci-cd-workflows).
