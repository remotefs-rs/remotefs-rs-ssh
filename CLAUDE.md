# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

`AGENTS.md` holds the full agent contract for this repository. Read it before
making changes; this file summarizes the parts needed most often.

## Commands

Every task runs through a [`just`](https://just.systems) recipe. Do not bypass a
recipe with an ad hoc `cargo` or tool command. If a recurring task has no
recipe, add one under `just/` before using it. Run `just` to list all recipes.

```sh
just build                 # cargo build --all-targets
just release               # release build
just test                  # cargo test --all-targets, then --doc
just coverage              # cargo llvm-cov, writes lcov.info
just fmt                   # dprint fmt (Markdown, Rust, TOML, YAML)
just fmt_check             # dprint check
just lint "-- -D warnings" # alias of `just clippy`, runs with --all-features
just doc                   # cargo doc --all-features with RUSTDOCFLAGS="-D warnings"
just deny                  # cargo deny check
just scan_secrets          # trufflehog filesystem
just check                 # the full local quality gate
just setup_githooks        # point core.hooksPath at .githooks
just changelog_preview 0.1.0
just changelog 0.1.0
just publish "--dry-run --allow-dirty"
```

`just check` is the required gate before declaring work done. It chains
`fmt_check`, Clippy with warnings denied, `doc`, `deny`, and `test`.

None of the SSH backend features are enabled by default in `--all-targets`
builds beyond `find` and `libssh2` (the crate's own defaults). Backend-gated
code (`libssh`, `russh`) and their examples/benches only compile when you pass
the matching feature explicitly, e.g. `just build "--features russh"` or
`just test "--features libssh,russh"`.

To run one test, pass the filter through the recipe's `args`:

```sh
just test "absolutize_path"
just test "-- --nocapture"
```

Never request build or test parallelism above eight from the CLI. That is an
invocation constraint only; do not write the cap into tracked files.

If a required tool is missing, say so. Never claim a check passed or silently
swap in a weaker command.

## Architecture

remotefs-ssh is a [remotefs](https://github.com/remotefs-rs/remotefs-rs)
client implementation providing SCP/SFTP access over SSH. It is a
library-only crate (`src/lib.rs`, crate name `remotefs_ssh`) — there is no
binary target.

- **Backend abstraction.** `src/ssh/backend.rs` defines the common trait and
  gates three interchangeable SSH implementations behind Cargo features:
  `libssh2` (default, C library), `libssh` (C library, not `Sync`), and
  `russh` (pure Rust, async under the hood, requires `tokio`). Each backend
  lives in its own submodule and is `#[cfg(feature = "...")]`-gated end to
  end, including its examples (`examples/scp.rs`, `examples/scp-russh.rs`,
  `examples/sftp.rs`, `examples/sftp-russh.rs`) and benches (`benches/`).
  `libssh-vendored` / `libssh2-vendored` swap in vendored builds of the
  corresponding C library.
- **Command layer.** `Justfile` is a thin importer. Each recipe group lives in
  its own file under `just/` (`build`, `test`, `code_check`, `changelog`,
  `publish`) and carries a `[group(...)]` attribute so `just --list` stays
  organized. Recipes take an `args=""` passthrough rather than hard-coding
  flags.
- **Formatting is dprint, not cargo fmt.** `dprint.json` owns Markdown, TOML,
  and YAML, and delegates `.rs` files to nightly rustfmt through its exec
  plugin. `rustfmt.toml` uses nightly-only options
  (`imports_granularity`, `group_imports`), which is why nightly is required.
  Always format with `just fmt`.
- **Release path.** Commits follow Conventional Commits and `cliff.toml` turns
  them into `CHANGELOG.md`. Publishing goes through `just publish`
  (`cargo publish --locked`); version bumps live in `Cargo.toml`.
- **Supply-chain policy.** `deny.toml` is strict: license allowlist,
  `yanked = "deny"`, `unmaintained = "all"`, wildcard versions denied, and
  crates.io as the only allowed source. Runs with `all-features = true`, so a
  new dependency behind any backend feature is still checked.
- **Tests can spin up containers.** `dev-dependencies` include
  `testcontainers`; some tests (see `src/ssh/container.rs`) start real SSH/SFTP
  containers rather than mocking the wire protocol. Docker must be available
  locally for those to pass.

## Conventions

- Toolchain is pinned to Rust 1.98.0, edition 2024 (`rust-toolchain.toml`).
  `package.rust-version` in `Cargo.toml` is the crate's published MSRV
  (currently 1.88.0) and is intentionally lower than the pinned dev
  toolchain — do not "sync" the two.
- Use `module_name.rs`; never `mod.rs`.
- Public library items need canonical rustdoc, including a runnable example.
  `just test` runs doctests, and `just doc` denies warnings.
- Named format placeholders, not positional `{}`.
- Prefer `#[expect]` with a reason over `#[allow]`.
- Keep `Cargo.toml` dependency and feature entries alphabetically sorted, with
  bare minimal versions.
- Conventional Commits, imperative and lower-case. No agent attribution,
  session links, or agent `Co-Authored-By` lines.
- Do not stage planning state. `docs/superpowers/`, `.superpowers/`, and
  `.claude/plans/` are gitignored and dprint-excluded.
- After editing a Markdown file that contains a table, run
  `fmt-md-tables -i <file>`.
- After any change under `.github/workflows/`, run `zizmor .github/workflows`
  until it exits clean. Pin actions to a full commit SHA with the matching tag
  in a trailing comment, declare least-privilege permissions, and set
  `persist-credentials: false` on checkout.
