---
status: active
---

# NDS alpha development plan

This is the active implementation sequence for the product decisions in
[ADR 0017](../decisions/0017-extensible-offline-kernel.md) and
[ADR 0018](../decisions/0018-control-and-diagnostic-profiles.md). Those decisions
own the requirements; this plan owns sequencing and completion scenarios.
Repository membership remains in the [catalog](../../contracts/module-catalog.json).
Native manifests, locks and build records own changing technical identities.
The [system standard](../17-system-standard.md) supplies stable clause references
for a cohort's scope and acceptance; private engineering adaptations remain with
the governance owner.

## Starting point

Keep the existing pure Rust domain/application boundary, authenticated client
and device proofs, encrypted Vault replica/outbox/conflict mechanisms, native
task/result/drain ownership, telemetry SDK and client generation. GDS already has
an owned catalog and existing-commit sync engine. RDS has useful authority and
session integration, while its native engine still comes from the archived
product. These are reusable implementations, not a completed extensible system.

The principal gaps are the multi-space identity model, device/action policy,
general module lifecycle, fully owned functional workflows, actual update
activation and coherent target acceptance. New contracts in source have not
already changed a running application.

## Delivery sequence

### Cohort progression

Work on one coherent implementation cohort before adopting its dependants:

1. Capture the selected source/lock baseline, affected clauses, real callers,
   retained state and the observable outcome. Review the actual owning code.
2. Change canonical contracts and providers with their real consumers. Define
   upgrade/failure behavior before a migration can change retained data.
3. Run owning checks and meaningful isolated adapter/migration acceptance.
   Review concurrency, caller loss, scope replacement and unresolved effects.
4. Qualify the affected client/host roles and diagnostic path. Record unsupported
   or unexecuted targets honestly; do not activate a live route to hide a gap.
5. Publish through the existing signed task → dev → main path, verify exact-main
   checks, then adopt immutable sources in dependent assemblies. Remove superseded
   paths only with their qualified replacement.

Contract/source, component checks, packaged artifacts and actual target behavior
are separate exit conditions. A failed gate keeps the cohort open. This plan
does not issue deployment, protected-client maintenance or runtime-window authority.

### Alpha completion order and independent module work

Complete the following integrated outcomes in order. Existing P0–P10 sections
remain the requirement owners; this table makes their next release gates explicit.

| Order | Cohort | Required completion |
| --- | --- | --- |
| 1 | P1b, retained-data safety | Finish the original-authority local-read gate; publish the qualified membership providers and Shared/Mobile/Web consumers as one compatible Root composition. Preserve personal ciphertext and pending operations across personal/A/B selection and same-owner renewal. |
| 2 | P1c and early P2 | Exercise one real system-tool action through stable device placement and fullauto/manual policy, carrying its original principal and operation context into diagnostics. |
| 3 | P2 diagnostic vertical | Qualify the actual signed client → Server → collector → OpenObserve route, all three diagnostic modes, scoped query/admin access, bounded durable delivery, one-week storage retention and incident recovery. Reuse it in every subsequent cohort. |
| 4 | P3/P4 and module consumers | Close schema-one history reconciliation, offline/conflict and retained operation/result/drain gaps through real consumers. Adopt independently qualified GDS/RDS/browser/cache interfaces without adding another identity, scheduler or telemetry engine. |
| 5 | P8 | Qualify one real application and module activation through the selected installed/launcher owner, including failed startup and retained-data compatibility. Verified staging alone does not finish this gate. |
| 6 | P9 | Requalify the coherent Linux, Android and Web journeys, then each additionally claimed native role. Use actual artifacts, existing acceptance drivers and explicit unsupported states. |
| 7 | P10 | Publish the exact accepted composition, authenticated artifacts and useful installation/run instructions, with the component/target evidence and remaining exclusions stated together. |

GDS, RDS, network-cache, browser extension and CI/runner implementations have
independent development owners. NDS owns its consuming contracts, original-scope
authorization, retained lifetimes, shared presentation, diagnostics and coherent
assembly. Their delivery inputs are immutable sources/artifacts, actual owning
checks, supported capabilities and lifecycle/error semantics. A dependency stays
explicit without stopping unrelated kernel/client work or declaring its module
complete from a compatible signature.

