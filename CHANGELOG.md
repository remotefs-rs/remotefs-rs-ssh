# Changelog

All notable changes to this project are documented in this file.

## 0.9.0

Released on 2026-09-01

### Added

- **ssh:** support connection attempts across backends
- **ssh:** support bind address across backends
- **ssh:** support bind interface across backends
- **ssh:** support TCP keepalive across backends
- **russh:** support SSH compression config
- **russh:** support host key certificates
- **ssh:** support public key authentication config
- **ssh:** support accepted public key algorithms
- **ssh:** support certificate files
- **russh:** support CA signature algorithms
- **russh:** support adding keys to agent
- **ssh:** support proxy jump connections
- **ssh:** support server alive intervals
- **ssh:** support agent forwarding
- **ssh:** support remote forwarding

### Fixed

- **libssh:** preserve explicit port over ssh config
- **ssh:** honor connect timeout across backends
- **libssh2:** honor configured identity files
- **ssh:** use resolved endpoint for libssh
- **ssh:** harden timeouts and forwarded channels

> Treat zero connection timeouts as disabled across libssh and russh, including ProxyJump handshakes. Bound forwarded channels to prevent resource exhaustion and document the supported SSH configuration options.

## 0.8.6

Released on 2026-08-31

### Fixed

- **russh:** match the check_server_key signature and drop unused russh-keys

> Update Handler::check_server_key to accept russh 0.63's
> PublicKeyOrCertificate instead of the removed PublicKey parameter type.
> Also drop the unused russh-keys dependency, which pulled a
> vulnerable russh-cryptovec release (RUSTSEC-2026-0153); the crate
> already gets everything it needs from russh::keys.

- resolve clippy warnings under the all-features lint gate

> Build Metadata with struct-update syntax instead of a default plus
> field reassignment, drop needless borrows in the example list_dir
> calls, and pass BenchmarkCtx::new directly instead of wrapping it in
> a closure. just lint "-- -D warnings" was failing on all of these.

- **libssh:** reverse argument for `sftp::symlink` call. Path and target were reversed.

### Build

- bump russh, testcontainers, and rust-version

> Move russh to 0.63, testcontainers to 0.28, and the published MSRV to
> 1.89.0; relax the path-slash pin to a bare minimal version.

### Style

- reformat markdown and rust sources with dprint

> Run just fmt over the repository. No behavioural change.

## 0.8.5

Released on 2026-06-08

### Fixed

- **russh:** close SFTP handles with awaited shutdown to stop handle leak

> russh-sftp's `File::drop` closes handles via `close_nowait`, which frees
> the handle server-side but never decrements the client's open-handle
> counter. Reads (one handle per chunk) and writes (one handle per upload)
> relied on `Drop`, so the counter climbed monotonically until the
> negotiated limit was reached, failing later operations with
> `Limit exceeded: Handle limit reached`.
>
> Both paths now close handles via an awaited `shutdown`, which decrements
> the counter. The reader also moves open+seek into the spawned task.

- **russh:** authenticate RSA keys with rsa-sha2-256/512

