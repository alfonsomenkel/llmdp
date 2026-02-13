use crate::adapters::LanguageAdapter;
use serde_json::json;
use std::process::Command;

pub struct RustAdapter;

fn run_cargo_check(repo: &str, args: &[&str], check_name: &str) -> Result<bool, String> {
    let status = Command::new("cargo")
        .args(args)
        .current_dir(repo)
        .status()
        .map_err(|err| format!("failed to run cargo {check_name}: {err}"))?;

    Ok(status.code() == Some(0))
}

impl LanguageAdapter for RustAdapter {
    fn run(&self, repo: &str) -> Result<serde_json::Value, String> {
        let fmt_ok = run_cargo_check(repo, &["fmt", "--", "--check"], "fmt -- --check")?;
        let clippy_ok = run_cargo_check(repo, &["clippy", "--", "-D", "warnings"], "clippy")?;
        let tests_ok = run_cargo_check(repo, &["test"], "test")?;

        Ok(json!({
            "fmt_ok": fmt_ok,
            "clippy_ok": clippy_ok,
            "tests_ok": tests_ok
        }))
    }
}
