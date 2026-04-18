# Changelog

## 0.8.3

Released on 2026-04-18

### Fixed

- **scp:** mtime wrongly treated as UTC; use `stat` for timezone-free epoch
  > SCP listings parsed the `ls -l` timestamp as if it were UTC, so clients
  > rendering via `DateTime<Local>` displayed a UTC wall clock instead of
  > the server's local time. Override the `ls`-parsed mtime with the Unix
  > epoch obtained from `stat -c %Y` (GNU) or `stat -f %m` (BSD), probed
  > once per session and cached. Falls back to the `ls` parser on hosts
  > where neither flavor is available. Closes
  > [veeso/termscp#416](https://github.com/veeso/termscp/issues/416).

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
  > - SshSession and Sftp trait implementations for russh
  > - SCP send/recv over russh channels with proper ACK handling
  > - SFTP via russh-sftp
  > - Authentication: password, public key, and SSH agent
  > - Algorithm preference negotiation from SSH config
  > - NoCheckServerKey default handler for server key verification
  > - Benchmarks, examples, and full test coverage

### CI

- benchmark workflow- run test workflow once on pr

### Miscellaneous

- claude.md

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

### Build

- ssh2-config 0.7- 0.7.2

## 0.7.2

Released on 31/01/2026

- `ssh2-config` bumped to `0.7.0`

## 0.7.1

Released on 09/11/2025

- MSRV bumped to 1.88.0
- Fixed compatibility with hosts running with fish set as default shell

## 0.7.0

Released on 01/09/2025

- **BREAKING**: Support for multiple SSH backends:
  - Added new feature to enable **libssh2** backend:
    - Use `libssh2` feature to enable the backend
    - Use `libssh2-vendored` to build the backend with vendored libssh2
  - Added support for [libssh](https://www.libssh.org/) backend
    - Use `libssh` feature to enable the backend
    - Use `libssh-vendored` to build the backend with vendored libssh
  - Removed `new`; use `libssh2` and `libssh` constructors instead.

## 0.6.4

Released on 15/08/2025

- ssh2-config 0.6.0

## 0.6.3

Released on 21/07/2025

- Fixed issue with SSH authentication:
  - if the key is resolved and it fails to authenticate, if a password is provided, try to authenticate with the
    password before returning an error.

## 0.6.2

Released on 16/05/2025

- [Issue 9](https://github.com/remotefs-rs/remotefs-rs-ssh/pull/9): fixed label regex groups and add support for parsing
  ls output on systems that include SELinux labels, POSIX ACLs, and extended attributes

## 0.6.1

Released on 27/03/2025

- bump `ssh2-config` to `0.5.4`

## 0.6.0

Released on 15/03/2025

- bump `ssh2-config` to `0.4.0`
- edition `2024`

## 0.5.0

Released on 26/10/2024

- `SshKeyStorage` must be `Sync` and `Send`

## 0.4.1

Released on 07/10/2024

- Removed unused dep: `users`

## 0.4.0

Released on 30/09/2024

- bump `remotefs` to `0.3.0`

## 0.3.1

Released on 09/07/2024

- Fix: parse special permissions `StT` in ls output

## 0.3.0

Released on 09/07/2024

- Fix: resolved_host from configuration wasn't used to connect
- `SshOpts::method` now requires `KeyMethod` and `MethodType` to setup key method
- Feat: Implemented `SshAgentIdentity` to specify the ssh agent configuration to be used to authenticate.
  - use `SshOpts.ssh_agent_identity()` to set the option

## 0.2.1

Released on 06/07/2023

- If ssh configuration timeout is `0`, don't set connection timeout

## 0.2.0

Released on 09/05/2023

- `SshOpts::config_file` now requires `SshConfigParseRule` as argument to specify the rules to parse the configuration
  file

## 0.1.6

Released on 19/04/2023

- Fixed relative paths resolve on Windows

## 0.1.5

Released on 18/04/2023

- Fixed relative paths resolve on Windows

## 0.1.3

Released on 10/02/2023

- Fixed client using ssh2 config parameter `HostName` to resolve configuration parameters.
- Bump `ssh2-config` to `0.1.4`

## 0.1.2

Released on 30/08/2022

- SshKeyStorage trait MUST return `PathBuf` instead of `Path`

## 0.1.1

Released on 20/07/2022

- Added `ssh2-vendored` feature to build libssl statically

## 0.1.0

Released on 04/01/2022

- First release
