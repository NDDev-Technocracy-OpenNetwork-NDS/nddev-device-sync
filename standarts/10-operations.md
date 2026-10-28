# Operations

Every service exposes a health endpoint and emits startup, shutdown, version,
dependency, migration and degraded-state events. Graceful shutdown drains
bounded queues, closes database pools and reports the final delivery result.

The server agent is outbound-first and reports inventory and health using its
device identity. Its supported typed operations use authenticated original-space
authority and the effective fullauto/manual policy. Fullauto requires no separate
per-action user checklist; unknown operations and cross-space authority remain
refused. Actual OS/provider restrictions are reported honestly. SSH credentials are vault records with a defined
purpose and scope; they are never copied into logs or deployment scripts.

The alpha operational loop is:

1. build and attest artifacts;
2. deploy a pinned server image;
3. run migrations with a dedicated migration identity;
4. verify health, sync, vault, telemetry and alert smoke checks;
5. enroll or revoke devices explicitly;
6. watch OpenObserve and Rust alert state;
7. promote or stop the release channel based on receipts.

The product's no-backup/no-recovery policy is intentional and must be visible
in operator documentation. Operational dashboards must show storage pressure,
event loss, database connectivity, migration version and device enrollment
state so the absence of recovery copies is never hidden.

