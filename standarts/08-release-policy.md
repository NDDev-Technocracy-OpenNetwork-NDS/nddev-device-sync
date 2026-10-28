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
[ADR 0009](decisions/0009-first-alpha-product-contract.md), extended by
[ADR 0017](decisions/0017-extensible-offline-kernel.md). Store channels use
their platform update mechanism; direct Android APK distribution preserves the
package signing identity. An update cannot discard unsent work or reset stores.

## Kernel and module updates

NDS and independently delivered modules have separate package versions. The
release contract binds publisher/module identity, kernel API compatibility,
dependency constraints, platform/architecture and host/viewer role, artifact
digest/provenance and data schema/migration compatibility. Product candidate,
package version, protocol API and storage schema remain distinct. A mutable
`latest` label or a lexically larger version is not an accepted artifact.

One built-in update owner serves module and application callers; do not create
competing update engines in a tool, UI and launcher. Existing external OS update
plan/status reporting is a different capability. Reuse the current signed
verifier/stager while implementing its missing activation boundary in the
owning Updater and platform composition. A staged pointer is not an installed
launcher pointer, and successful download is not successful activation.

An update follows these owned transitions:

1. Discover a compatible candidate under the current publisher/channel policy.
2. Download into owned bounded staging and verify exact manifest/artifact bytes.
3. Retain the exact installed owner and seal its new work admission. Persist
   pending state, join admitted effects and require positive resource drain.
4. Apply a qualified migration and switch only the selected module/application
   package through its platform installation/launcher contract. A controlled
   restart is allowed; repeated unknown stop/switch attempts are not success.
5. Start the selected version, verify its actual identity and required readiness,
   then publish the new installed/running observation and reopen admission.

Each device reports desired, staged, installed, running and last failed versions
separately. A synchronized update policy is not proof of local installation and
does not silently install a new publisher or grant extra permissions. Offline
devices keep their actual version and catch up through compatibility policy.

An interruption before activation preserves the old installed package. A failed
activation uses the retained previous executable only if its data compatibility
is still valid; rollback must never downgrade incompatible migrated records.
Retaining executable versions is not a backup of user keys, databases or Vault.
Unknown cleanup/activation remains closed with an actionable retained result.
The updater and its parent survive caller loss long enough to record the actual
terminal effect; a disappeared process or joined waiter alone is not that receipt.

A package shared by standalone callers or other NDS owners requires their
disposition before replacement. Drain only resources the updater can positively
bind to that installed generation. Unknown consumers or unresolved work block
activation; there is no global shutdown, foreign-session close or blind restart
fallback.

Linux/native workers can be independent packages under their supervised module
contract. Static kernel libraries and application-bound UI code update through
their owning app bundle. Mobile/store and browser app-shell rules remain binding;
there is no claim of downloading arbitrary native plugins into Web or iOS.
A remote native module is explicitly hosted on an admitted device and is not
reported as a local browser/mobile executable.

Qualification exercises real current installation and launcher identity, module
and full-app updates, busy/drain failure, caller loss, interrupted downloads,
invalid signatures, dependency incompatibility and forward migration. A fabricated
installed version or a byte sentinel cannot qualify a real application switch.

