# Product boundary

`nddev-device-sync` is a distributed user system with an extensible kernel for
devices, servers, functional modules and tools. A user has a personal setup and
can participate in multiple organizations. Its accepted clients use Flutter
for desktop/mobile and React with strict TypeScript for web under
[ADR 0010](decisions/0010-react-browser-presentation.md), a Rust client core,
Rust server components, and a server-side
observability pipeline.

The product provides these functions through common kernel services and typed
module/tool contracts:

- the GitHub module (`github`, formerly GDS);
- the remote module (`remote`, formerly RDS);
- sysinfo;
- clipboard;
- cleaner;
- updater;
- agent browser automation;
- harness account management;
- company membership and invitations;
- server inventory and health;
- logs, metrics, traces and alerts.

The kernel owns common identity, security, connections, synchronization,
application composition, updates and observability. Modules own their complete
advertised functions and can extend the system with scoped data and operations.
Official alpha modules use the same versioned boundary intended for future
user-developed extensions. A standalone engine may also consume that boundary
through a local host. See [ADR 0017](decisions/0017-extensible-offline-kernel.md).

Existing tools retain their private state and safety policy until a deliberate
NDS-owned migration is qualified. Archived implementations are development
groundwork; a wrapper alone does not implement the advertised full GitHub/remote
function. Do not copy provider databases, erase work or bypass their controls.

The product owns device identity, synchronization metadata, connection
metadata, module state, alert state and the encrypted vault protocol. It does
not become a general-purpose remote shell or a replacement for Git. Its built-in
updater owns verified NDS/module delivery and composes platform installers;
it does not silently replace the host's unrelated package management.

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