Ordinary compiler, unit and source checks may use the selected authoring-host
toolchain and managed local cache. Actual applications, GUI, mobile emulators
and system integration use their assigned execution targets. A supported local
filesystem does not require remote-filesystem admission for a local code check;
it also does not establish cross-host storage semantics. Preserve active work,
native leases and resource headroom rather than duplicating full build trees.

### P0. Public source and private engineering boundary

The source hierarchy now has a checked public recursive checkout and two private
contexts. The system-standard entrypoint and private NDS engineering profile
establish the adoption basis; they do not establish whole-system conformance.

Review active component instructions, native metadata, generated outputs,
public check prerequisites and retained history. Keep private policy/adoption
material outside public source and CI output. Preserve legitimate public
dependency attribution and licenses. Removing current references does not erase
historical disclosure; no history rewrite or visibility change is incidental.

Produce one coherent inventory from existing manifests and locks: owning
components, real contracts, supported consumers and remaining integration work.
Keep implementation, packaged support and runtime evidence distinct. Use existing
checks and GitHub task ownership rather than another catalog or checking service.

Done when a public checkout can run its supported prepared checks without a
private policy corpus, current instructions match those checks, and each remaining
privacy or compatibility issue has a bounded disposition.

### P1. Principal, spaces, devices and control profiles

Owners: Core, Protocol, Accounts, Organizations, Server, Vault; Agent and shared
clients consume the same contract.

Implement this foundation in three bounded cohorts:

- **P1a — original scope:** introduce the canonical principal/space context through
  an actual existing use case and its wire/store boundaries. Preserve old IDs,
  encrypted ownership and pending operations; qualify the supported migration.
- **P1b — membership:** personal plus multiple organizations, invitations/roles
  and current admission through that context. Exercise cross-space denial and
  stale/cached result disclosure before dependent features adopt it.
- **P1c — devices and control:** stable device proof, the accepted purpose/placement
  profiles and effective fullauto/manual policy through one real system-tool
  action. Existing explicit restrictions survive; names grant no authority.

Each cohort updates its actual clients and tests before the next adopts it.
Do not add unused generic context types or fork the authorization/credential store.

Separate stable user identity from personal/organization membership. Support one
personal space and several organizations, current role and device grants. Add the
agreed device purpose profiles, clear naming and workload placement. Implement
fullauto/manual as effective authenticated policy, preserving explicit older
restrictions rather than reinterpreting them as new default grants.

Preserve established IDs, device keys, Vault keys, ciphertext/AAD, queued
operations and receipts through supported migrations. A new active space never
relabels an old operation or encrypted record. Reuse existing authorization and
generation fences; no parallel identity database or permissions service.

Done when one user operates in a personal space and two organizations; colliding
local IDs, role changes and late results cannot cross scope. A real supported
action runs through fullauto without a prompt or NDS sandbox, while configured
manual restrictions refuse it. Fresh and retained-data paths pass actual store
and consumer checks.

### P2. An early end-to-end diagnostic path

Owners: existing SDK/collector, Server, Accounts, Agent and shared clients.

Take one existing real action, such as a system-information query, through UI
and agent entrypoints, the owning module, correlated diagnostics and searchable
OpenObserve records. Implement standard/debug/off, scoped access and one-week
retention without duplicating telemetry producers at every hop.

Add the separate encrypted global-administrator credential and proof/session
lifecycle through the existing identity and native credential owners. Members
see their own records, organization administrators see theirs, and global
diagnostic access requires the extra key. Storage credentials stay server-side.

Done when actual logs, metrics and traces for the selected workflow are
correlated and searchable; secret canaries remain absent. Exercise debug/off,
wrong-scope queries, key replay/revocation, queue saturation, expiry and recovery.
Ordinary work continues during a diagnostic outage; losses and unknown delivery
remain visible. OpenObserve is the current review surface; external notification
delivery is not claimed. Every later phase includes this diagnostic path.

Close the actual remaining boundaries rather than installing parallel exporters:

- Keep one SDK envelope and production operation owner. Preserve real trace
  parentage and original authenticated scope through handoffs; untrusted trace
  fields are correlation input, never authorization. Count retries, refusals,
  unresolved completion and losses without repeating the same error at each hop.
