# ADR 0008: NDS-owned screen viewing

Status: accepted implementation direction, 2026-10-14; runtime qualification pending.

Amended by [ADR 0017](0017-extensible-offline-kernel.md): NDS RDS owns the full
remote-display implementation; archived libraries are migration groundwork.
Existing identity, OS-consent and view-only boundaries remain binding.

## Decision

The accepted screen-view outcome uses separate NDS-owned endpoints on viewer
and host, composed from the existing Rust RDS transport and media libraries.
Existing native RDS identities, stores, issuers and allowlists retain their
meaning. Native session labels or a successful connection do not prove that a
peer is the selected NDS device. No legacy key extraction or implicit trust
migration is part of this decision.

Core owns the binding, finite-view and frame-lifetime rules. The NDS server
owns authoritative connection metadata and rechecks current owner, device,
binding revision and replay conditions atomically. Binding requires both fresh
enrolled-device authority and possession of the distinct endpoint key, tied to
the same purpose, origin, device, nonce, expiry and revision. Cached metadata
is never permission to start or reopen a view.

Accounts owns endpoint credentials in a distinct purpose namespace of its
existing platform credential adapter. Enrollment and legacy RDS key slots are
not reused. RDS owns transport, capture and codec I/O; shared Flutter owns the
presentation. Private keys, native grants and frame bytes remain outside
presentation metadata, synchronized records and telemetry. No additional
credential database, execution framework or media gateway is introduced.

## Host authority and lifetime

A host starts disabled. Capture requires an explicit local enable action,
current NDS authorization and the platform's capture access. Restored or remote
metadata cannot enable it. On X11, display authentication is an access boundary,
not a per-application capture consent dialog; the local action remains required.

The NDS-owned host issues a finite native DesktopView grant using its own
endpoint key, only for the verified viewer and accepted view intent. There is
no server-wide native grant issuer. Keyboard, pointer, clipboard forwarding,
arbitrary peers, TCP forwarding and file synchronization are outside this scope.
Known revocation, local disable, logout, expiry and disallowed lifecycle end the
owned view and invalidate retained frames. Reconnection cannot silently renew
permission. Frames remain bounded, ephemeral and subject to freshness checks.

## Transition and acceptance

Source/build qualification does not advertise a working viewer or host. Each
claimed capability requires actual authorized source pixels through the Rust
adapter and shared Flutter client, including refusal, cancellation, expiry,
revocation and stale-frame cases. Browser and native targets qualify separately.
Use the existing browser-capable Rust transport with an encrypted blind relay;
do not add another transport stack to obtain browser support.

Preserve existing release locks and credentials. This decision authorizes the
implementation boundary, not live host activation or legacy policy changes.
The no-backup/no-recovery policy remains unchanged.

References: [X11 access model](https://www.x.org/guide/communication/),
[Iroh browser transport](https://docs.iroh.computer/languages/wasm-browser).
