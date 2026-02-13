use crate::adapters::{AdapterError, AdapterFacts, LanguageAdapter};
use serde_json::Value;
use std::path::Path;
use std::process::Command;

pub struct RustAdapter;

fn run_cargo_check(repo: &Path, args: &[&str], check_name: &str) -> Result<bool, AdapterError> {
    let status = Command::new("cargo")
        .args(args)
        .current_dir(repo)
        .status()
        .map_err(|err| {
            AdapterError::operational(format!("failed to run cargo {check_name}: {err}"))
        })?;

    Ok(status.code() == Some(0))
}

impl LanguageAdapter for RustAdapter {
    fn run(&self, repo: &Path) -> Result<AdapterFacts, AdapterError> {
        let fmt_ok = run_cargo_check(repo, &["fmt", "--", "--check"], "fmt -- --check")?;
        let clippy_ok = run_cargo_check(repo, &["clippy", "--", "-D", "warnings"], "clippy")?;
        let tests_ok = run_cargo_check(repo, &["test"], "test")?;

        let mut facts = AdapterFacts::new();
        facts.insert("fmt_ok".to_string(), Value::Bool(fmt_ok));
        facts.insert("clippy_ok".to_string(), Value::Bool(clippy_ok));
        facts.insert("tests_ok".to_string(), Value::Bool(tests_ok));

        Ok(facts)
    }
}