- Native client delivery must survive its declared restart/outage cases through
  the existing owning storage/runtime boundary. The current RAM-only native feed
  and durable browser queue have different guarantees. Do not describe either
  as remote acknowledgement or exactly-once delivery. Bound bytes, age, retry
  duration and shutdown, and expose overflow/expiry/unknown delivery.
- Implement standard/debug/off as one effective collection and export policy.
  Debug remains finite and scoped; off must not silently continue another
  exporter or retain a hidden content stream. Functional journals and action
  results remain independent of diagnostic availability.
- Enforce member, organization-administrator and key-authenticated global
  diagnostic access at the trusted NDS boundary, including list/count/detail and
  export. Storage administrator tokens stay server-side. Do not rely on a
  backend edition or browser UI to supply rights it does not implement.
- Bring the selected collector/storage configuration to the agreed one-week
  diagnostic retention, including supported stream overrides and expiry
  behavior. Collector retention, local delivery expiry, functional history and
  build-cache lifetime are distinct policies; changing one does not change the
  others or authorize deletion of existing data.

Acceptance searches a fresh operation's real log records, native spans and a
positive metric datapoint with matching source/scope; a readiness response or
local JSON log is insufficient. Include receiver refusal/partial acceptance,
collector outage, restart and recovery, hostile content, wrong-scope queries and
incident open/acknowledge/resolve. Explain missing signals and unsupported
delivery instead of inventing an observation or adding an unbounded retry loop.

### P3. Connection and state synchronization

Owners: Core, Protocol, Accounts, Vault and Server.

Extend the existing durable journal, encrypted replica, revision checks,
outbox/inbox, conflicts and history checkpoints to the new spaces and module
state. Retain delivery progress so another device can catch up after the source
device exits. Qualify authorized reachable-device operation without assuming
that offline authorization makes an unreachable endpoint available.

Keep desired state, local bindings, observations and commands/media distinct.
GitHub remains the project-code/release authority; NDS does not mirror every
working directory. An action with external effects has its own outcome protocol,
not automatic replay from a settings queue. Use causal/revision rules rather
than largest wall-clock timestamp for concurrent changes.

Done when two devices survive offline edits and restart, duplicate/reordered
delivery, concurrent conflict, stale deletion, expired history and reconnect.
Scope changes, offline expiry and revoked membership preserve encrypted pending
work and block effects that no longer have authority.

### P4. One module/tool runtime contract

Owners: Core application, Protocol, Agent and actual module consumers.

Introduce the versioned namespaced boundary through an existing small tool and
a real functional module. Cover discovery, compatibility, dependencies, scoped
configuration, effective action profile, execution, health and exact-owner
shutdown. Reuse retained task/results and bounded native I/O; do not build a
generic plugin VM, duplicate scheduler or unused provider framework.

Kernel libraries can remain statically composed. Independently delivered
engines use the supported supervised boundary. Alpha admits official modules;
the contract must not require a built-in enum or private UI switch for each
ordinary future module. Unknown interfaces and privileges remain unsupported.

Done when both real consumers use this path, version mismatch refuses before an
effect, and cancellation/replacement/lost acknowledgement cannot act on a new
owner. A failed or unknown cleanup keeps its original resources accounted for.
Applications and agents exercise the same use case and result contract.

### P5. Complete GDS workflows

Owners: GDS with kernel identity, Vault, workspace and client consumers.

Finish the existing owned engine: accessible GitHub catalog, stable repository
identity, workspace connection, fetch, eligible publication of existing commits,
fast-forward and actionable divergence/conflict handling. Keep uncommitted work
and active writers intact. Automatic commits, force updates and history rewriting
are not added by the fullauto admission setting.

Use the declared Git hierarchy for source placement. A shared module may be
embedded under several consumers while retaining one repository identity;
distinguish those declared locations from competing standalone roots. Exercise
this with the root server and the public example's nested server before
claiming complete recursive placement auditing.

Done when an actual authorized repository workflow converges between devices,
including rename, offline restart, a dirty checkout, concurrent divergence,
source/credential refusal and cancellation. Exercise the declared repository/
device capacity and observable backpressure. A legacy installed GDS executable
is not a hidden prerequisite.

