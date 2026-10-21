# ADR 0012: shared NDS Cargo artifacts

Status: accepted owner direction, 2026-10-21.

NDS development uses one installation-local Cargo target directory for its
registered module checkouts. The cache lives outside Git and is partitioned by
the pinned Rust compiler identity and host. Cargo retains its own separation
by target triple, profile, package identity, features and compiler inputs.
Dependency sources, lockfiles and native provider verification remain unchanged.

`build.build-dir` shares intermediate artifacts as well as the ordinary target
directory. Flutter Native Assets may retain its required final staging target;
its Rust compiler intermediates use the same shared build directory.

The umbrella's `scripts/cargo-cache.py setup` installs a generated Cargo
configuration at the common parent of the NDS checkouts. Cargo's normal parent
configuration discovery then covers root and nested manifests. Refuse to
overwrite unrelated configuration. Private paths are generated locally and
never committed. The `run -- ...` entry selects the complete pinned Rust tool
directory so Cargo, rustc, rustdoc and Clippy agree; a mixed compiler toolchain
must not populate the shared lane.

Build and acceptance consumers obtain the effective target directory from
`cargo metadata`, or use the actual artifact reported by Cargo. A repository-
local `target/` path is not an artifact contract. Frozen release/qualification
artifacts are copied to their owned staging directory and checked by digest;
the mutable cache itself is not release evidence.

Nested provider builds that would contend with their parent's Cargo lock use
a stable, source-scoped subdirectory of the same cache. Explicit isolated
acceptance/coverage targets remain permitted and are recorded with the build.
Do not remove whole shared caches from a module's cleanup or `cargo clean`.
Retire only an idle, identified build lane; preserve frozen evidence and live
artifacts. CI trust boundaries and independently pinned native provider inputs
are not merged by this local cache policy.

This adds no build daemon, distributed cache service, new runtime technology
or change to application data. Measure cold/warm behavior and disk use; cache
hits never substitute for owning tests or source verification.

References: [Cargo configuration](https://doc.rust-lang.org/cargo/reference/config.html),
[Cargo build cache](https://doc.rust-lang.org/cargo/reference/build-cache.html).
