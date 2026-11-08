# Product integration

A product remains usable through its own entry point. An NDS adapter adds the
shared capabilities appropriate to that integration without making an NDS account,
server or collector a hidden prerequisite for standalone use.

## Adapter boundary

The product publishes a versioned interface and reports its implemented
capabilities. The adapter translates NDS requests, supplies the original admitted
context and owns cancellation and shutdown of its work. Unknown versions and
unsupported platforms are explicit outcomes.

Product settings and synchronized NDS settings have explicit owners. Local
credentials and runtime handles do not become synchronized configuration. An
external effect keeps its own operation identity and outcome; a timeout or a lost
caller does not prove that the effect was undone.

Products expose useful local diagnostics independently. The adapter connects
those observations to NDS correlation and delivery. Export configuration does not
prove receipt or searchable telemetry, and embedding does not install a second
process-global subscriber.

## Remote screens

[NDDev Remote](https://github.com/NDDev-Technocracy-OpenNetwork/nddev-remote)
owns connection, transport, capture, codec, viewing and authorized native input.
Its [integration contract](https://github.com/NDDev-Technocracy-OpenNetwork/nddev-remote/blob/main/docs/integration.md)
separates that engine from NDS authority, synchronization and telemetry.

The current NDS remote adapter has a read-only manager API and an opt-in Linux
X11 screen path. Its qualification and dependency locks remain separate from the
standalone product. Existing endpoint identities and native consent remain with
their owner. A screen connection does not imply shell, file or clipboard access.

## Browser automation

Browser-use contains a Rust host, MV3 extension, Bun/Playwright executor and agent
CLI. Desktop uses native messaging; Android uses the existing ADB/DevTools route.
The implementation scopes automation to the agent's own browser sessions and
tabs, with bounded execution and cancellation.

This is the starting point for an independent browser automation product.
Standalone packaging, its public name and the NDS adapter transition remain
separate work. Existing browser profiles, host registration, session ownership,
artifact access and observation choices must survive that transition.

## System information

[NDDev Systeminfo](https://github.com/NDDev-Technocracy-OpenNetwork/nddev-systeminfo)
owns native collection, its tray application, configuration, installation and releases.
It is independently usable on Windows, macOS and Linux.

The NDS sysinfo adapter reads the product's v1 endpoint through its pinned provider
contract. Agent and the shared client composition select the same adapter revision.
The adapter does not start a second collector. Missing or stopped providers and unavailable
readings remain explicit; NDS identity and telemetry remain with the integration.

## Other integrations

GitHub owns repository-provider and Git-workspace behavior. Clipboard,
cleaner and updater currently adapt their native providers; extracting a complete
product includes the functionality that still lives in those providers. Cache has
its own service and client/pool/egress packages. Their source boundaries and
current limits are listed in [status](status.md).

Independent packaging alone does not qualify an NDS integration. Each consumer
checks its selected API, lifecycle, platform behavior and retained-data transition.

## Design system

[OpenNetwork Design System](https://github.com/NDDev-Technocracy-OpenNetwork/nddev-opennetwork-design-system)
publishes the standalone tokens, web kit and native theme bindings under Apache-2.0, with the
bundled assets retaining their own licenses. The NDS source assembly selects its
[public fork](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-opennetwork-design-system)
at a qualified upstream commit. Applications retain their own dependency pins; a source-root pin
does not update an application theme or qualify a new package version for that application.
