# ADR 0015: NDS-owned agent browser automation

Status: accepted owner direction, 2026-10-25; implementation and qualification pending.

## Context

Agents perform interactive browser work through the third-party Playwriter
CLI and Chrome Web Store extension. That extension auto-updates with the
`debugger` permission on all sites, and its relay accepts unauthenticated
loopback clients on a fixed port: any local process can obtain raw CDP for
attached tabs or execute code. Its Playwright is a publisher fork, and vendor
telemetry, cloud and tunnel paths ship in the same package. Since Chrome 136,
remote-debugging switches no longer apply to the default profile; an
extension using `chrome.debugger` is the supported route to the owner's open,
authenticated browser.

## Decision

The `browser-use` module (`nddev-device-sync-browser-use`) replaces that
dependency with NDS-owned components. The owner selects full autonomy inside
the agent area: an agent may do anything a Playwright program can do there.

- An NDS-owned Manifest V3 extension with a pinned identity attaches
  `chrome.debugger` only to agent tabs: tabs in NDS-managed tab groups, one
  group per project session, including popups they open and tabs the owner
  moves into such a group. Other owner tabs are never attached. Moving a tab
  out, closing it, the extension's control switch or uninstalling revokes it.
  Chrome's debugging indicator remains visible.
- The extension reaches the Rust host through Chrome native messaging, whose
  `allowed_origins` Chrome enforces. Agent clients use a per-user Unix socket
  in a private runtime directory. There is no TCP listener, remote-debugging
  endpoint, tunnel, cloud service or vendor telemetry.
- Each session runs agent code in its own executor process with pinned
  upstream `playwright-core` on Bun, connected to a session-scoped CDP endpoint
  on that private socket. The executor has the invoking user's authority, as
  the harness's shell already does; it is not a sandbox and page content has
  no path to it. Pages, DOM and results are untrusted data for the caller.
  Screenshots and other artifacts stay in a session-private directory removed
  with the session.
- Sessions, tabs, in-flight commands, message sizes and execution time are
  bounded. Timeout or cancellation terminates the executor, so no action
  continues afterwards. Closed targets, replaced documents, disconnects and
  revocation produce stable typed errors, never success on another page.
- Telemetry uses the existing SDK/protocol allowlist: `browser.*` event names,
  outcome, duration and stable error types. URLs, titles, selectors, code,
  page content, cookies, headers and screenshots are never telemetry.
  A standalone host keeps schema-validated local output; export follows agent
  composition under its existing service identity.

## Limits

Page JavaScript dialogs, file choosers and page input are CDP operations.
Browser-chrome and OS surfaces are not: passkey account choosers, platform
user verification, permission prompts and native dialogs. Real authenticators'
user presence and verification are never bypassed. Unattended passkey sign-in
in agent tabs needs a distinct agent credential presented through the
experimental CDP WebAuthn virtual authenticator; its key custody, registration
and revocation require a separate decision before implementation.

## Integration and transition

The manifest contract adds capability `control_browser` and permission
`browser_control`. Core enums, consent labels, Vault-synchronized module
configuration, agent composition and telemetry export are the integration
step; older replicas must refuse the unknown permission. The host is a
Chrome-spawned process outside the agent admission budget; agent clients use
short bounded local exchanges, and `adapter-io` gains no listener.
Linux qualifies first; other platforms claim support only after their own
evidence.

Playwriter remains the ADR 0009 and STD-0002 interactive browser tool until
the owner accepts qualified browser-use. This decision does not change
existing locks, alpha acceptance or release blockers.

## Agent passkey custody

Status: proposed amendment; the owner asked the NDS session to choose custody, and it recommends
NDS Vault for agent credentials. Implementation needs the integration listed below.

- Owner passkeys stay with Chrome and Google Password Manager. NDS never exports or copies them
  and never answers their user verification.
- An agent credential is a separate WebAuthn credential that the owner registers for one RP ID,
  origin and project session. It is never an owner passkey, an NDS device-signing or session key,
  or a shared browser credential record.
- Vault holds it as its own typed record, not as a password record. The existing record
  encryption binds it to owner, origin, entity, operation and revision. Server decryption
  authority under ADR 0006 applies to it as to every Vault record.
- Release goes only to the browser-use host on the same device, for one operation of the bound
  session. It requires a fresh `browser_control` consent check and a fresh Vault read. The host
  keeps the key in zeroizing memory and imports it into a virtual authenticator of that agent tab
  over CDP. Agent code, the executor, FFI, public DTOs, logs and telemetry never carry it.
- After the operation the host reads the new signature counter back, writes it to the record and
  removes the authenticator. Cancellation, timeout, host loss and revocation zeroize the memory;
  Chrome removes the authenticator with its CDP session.
- Registration and revocation are explicit owner commands. Revocation deletes the record; the
  public key stays trusted at the RP until the owner removes it there, which NDS reports and
  cannot do itself.
- Scope: the CDP WebAuthn domain is experimental; a passkey step needs the agent tab in front; RPs
  that require attestation or a platform authenticator may refuse a virtual one. In an isolated
  Chrome for Testing profile, an exported credential imported into a new authenticator signed in
  with an advancing counter, and the authenticator was gone after its CDP session detached.
- Integration: a core record kind, a Vault adapter for the browser-use custody port, agent
  consent wiring and a typed host operation that keeps the key out of agent code. Until it
  exists, agents keep no persistent credential.

References: [chrome.debugger](https://developer.chrome.com/docs/extensions/reference/api/debugger),
[native messaging](https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging),
[remote debugging change](https://developer.chrome.com/blog/remote-debugging-port),
[service worker lifetime](https://developer.chrome.com/docs/extensions/develop/concepts/service-workers/lifecycle).
