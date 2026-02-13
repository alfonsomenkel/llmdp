# llmdp Architecture

## Overview

`llmdp` is a deterministic language quality adapter and orchestration CLI.
It does two things:

1. Collects deterministic quality facts from a target repository via language-specific adapters.
2. Delegates contract evaluation to `llmc` and returns `llmc`'s exit code.

`llmdp` intentionally does not implement policy/contract logic. Contract evaluation lives in `llmc`.

External dependency:
- `llmc` repository: [https://github.com/alfonsomenkel/llmc](https://github.com/alfonsomenkel/llmc)
- Runtime expectation: `llmc` binary is available on `PATH`

## High-Level Flow

Command shape:

```sh
llmdp run --repo <path> --language <rust|node> --contract <path> [--write-facts <path>]
```

Execution flow:

```text
Parse CLI args (clap)
  -> Validate --repo exists
  -> Validate --contract exists
  -> Select adapter by --language
      - rust -> RustAdapter
      - node -> NodeAdapter
  -> Run adapter to produce JSON facts
  -> Write facts to:
      --write-facts path, or <repo>/.llmdp_facts.json
  -> Execute:
      llmc --contract <contract> --output <facts_path>
  -> Exit with llmc exit code
```

Operational failures (invalid paths, unsupported language, facts write failure, `llmc` spawn failure) return exit code `3`.

## Component Breakdown

### 1. CLI / Orchestrator

File: `src/main.rs`

Responsibilities:
- Parse `run` command arguments with `clap`.
- Validate input paths.
- Dispatch to the selected adapter.
- Persist facts JSON.
- Invoke `llmc` as a subprocess.
- Propagate `llmc` process exit code.

### 2. Adapter Interface

File: `src/adapters/mod.rs`

Core trait:

```rust
pub trait LanguageAdapter {
    fn run(&self, repo: &str) -> serde_json::Value;
}
```

This keeps language-specific command execution separate from CLI orchestration.

### 3. Rust Adapter

File: `src/adapters/rust.rs`

Runs in target repo:
- `cargo fmt -- --check` -> `fmt_ok`
- `cargo clippy -- -D warnings` -> `clippy_ok`
- `cargo test` -> `tests_ok`

Any command execution error maps to `false` for that fact.

### 4. Node Adapter

File: `src/adapters/node.rs`

Behavior:
- If `package.json` is missing, unreadable, or invalid JSON, returns `{}`.
- Reads `package.json.scripts` and only emits facts for scripts that exist:
  - `lint` -> `lint_ok`
  - `test` -> `tests_ok`
  - `build` -> `build_ok`
  - `typecheck` -> `typecheck_ok`
- If `package-lock.json` exists, runs `npm audit --audit-level=high` and emits `audit_ok`.

This sparse-facts model is intentional: missing scripts produce omitted fields, not `false`.

## Facts and Contracts Boundary

- `llmdp` is responsible for fact generation only.
- `llmc` is responsible for contract interpretation and pass/fail verdicts.
- `llmdp` writes facts to JSON and invokes `llmc` with:

```sh
llmc --contract <contract_path> --output <facts_json_path>
```

This enforces a clean separation:
- deterministic checks in adapters
- policy logic in contracts evaluated by `llmc`

## Error and Exit Code Model

Exit code behavior:
- `0`, `1`, etc.: forwarded from `llmc` verdict execution.
- `3`: `llmdp` operational/runtime failure (bad inputs, unsupported language, write failure, `llmc` invocation failure, missing child exit status).

## Test and CI Architecture

Integration tests:
- `tests/rust_quality_gate.rs`: end-to-end Rust adapter behavior via `llmdp run`.
- `tests/node_adapter.rs`: end-to-end Node adapter behavior, using a fake `npm` in `PATH`.

CI workflow (`.github/workflows/ci.yml`):
- Checks out and builds `llmc` from `alfonsomenkel/llmc`.
- Adds built `llmc` binary to `PATH`.
- Builds and validates `llmdp` (`fmt`, `clippy`, `test`).

This ensures `llmdp` is validated with its runtime dependency available.

## Extending the System

To add a new language adapter:

1. Implement `LanguageAdapter` in `src/adapters/<language>.rs`.
2. Export module/type from `src/adapters/mod.rs`.
3. Add selection branch in `src/main.rs`.
4. Add integration tests in `tests/`.
5. Document facts and behavior in `README.md` and update `CHANGELOG.md` as needed.