### P6. Complete RDS workflows

Owners: NDS RDS host/session/transport/viewer and platform adapters.

Migrate reusable archived code into an NDS-owned implementation with its own
contract, tests, provenance and lifecycle. Preserve existing identities and
consent rather than replacing the legacy installation in place. Start with
actual host/viewer connection, frame progress, reconnect, expiry, revoke and
clean closure. Viewing is the current accepted baseline; keyboard/pointer input
requires its explicit product-scope amendment before implementation.

Done when actual qualified host/viewer pairs work through the kernel and source
loss/reconnect cannot change endpoint identity or restore revoked authority.
Qualify platform host and viewer roles separately. Metadata or a report file
alone does not establish presented frames, graceful exit or recovery.

### P7. Finish browser and tool integration

Owners: browser-use, sysinfo, clipboard, cleaner, account adapters and clients.

Adopt the compatible qualified browser engine, including its actual wire and
script outcome types. Preserve exact session ownership and terminal release,
timeout/abort behavior and replacement safety. Keep its standalone product and
NDS integration on the same owning implementation.

Qualify real system information, OS clipboard sharing, owned cache cleanup and
supported official account operations through the common runtime. Each tool
keeps its own provider state and useful scope. An unavailable provider or
unsupported account operation remains explicit, not a fake successful adapter.

Done when user and agent calls exercise each declared alpha function with actual
dependencies, scoped diagnostics, failure, cancellation and shutdown. No broader
platform/provider support is inferred from a compiled interface.

### P8. Real application and module updates

Owners: Updater, supported platform installation/launcher owners and kernel
lifecycle. Read-only external OS-update reporting remains a distinct capability.

Extend verified discovery/staging to actual activation: identify the installed
application, verify the candidate, close admission, await a durable boundary,
switch the owned installation, restart and observe the new running result.
Support independently versioned module updates and explicit compatibility.

Done when one actual application update and one supported module update preserve
keys and pending work. Exercise interruption, bad signature, wrong target,
incompatible schema and failed startup. Restore an older executable only when
it can read the retained data; otherwise retain a clear roll-forward disposition.
Retaining a previous program artifact is not a backup of user databases.

### P9. Coherent clients and platform acceptance

Owners: shared composition, Desktop, Mobile, Web and design system.

Keep consumer pins, generated Rust/Dart/WASM/types, localization and actual
artifacts coherent throughout earlier phases. Do not postpone broken consumer
updates until this phase. Here qualify the full assembled product and consistent
user/agent presentation of spaces, devices, modes, modules and diagnostics.

First exercise the Linux application, an explicitly owned Android emulator and
the actual Web client. Then close the declared macOS/Windows/iOS target matrix
with their own platform evidence. Test sign-in/enrollment, real module actions,
offline restart, multi-tab behavior where applicable, conflict resolution,
profile/identity changes, update and shutdown without clearing retained data.
Builds, emulators, physical devices and production signing keep distinct claims.

### P10. Alpha publication and operation

Owners: existing component release/build/deployment owners and Root assembly.

Publish one coherent product composition with independently identified module
artifacts, compatibility, integrity/provenance and useful self-hosting commands.
Make catalog and capability labels agree with actual selected support. Keep the
component matrix complete, including data-only/shared components and explicitly
unsupported runtime roles; a catalog entry is not a working feature.

Done when a fresh installation and a retained-data upgrade run the declared
workflows, telemetry can explain their results, and each claimed platform has
its actual acceptance. Source/CI, publication, installation and running-product
states remain distinct. Live operational selection has its own fresh target,
protected-work and recovery authority; this plan creates no maintenance window.

## Current checkpoint and next increment

The [P1a personal-context source cohort](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync/pull/118)
is delivered through Vault, GDS, Agent, Shared, Mobile and Web. It preserves
original encrypted ownership and propagates the retained native follower read
through the actual consumers. The isolated Linux, browser and selected Android
emulator flows passed. Android exercised sign-in, encrypted Vault/text sync,
retained session and record across two app processes, client replacement,
device revocation and logout. Its owning test driver preserves preexisting ADB
bindings and the installed application, using an isolated test namespace.
The [Shared harness change](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-desktop/pull/142)
adds the explicit remote-ADB debug port without changing the product runtime.

