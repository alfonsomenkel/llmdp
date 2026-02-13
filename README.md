# llmdp
[![CI](https://github.com/alfonsomenkel/llmdp/actions/workflows/ci.yml/badge.svg)](https://github.com/alfonsomenkel/llmdp/actions/workflows/ci.yml)

LLMDP v0.3.0 is a deterministic language adapter for quality gates.

LMDP requires a contract and delegates validation to llmc, and does not implement validation logic itself.
It executes:

- `cargo fmt -- --check`
- `cargo clippy -- -D warnings`
- `cargo test`

It generates structured facts, invokes `llmc`, and exits with `llmc`'s exit code.

## Adapter Philosophy
- LLMDP is a strict, deterministic quality gate.
- LLMDP is not a generic language signal collector.
- Adapters should emit reproducible quality facts from deterministic checks.
- For non-applicable checks (for example, missing Node scripts), LLMDP omits the field instead of emitting `false`.
- Contract policy and pass/fail interpretation remain in `llmc`; LLMDP only produces facts.

## Supported Languages
- `rust`
- `node`

## Usage
```sh
llmdp run --repo <path> --language rust --contract <path> [--write-facts <path>]
llmdp run --repo . --language rust --contract <contract>
llmdp run --repo . --language node --contract <contract>
```

`--language` is required. Currently supported values:
- `rust`
- `node`

For `node`, LLMDP only runs these scripts when they exist: `lint`, `test`, `build`, `typecheck`. Missing scripts are skipped and no corresponding *_ok field is emitted.
For `node`, if `package-lock.json` exists, LLMDP also runs `npm audit --audit-level=high` and emits `audit_ok`.

## Behavior
- LLMDP generates structured JSON facts.
- Facts are printed to stdout.
- If `--write-facts` is provided, that path is used.
- Otherwise, LLMDP writes to `<repo>/.llmdp_facts.json` temporarily.
- LLMC consumes the facts and prints a verdict.
- LLMDP exits with LLMC’s exit code.

## Build
```sh
cargo build
cargo build --release
cargo test
cargo install --path .
```

Installed binaries are placed in `~/.cargo/bin`.
