# Technology standard

The alpha technology baseline is intentionally small and pinned by each
repository's lockfile:

| Concern | Standard |
| --- | --- |
| Core language | Rust 1.99 stable, edition 2024 |
| Native UI | Flutter 3.47+ and Dart stable |
| Desktop background | One owned Rust background lifetime, typed local bridge and bounded agent capabilities; qualification required by ADR 0009 |
| Mobile bridge | Rust core through a generated native bridge |
| Browser UI | React, Vite, strict TypeScript, TanStack Router/Query/Form and shadcn/ui under ADR 0010 |
| Browser runtime | Stable Rust WebAssembly, typed browser credential/durability adapters under ADR 0007 |
| Web tooling | Bun lockfile, TypeScript compiler, Biome, focused component/browser tests |
| Server | Rust async service with Tokio and Axum |
| API contract | OpenAPI 3.1 for HTTP; JSON Schema draft 2020-12 for events |
| Local state | SQLite with migrations, WAL and bounded outbox/inbox |
| Server state | PostgreSQL 18.x through SQLx with checked migrations |
| Secrets | Native keyring plus encrypted vault records and server-managed owner key envelopes |
| Identity | Email OTP, GitHub OAuth PKCE and WebAuthn passkeys; separate per-device signing keys |
| Email delivery | Server-owned transactional email adapter; provider-neutral port |
| Transport | HTTPS signed request/response replication with rustls and W3C trace context |
| Observability | `tracing`, OpenTelemetry, Vector and OpenObserve |
| Infrastructure | Docker Compose only for alpha; `just` as command runner |
| Rust checks | rustfmt, clippy, nextest, cargo-deny, cargo-audit, coverage |
| Flutter checks | dart format, dart analyze, Flutter unit/widget/integration tests |
| Design | `nddev-opennetwork-design-system`, pinned alpha release |

Tauri, Kubernetes, service meshes, runtime native plugins and additional
message brokers are outside the alpha baseline. They may be proposed through an
ADR when a measured requirement justifies the added surface.

[ADR 0010](decisions/0010-react-browser-presentation.md) replaces the Flutter Web
presentation choice. Older immutable locks retain their Flutter baseline until
the consuming assembly adopts and qualifies the replacement; native Flutter and
Rust backend/domain requirements remain unchanged.

[ADR 0006](decisions/0006-agent-first-sign-in-and-vault-access.md) distinguishes
the current identity contract from historical two-method locks and
records server authority for automatic vault key delivery and replica catch-up.

Dependencies are added only with a purpose, owner, license decision, update
path and removal condition. A package must not be added merely to wrap one
standard-library operation.

The first-alpha assembly adopts the latest stable releases of this approved
stack through a checked component set, as required by
[ADR 0009](decisions/0009-first-alpha-product-contract.md). The table identifies
the current technology family; exact versions and source identities belong to
the qualified locks and release manifest. A documented compatibility pin needs
an upstream issue and an explicit removal condition.

Local module builds share the compiler-scoped Cargo artifact cache defined by
[ADR 0012](decisions/0012-shared-cargo-cache.md). Cargo metadata owns effective
artifact paths; the complete pinned Rust toolchain owns each cache lane.
