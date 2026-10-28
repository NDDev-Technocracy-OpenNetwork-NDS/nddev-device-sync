# Observability and telemetry standard

Rust processes, Flutter/React clients, agents, modules and operational tools use
one typed diagnostic contract. Their collection follows the selected profile
under [ADR 0018](decisions/0018-control-and-diagnostic-profiles.md). Observability
explains actual behavior; it does not replace error handling or the functional
journal. A failed diagnostic sink does not turn a successful business action
into a failed action or claim that an uncertain effect was rolled back.

## Canonical envelope

Use applicable fields with stable names and bounded cardinality:

```text
timestamp
severity
service.name
service.version
deployment.environment
release.channel
release.version
source.repository
source.commit
module
event.name
operation.id
operation.attempt
space.id
device.id
server.id
tenant.id
user.id (pseudonymous where possible)
trace_id
span_id
error.type
duration_ms
outcome
```

Common process/build metadata is attached centrally, including startup,
shutdown, migration, TLS reload and process failure, not only HTTP spans.
W3C trace context crosses service/module boundaries. Build/test evidence adds
build ID, toolchain, target, suite, schema/migration version and artifact digest.
Metric labels never use unbounded IDs, user content or raw URLs. Durations use
local monotonic measurements; source time and receiver time retain their actual
meaning. Cross-device wall-clock ordering is not proof of causal order.

## Useful coverage without noise

Each admitted action has a stable operation identity and correlated trace.
Async handoffs and retries retain the initiating actor/space and distinguish
attempts. Significant state changes and terminal outcomes explain what actually
happened. Refused admission, failed-before-send, requested cancellation, known
completion, partial effect and unknown completion stay distinct.

The owning boundary records the classified cause once. Forwarders preserve its
identity and add relevant context without printing the same exception at every
layer. Repeated unchanged polling and high-frequency observations belong in
bounded metrics or aggregated diagnostics, not repetitive success messages.
Routine detail may be sampled; action failures, security transitions and terminal
outcome records are not intentionally sampled. Forced process loss can prevent
a final record, so missing completion remains unknown rather than successful.

## Modes

- **Standard** is the default: useful lifecycle/security transitions, operation
  outcomes, warnings/errors, metrics and correlated traces. It excludes arbitrary
  private content and repetitive low-value detail.
- **Debug** adds authorized operation inputs/outputs and diagnostic state useful
  for development, with explicit scope and finite collection/resource limits.
  Its effective scope/lifetime is visible. It can include permitted non-secret
  AI/tool interaction content; it does not silently enable continuous screen
  recording, capture every file or inspect arbitrary object internals.
- **Off** stops new diagnostic collection and export. Discard unsent records of
  the affected diagnostic scope, account for requests already in flight and do
  not replay discarded data after re-enabling. Existing centralized history
  expires under retention. Functional journals, current health/error presentation
  and authorization remain operational; they are not a covert telemetry export.

Every collecting profile excludes OTPs, passwords, tokens, cookies, private keys,
authorization headers, resolved Vault secrets and secret-bearing URLs before
buffering or export. Inputs, output and exceptions are classified through owned
typed projections, never an arbitrary Debug/object/command-line/environment dump.
Debug expands permitted detail, not credential visibility. Content capture is
separate from ordinary diagnostic metadata and never becomes a metric label.

Profile changes are authorized against the original scope. A retained event or
late result cannot inherit another user's or replacement lifetime's permission.
Report configured profile, actual collection/export state, delivery freshness
and loss separately. A selected profile is not proof that collection works.

## Delivery and alerts

```text
Applications/modules -> owned SDK/intake -> Rust policy + Vector -> OpenObserve
```

Redaction happens before events leave a process. Reuse one owned event/redaction
contract; collectors may apply additional checks. Only the central pipeline holds
OpenObserve ingestion credentials. Native deployment configuration owns the
actual route and its signal-specific hops; applications do not bypass the policy
boundary to write directly to storage. Vector may collect explicitly owned
journald/container sources under the same classification and access rules.

Queues, batches, retry count/backoff, storage retention and disk use are bounded.
When the network or centralized store is unavailable, ordinary work continues;
retain diagnostics within the local spool's finite capacity and send after
recovery. Overflow, expiry and failed persistence expose their measured loss or
unknown state. Retrying an acknowledged record does not create a new operation.
An unavailable diagnostic path cannot bypass an authoritative business receipt
or turn successful transport into a claim of searchable storage.

## Access and retention

Members see their own diagnostic records. Organization administrators see all
records in their organization, including member records, but not those members'
separate personal spaces. Global NDS diagnostic administration sees all spaces
only through the separate key-authenticated role in ADR 0018. Enforce this in
trusted ingest/query/export boundaries, including detail, counts and artifacts;
UI filters and caller-supplied scope are not grants.

OpenObserve is the current centralized store and review surface. Do not give
ordinary clients shared storage credentials that bypass NDS access policy.
No external notification channel or actual destination delivery is claimed.

Diagnostic logs, trace/metric history and permitted content use one-week
retention. Local retry and centralized ingestion retain the original expiry;
repeated delivery cannot extend it. Bound local disk use independently of age.
Qualify actual expiration and storage cleanup rather than assuming that a setting
proves immediate physical erasure. Current incident/configuration state and
authoritative business records have their own lifecycle, not a seven-day timer.

Every service has liveness, readiness and dependency health. An alert has an
owner, severity, deduplication, acknowledgement, silence and resolution state,
with a finite incident lifecycle. Verify notification delivery when a channel
is configured; a created incident is not evidence of a delivered notification.
The visible diagnostic profile and known delivery failure remain honest even
when collection is off. Missing/stale data cannot prove incident recovery.

## Acceptance signals

For implemented flows observe latency/errors, sync conflict rate and outbox age,
device/agent heartbeat age, database and migration health, resource pressure,
telemetry delivery age/drops and release health. A dashboard with stale or
missing telemetry is degraded. Acceptance follows an actual correlated event
into OpenObserve and exercises an alert through resolution; mock events do not
prove the application path.
