# nddev-device-sync working contract

This public repository is the NDS source root. It owns standards, generic
assembly contracts and exact Gitlinks to independent component repositories.
Public modules live under modules/ and small system tools under tools/.
Keep component implementation and release ownership in those repositories.
Private estate remains a separate protected checkout; never add its Gitlink here.
Never add real credentials, owner addresses, private organization policy,
estate topology, machine inventory or runtime evidence.

Read the [engineering contract](standarts/13-engineering-and-agent-standard.md),
[development workflow](standarts/16-development-workflow.md), current Git state
and the owning module's AGENTS.md, standarts.lock and module.yaml before work.
The owner defines outcomes and trust boundaries; the agent completes authorized
implementation, verification and delivery without repeatedly asking about
routine engineering choices. Preserve unrelated dirty work and live services.
Local build tooling and shared Cargo artifacts follow
[ADR 0012](standarts/decisions/0012-shared-cargo-cache.md).

## Boundaries

- Domain/application stay independent of OS, UI and concrete I/O.
- Native credentials and unwrapped vault keys remain outside metadata stores,
  presentation DTOs and telemetry. Server owner-key envelopes use a wrapping
  key outside PostgreSQL.
- Product identity is separate from device proof, cryptographic key possession
  and third-party harness accounts. Read the locked identity contract and
  ADR 0006: three sign-in methods and automatic vault access are the accepted
  target; the server owns committed state and automatic vault key delivery.
- Modules are compiled-in and manifest-driven; planned entries are not working
  adapters. Add only code used by an accepted behavior.
- Existing GDS, RDS and system tools own their state and safety policies.
  Use explicit adapters; do not copy their databases or bypass their controls.
- Harness account switching requires declared official provider support.
- Follow the approved branch/PR workflow. No implicit releases, deployment,
  credential changes, backup/recovery or unrelated cleanup.

## Verification

Run `just standards-check` and `just catalog-check` before review.
These are structural checks only. Runtime repositories own behavioral tests;
report exactly which code, dependencies and platforms were exercised.
