# nephren — AGENTS.md

CLI v2ray/xray config generator & subscription manager. Written in Rust (edition 2024).

## Dev commands

```sh
cargo run                          # run the CLI
cargo build                        # debug build
cargo build --release              # release build
cargo clippy                       # lint (no clippy.toml)
cargo fmt                          # format (uses rustfmt defaults)
```

- No test suite exists (no `tests/` dir, no `#[test]` in source).
- No CI pipeline.
- Nix flake available for dev shell (`nix develop`); drops into fish shell.

## Config

- Stored as JSON at `~/.config/nephren/config.json`.
- Fields: `subscriptions`, `nodes`, `active_node`.
- `--config-path <PATH>` flag overrides location.
- **Mutation safety**: `WithContext<T>` panics on drop if data was mutated but `save()` was not called. Always call `.save()` after mutating config.

## CLI subcommands

```
switch <id>                        # activate node by id or name (alias for `nodes activate`)
subscriptions|sub|s  list|add|remove|rm|r|pull
nodes|n              list|activate|active|a
debug                parse
```

`list` commands support `--style table|json|rust` (default: table).

## Protocols

Only **vmess** and **vless** are implemented via `enum_dispatch`. vmess URL parsing is broken (`share/subscription.rs:parse_link` explicitly bails on `vmess://` links despite having a `vmess` parser). vless uses `serde_qs` for query params.

## Architecture notes

- Entrypoint: `src/main.rs` — clap derive, tokio runtime, `simple_logger` (timestamps off, colors on).
- `enum_dispatch(Exec)` on the `Command` enum replaces dynamic dispatch — every subcommand struct impls `Exec`.
- Node/subscription lookup by id (exact string match) **or** name.
- Subscription fetch → base64 decode → line-by-line link parsing.
- `src/share/` holds utility code (`parse` for debug, base64 decoding).
