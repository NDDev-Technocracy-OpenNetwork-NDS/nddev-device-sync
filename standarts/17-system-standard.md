# NDS system standard

Contract revision: **1**, first-alpha target. Accepted product decisions are
binding; implementation and migration are qualified separately.

This is the system entrypoint for kernel, server, modules, tools and clients.
The clauses below identify their canonical rules, accountable owners and
observable acceptance. Linked product documents own the detailed rule; this
index does not create another editable API, descriptor, version registry or
copy of private NDDev engineering standards.

## Authority and compatibility

The owner-selected product boundary and accepted ADRs govern the target.
Current schemas, code, native manifests and dependency locks describe what an
actual source composition implements. An older lock retains its original
meaning. Reaching the target requires an explicit provider/consumer and retained
data transition, rather than loosening old validators or relabeling results.
Changes to a product invariant require an ADR and updates to its canonical rule
and affected acceptance. Routine implementation follows the existing contract.

Clause IDs are stable review references, not wire fields, privileges or proof
of conformance. Acceptance is assessed for the actual affected source, artifact,
platform and operation. The [active plan](plans/alpha-development.md) owns
sequencing; source/CI/target evidence owns implementation status.

## Source and responsibility

| Clause | Canonical rule | Accountable owners | Observable acceptance |
| --- | --- | --- | --- |
| NDS-S01 | [Public source and two private contexts](11-repository-topology.md) | Root, component repositories | A credential-free recursive checkout contains the selected public sources; supported public checks need no private policy or Estate. |
| NDS-S02 | [Kernel and module responsibilities](02-architecture.md#boundaries) | Core application/domain, Agent, Server, functional modules | A representative operation follows one owning use case; OS/UI/provider code does not become kernel domain logic. |
| NDS-S03 | [Complete functional modules](01-product.md) | github, remote, browser-use and tools | The advertised real workflow works without a mandatory separately installed archived product. Reused libraries retain attribution and their actual responsibilities. |
| NDS-S04 | [One contract and generated consumers](02-architecture.md#extension-contract-and-trust) | Protocol, each payload owner, generated consumers | An interface change has an identified provider and all actual consumers; regenerated projections agree and incompatible versions refuse before effects. |

## Principal, device and action context

| Clause | Canonical rule | Accountable owners | Observable acceptance |
| --- | --- | --- | --- |
| NDS-I01 | [Stable identity and spaces](03-identity-and-vault.md#product-identity) | Core, Accounts, Organizations, Server, Vault | One user uses a personal space and two organizations; role changes and colliding local IDs never cross data/credential boundaries. |
| NDS-I02 | [Device identity and placement](02-architecture.md#devices-and-workload-placement) | Core, Accounts, Agent, Server | Rename/reclassification preserves device proof; a project workload runs only at its assigned admitted device. Names never confer authority. |
| NDS-I03 | [Fullauto/manual](decisions/0018-control-and-diagnostic-profiles.md#decision) | Trusted action boundary, Agent, clients | New fullauto setups perform supported actions without per-action prompts or an NDS sandbox; existing manual restrictions and other spaces remain effective. |
| NDS-I04 | [Effect and disclosure lifetimes](02-architecture.md#lifetimes-results-and-updates) | Native task owners, Server, clients | Revoke/switch during an await prevents later work or result disclosure from borrowing the replacement's authority; already sent effects remain accounted for. |

## State and synchronization

| Clause | Canonical rule | Accountable owners | Observable acceptance |
| --- | --- | --- | --- |
| NDS-D01 | [State classes](02-architecture.md#spaces-and-state) | Kernel stores and each module | Desired state, local bindings, observations and commands/media remain distinguishable; settings replay cannot execute an arbitrary browser action or update. |
| NDS-D02 | [Durable mutation and commit](04-sync-and-data.md#operation-protocol) | Protocol, Vault, Server, module state owner | Outbox-before-local-ack, verified inbox application and atomic committed results survive restart, duplicate delivery and idempotency-key misuse. |
| NDS-D03 | [Conflict and freshness](04-sync-and-data.md#operation-protocol) | Kernel synchronization and module merge rules | Offline concurrent edits preserve both revisions and expose actionable conflicts to the authorized user/agent; wall-clock order does not erase them. |
| NDS-D04 | [Offline work and catch-up](04-sync-and-data.md#stores) | Accounts, Vault, Server, connection owners | Admitted work survives coordinator/source-device loss within its existing grant; reconnect revalidates and catches up after retained-history expiry without losing pending work. |
| NDS-D05 | [Keys and retained data](03-identity-and-vault.md#vault) | Accounts, Vault, Server, migrations | Fresh and supported-upgrade paths preserve IDs, keys, ciphertext/AAD, tombstones and pending operations; an organization migration never relabels personal ciphertext. |

## Module execution and results

| Clause | Canonical rule | Accountable owners | Observable acceptance |
| --- | --- | --- | --- |
| NDS-M01 | [Packaging, compatibility and trust](02-architecture.md#extension-contract-and-trust) | Protocol, Agent, module/package owners | A real tool and functional module use the same boundary for discovery, effective admission, execution and health. Publisher/name spoofing and unknown privileges refuse. |
| NDS-M02 | [Retained work and terminal results](02-architecture.md#lifetimes-results-and-updates) | Existing runtime/task and provider owners | Caller loss cannot lose an admitted mutation or cleanup result. Known-unsent refusal, completed effect, completed failure and unresolved effect retain different meanings. |
| NDS-M03 | [Shutdown and replacement](02-architecture.md#lifetimes-results-and-updates) | Agent, shared clients, module workers | New admission closes first; all captured work/resources are accounted for; conflicting replacement waits for positive exact-owner drain. Failure remains visible and retains ownership. |
| NDS-M04 | [Honest status and finite work](13-engineering-and-agent-standard.md#ponytail-the-smallest-correct-system) | Owning runtime and provider adapter | Disabled, unconfigured, unsupported, running, ready, degraded and unresolved states are distinguishable. Saturation refuses/backpressures within declared bounds rather than leaking tasks or fabricating readiness. |

## Diagnostics and operation

| Clause | Canonical rule | Accountable owners | Observable acceptance |
| --- | --- | --- | --- |
| NDS-O01 | [One useful diagnostic contract](15-observability-and-telemetry-standard.md#useful-coverage-without-noise) | Observability SDK/intake and each operation owner | A real operation retains its identity/trace across hops; significant outcomes and causes are useful without repeated exception/polling noise. |
| NDS-O02 | [Standard/debug/off](15-observability-and-telemetry-standard.md#modes) | SDK/policy, authorized clients | Debug records permitted useful content; hostile secret canaries stay absent before buffering/export. Off stops collection/export without breaking functional state or presenting hidden export as health. |
| NDS-O03 | [Scoped diagnostic access](15-observability-and-telemetry-standard.md#access-and-retention) | Accounts, Server/policy, trusted ingest/query | Members see themselves, org admins their org, and a global diagnostic admin all spaces only with the separate key-authenticated role; count/detail/export paths obey the same boundary. |
| NDS-O04 | [Finite delivery, retention and incidents](15-observability-and-telemetry-standard.md#delivery-and-alerts) | SDK spool, Rust policy, Vector, OpenObserve | Outage does not stop ordinary work; queued records retain expiry, overflow/loss is visible, and actual searchable records plus incident resolution are observed. No external notification delivery is inferred. |

## Delivery, clients and acceptance

| Clause | Canonical rule | Accountable owners | Observable acceptance |
| --- | --- | --- | --- |
| NDS-R01 | [Independent module and application updates](08-release-policy.md#kernel-and-module-updates) | Updater, package/launcher owners | A signed compatible artifact is staged, the actual installed generation is drained, then switched/restarted and identified as running. Interrupted/unknown activation never claims success or downgrades incompatible data. |
| NDS-R02 | [Shared presentation contract](09-i18n-and-visual-system.md) | Shared Rust, Flutter, React, design system | Generated boundaries/catalogs agree; English/Russian and accessible states explain the same real outcome. UI is not a second permission, synchronization or update engine. |
| NDS-R03 | [Target and capacity qualification](07-quality.md#contracts-integration-and-release) | Each actual host/viewer/client and release owner | Real Linux, PC-owned Android emulator and Web workflows qualify their advertised roles. The retained 32-device/1024-repository alpha targets require a finite declared workload/resource envelope, not descriptors or an unlimited load claim. |
| NDS-R04 | [Data preservation and recovery policy](03-identity-and-vault.md#recovery-policy) | Data owner, migrations, operators | Changes preserve operational state; no prohibited recovery copies are created. Executable rollback is allowed only with retained-data compatibility and does not promise user-data recovery. |

## Using the standard

For a bounded change, identify affected clauses, the canonical source owner,
actual consumers, retained state and its observable completion. Exercise relevant
failure and concurrency paths through that implementation. Report four distinct
levels where applicable: contract/source, build/component checks, packaged
artifact and actual supported-target behavior. An unexecuted or unavailable
level remains unverified; a structural pass never promotes it.

One cohort is integrated before dependent migrations start. Existing instructions
and locks keep working until their qualified replacement is adopted. Remove
superseded paths with the real consumer change. Runtime deployment, protected
client maintenance and operational windows retain their separate authority.
No standard edit restarts a device or activates a module.
