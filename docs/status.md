# Current implementation

This is a source map for the alpha, not a declaration of complete platform or
deployment acceptance. Gitlinks select the root's exact source snapshot. Each
component's README, dependency locks, tests and releases describe its own revision.

| Component | Implemented responsibility and main remaining boundary |
| --- | --- |
| Core | Pure identity, device/space, module-admission, sync and screen rules; no native I/O |
| Protocol | Versioned HTTP, synchronization, membership, screen and diagnostic schemas |
| Accounts | Sign-in, device proof, credential stores, authority fences and signed client transport |
| Organizations | Membership, invitation and administrative commands through Accounts/Server |
| Vault | Encrypted SQLite/OPFS journal, pending operations, conflict handling, checkpoints and retained replication |
| Server | PostgreSQL-backed identity, membership, sync and signed diagnostic ingress; explicit migrations |
| Observability | Shared SDK, bounded delivery and collector/query/incident service; actual receipt is separate evidence |
| Agent | Eight compiled-in integrations, shared bounded native I/O, CLI and retained operation ownership |
| Desktop | Shared Rust composition and generated bridge, Flutter presentation and native packaging |
| Mobile | Flutter shell consuming the selected Shared packages and native lifecycle |
| Web | React/TypeScript shell, shared Rust/WASM, browser credentials/storage and offline assets |
| GitHub | Local status worker, catalog collection and optional existing-commit synchronization; integrated permission/replica lifetimes remain coupled |
| Remote | Manager observation and opt-in Linux X11 screen library; standalone Remote has separate ownership and qualification |
| Browser-use | Rust native host, extension, executor, CLI and Android link; independent product extraction remains |
| Cache | Existing server/client integration; product extraction and runtime migration are deferred to the owning cache work |
| Sysinfo | Native-provider observations; the adapter is not the full system-information product |
| Clipboard | Native archive access and explicit browser copy; provider/archive ownership remains |
| Cleaner | Provider observation and an opt-in development-cache maintenance executable |
| Updater | Native policy/report observation; full NDS application installation and activation need consumer acceptance |
| Design system | Self-contained OpenNetwork tokens, web kit and native bindings; source and package qualification are tracked in its repository |
| Self-hosting example | Synthetic configuration consuming the selected server Compose contract |
| Organization profile | Public introduction and assets, with no runtime execution |

## Coupling to address

Agent's fixed module list requires a rebuild to change its compiled composition.
The shared client's `NdsClient` joins identity, Vault, catalog, module control,
organizations, remote-screen lifetime and telemetry. Vault's native replica owner
also retains identity, catalog and delivery lifetimes. Those responsibilities need
explicit ownership during extraction, rather than another layer over the same
shared object.

Several native integrations still depend on separately versioned providers.
Taking ownership of a product includes its engine and standalone lifecycle, while
NDS-specific authority and state mapping remain in the adapter. API and storage
compatibility must be verified at that boundary.

## Open design work

The module host and synchronization data model remain design decisions. Existing
descriptors are compiled-in metadata, and the current journal remains the current
implementation. Hot replacement, arbitrary user modules, application update
activation and cross-platform screen acceptance are not completed by this source
reorganization. The independent-products decision replaces the earlier roadmap
as the starting point for planning those changes.

Cache implementation and migration are outside this preparation. Source-catalog reconciliation does not switch active cache routing, credentials or services.
