# NDS source root

Read README.md, docs/architecture.md, docs/integration.md and the affected
component's README.md and AGENTS.md before changing it.

This repository owns the public source catalog, assembly schemas and exact
component Gitlinks. Component code, dependency locks and releases belong to their
repositories. The organization profile is source-only. Preserve repository
identities, published revisions, existing worktrees and unrelated changes.

Products own their complete behavior and can run independently. NDS adapters add
identity, synchronization, telemetry and managed integration. Keep product engines
independent of NDS credentials, replica stores and process-global initialization.
The current compiled-in assembly remains in place until its callers migrate.

Credentials and unwrapped keys remain outside presentation DTOs, synchronized
metadata and telemetry. Preserve original actor, device and space bindings through
asynchronous work, cancellation and shutdown. A manifest, setting or UI selection
does not grant access. Existing data, endpoint identities, OS consent and uncertain
operation outcomes survive migrations.

Keep public source self-contained and generic. Never add credentials, real device
inventory, deployment values, private organization material or runtime evidence.

Run `just check` for checks that need no compilation. `just test` exercises the
root's tooling, including real Cargo fixtures, on a build/test host or CI.
Component acceptance follows its owning repository. Report the exact source and
platforms exercised; a source/catalog check is not runtime acceptance.

Root automation is an independent Cargo workspace in automation/. Never create
an ancestor workspace around the component repositories. Build the source
verifier on an execution host and install only a source-matching artifact before
running catalog checks on a source machine. The Cargo cache helper and its
acceptance fixtures remain with their existing implementation until their owning
transition.