> russh maps a `None` hash algorithm to the legacy `ssh-rsa` (SHA-1)
> signature, which OpenSSH 8.8+ rejects by default. RSA public key
> authentication therefore failed against modern servers with
> "public key authentication failed", while the libssh2/libssh backends
> negotiate rsa-sha2-256/512 transparently (termscp#422).
>
> RSA keys now attempt rsa-sha2-512, then rsa-sha2-256, then legacy
> ssh-rsa, stopping early if the server stops offering publickey auth.
> Non-RSA keys are unaffected (russh ignores the hash for them).
>
> The test container was bumped from OpenSSH 8.6 (which still accepts
> SHA-1, masking the bug) to 10.2 so the suite exercises modern signature
> negotiation. The unrealistic single-algo KexAlgorithms pin in the test
> ssh config was modernized accordingly. Added a regression test for key
> auth via the ssh config IdentityFile directive.

- **russh:** wire SSH agent authentication

> The russh backend never read `SshOpts::ssh_agent_identity`, so it
> silently ignored the SSH agent and fell straight through to key/password
> auth. Users whose keys live only in an ssh-agent (common on macOS) could
> not authenticate, unlike the libssh2/libssh backends (termscp#422).
>
> russh now authenticates via the agent first (agent -> key -> password),
> mirroring the other backends: it connects to the agent over
> `SSH_AUTH_SOCK`, enumerates identities, filters by `SshAgentIdentity`,
> and signs challenges through the agent. RSA identities request
> rsa-sha2-512/256 before legacy ssh-rsa, same as direct key auth. Agent
> auth is Unix-only; other platforms report it unavailable and fall
> through to the remaining methods.
>
> Added a testcontainers regression test that spawns an ssh-agent, loads
> the mock key, and authenticates through the agent alone.

- **libssh:** order symlink arguments correctly

> The libssh backend passed the link path and target to libssh-rs's
> symlink(target, dest) in the wrong order. OpenSSH's older sftp-server
> reversed the SSH_FXP_SYMLINK arguments, which cancelled the mistake;
> the 10.2 server now used by the test suite no longer does, so symlinks
> were created at the wrong path. Order the arguments correctly.

## 0.8.4

Released on 2026-06-08

### Fixed

- bump russh 0.6.1

## 0.8.3

Released on 2026-04-18

### Fixed

- **scp:** use stat epoch for mtime instead of TZ-ambiguous ls

> ls -l prints times in the server's local TZ without an offset, so
> parse_lstime treats the naive datetime as UTC. Downstream clients that
> render via DateTime<Local> then show UTC wall-clock instead of local.
> Only ScpFs is affected; SftpFs uses the protocol's epoch directly.
>
> Probe stat flavor once per session (GNU via stat --version, else BSD
> via stat -f %m /, else Unsupported) and cache on the session. list_dir
> overrides mtimes with one batched stat -c '%Y %n' / -f '%m %N' call
> keyed by basename; stat overrides with stat -c %Y / -f %m. Falls back
> to the ls parser on hosts exposing neither flavor.
>
> Closes termscp#416.

## 0.8.2

Released on 2026-03-24

### Fixed

- **sftp:** eliminate shell command dependency from SFTP client

> Replace shell commands (pwd, rm -rf, cp -rf) with pure SFTP protocol
> operations so the SFTP client no longer requires shell access on the
> remote server.
>
> - Use sftp.realpath(".") instead of cmd("pwd") for working directory
> - Implement recursive remove_dir_all via readdir/unlink/rmdir
> - Implement recursive copy via readdir/mkdir/open_read/open_write
> - Fix libssh realpath to use canonicalize (SSH_FXP_REALPATH) instead
>   of read_link (SSH_FXP_READLINK)
> - Fix libssh symlink resolution in readdir to use read_link directly

## 0.8.1

Released on 2026-03-21

### Performance

- **russh:** streaming PipelinedSftpReader with bounded pre-fetch

> Replace the fully-buffered pipelined_sftp_read with a streaming
> PipelinedSftpReader struct that fetches 16 MiB batches (4 concurrent
> 4 MiB chunks) and pre-fetches up to 2 batches ahead. This caps memory
> at ~48 MiB regardless of file size while keeping pipelined throughput.
>
> Adds a 20 MiB byte-order test to verify data arrives sequentially.

## 0.8.0

Released on 2026-03-20

### Added

- add russh pure-Rust SSH backend

> Add a new `russh` feature providing a pure-Rust SSH backend using the
> russh, russh-keys, and russh-sftp crates. This eliminates the need for
> system C libraries (libssh2/libssh) when the russh backend is selected.
>
> The implementation includes:
>
> - SshSession and Sftp trait implementations for russh
> - SCP send/recv over russh channels with proper ACK handling
> - SFTP via russh-sftp
> - Authentication: password, public key, and SSH agent
> - Algorithm preference negotiation from SSH config
> - NoCheckServerKey default handler for server key verification
> - Benchmarks, examples, and full test coverage

### Performance

- buffer entire file in libssh SFTP reads to reduce round-trips

> Replace the streaming SftpFileReader (one SFTP round-trip per read()
> call) with a buffered approach that reads the entire file into memory
> using a 256 KiB buffer before returning. This pre-fetches the file data
> in open_read() rather than deferring reads to the caller's smaller
> 64 KiB buffer loop, reducing total round-trips.
>
> Unlike the russh backend which uses true concurrent pipelining (multiple
> file handles reading different chunks via tokio::spawn), this approach
> is still sequential due to libssh-rs serializing all operations through
> a session-level mutex. True pipelining would require the AIO FFI
> (sftp_aio_begin_read/sftp_aio_wait_read) which libssh-rs does not
> expose at the safe API level.

## 0.7.2

Released on 2026-01-31

### Build

- ssh2-config 0.7
- 0.7.2

## 0.7.0

Released on 2025-09-01

### Breaking changes

- Support for multiple SSH backends. Support for libssh.org (#10)

> Removed `From<SshOpts>`; Removed `new`; use `libssh2` and `libssh` constructors instead; Renamed ssh2-vendored feature to libssh2-vendored

- fix: Benchmarks for libssh2

- feat: libssh backend

- test: Added libssh tests

- fix: Restore From for SshOpts into SftpFs/ScpFs

- fix: broken code for libssh backende

- style: lint

### Added

- Breaking: Support for multiple SSH backends. Support for libssh.org (#10)

> - feat!: Support for multiple SSH backends. Support for libssh.org
>
> Added new feature to enable **libssh2** backend; Added a new feature to enable **libssh** backend. Renamed ssh2-vendored feature to libssh2-vendored. Removed `new`; use `libssh2` and `libssh` constructors instead. By default libssh2 feature is enabled. See README for more details

### Fixed

- capped read

## 0.6.4

Released on 2025-08-15

### Build

- ssh2-config 0.6.0

## 0.6.3

Released on 2025-07-21

### Fixed

- Fixed issue with SSH authentication

> if the key is resolved and it fails to authenticate, if a password is provided, try to authenticate with the password before returning an error

### Style

- Lint

## 0.6.2

Released on 2025-05-16

### Fixed

- label regex groups and add support for parsing ls output on systems that include SELinux labels, POSIX ACLs, and extended attributes (#9)

## 0.6.1

Released on 2025-03-27

### Fixed

- ssh2-config 0.5

## 0.6.0

Released on 2025-03-15

### Breaking changes

- Edition 2024; ssh2-config 0.4

> Edition 2024; ssh2-config 0.4

### Added

- Breaking: Edition 2024; ssh2-config 0.4

## 0.5.0

Released on 2024-10-26

### Fixed

- test is sync and send
- ci

## 0.4.1

Released on 2024-10-07

### Fixed

- removed users dep

## 0.4.0

Released on 2024-09-30

### Added

- remotefs 0.3

### Fixed

- docker compose ci
- docker compose ci

## 0.3.1

Released on 2024-07-09

### Fixed

- parse special permissions `StT` in ls output

## 0.3.0

Released on 2024-07-09

### Added

- ssh-agent configuration to authenticate

### Fixed

- resolved host from configuration wasn't used
- clippy
- resolved host from configuration wasn't used
- connect

## 0.2.1

Released on 2023-07-06

### Fixed

- tests

## 0.2.0

Released on 2023-05-09

### Added

- ParseRule for ssh2 config file

### Fixed

- docs
- changelog

## 0.1.6

Released on 2023-04-19

### Fixed

- path must be resolved after absolutize

## 0.1.5

Released on 2023-04-18

### Fixed

- Fixed relative paths resolve on Windows
- path-slash 2
- syntax
- bad code

## 0.1.0

Released on 2022-01-04
