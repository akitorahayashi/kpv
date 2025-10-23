## Overview

[![CI Status](https://github.com/akitorahayashi/kpv/actions/workflows/ci-workflows.yml/badge.svg)](https://github.com/akitorahayashi/kpv/actions/workflows/ci-workflows.yml)

A friendly CLI for stashing and re-attaching `.env` files across projects.

## Features

- **save** (`sv`) &mdash; capture the current directory's `.env` under a named key (`kpv save <key>` or `kpv sv <key>`)
- **link** (`ln`) &mdash; symlink a saved `.env` back into the working tree (`kpv link <key>` or `kpv ln <key>`)
- **list** (`ls`) &mdash; enumerate the keys already managed by `kpv` (`kpv list` or `kpv ls`)

## Architecture Layers

`kpv` keeps the binary lean by funnelling everything through three library layers:

- `src/commands.rs` holds the public API used by both the CLI and integration tests. It wires dependencies, performs user-facing logging, and returns `kpv::error::KpvError` on failure.
- `src/core/` encapsulates the business rules via command structs (save/link/list) that implement a shared `Execute` trait. Each command decides when to error without performing I/O.
- `src/storage.rs` provides the `Storage` trait plus the `FilesystemStorage` implementation that talks to the filesystem, keeping path resolution and symlink logic in one place.

This separation keeps side effects at the edge, makes core logic testable with mocks, and clarifies where to add new behaviors.

Example session:

```bash
$ kpv save web-app
✅ Saved: ./.env -> 'web-app'

$ kpv list
📦 Saved keys:
- web-app

$ kpv link web-app
🔗 Linked: 'web-app' -> ./.env
```

> **Heads-up:** `kpv link` refuses to overwrite an existing `.env`. Remove or rename the file first if you truly want to replace it.

## Installation

```bash
cargo install --path .
# or
cargo build --release
```

The optimized binary lives at `target/release/kpv`.

## Automation With `just`

The project uses [`just`](https://github.com/casey/just) for automation. After installing `just`, the following recipes keep daily tasks tidy:

- `just setup` &mdash; pre-fetch dependencies.
- `just format` &mdash; run `cargo fmt` across the workspace.
- `just lint` &mdash; format check plus `cargo clippy --all-targets --all-features -D warnings`.
- `just test` &mdash; run all tests.
- `just build-release` &mdash; build the optimized release binary.
- `just clean` &mdash; remove build and tool caches.

See the inline comments in `justfile` for additional utilities.

## Testing Culture

Tests follow standard Rust conventions:

- **Unit Tests**: Located within `src/` modules (e.g., `src/storage.rs`) to cover low-level helpers and filesystem boundaries.
- **Core Logic Tests**: The command pattern is covered inside `src/core/` with mock storage implementations, ensuring business rules can evolve without touching the filesystem.
- **Integration Tests**: Located in the `tests/` directory. Each `.rs` file (e.g., `tests/cli_commands.rs`, `tests/commands_api.rs`, `tests/cli_flow.rs`) is compiled as a separate crate, testing the public API and full user workflows from an external perspective.
- **Common Utilities**: Shared test code like `TestContext` resides in `tests/common/mod.rs` and is included in integration tests via `mod common`.

Tests involving filesystem modifications use `serial_test` to ensure they run sequentially and avoid conflicts. Run all tests via `cargo test` or `just test`.

## Storage Layout

`kpv` keeps everything under `~/.config/kpv/<key>/.env`, making it easy to inspect or back up the managed secrets.

```text
~/.config/kpv/
  web-app/
    .env
  analytics-service/
    .env
```

## Contributing

1. `just setup`
2. Implement your change and update/extend the relevant tests.
3. `just lint` and `just test`
4. Open a PR &mdash; the CI mirrors these commands via reusable GitHub Actions workflows.

Thanks for helping keep `.env` juggling painless!
