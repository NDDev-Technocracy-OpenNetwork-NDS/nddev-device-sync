# Product boundary

`nddev-device-sync` is a distributed control plane for NDDev devices, servers,
development tools and agent-work harnesses. Its accepted clients use Flutter
for desktop/mobile and React with strict TypeScript for web under
[ADR 0010](decisions/0010-react-browser-presentation.md), a Rust client core,
Rust server components, and a server-side
observability pipeline.

The product coordinates these modules through typed contracts:

- GitHub Device Sync (GDS);
- Remote Device Sync (RDS);
- sysinfo;
- clipboard;
- cleaner;
- updater;
- agent browser automation;
- harness account management;
- company membership and invitations;
- server inventory and health;
- logs, metrics, traces and alerts.

Each existing tool remains the owner of its own data and safety policy until a
deliberate migration is approved. The control plane consumes stable adapters,
receipts and health contracts rather than copying private databases.

The product owns device identity, synchronization metadata, connection
metadata, module state, alert state and the encrypted vault protocol. It does
not become a general-purpose remote shell, a replacement for Git, or a second
package manager.

The first active application contains an owner-selected set of working modules,
real user workflows and observable failure handling. The catalog is a map of
boundaries, not a requirement to implement all modules at once. Health endpoints,
empty screens and module descriptors alone do not constitute that application.
The accepted scope includes all seven tool/account modules listed above, with
Linux/macOS/Windows desktop and Android/iOS mobile clients. Capability and
execution-host boundaries remain explicit; this scope is delivered in verified
increments rather than represented by nonfunctional placeholders.

The owner also requires a browser client with durable offline work and
cross-device synchronization under [ADR 0007](decisions/0007-offline-browser-client.md).
This is accepted product scope; browser runtime support still requires its own
implementation and qualification. A catalog reservation is not that evidence.
