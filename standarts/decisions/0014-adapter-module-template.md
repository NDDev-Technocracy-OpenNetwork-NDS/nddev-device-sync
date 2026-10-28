# ADR 0014: thin adapter module template

Status: accepted owner direction, 2026-10-22.

Amended by [ADR 0017](0017-extensible-offline-kernel.md): this template applies
to thin I/O adapters and focused tools. It is not the full implementation
boundary of GDS, RDS or another functional NDS module.

Thin native integration adapters — the modules that bind an external native
or provider surface to `adapter-io` (`sysinfo`, `cleaner`, `updater`,
`clipboard`, `rds` and future thin integrations) — share one template. The
template is the shape `sysinfo` already demonstrates. Heavy adapters that
own a service runtime, such as `gds`, keep their own internal layout under
the same rules where they apply.

## Layout

- `src/lib.rs` owns the public surface: a `//!` contract comment, the typed
  port trait (`ReadX`/`X` behavior), the module `Error` enum and the
  compiled-in `MANIFEST` constant reading `contracts/module.json`.
- `src/adapter.rs` owns the native IO implementation behind the port trait.
  The name is `adapter.rs`, not `native.rs`; historical `native.rs` files
  are renamed when the adapter next changes.
- `src/view.rs` owns read models and view DTOs where the adapter exposes
  them. Free functions and shared helpers that all adapters need live in
  `adapter-io`, not in copies across repositories.

## Error form

The module error is a `pub enum Error` in `lib.rs` deriving
`Debug` and `thiserror::Error`: `Io(#[from] IoError)` transparently wraps
`adapter-io` failures and domain variants carry the adapter's own failure
vocabulary (`UnsupportedVersion`, `InvalidSample`, `UnsupportedPlatform`,
…). `PartialEq`/`Eq` are derived where payloads allow. Callers never unwrap
`IoError` variants manually; one `map_io` mapping lives in `adapter-io`.

## Safety and capability bounds

Adapter crates compile with `#![forbid(unsafe_code)]`; genuinely unsafe
native calls live in the pinned external provider crate, not in adapter
source. `crates/cli` binaries and other non-adapter crates follow the same
forbid rule unless a documented variation applies. Capability gates stay in
`adapter-io` (`LocalIo`, `NativeIo`, `ProcessIo`), never in `unsafe` blocks.

## Evidence

- Unit tests cover pure rules: view construction, validation, sample
  parsing and error mapping. They need no provider.
- `tests/provider.rs` (or the equivalent documented lane) exercises the
  real provider where one exists; mocks are never represented as platform
  acceptance, per the engineering standard.
- `provider.lock.json` pins an external provider where the adapter fetches
  one; its provenance stays exact.

References: [standard 07 quality](../07-quality.md),
[standard 13 engineering and agent contract](../13-engineering-and-agent-standard.md),
[ADR 0013](0013-estate-hygiene-and-standards-adoption.md).
