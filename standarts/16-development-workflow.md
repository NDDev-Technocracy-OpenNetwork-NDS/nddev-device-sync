# Development workflow and consistency

## Bounded delivery

1. Confirm the canonical owner, current implementation and accepted outcome.
   Identify affected [system clauses](17-system-standard.md), actual consumers
   and retained data. Separate defects, contract changes and new product features.
2. Implement the smallest complete behavior with its errors, bounds and telemetry.
   Reuse or remove existing draft code according to real callers; do not publish
   placeholders merely because a file already exists.
3. Exercise production paths with the [quality gates](07-quality.md). Inspect
   the result; never report unexecuted checks or mocks as acceptance.
4. Update generated consumers through their source and remove obsolete code,
   flags and misleading documentation. Review the coordinated diff.
5. Deliver signed Conventional Commits through working branch -> dev -> main,
   with CODEOWNERS, protected branches, immutable CI actions and minimal tokens.
6. Report the exact source and verification. Runtime changes, release artifacts
   and live deployment have distinct completion evidence.

Standards releases are immutable compatibility anchors. Existing module locks
are not silently reinterpreted after a standards edit; consumers update through
a subsequent source release and a coordinated contract change. Keep an explicit
compatibility note while an older lock remains valid. Do not move old tags or
blindly bump pins to make metadata appear current.

## Consistency

- Server-side mutations are authoritative, idempotent and revision-checked.
- Read models expose revision/freshness; offline synchronization follows the
  canonical operation protocol.
- Identity and tenant/device authorization are enforced at the owning boundary.
- Secrets never enter public DTOs, telemetry or Git.
- Migrations are forward-reviewed and tested for supported transitions with
  dedicated privileges; runtime readiness checks its required schema.
- Build/release evidence identifies source, standards/protocol/schema versions,
  toolchain, dependency lock hashes and artifact digest.

An active application means its selected modules perform useful real workflows,
including errors and observability. Catalog entries and scaffolding are not
completion. The owner selects the first product scope; implementation stays
within that scope rather than creating every future repository.

Backup/recovery, advanced deployment orchestration and an in-process native
plugin ABI remain deferred. ADR 0017 accepts independently delivered functional
modules through the existing supervised execution boundary and multiple
personal/organization spaces. The engineering contract permits autonomous work
within those outcomes, not silent expansion of runtime authority. Company
membership and scoped RDS administration retain ADR 0011's isolation rules.
