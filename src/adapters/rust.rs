use crate::adapters::LanguageAdapter;
use serde_json::json;
use std::process::Command;

pub struct RustAdapter;

impl LanguageAdapter for RustAdapter {
    fn run(&self, repo: &str) -> serde_json::Value {
        let fmt_status = Command::new("cargo")
            .arg("fmt")
            .arg("--")
            .arg("--check")
            .current_dir(repo)
            .status();

        let fmt_ok = match fmt_status {
            Ok(status) => status.code() == Some(0),
            Err(_) => false,
        };

        let clippy_status = Command::new("cargo")
            .arg("clippy")
            .arg("--")
            .arg("-D")
            .arg("warnings")
            .current_dir(repo)
            .status();

        let clippy_ok = match clippy_status {
            Ok(status) => status.code() == Some(0),
            Err(_) => false,
        };

        let tests_status = Command::new("cargo").arg("test").current_dir(repo).status();

        let tests_ok = match tests_status {
            Ok(status) => status.code() == Some(0),
            Err(_) => false,
        };

        json!({
            "fmt_ok": fmt_ok,
            "clippy_ok": clippy_ok,
            "tests_ok": tests_ok
        })
    }
}