Physical-phone and iOS runtime qualification remain open. These selected flows
do not complete P1, the full client/module matrix, searchable diagnostics or
alpha acceptance. Reuse the canonical Shared acceptance harness instead of
committing temporary application IDs or duplicate fixture helpers into Mobile.

The P1b candidate now implements personal/multiple-organization membership
through those existing boundaries. The
[Shared composition](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-desktop/pull/144),
[Web consumer](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-web/pull/72)
and [Mobile consumer](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-mobile/pull/86)
retain the original personal owner and captured selection/revision across awaited
operations. Isolated Linux, real browser and the selected Android emulator
scenarios passed personal/A/B selection and invitation isolation. Browser
acceptance also drops an actual committed reply and explicitly retries the same
command after connection verification; Android retains encrypted Vault state
across two application processes. Integration-test artifacts remain private and
cannot substitute for a clean distributable package.

The Shared source cohort is delivered to MAIN. Web and Mobile source updates
are delivered to dev; final MAIN publication, clean final artifacts and the
coherent Root cohort remain open. Native qualification must use the selected
execution, storage and tool-profile route; a local profile alone does not prove
remote artifact storage or an admitted execution target.

The kernel's
[native workspace-storage operation](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync/issues/124)
is a separate cross-host provider/consumer capability. It is not a prerequisite
for ordinary code checks on an admitted local filesystem. Its eventual remote
consumer still needs supported storage semantics and a retained original-target
lease; Git synchronization, display transport and journal metadata do not supply
a shared compiler filesystem.

Close the [Vault local-read publication gate](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-vault/issues/134)
before final adoption: a read retained across cancellation and same-owner
reauthentication must validate its original authority, preserve encrypted state
and permit a new explicit read. Its source correction and static checks do not
substitute for the actual native fixture and consumer qualification.

P1c starts after the coherent cohort is accepted, with stable device
purpose/placement and a real fullauto/manual action. Carry the minimal correlated
diagnostic context through both increments; P2 closes the searchable,
access-controlled route. These P1b results do not complete the remaining module,
update, diagnostic or full-alpha acceptance.

Each increment changes the canonical owner and its actual consumers together,
preserves retained data, exercises relevant positive and denied/failure paths,
and reports the result at its real boundary. Pure rule tests do not finish a
store, platform or delivery claim. No duplicate manager, private-corpus copy or
speculative abstraction is an implementation milestone.

Keep component tasks in the existing GitHub ownership hierarchy and link lasting
decisions to their canonical contract. Update this one active sequence when
scope changes; retire it when its work is complete. Estimate calendar delivery
after the first coherent vertical exposes actual migration/integration cost.

## Deferred follow-up: managed VPN design

After the current alpha work, discuss an NDS-owned VPN module with the owner
before implementation. The intended outcome is a protected network between the
actor's devices through an operator-selected gateway, improving reachability
and access to shared services such as the development cache. Personal and
organization spaces remain isolated; network membership does not grant access
to another space or replace service authentication.

Use the [Happ-proxy repositories](https://github.com/orgs/Happ-proxy/repositories)
as a reference for study, with license and security review before reuse. This is
an owned-device/organization capability, without a subscription business model.
No engine, routing policy or fallback transport is selected by this plan.

The later design discussion must cover gateway and peer identity, device/key
revocation, full versus selective routing, DNS and IPv6 behavior, reconnect and
bounded fallback, gateway loss, cache/RDS coexistence, supported OS networking
interfaces and safe shutdown/update. Fallback must preserve authentication,
scope and the selected traffic policy. Reuse the kernel's identity, lifecycle
and diagnostics rather than another account or orchestration system.

Compare measured latency, throughput, resource use and recovery under declared
packet loss and gateway failure. A tunnel does not repair an unavailable
physical link; reliability and performance claims need actual target evidence.
Private topology, endpoints and keys remain outside public source and telemetry.
This deferred item creates no current repository, dependency, VPN installation,
route change, cache activation or maintenance-window authority.
