# NDDev Device Sync

NDS connects independently usable products across devices. It adds shared identity,
synchronization, telemetry and managed integration around their own engines and
interfaces. Each product owns its functionality and releases.

The architecture is a phone with apps: NDS provides the common host, and products
connect through adapters. The current alpha still composes its native modules as
Rust libraries. Installing and replacing modules while the host runs is future
work; separate repositories alone do not provide that behavior.

See [architecture](docs/architecture.md), [integration](docs/integration.md),
[current implementation](docs/status.md) and [development](docs/development.md).
The [product decision](docs/decisions/0022-independent-products.md) records the
direction and the boundary of the upcoming refactor.

## Source

This repository pins the public component repositories through exact Gitlinks.
The [catalog](contracts/module-catalog.json) describes those source boundaries;
runtime descriptors and application locks have their own owners.

```text
nddev-device-sync/
  modules/       kernel, server, clients and product integrations
  tools/         focused native integrations
  contracts/     source catalog, module and application schemas
  docs/          product, integration and development documentation
  scripts/       source and contract tooling
```

```sh
git clone --recurse-submodules https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync.git
cd nddev-device-sync
just setup
just check
```

`just checkout` initializes the selected public source recursively.
`just component <id> <recipe> ...` invokes a component's own justfile after checking
its repository and exact selected revision. Component development and acceptance
are described in each component's README. The
[self-hosting example](modules/estate-example/README.md) contains synthetic inputs
and consumes the server's Compose contract.

NDDev Remote is an independent product with an NDS adapter. Browser automation
already has its own host, extension, executor and CLI; its extraction as an
independent product is planned. Existing versions, wire formats, application locks
and stored data retain their current meaning during that work.

Desktop and mobile use Flutter presentation. Web uses React and TypeScript over
the shared Rust/WASM composition. Rust owns identity, encrypted state and native
operations. Platform support and runtime qualification belong to the implementing
component and its consumers.

## License

AGPL-3.0-only. See [LICENSE](LICENSE) and [TRADEMARKS.md](TRADEMARKS.md).
Part of [NDDev OpenNetwork](https://nddev.ai).
[Release history](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync/releases).
