# AGENTS.md

## What this is

CLI tool for managing v2ray/xray proxy configs. Manages subscriptions (remote node lists), parses proxy links, and switches active nodes. Config lives at `~/.config/nephren/config.json`.

## Build & check

```sh
cargo build
cargo clippy
cargo fmt --check
```

No test suite exists yet. Always run `cargo clippy` and `cargo fmt --check` before considering work done.

## Dev environment

Nix flake provides the toolchain via fenix (Rust latest + clippy + rustfmt + cargo-deny + cargo-watch + rust-analyzer). Enter with `nix develop` (drops into fish). If not using Nix, Rust 2024 edition is required (`edition = "2024"` in Cargo.toml).

## Architecture

- **Entry**: `src/main.rs` — clap `Parser`, logs to stderr via `simple_logger`, dispatches to `Command`

- **Commands** (`src/commands/`): `enum_dispatch` over `Exec` trait. Top-level: `Switch`, `Subscriptions`, `Nodes`. Each subcommand module has its own file.

- **Context** (`src/context.rs`): `Context` holds `config_path`. `WithContext<T>` wraps loaded data — **panics on drop if mutated but not saved** (via `DerefMut` setting a `Cell<bool>` flag). Always call `.save()` after mutating config through `WithContext`.

- **Models** (`src/models/`): `Config` (serde JSON), `Subscription` (id, name, url), `Node` (id, name, address, port, `NodeKind`). IDs are `i32` newtypes with `.next()` for incrementing.

- **Parse** (`src/parse/`): Only `vmess://` links are supported. `vless.rs` is a stub.

- **share** (`src/share/`): `parse_base64_input` decodes base64 subscription body, dispatches each line to `parse_link`.

## Key conventions

- `Switch` is an alias for `Nodes::Activate` — same logic, different CLI surface.
- Subscription aliases: `sub`, `s`. Node aliases: `n`. Activate aliases: `active`, `a`. Remove aliases: `rm`, `r`.
- List commands support `--style table|json|rust` (default: table, rendered with `tabled`).
- Node lookup and subscription lookup accept both numeric ID and name string.
- `NodeKind` is tagged enum with `#[serde(tag = "kind_name")]`. Adding a new protocol requires updating `NodeKind`, `Display`, `parse_link`, and adding a model + parser module.
