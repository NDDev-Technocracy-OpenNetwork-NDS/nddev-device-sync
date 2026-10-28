# ADR 0020: a system standard with staged adoption

Status: accepted owner direction, 2026-10-28; component adoption pending.

The owner requests NDS-specific standards followed by a careful, planned
alignment of the system. The accepted architecture and trust decisions in ADRs
0017–0019 remain the product basis.

## Decision

The public [system standard](../17-system-standard.md) is the common entrypoint
for stable product clauses, their canonical documents, responsible components
and observable acceptance. Detailed product rules remain in their existing
owning documents. Protocol/code/schema owners retain actual API and wire
identity; the index is not another runtime registry or a claim that descriptor
v1 already implements the target.

Private NDDev engineering standards and their NDS application profile remain
in the private governance repository. Its native edition selection owns that
basis. Internal requirement mappings and conformance findings are not copied
into public source or public CI. Public build/product contracts remain usable
without access to that repository.

The [existing alpha plan](../plans/alpha-development.md) owns implementation
order. Start with scope/device foundation and a small actual diagnostic path;
then evolve synchronization and the module boundary through real consumers,
complete functional modules, and qualify independent updates and clients.
Do not begin all component rewrites in parallel or introduce a new manager,
policy compiler, scheduler or release framework for this process.

## Adoption boundary

Each cohort records its owning code, affected system clauses, actual callers,
retained-data transition, verification and remaining limits. Contract/source,
component checks, packaged artifacts and actual target behavior stay distinct.
Unknown effect/cleanup and unavailable target evidence never become passes.

Update canonical owners and consumers together through existing signed task →
dev → main delivery. Existing issued locks and tags remain immutable. Adopt new
component sources only after their own relevant gates and the affected consumer
checks. Keep user data and independent work; remove obsolete paths when their
qualified replacement is adopted. One successful cohort enables its dependants.

No new runtime, resource, credential, recovery-copy, protected-client or deployment
authority follows from the standards baseline. Existing operational restrictions
and actual change windows remain in force.
