# Synchronization and data ownership

## Stores

- Native client metadata and offline queues use per-user SQLite. Browser
  durability must satisfy [ADR 0007](decisions/0007-offline-browser-client.md)
  through its qualified browser storage adapter and the same transaction rules.
- The server's authoritative control state uses PostgreSQL 18.
- Logs, metrics and traces use OpenObserve.
- Secret bytes use the encrypted vault and native keyrings.
- Project source remains in Git and is not copied into the sync database.

GitHub carries project history and code/release distribution. NDS synchronizes
its own desired state, scoped module records and observations. Git content is
not mirrored into this journal; commands and transient media use their owning
operation/stream protocol.

The server is the authority for committed NDS control state. Devices have durable
local state and pending changes: startup,
enrollment, reconnect and resume initiate reconciliation without depending on
another device. Bounded background catch-up continues from its durable cursor
until there is no remaining history. Expose whether the replica is catching up,
its pending/conflicted mutations and its last fully caught-up observation;
receiving one partial page does not establish freshness. Authorized local work
and reachable-device operations continue while the coordinator is unavailable,
within the existing offline grant. Availability of a cached grant does not
prove network reachability or current remote membership.

The server retains a durable encrypted operational journal/record store and
delivery acknowledgements so another device can catch up after the source device
has disconnected. This is not a complete copy of every working directory or a
backup feature. The server-managed wrapping-key trust boundary in standard 03
remains explicit; encrypted storage alone does not imply client-only E2EE.
Retention is finite. Committed history may be compacted into authenticated
current-state checkpoints while retaining unresolved conflicts and delivery
boundaries. Capacity exhaustion backpressures admission instead of pretending
to accept a discarded mutation. Offline devices beyond retained history require
an explicit fresh-state catch-up path; pending local mutations remain preserved
and reconciled against that state.

## Operation protocol

Every mutation has an authenticated actor, personal/organization `space_id`,
module namespace, `operation_id`, `device_id`, `entity_id`, base revision,
payload schema version and idempotency key. The server assigns a monotonic
`server_seq`. Clients persist an outbox before acknowledging a local mutation,
then advance an inbox cursor only after applying a verified server operation.

The idempotency scope, request fingerprint and retention are explicit. Reusing a
key with a different request is rejected. Entity state, revision, operation and
original result commit atomically. Concurrent duplicates return that result.
Original scope is part of the idempotency and cursor boundary. Switching a UI
space cannot replay another space's outbox, borrow its receipt or decrypt its
records. Existing tenant-scoped wire/storage versions require a versioned
migration; adding a label to an old DTO does not implement the new authority.
Sequence/cursor semantics must not skip an operation that commits later; a
database sequence allocation alone does not guarantee commit order. Expired
history requires an explicit fresh-state synchronization path, never silent loss.

Retries are bounded and use exponential backoff. Replaying the same supported
state operation is safe and returns the original result. This promise is not
extended to arbitrary module commands with external effects. Those require
their own operation identity, deduplication and retained terminal receipt;
unknown execution or write completion must not be retried as if nothing happened.

Base revisions and commit relationships determine concurrent changes. Wall-clock
timestamps describe observations, not a winner. The kernel detects and preserves
conflict branches; the owning module defines compatible merges and resolution
semantics. Conflicts create an explicit conflict
record containing both revisions and a resolution state; silent last-write-wins
is not used for credentials, devices, permissions or release policy.

Space/user ownership comes from authenticated context and is checked
for every entity and cursor. Offline clients expose pending/conflicted state
and freshness; they cannot claim immediate global consistency while disconnected.
On reconnect, current authorization is checked before accepting offline writes.
Rejected or expired authority does not justify deleting the retained local work
or reporting it committed. Scoped conflict views are available to authorized
users and agents without exposing another space's private contents in telemetry.

Peer delivery authenticates the exact device, actor, original space and grant.
Forwarding preserves the immutable original author/proof and causal parents;
a transport peer does not become the author. Pending changes based on another
pending change must retain that relationship through later server reconciliation.
It may transport permitted pending state or commands; a peer acknowledgement is
not a server commit. Do not introduce a second election/consensus authority or
equate a connected transport with permission. Reuse the canonical versions and
operation identities through peer delivery and subsequent server reconciliation.

The kernel verifies the common encrypted envelope and stores an admitted
module's bounded opaque records independently of whether its payload decoder is
currently installed. Stored/caught-up envelope state and module-applied state
are separate observations. Unsupported payload schemas remain unavailable to
the module; they cannot crash the whole replica, acquire permissions or be
reported as successfully applied. Existing wire versions need an explicit
compatible transition before these new envelope semantics are advertised.

[ADR 0009](decisions/0009-first-alpha-product-contract.md) defines the first-alpha
target for continuous native execution, seven-day offline authorization, complete
supported module-data collection and bidirectional GDS commit synchronization.
These are acceptance requirements, not claims that the current runtime has
already implemented them.

## Data classes

1. Device and module state.
2. Connection registry: endpoints, ports, labels, capabilities and health
   policy.
3. Harness account metadata and encrypted credential references.
4. Server inventory and health snapshots.
5. Audit records and alert incidents.

Each class has a schema version, owner, retention policy and redaction rules.
PostgreSQL migrations are forward-reviewed and exercised from an empty
database in CI.
