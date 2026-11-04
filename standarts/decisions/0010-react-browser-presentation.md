# ADR 0010: React browser presentation over the Rust runtime

Accepted 2026-10-17. The React source migration is delivered in the Web assembly.
The [active alpha plan](../plans/alpha-development.md#current-checkpoint-and-next-increment)
records current browser coverage and outstanding acceptance.

## Decision

The browser presentation uses React, Vite, strict TypeScript, TanStack Router
and Query, and shadcn/ui. Desktop and mobile retain Flutter. This replaces the
Flutter-web presentation choice in ADRs 0007 and 0009; their offline, identity,
data-preservation, update and observability requirements remain binding.
The previous Flutter browser assembly provided the migration baseline. Web
release and deployment follow their own acceptance and authorization gates.

Core domain/application, backend services, synchronization, authorization and
cryptography remain Rust. The browser compiles the applicable Rust adapters to
WebAssembly and exposes a narrow generated typed presentation boundary. Replace
Flutter-specific bridge composition without copying its Rust behavior into
TypeScript. Browser I/O adapters may use typed browser APIs; unwrapped keys and
resolved secrets never enter React state, Query caches or presentation DTOs.

## Presentation and tooling

- Author application code, browser workers and configuration in TypeScript;
  emitted JavaScript and generated WASM glue are build artifacts. Enable
  `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes` and
  `allowJs: false`. Do not hide errors with `any`, unchecked assertions or
  handwritten duplicate transport types.
- Use thin file routes under `src/app/routes`, with real screens/actions under
  `pages`, `features` and `entities`, and shared adapters, i18n and UI primitives
  under `shared`. Create only layers needed by implemented behavior.
- Query owns remote read caching; Rust owns the durable replica and pending
  mutations. Keys include authenticated company/user/device scope as applicable.
  Cancel and clear presentation caches on identity transitions. Do not persist
  a second mutation queue or make a route guard an authorization authority.
- Forms use TanStack Form and explicit typed boundary validation where needed.
  Protocol projections are generated from their canonical owner; a generator
  requiring a different compiler runs in isolated tooling, not a second web
  TypeScript dependency graph.
- Bun owns the one dependency lockfile. Pin current stable, compatible inputs;
  Biome and the TypeScript compiler are separate checks. Vite transpilation
  alone does not establish type correctness. No JavaScript backend or SSR
  service is introduced.
- shadcn/ui components consume the existing OpenNetwork design tokens. Preserve
  English/Russian message identities and format contracts through generated
  projections of the canonical catalog. Preserve accessibility, focus, theme,
  keyboard and reduced-motion behavior. Do not import unrelated product scope
  or additional locales from a tooling template.

## Migration and acceptance

Retain same-origin HTTPS, protected cookie sessions, exact-origin/CSRF checks,
device proofs and browser credential storage. Preserve encrypted pending work,
database identity and immutable dispatched operations across the app-shell
upgrade. Do not clear an existing browser database to make the new UI pass.
Only manifest-verified static assets enter the service-worker cache; identity,
Vault, API and telemetry responses do not. Reuse the canonical Rust telemetry
contract and pipeline, with bounded redacted presentation failure reporting.

Acceptance compares real workflows: login/enrollment, local edits, offline full
restart, reconnect/replay, conflicts, multiple tabs, logout/revocation and an
upgrade with unsent work. Run the same applicable native regressions and actual
Playwright browser checks. Retire the Flutter browser build path and obsolete
assets when the replacement is qualified; do not maintain two browser products.

References: [shadcn/ui with Vite](https://ui.shadcn.com/docs/installation/vite),
[TanStack external data loading](https://tanstack.com/router/latest/docs/guide/external-data-loading),
[TypeScript strict mode](https://www.typescriptlang.org/tsconfig/strict),
[exact optional properties](https://www.typescriptlang.org/tsconfig/exactOptionalPropertyTypes.html).
