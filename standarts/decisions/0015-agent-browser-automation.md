# ADR 0015: NDS-owned agent browser automation

Status: accepted owner direction, 2026-10-25; amended by owner on 2026-10-28.
Implementation and qualification remain cohort-specific.

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
  bounded. Script timeouts follow the owner amendment below. Cancellation,
  owned close, revocation and shutdown retain their exact-owner drain and
  termination boundaries. Closed targets, replaced documents, disconnects and
  revocation produce stable typed errors, never success on another page.
- Telemetry uses the existing SDK/protocol allowlist: `browser.*` event names,
  outcome, duration and stable error types. URLs, titles, selectors, code,
  page content, cookies, headers and screenshots are never telemetry.
  The host emits through the same SDK in standalone and NDS composition;
  extension, executor and CLI events reach it through their existing channels.
  Schema-validated local output is an SDK sink, not another telemetry pipeline.
  Export uses the configured owning service identity.

## Owner amendment: timeout and extension permissions

### D1: abort the timed-out script

A script timeout seals browser-call admission for that exact script. The
executor propagates its abort signal through an `AsyncLocalStorage` context,
exposes the signal to the script, and checks it in Playwright instrumentation
before dispatching later calls. Async callbacks and timers created by that
script retain its cancelled context; a later script cannot revive it.
A confirmed abort preserves the executor, connection and state for subsequent
admitted work.

On the timeout path the host waits only a short, finite abort-confirmation grace
period, covered by the owning operation budget. Without confirmation it stops
and reaps that executor before replacement. This changes timeout handling, not
the positive owned-session close/drain barrier. Cancellation or revocation
cannot obtain a successful close from a script-abort acknowledgement alone.

The guarantee is no new module-mediated browser action by timed-out code.
An action already dispatched to Chrome may complete; killing the executor also
cannot retract it. Neither path promises website-effect rollback or makes the
executor a sandbox for arbitrary Node code. Timeout remains a typed failure.

### D2: permanent scripting and host permissions

The extension declares required `scripting` permission and
`host_permissions: ["<all_urls>"]`, rather than optional, per-use grants.
Chrome grants these broadly; the extension enforces the narrower agent area.
It may use scripting only on a currently admitted NDS agent-group tab to remove
page iframe elements whose source is `chrome-extension://` before debugger
attach or reattach. This does not grant access to another extension's protected
frame contents or to ordinary owner tabs.

Admission uses the captured group life and exact tab/attachment identity, with
revalidation after asynchronous boundaries before further browser operations.
Moving a tab out, disabling control or ending that life denies new work.
Already admitted cleanup/attach work participates in the same owned-life drain;
a stale continuation cannot operate on a replacement owner. There is no
all-sites background DOM cleanup. Store permission justifications must describe
this actual use; the grant is not an installation or rollout instruction.

Qualification must exercise late Playwright calls after timeout (including
callbacks/timers), state and connection reuse after confirmed abort, and
stop/reap plus a fresh debugger connection when replacement is necessary.
It must also exercise foreign tabs, moved tabs, late attach/reattach and revoked
lives, proving that scripting remains agent-tab-only and cannot revive a closed
life. Existing owned-close, unknown-acknowledgement and privacy gates remain.

This amendment applies to browser-use cohorts that implement and qualify these
semantics. Older immutable cohorts and standards locks retain their recorded
meaning; no tag is moved or lock silently reinterpreted. Alpha acceptance must
identify the selected qualified host/extension pair.

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

References: [chrome.debugger](https://developer.chrome.com/docs/extensions/reference/api/debugger),
[native messaging](https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging),
[remote debugging change](https://developer.chrome.com/blog/remote-debugging-port),
[service worker lifetime](https://developer.chrome.com/docs/extensions/develop/concepts/service-workers/lifecycle),
[scripting permissions and targets](https://developer.chrome.com/docs/extensions/reference/api/scripting).
