# Observability ownership

The canonical event envelope, operating modes and quality signals are defined
once in [the telemetry standard](15-observability-and-telemetry-standard.md).

```text
Rust / Flutter / agents
          |
          v
owned intake/policy + Vector -> OpenObserve
```

The standard/debug/off collection profile and diagnostic access roles follow
[ADR 0018](decisions/0018-control-and-diagnostic-profiles.md). Processes exclude
secrets before emission. The Rust observability service normalizes
events and owns alert state; Vector bounds collection, buffering and routing;
OpenObserve is the current centralized store and review surface; no external
notification destination is configured by this contract. Clients never receive its ingestion
credentials. Vector may also collect journald and container logs.

The pipeline reports its own delivery failures and freshness. A service is not
observability-ready because it emits JSON or sets an enabled flag. Acceptance
must demonstrate collection, correlation, searchable delivery, a triggered
incident and its resolution, with destination delivery qualified separately
when such a channel is added. Operational alerts include dependency failure,
stalled sync, expired heartbeat, telemetry loss and storage pressure.
