# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

remotefs-ssh is a Rust library providing SFTP and SCP file transfer clients implementing the `remotefs::RemoteFs` trait. It supports two SSH backends via cargo features: `libssh2` (default, via `ssh2` crate) and `libssh` (via `libssh-rs` crate). At least one backend must be enabled.

## Build Commands

```bash
cargo build                    # Build with default features (libssh2)
cargo lint                     # Clippy with all backends (-Dwarnings)
cargo test-all                 # Test both backends
cargo test-libssh2             # Test libssh2 only
cargo test-libssh              # Test libssh only (no default features)
cargo test -- test_name        # Run a single test
```

These aliases are defined in `.cargo/config.toml`.

## Architecture

### Backend Abstraction

The crate is generic over SSH backends via two traits in `src/ssh/backend.rs`:

- **`SshSession`** – abstracts SSH connection, command execution, SCP, and SFTP session creation. Has associated type `type Sftp: Sftp`.
- **`Sftp`** – abstracts SFTP file operations (mkdir, stat, readdir, open, rename, etc.)

Backend implementations live in `src/ssh/backend/libssh2.rs` and `src/ssh/backend/libssh.rs`.

### Client Types

- **`SftpFs<S: SshSession>`** (`src/ssh/sftp.rs`) – SFTP client implementing `RemoteFs`
- **`ScpFs<S: SshSession>`** (`src/ssh/scp.rs`) – SCP client implementing `RemoteFs`. Uses shell commands and `ls` output parsing (`src/utils/parser.rs`) for directory operations.

Both are generic over the backend, selected at compile time with no runtime dispatch.

### Configuration

- **`SshOpts`** (`src/ssh/mod.rs`) – builder-pattern config for host, port, credentials, SSH agent, key methods, and config file path.
- **`Config`** (`src/ssh/config.rs`) – resolves SSH config files. Priority: SshOpts > SSH config > defaults.

### Authentication Order

SSH agent → RSA key (via `SshKeyStorage` trait) → password.

## Testing

Tests require Docker. They use `testcontainers` to spin up an OpenSSH server container. Test files are in `src/ssh/sftp/tests/` and `src/ssh/scp/tests/`, with per-backend modules (`libssh.rs`, `libssh2.rs`).

## Conventions

- Rust edition 2024, MSRV 1.88.0
- `rustfmt` config: `group_imports = "StdExternalCrate"`, `imports_granularity = "Module"`
- Clippy must pass with zero warnings (`-Dwarnings`)
- Conventional commits (conventionalcommits.org)
- Update CHANGELOG.md for changes
- Minimize dependencies; all protocols must be optional via features
