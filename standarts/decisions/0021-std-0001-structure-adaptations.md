---
status: accepted
date: '2026-10-25'
adaptation:
  standard:
    id: std-0001
  rules:
  - R-0018
  paths:
  - TRADEMARKS.md
  - modules
  - rust-toolchain.toml
  reason: The owner-approved NDS system ADR 0013 (2026-10-22) and ADR 0016 (2026-10-25) keep NDS-owned
    material at fixed top-level locations of the system root.
  result: '`TRADEMARKS.md`: Trademark usage terms are a deliberate legal document kept beside LICENSE
    at the repository root. `modules`: The owner-selected NDS monorepo groups independently versioned
    module and tool Git submodules. `rust-toolchain.toml`: Pinned Rust toolchain for contract tooling
    in a repository without a Cargo workspace; the rust profile requires Cargo.toml so the pin is declared
    as a variation.'
  verification: Review against NDS system ADR 0013 and ADR 0016; the root catalog check validates NDS descriptors
    and locks, and the repository's own checks exercise the remaining paths.
---
# ADR 0021: std-0001 structure adaptations

The owner approved [ADR 0013](0013-estate-hygiene-and-standards-adoption.md) and [ADR 0016](0016-module-and-tool-monorepo.md), which keep NDS-owned material at
deliberate top-level locations. This record scopes those decisions to the
system root under std-0001 R-0037 and replaces the
earlier manifest variations without changing them.

- `TRADEMARKS.md` (legal): Trademark usage terms are a deliberate legal document kept beside LICENSE at the repository root.
- `modules` (package): The owner-selected NDS monorepo groups independently versioned module and tool Git submodules.
- `rust-toolchain.toml` (tool-config): Pinned Rust toolchain for contract tooling in a repository without a Cargo workspace; the rust profile requires Cargo.toml so the pin is declared as a variation.
