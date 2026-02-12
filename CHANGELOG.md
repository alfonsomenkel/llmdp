# Changelog

All notable changes to this project are documented here.

---

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
