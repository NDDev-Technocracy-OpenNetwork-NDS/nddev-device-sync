# ADR 0011: companies, invited users and scoped RDS administration

Status: accepted owner direction, 2026-10-17; implementation and acceptance pending.

Amended by [ADR 0017](0017-extensible-offline-kernel.md): one stable user has a
personal space and may belong to multiple organizations. The exactly-one-company
restriction below is historical; invitations, personal isolation, scoped admin
access, retained identities and qualified migration remain binding.

## Ownership and roles

A company is the top-level NDS tenant. Reuse the existing stable tenant identity;
do not create a second parallel tenant/company authority. In the first alpha,
each user belongs to exactly one company; account identity and its private data
are not shared between companies. A company has active members with exactly two
roles: `admin` and `user`. Its creator is its first
administrator. Creation is an explicit authenticated action within the operator's
registration policy, never a first-visitor instance claim. Existing bootstrap
identity restrictions remain until the qualified membership transition.

Users join through an administrator's invitation bound to the verified intended
identity and company. Invitations are expiring, revocable and single-use;
acceptance and membership creation are atomic. Possession of an invite link alone
does not authenticate a user. Persist protected invite verifiers, redact tokens
and identities, bound outstanding invitations and recheck current inviting
authority on acceptance. Role changes and removals require current administrative
authority and cannot remove the company's last active administrator.

Both roles have the existing personal NDS/module capabilities. The administrator's
additional product-data access is only RDS viewing of enrolled devices owned by
active users of the same company. Company membership/invitation management is
administrative. No administrative role grants another user's Vault keys, secrets,
GDS data, clipboard, sync stream or account credentials. A user accesses their own
devices; an administrator's connection does not transfer ownership of a device.

Each enrolled device identity belongs to exactly one user. Switching accounts
cannot rebind existing keys, pending operations or ciphertext to another owner.
Preserve tenant/user/device scope through sessions, replica cursors, browser
storage, telemetry and all module operations. A claimed company ID or UI role is
never proof of authority. No cross-company administrative access is implied.

## Core and module boundary

Core domain owns company membership, roles, device ownership and access decisions;
Core application owns membership and connection use cases and their ports. The
`organizations` module owns typed client I/O for company and membership workflows.
The existing Rust server composes the authoritative HTTP/PostgreSQL adapters.
Protocol owns generated wire projections. No separate authorization service,
duplicate user database or frontend policy engine is introduced.

The authoritative access decision considers current company membership, actor,
target owner, action, session and device status. RDS additionally retains host
enablement, OS capture consent, expiry and local disable. Revocation or a role
change invalidates affected connection authority; existing streams need bounded
revalidation and closure. Personal authorization remains owner-scoped even for
administrators. Role changes never become permission to decrypt another vault.

## Enrollment, synchronization and migration

Support up to32 active devices per user and ordinary onboarding of all32 within
one hour, including an initial batch of eight. Keep registration, proof attempts,
pending challenges and work queues finite. Apply authenticated enrollment budgets
per owner so users behind one network do not consume each other's account budget.
Unauthenticated sign-in protections remain separate. Qualify actual onboarding,
not just a fixture that shares one already-authenticated session.

Batch connection and synchronization retain independent device proofs, durable
cursors and outcomes. Admit bounded concurrent work and preserve progress when
one device is offline, revoked or slow. A batch does not share a device identity,
skip verification or report all devices caught up after only one completes.

The existing owner's tenant/user/device IDs, Vault key binding, encrypted records,
pending requests and receipts survive migration unchanged. Add a company and an
administrative membership around that established ownership. Replace singleton
constraints and global account quotas deliberately; do not rename or reset
encrypted ownership to fit the new schema. Fresh and existing databases require
separate acceptance before any live migration.

## Acceptance

Exercise invitation issue/expiry/revoke/accept/replay, parallel account activity,
role changes and last-admin protection. With real separate users and companies,
prove own-device access, same-company admin RDS access, user-to-other-user denial,
cross-company denial and admin denial for other users' personal data. Exercise
revocation during an active view and preserve OS capture permission behavior.
Include storage/cache isolation, overlapping batches, actual32-device enrollment,
1024 repositories per owner, restart and common redacted audit telemetry.

This supersedes the deferred-organization and broad admin-visibility wording in
ADR 0009. Billing, enterprise SSO, custom roles, remote input/shell/file transfer,
backup/recovery and distributed infrastructure remain outside this change.

References: [OWASP tenant isolation](https://cheatsheetseries.owasp.org/cheatsheets/Multi_Tenant_Security_Cheat_Sheet.html),
[OWASP authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html).
