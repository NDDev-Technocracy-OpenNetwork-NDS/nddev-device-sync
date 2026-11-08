# 0022: Independent products and NDS integration

Status: accepted direction, 2026-11-08.

## Decision

Functional modules become complete, independently usable products. They own their
engine, standalone entry points, public interfaces, state model and release
lifecycle. NDS connects them through adapters and provides shared identity,
synchronization, telemetry and managed integration.

The product model is a phone with apps. A product's standalone use does not require
an NDS account or deployment. NDS integration adds capabilities around the same
implementation. Product-specific behavior does not move into a central kernel or
shared presentation object merely to make that integration possible.

NDDev Remote's independent product work continues under its existing owner.
Browser automation is a subsequent independent product extraction. Cache extraction and runtime migration are deferred to the owning cache work. Other current
integrations need an ownership and dependency review before their migration.

## Transition

This preparation changes documentation and source-catalog scope. It does not
replace the module host, change a wire protocol, select a new sync algorithm,
rename packages or activate a deployment. Current component Gitlinks and issued
release/dependency snapshots retain their meaning.

The implementation plan must identify each product's complete functionality,
the NDS adapter API, state and credential owners, lifecycle and cancellation,
upgrade/migration order and standalone plus integrated acceptance. Existing keys,
identities, pending work, OS consent and user data must survive the transition.
