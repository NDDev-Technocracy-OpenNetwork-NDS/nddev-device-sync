# Product decisions

- Public source and self-hosting are first-class requirements.
- Owned Rust, Flutter, TypeScript and public design-token sources use AGPL-3.0-only;
  third-party sources retain their compatible licenses. NDDev OpenNetwork
  attribution and the https://nddev.ai link remain in product surfaces.
- Email OTP, GitHub OAuth PKCE and passkeys are sign-in methods for
  verified users under the bootstrap and invitation policy in ADR 0011. Vault access prepares automatically
  after sign-in through server-managed keys, independently of other devices.
  Historical locks retain their original compatibility scope.
  See [identity](03-identity-and-vault.md) and
  [ADR 0006](decisions/0006-agent-first-sign-in-and-vault-access.md).
- Agent interfaces, Flutter and React consume the same Rust use cases through typed,
  bounded capabilities and redacted results. Native consumers use secret
  references; the control plane preserves each existing provider's policies.
- The server owns committed state; devices catch up at startup and reconnect.
  Server-managed vault keys use a wrapping key outside PostgreSQL. The server
  has decryption authority; this supersedes the client-only E2EE boundary.
- Docker Compose is the alpha deployment path. Companies, invited users and
  narrowly scoped administrative RDS access are defined by
  [ADR 0011](decisions/0011-company-membership-and-device-access.md).
- Telemetry export is enabled by default and operator-controlled. Normal/debug
  modes share the same redaction guarantees and observable delivery state.
- Desktop, mobile and server have separate versioned artifacts in the coordinated
  alpha -> beta -> release train.
- [ADR 0017](decisions/0017-extensible-offline-kernel.md) defines a common kernel
  with full functional modules, autonomous work within existing offline grants,
  personal plus multiple organization spaces and unified device profiles with
  enforced workload placement. GitHub handles project code; NDS retains its own
  encrypted state journal and delivery. Official alpha modules use the extension
  boundary intended for future user-developed modules. Package versions and
  compatibility are explicit; controlled restarts during verified NDS/module
  updates are allowed. Earlier compiled-in-only and one-company restrictions
  are superseded, not retroactively implemented by changing these documents.
- Existing tools retain authority until approved migrations pass compatibility
  checks. Their repositories and private state are not disposable scaffolding.
- [ADR 0009](decisions/0009-first-alpha-product-contract.md) records the accepted
  NDS 1 first-alpha outcomes: sign-in-driven device enrollment and module setup,
  seven-day offline access, continuous synchronization where the platform permits,
  bidirectional GDS commit sync, cross-device screen viewing, NDS application
  updates and shared observability. Its acceptance matrix distinguishes target
  behavior from the currently qualified implementation.
- Backup/recovery remains disabled, including after the first release, until an
  explicit future product decision.
