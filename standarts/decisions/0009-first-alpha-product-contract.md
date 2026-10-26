# ADR 0009: NDS 1 first-alpha acceptance

Status: accepted owner direction, 2026-10-15; implementation and acceptance pending.

## Product boundary

NDS 1 supports at most 32 active devices per user. The original single-owner
baseline transitions to companies and invited users under
[ADR 0011](0011-company-membership-and-device-access.md). Signing in establishes the
account context, enrolls the current installation through the existing device
proof, prepares Vault access and exposes the module catalog. Adding a module
records its desired state for that device and reports actual installation,
permission and readiness separately. All clients use the same Core rules.
Presentation is Flutter on desktop/mobile and React/TypeScript on web, as
amended by [ADR 0010](0010-react-browser-presentation.md).
OS-required bindings remain narrow I/O adapters.

There are six existing native tool adapters: Sysinfo, Clipboard, Cleaner,
Updater, GDS and RDS. The distinct harness catalog contains eight names:
Claude Code, Codex, pi, agy, Devin, Grok, Cursor and OpenCode. Catalog presence
does not assert an implemented provider capability. Use declared provider APIs
and protected credential references for supported operations.

Company membership has admin and user roles under ADR 0011. An administrator
manages membership and may view members' devices through authorized RDS; personal
Vault, secrets and GDS data remain owner-only. Company isolation and membership
are now alpha acceptance requirements, not claims about the current singleton
server implementation.

## Shared state and continuous operation

All NDS-managed configuration, supported module observations, actions and
terminal outcomes have a canonical owner, scope, revision and freshness.
Core defines their models and transitions; adapters perform I/O. Provider
databases remain provider-owned, Git content remains in Git and secrets remain
in Vault. Screen frames are transient media, not telemetry or a recording feature.
Collection covers each installed module's supported data contract, including
unavailable, refused, pending, conflicted and failed outcomes.

Native desktop and headless installations reconcile through a single owned
Rust background lifetime, independently of an open Flutter window. Mobile and
web adapters use the execution opportunities provided by their OS/browser and
catch up on resume/reconnect. A suspended process must not claim to be online.

Offline authorization is limited to seven days since the last successful
server verification. Online renewal requires current owner/session/device
authority; an offline clock, restart or cached object cannot renew it. Expiry,
local logout and known revocation lock protected use while preserving encrypted
unsent work. Reconnect revalidates before upload. Existing shorter grants retain
their original expiry until a new authorized server decision; this ADR does not
retroactively extend them. Remote revocation cannot be learned while disconnected.

## GDS

Owner amendment, 2026-10-24: GDS is the existing `nddev-device-sync-gds` module
of NDS (`nddev-device-sync`). The canonical desktop/server workspace is
`~/Developer/NDS/<github-owner>/...`. Personal accounts and organizations have
separate owner directories. GitHub repositories remain flat; local containment
follows declared Git submodules and their pinned revisions rather than creating
an additional flat checkout for every nested repository. Stable GitHub IDs
retain identity through rename or transfer. Existing work, including the earlier
`~/Developer/nddev` layout, is preserved; relocation requires an explicit owner
decision. Older immutable standards locks retain their original path contract
until their consumer adopts this amendment through a coordinated source update.
Mobile storage uses the same logical layout in an authorized
app-owned workspace. Browser filesystem capabilities must be explicitly granted
and reported; a remote workspace view is distinct from a local checkout.

An enabled GDS module maintains the accessible repository catalog and performs
bidirectional synchronization of existing commits. It fetches remote changes,
publishes eligible local commits and updates clean worktrees by fast-forward.
Uncommitted work is preserved. Divergence, protected-branch refusal, occupied
paths, active writers and permission failures block the affected repository
with an actionable outcome. Do not invent commits, force-push, reset dirty work,
rewrite history or turn synchronization into implicit pull-request approval.

