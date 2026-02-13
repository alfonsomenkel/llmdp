# Changelog

All notable changes to this project are documented here.

---

## [Unreleased]

### Changed
- Formalized adapter contract in code with typed `AdapterFacts`, typed `AdapterError`, and `LanguageAdapter::run(&Path, ...)`.
- Main orchestration now serializes typed adapter facts and handles fact serialization failures as operational exit `3`.
- Adapter execution failures now return operational exit code `3` instead of being collapsed into `false` facts.
- Rust adapter now treats `cargo` invocation failures as operational failures.
- Node adapter now treats `npm` invocation failures for required checks as operational failures.

### Added
- Documented deterministic failure semantics in `README.md` and `ARCHITECTURE.md`.
- Integration tests for missing tool operational failures (`cargo`/`npm`) returning exit code `3`.

## [0.3.0] - 2026-02-12

### Added
- Node language adapter
- Node integration tests
- Multi-language fact generation support

### Changed
- `--language` flag now supports `rust` and `node`

## [0.2.0] - Adapter + Language Flag

### Added
- Required `--language <LANG>` argument for `llmdp run`
- Language adapter abstraction (`LanguageAdapter` trait)
- Rust-specific adapter implementation (`RustAdapter`)

### Changed
- `run` command now selects facts generation by language adapter
- Unsupported languages exit with code `3`

## [0.1.0] - Initial Release

### Added
- Rust-first deterministic quality gate
- Executes:
  - cargo fmt -- --check
  - cargo clippy -- -D warnings
  - cargo test
- Structured facts generation (fmt_ok, clippy_ok, tests_ok)
- Integration with llmc for contract validation
- Exit code propagation from llmc
- Optional --write-facts flag
- End-to-end integration tests (pass and fail cases)
- Makefile with build/test/install targets
- CI support
