# llmdp
[![CI](https://github.com/alfonsomenkel/llmdp/actions/workflows/ci.yml/badge.svg)](https://github.com/alfonsomenkel/llmdp/actions/workflows/ci.yml)

LLMDP v0.1 is a deterministic Rust quality gate.

It always requires a contract and does not implement validation logic itself.
It executes:

- `cargo fmt -- --check`
- `cargo clippy -- -D warnings`
- `cargo test`

It generates structured facts, invokes `llmc`, and exits with `llmc`'s exit code.

## Usage
```sh
llmdp run --repo <path> --contract <path> [--write-facts <path>]
```

## Behavior
- Facts are written to a file.
- If `--write-facts` is provided, that path is used.
- Otherwise, LLMDP writes to `<repo>/.llmdp_facts.json` as the temporary facts file.
- LLMC prints verdict JSON.
- LLMDP propagates LLMC exit codes.

## Build
```sh
cargo build
cargo build --release
cargo test
cargo install --path .
```

Installed binaries are placed in `~/.cargo/bin`.