Catalog reading grants no Git write authority. The separate device-local
`git_write` permission authorizes bounded checkout connection and existing-commit
publication/update, subject to the original account, source, credential and
module-consent checks. Removing it prevents subsequent Git mutations without
deleting existing work. Checkout bindings identify the NDS server, tenant, user,
device and stable GitHub repository/owner IDs. Encrypted NDS state owns one
connection intent per repository across the account. Physical Git markers stay
device-local additional witnesses, never grants; 1024 repositories on32 devices
do not create32,768 replicated binding entities.
Older clients which do not understand this permission must refuse it; existing
read-only descriptors remain valid under their immutable standards locks.

Discovery, filesystem placement, operation ownership and restart progress use
one canonical NDS model. Relocation preserves existing repositories, worktrees
and user edits. Every scan, queue, Git operation, retry and resource allocation
has finite bounds and observable completion.

The first-alpha capacity target is at least 1024 repositories per owner across
32 devices. Qualify concurrent initialization, unchanged repeated scans,
conflict settlement, restart and sustained synchronization at that size.
Shared repository/owner facts are not duplicated per device. Account for
device-specific observations separately; preserve original requests and receipts
under pressure rather than hiding saturation by increasing quotas.

## RDS

An enabled RDS viewer can view any enrolled, authorized target with its RDS
host module enabled and platform capture access granted. Viewer and host support
are qualified separately on desktop, Android, iOS and web. Unsupported OS
capabilities remain explicit; they are not counted as working implementations.

Use the existing Rust RDS transport and media boundary, Core authorization and
frame lifetime rules, Accounts credentials and typed presentation viewers. Platform
capture consent, revocation, expiry and local disable remain binding. This is
view-only scope: no remote keyboard/pointer, shell or file-transfer expansion.

## NDS application updates

NDS owns update discovery, verified download/staging, progress and an Update
action for its own application. A canonical release manifest binds source,
platform, channel, digest, publisher verification and data compatibility.
Activation preserves keys and queued work and waits for owned operations to
reach a safe durable boundary. Interrupted or invalid downloads cannot replace
the installed application; incompatible data cannot be silently downgraded.

The current alpha engineering phase uses project-built artifacts. Platform
registrations, production signing identities and store submissions are deferred
until separately approved release preparation; they do not block this phase.
Build integrity, provenance, compatibility and actual workflow acceptance remain
required. Debug and simulator builds do not qualify production distribution.

Store-distributed mobile builds use their store update mechanism. A separately
distributed Android APK uses the system installer, the established signing
identity and explicit platform approval. iOS uses supported Apple distribution.
Web updates preserve active work and use the verified app-shell transition.
The existing Updater tool's policy/report adapter remains a separate capability.
No backup/recovery or general executable-plugin system is introduced.

## Observability and acceptance

The shared Rust SDK and server pipeline collect correlated, redacted state and
operation outcomes from every NDS device and enabled module. Scope, timestamps,
source/build identity, retries, refusal, conflict, loss and delivery status are
explicit. Queues and retention remain bounded. A transport acknowledgement is
not evidence that an event is searchable in the server's observability system.

Use the latest stable versions of the approved stack, recorded with immutable
inputs. Compatibility exceptions require a concrete upstream issue, tested
reason and removal condition. Beta/nightly adoption is not implied. One release
manifest identifies the accepted component set; internal migration numbers and
historical immutable tags do not create separate NDS product generations.

Acceptance requires real account/device enrollment, seven-day offline boundary
and reconnect, module lifecycle/data collection, bidirectional Git synchronization,
authorized screen pixels, interrupted application updates and searchable shared
telemetry on the claimed targets. Browser interaction uses Playwriter and Android
qualification uses ADB against an explicitly owned emulator/device. Other claimed
platforms require their own actual evidence. Existing execution safeguards and
branch protection remain in force; compilation alone cannot close these gates.

References: [Android media projection](https://developer.android.com/media/grow/media-projection),
[Android background limits](https://developer.android.com/develop/background-work/services/fgs/timeout),
[Android in-app updates](https://developer.android.com/guide/playcore/in-app-updates),
[Apple distribution rules](https://developer.apple.com/app-store/review/guidelines/).
