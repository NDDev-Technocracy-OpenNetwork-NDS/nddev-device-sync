# Identity, authorization and vault

## Product identity

NDS supports email OTP, GitHub OAuth with PKCE and
passkeys for one account. There are no passwords or hidden test-login paths.
[ADR 0006](decisions/0006-agent-first-sign-in-and-vault-access.md) supersedes the
two-method restriction in ADR 0003. Advertise a method only when the running
instance enables its implementation; preserve the original scope of older locks.

An operator privately configures the bootstrap owner address. No public
registration or first-visitor ownership claim exists. The owner has a stable
internal user ID, independent of the email address; identity comparisons follow
one explicit normalization rule. Real addresses and provider credentials never
belong in public source, fixtures or manifests.

[ADR 0011](decisions/0011-company-membership-and-device-access.md) adds companies
and invited users. [ADR 0017](decisions/0017-extensible-offline-kernel.md)
supersedes its one-company restriction: a stable user identity has a personal
space and may hold memberships in multiple organizations. The creator is the
first administrator; existing singleton/tenant-bound identity implementations
require an explicit migration and qualification. All sign-in methods resolve
verified users through the same membership policy.
Administrative access extends to same-company remote screen, not another user's Vault or
personal module data. Diagnostic visibility follows
[ADR 0018](decisions/0018-control-and-diagnostic-profiles.md): members see their
own records, organization administrators see their organization's records, and
a separately key-authenticated global NDS administrator sees all diagnostic
spaces. Organization administration does not include a member's personal space.
The global diagnostic credential is distinct from Vault encryption/wrapping
keys; it cannot make secrets into permitted diagnostic content.
Organization-owned shared data uses its explicit space
and policy. Selection of a space never changes another retained operation's
actor, credential or authorization lifetime. Leaving or being removed from one
organization cannot rename the personal space or another membership's data.
The existing bootstrap's stable IDs and keys are preserved.

OTP challenges use a cryptographically secure generator, finite lifetime,
bounded attempts, resend limits and bounded storage. A resend invalidates the
previous code. Successful verification atomically consumes the challenge;
concurrent replay must not create another session. Persist a protected verifier,
not a plaintext code or an easily brute-forced unkeyed hash of a short code.
Responses must not reveal whether an address is permitted. Enforce both subject
and source abuse limits without unbounded rate-limit state.

The server sends codes through a narrow email-delivery adapter. Delivery
credentials belong to the operator's secret store. Provider acceptance is not
proof of mailbox delivery. Sessions have explicit expiry, revocation and
authorization scope. Email possession authenticates the account; it does not
prove possession of a device key or supply a vault encryption key.
Email OTP trusts mailbox control; it is not phishing-resistant multifactor
authentication. Delivery and authentication secrets never become log fields.

GitHub sign-in uses an unguessable single-use state, PKCE S256, an exact
allowlisted callback and bounded one-time handoff to the initiating client.
The server revalidates the provider identity after code exchange and matches
its stable numeric ID to an explicit private owner binding. Never merge accounts
by a client-supplied identity, display name or unverified email. All methods
issue the same NDS session type and enforce the same authorization policy.
The operator stores the OAuth application secret separately; provider tokens
are transient for identity verification and are not retained as product
credentials. Linking a provider requires explicit authenticated revalidation
or an operator's preconfigured binding; it is not open registration.

Passkey registration and authentication use WebAuthn with exact configured RP
and origin binding, required user verification and bounded single-use server
challenges. Public credentials bind to the stable NDS user ID. Enrollment,
linking and revocation require authenticated revalidation; they never claim an
instance or merge users by a client-supplied label. Native and browser adapters
must report actual platform support and preserve the shared session policy.
Owners can list bounded public passkey metadata and remove a credential after
a recent sign-in. Removal atomically invalidates sessions linked to that
credential; legacy passkey sessions without a credential binding also expire.
Other sign-in methods retain their sessions. No credential verifier is a UI DTO.

## Device and service identities

A per-installation signing key is generated on the device; the private key
stays in native secure storage. After user authentication, a short-lived
enrollment challenge and proof-of-possession bind the public key and device
metadata to the authenticated owner. Revocation and explicit key-rotation
transitions are enforced by the server and preserve authorization boundaries.
Server records include their public identity, capabilities and heartbeat.

Device profiles, administrative scope and workload placement follow
[architecture](02-architecture.md). Reclassifying or renaming a device cannot
rebind its identity, transfer data ownership or enlarge permissions. Serving the
NDS coordinator is a workload on a device, not a second kind of device proof.

Service identities for agents, email delivery and telemetry are separate from
user/device credentials. Vector/OpenObserve credentials remain in the central
telemetry pipeline. Authorization is checked at the owning use case and storage
boundary, not inferred from a client-supplied tenant or user ID.

## Vault

The authenticated owner is entitled to vault access through all three sign-in
methods. The server delivers the owner's vault key to an authenticated,
enrolled, active device independently of other devices' availability. Native
clients prepare access automatically; authentication does not itself derive
the encryption key. See ADR 0006 for the explicit transition from older locks.

Harness tokens, SSH credentials and API keys are encrypted records. PostgreSQL
stores ciphertext and authenticated-encrypted owner key envelopes. A distinct
operator wrapping key remains outside PostgreSQL. The server has decryption
authority; do not describe this model as client-only E2EE.
Each record has an owner, personal/organization space, purpose, scope and
lifecycle. Adding organization spaces requires explicit key/envelope and
storage namespace design; never reuse a personal key as a company-wide grant or
relabel existing ciphertext to make a migration pass. Native credential stores
hold local secrets; metadata stores and UI events hold references or redacted
metadata, never plaintext secret bytes. Public DTOs and logs cannot expose
resolved secrets. OTP verifiers and session records follow the separate
authentication contract, not the vault encryption model.

The browser credential adapter and bounded cached offline authorization follow
[ADR 0007](decisions/0007-offline-browser-client.md), as amended by
[ADR 0009](decisions/0009-first-alpha-product-contract.md): the first-alpha target
is seven days from successful server verification, with authorized online renewal.
Existing shorter grants keep their original expiry until revalidated; the
implementation transition must be qualified before the longer window is advertised.
Its encrypted operational
store is distinct from both native keyrings and the ciphertext replica. Browser
storage must not be represented as native secure hardware or an immediately
revocable offline session.

The server creates or returns one established owner key transactionally and
rechecks account/session/device authorization at delivery. Existing locally
created vaults register their retained key once through authenticated native
HTTPS; a different key cannot overwrite the established binding. The device
stores delivered keys in its native credential store. Key material never enters
Flutter, agent transcripts or logs. Account switching uses only declared
official harness support. No password fallback or recovery workflow is added.

Records expose bounded cursor pages, committed revisions and observation
freshness. Deletion is an encrypted tombstone. Conflict resolution is explicit
and guarded by both revision and observed server sequence; concurrent changes
cannot silently erase a later conflict. Authenticate and validate every incoming
record before advancing an inbox page. A committed local outbox mutation returns
its transaction receipt without depending on a subsequent network/key-store read.

## Recovery policy

The product creates and retains no backups or recovery copies of the vault,
device keys, databases, local state or generated evidence. Server key envelopes
and native caches are the authorized operational stores, not recovery copies.
Loss of the server wrapping key and every usable native vault key leaves the
encrypted records unrecoverable. Email OTP cannot regenerate those keys.
Retry queues are bounded transport state with
explicit retention, not recovery copies.
