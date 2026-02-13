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

## Adapter Interface
The adapter contract is defined in `src/adapters/mod.rs`:

```rust
pub type AdapterFacts = serde_json::Map<String, serde_json::Value>;

pub enum AdapterError {
    Operational(String),
}

pub trait LanguageAdapter {
    fn run(&self, repo: &std::path::Path) -> Result<AdapterFacts, AdapterError>;
}
```

Semantics for all adapters:
- Check command exits non-zero: emit that fact as `false`.
- Check is not applicable: omit that fact key.
- Required check command cannot execute: return `AdapterError::Operational` and exit `3`.

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
- If `--write-facts` is provided, that path is used.
- Otherwise, LLMDP writes to `<repo>/.llmdp_facts.json`.
- LLMC consumes the facts and prints a verdict.
- LLMDP exits with LLMC’s exit code.

## Failure Semantics
- Determinism first: check outcomes must be reproducible from command results and repository state.
- Check command exits non-zero: emit the corresponding `*_ok` fact as `false`.
- Optional check is not applicable: omit that fact key.
  - Node examples: missing script (`lint`, `test`, `build`, `typecheck`) or missing `package-lock.json` for `audit_ok`.
- Required check command cannot be executed (for example, binary missing or spawn failure): operational failure with exit code `3`.
  - In this case, LLMDP exits before contract evaluation and does not write a facts file.
- Node `package.json` missing, unreadable, or invalid JSON: return `{}` (no checks are run).
- LLMC invocation failure (for example, `llmc` missing on `PATH`): operational failure with exit code `3`.

## Build
```sh
cargo build
cargo build --release
cargo test
cargo install --path .
```

Installed binaries are placed in `~/.cargo/bin`.
