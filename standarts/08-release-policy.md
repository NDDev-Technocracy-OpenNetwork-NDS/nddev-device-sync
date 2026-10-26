# Release policy

## Channels

The release sequence is:

```text
alpha → beta → release
```

The first product artifacts are `alpha`. Promotion requires all quality,
security, observability and compatibility gates for the preceding channel.

## Versions

The product being built is **NDS 1, first alpha**. The coordinated product
identifier is `1.0.0-alpha.<candidate>`. One release manifest binds the component
source revisions, artifact identities and data compatibility for that candidate.
Component tags identify artifacts within that same product train:

```text
desktop-v1.0.0-alpha.1
mobile-v1.0.0-alpha.1
server-v1.0.0-alpha.1
```

The candidate identifies a coordinated build, not a new product generation.
Desktop, mobile and server may ship independently only when the release manifest
records their compatibility. Existing immutable tags and library/source locks
retain their identities; no retroactive rename or blanket version bump is implied.

Stable release semantics begin at `v1.0.0`. During alpha, a contract change needs
an explicit coordinated transition for existing data and consumers. Internal
wire/storage migration numbers remain distinct from the user-facing product
version. Maintain one current implementation and only the verified transition
needed by actual retained data, rather than independent competing product lines.

## Artifacts

Current alpha engineering uses project-built artifacts, without new platform
registrations, production signing identities or store submissions. Build
integrity, provenance, compatibility and functional acceptance still apply.
Debug or simulator builds cannot be counted as production distribution.

For separately approved release distribution, desktop artifacts are signed per
operating system and architecture. Mobile
artifacts use the platform stores or signed distribution packages. Server
artifacts are signed container images with SBOM and build provenance. Release
receipts contain source commit, toolchain, dependency lock hashes, artifact
digests and migration version.

NDS application update discovery, staging and activation follow
[ADR 0009](decisions/0009-first-alpha-product-contract.md). Store channels use
their platform update mechanism; direct Android APK distribution preserves the
package signing identity. An update cannot discard unsent work or reset stores.

