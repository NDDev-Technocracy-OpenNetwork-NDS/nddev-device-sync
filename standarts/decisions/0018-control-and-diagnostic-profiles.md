# ADR 0018: trusted automation and diagnostic profiles

Status: accepted owner direction; implementation and migration pending.

## Decision

NDS exposes one action contract to applications and agents. A new setup defaults
to `fullauto`: a trusted agent has complete control of the devices and resources
in the personal or organization space it acts for. NDS adds no execution sandbox,
per-action permission prompts or manual capability checklist in this mode.
`manual` lets the owner configure narrower access. Other users' personal spaces
and other organizations remain separate.

The mode is an authenticated space/device policy, not an untrusted flag supplied
with a command. Identity, current membership, original target and generation
still determine whose environment is being controlled. Unknown interfaces,
invalid inputs and unavailable features remain errors. The runtime reports
actual OS/provider restrictions rather than fabricating privileges. Fullauto
changes action admission; it does not implement a missing module function or
change the synchronization algorithm into arbitrary command replay.

Existing explicit disables and restrictions survive migration as manual choices.
Changing a profile closes the old admission lifetime, accounts for work already
admitted and prevents a late result from acquiring the replacement's authority.
The existing task/result/drain owners remain responsible; no second executor or
permissions service is introduced.

## Diagnostic decisions

The product offers `standard`, `debug` and `off`. Standard is the default and
explains actual operations and relevant failures without repetitive noise.
Debug adds authorized input/output and state useful for development. Secrets
remain excluded in every mode. Off stops new diagnostic collection and export;
functional state, permission enforcement and user-visible errors still work.

Diagnostic payloads and history have a one-week retention policy. A retry does
not create a new retention period. Current configuration, active incident state
and authoritative business receipts retain their own functional lifetimes.
The storage implementation must establish its actual expiration/deletion bounds.

OpenObserve is the current centralized diagnostic store and review surface.
No external notification channel or destination delivery is claimed. The SDK,
bounded transport, Rust policy service and Vector keep their existing owners.
An outage does not stop ordinary work: queue within finite local limits, send
after recovery, and expose overflow, expiry, loss and unknown delivery honestly.

Members see their own diagnostic records. Organization administrators see the
organization's records, including its members' records. They do not thereby see
members' separate personal spaces. A global NDS administrator can see all
organizations and personal diagnostic spaces.

## Global administrative access

The global diagnostic role requires a separate encrypted administrative key in
addition to an established NDS identity. A normal session or an organization
administrator cannot self-promote into this role. Trusted operator enrollment
binds its public verification key and authorized principal.

Use established cryptographic and native credential-store facilities. Keep the
private key encrypted at rest; prove possession by signing a fresh, bounded,
single-use challenge bound to the intended NDS service and principal. The server
needs the verification key, not the private key. A successful exchange establishes
an explicit finite administrative session. The unlocked agent can then operate
without per-action human prompts. Revocation and rotation have owning lifecycle
operations and cannot be replaced by replaying an old proof.

This is an access credential, not the server's Vault wrapping key or a universal
data-decryption key. It does not authorize putting credentials into diagnostics.
Storage/host operators remain a separate trust boundary; an application login
key does not encrypt an entire OpenObserve installation against its operator.
This decision introduces no backup or recovery promise for the administrative key.

## Public contract boundary

Public source describes NDS behavior, interfaces and supported checks. Private
engineering policy and its adoption evidence remain private and are not copied
into product source, fixtures, artifacts or public CI output. Supported public
checks must not need access to a private policy corpus. Preserve legitimate
public dependency attribution and licenses.

## Changes to earlier contracts

- Add fullauto/manual to ADR 0017's scoped extension boundary. Replace a mandatory
  per-capability consent checklist as the only supported action-admission model.
  Preserve explicit manual restrictions, identity and original-scope fences.
- Extend the earlier organization-administrator model with diagnostic visibility
  and separately authenticated global diagnostic administration. Personal Vault
  secrets remain outside diagnostic DTOs and logs.
- Replace export-only opt-out with off as a collection/export policy. Discard
  unsent records of that diagnostic scope; account for already in-flight delivery.
  Existing stored history expires under retention rather than being presented as
  immediately erased. The functional journal is not a telemetry queue.
- Permit authorized non-secret content in the protected debug profile instead of
  treating every diagnostic mode as a metadata-only projection. A screenshot,
  continuous screen recording or indiscriminate filesystem dump is a separate
  feature, not an automatic consequence of selecting debug.

Old issued locks keep their original meanings. This source decision neither
changes running installations nor proves the new mode, access or retention works.

## Implementation and acceptance

Core and Protocol own profile/scope models; Accounts, Organizations and Server
own authenticated membership and diagnostic access. Agent and existing module
callers enforce the effective action policy. The SDK/collector owns diagnostic
projection, buffering and delivery; presentation exposes the same typed contract.

First qualify fresh and retained personal/organization data through those owners.
Exercise fullauto without per-action prompts, manual refusal, cross-space denial,
profile replacement during work and preserved user restrictions. Verify global
access with and without the key, replay/wrong-service/expired proof refusal and
revocation before disclosure. An unlocked administrative key never enters an
agent transcript, generated bridge, diagnostic record or presentation cache.

Then follow a real operation into searchable OpenObserve records with scope and
trace intact. Exercise secret canaries, duplicate/error noise, debug/off changes,
outage/overflow/recovery and retention without clearing an active incident from
missing evidence. Keep local admission, remote receipt and actual searchability
as distinct observations. Existing resources and data must survive failed cleanup.
