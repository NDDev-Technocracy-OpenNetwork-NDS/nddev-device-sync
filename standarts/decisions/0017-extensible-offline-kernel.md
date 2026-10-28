# ADR 0017: an extensible offline NDS kernel

Status: accepted owner direction, 2026-10-28; implementation and migration pending.

## Decision

NDS is one distributed user system with a common kernel, independently owned
functional modules and tools. The kernel provides identity, security, device
connections, synchronization, application composition, module lifetimes,
updates and observability. A kernel is a logical boundary; it need not be one
crate or process. Domain and application remain independent of concrete I/O.

1. A user has a personal space and may belong to several organizations.
   Identity is stable across spaces. Each operation, record, cursor, credential
   reference and retained resource has an explicit original space and actor.
2. Servers and personal computers are devices under one identity model.
   Deployment form, administrative scope, purpose profiles, capabilities and
   execution policy are distinct. Clear names help agents; names grant nothing.
3. Authorized local work and connections between reachable devices continue
   without the coordinator within existing offline authorization. Reconnection
   revalidates before committing changes. Local expiry, disable and known
   revocation remain effective; offline devices cannot learn remote revocation.
4. GitHub owns published project history and code/release distribution. NDS
   synchronizes its own state. The server retains durable encrypted changes,
   delivery progress and authoritative control metadata, without mirroring all
   project directories or transient media. History and concurrent revisions,
   rather than wall-clock order, determine reconciliation.
5. A module implements its advertised function. GDS owns GitHub synchronization;
   RDS owns remote-display behavior. Archived code is reusable implementation
   groundwork, not a mandatory independently installed legacy product. Ordinary
   maintained Git, codec, cryptographic and networking libraries remain valid
   dependencies. Existing provider state is preserved through explicit migration.
6. One versioned extension contract serves official and future user-developed
   modules. The alpha admits the official set. A declared permission or a signed
   package is not an authorization grant or proof that native code is sandboxed.
7. The application and modules have distinct versions and compatibility
   requirements. Built-in update management verifies, drains, activates and
   checks them. Controlled module or application restarts are acceptable;
   in-process native code replacement is not required. Static library version
   changes alone do not implement an independently installable module.

The detailed current rules live in [architecture](../02-architecture.md),
[identity](../03-identity-and-vault.md), [synchronization](../04-sync-and-data.md)
and [release policy](../08-release-policy.md). These documents describe the target,
not evidence that the current composition already provides it.

## Changes to earlier decisions

- Supersede the compiled-in-only restriction and external-tool-only product
  boundary in standards 01/02 and the central working instructions. Compiled-in
  components remain a supported packaging form for kernel internals and
  platform application bundles, not the only extension form.
- Supersede ADR 0011's exactly-one-company restriction. Retain its verified
  invitations, personal-data isolation, last-admin protection, device proofs,
  original ownership and bounded enrollment. Multiple spaces do not grant an
  administrator another user's personal Vault or GDS data.
- Supersede ADR 0014 as the complete shape of a functional module. Its thin
  adapter template remains appropriate for actual I/O adapters and small tools.
- Supersede the archived-provider dependency as the final RDS product boundary
  in ADR 0008/0009. Preserve existing identities, OS consent, session authority
  and view-only alpha scope. Remote input, shell and file transfer are not
  authorized by architectural ownership alone.
- Extend ADR 0009 with independent module delivery and updates. Preserve the
  existing Git-write consent and existing-commit synchronization policy:
  automatic commits, force updates and history rewriting are not implicit.
- Preserve the server-managed key trust model in ADR 0006. An encrypted journal
  is not a claim of client-only E2EE. Existing seven-day offline limits,
  32-device/1024-repository targets and the backup/recovery prohibition remain.
- Keep ADR 0016's repository layout and component histories. No relocation,
  catalog readiness promotion or reinterpretation of a published lock follows
  from this decision.

## Implementation sequence and completion conditions

| Cohort | Canonical owners | Completion condition |
| --- | --- | --- |
| Scope and device foundation | Core, Protocol, Accounts, Organizations, Server, Vault | Personal plus multiple organization spaces; stable device identity, profiles and enforced placement; existing IDs, keys, ciphertext and pending work preserved |
| Sync and connection foundation | Core, Protocol, Accounts, Vault, Server | Offline edits and authorized reachable-device operation; eventual delivery after the source device exits; explicit concurrent conflicts and expired-history catch-up |
| Extension ownership | Core application, Agent, Protocol, existing module callers | One actual module/tool path for discovery, scoped admission, supervision, results, health, drain and version negotiation; incompatible or unknown authority refused |
| Full functional modules | GDS, RDS, browser-use | Advertised workflows owned by NDS implementations; real user-facing operation with the kernel and standalone where supported; legacy installations are not mandatory |
| Verified delivery | Updater and platform application/launcher owners | Independent supported module update and full NDS update, actual activation/restart, compatibility and interrupted-update preservation |
| Product composition | Desktop, Mobile, Web, observability pipeline | Coherent accepted sources and actual Linux, PC-owned Android emulator and Web workflows, including searchable telemetry and honest failure states |

Reuse existing verified behavior. Add protocol and storage migrations only in
the owning cohort with executable consumers. Do not build an unused plugin
framework, duplicate scheduler, authorization database or telemetry stack.
Introduce a namespaced module registry through real GDS/RDS/browser-use callers.
Kernel-specific security vocabulary remains centrally authorized; an extension
cannot create privileges merely by naming a capability.

Qualify fresh and retained data, lost completion, concurrent replacement, scope
switch, offline expiry, update interruption and restart. A source lock, descriptor,
green subset or mocked provider is not a complete module or alpha acceptance.
Retain the original scope of earlier runtime receipts. Source changes do not
authorize live deployment, protected transport changes or an operational window.
