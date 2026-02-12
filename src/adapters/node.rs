use crate::adapters::LanguageAdapter;
use serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct NodeAdapter;

fn run_npm_script(repo: &str, script: &str) -> bool {
    let status = Command::new("npm")
        .arg("run")
        .arg(script)
        .current_dir(repo)
        .status();

    matches!(status, Ok(exit_status) if exit_status.code() == Some(0))
}

fn run_npm_audit(repo: &str) -> bool {
    let status = Command::new("npm")
        .arg("audit")
        .arg("--audit-level=high")
        .current_dir(repo)
        .status();

    matches!(status, Ok(exit_status) if exit_status.code() == Some(0))
}

impl LanguageAdapter for NodeAdapter {
    fn run(&self, repo: &str) -> Value {
        let package_json_path = Path::new(repo).join("package.json");
        if !package_json_path.exists() {
            return json!({});
        }

        let package_json_text = match fs::read_to_string(&package_json_path) {
            Ok(text) => text,
            Err(_) => return json!({}),
        };

        let package_json: Value = match serde_json::from_str(&package_json_text) {
            Ok(value) => value,
            Err(_) => return json!({}),
        };

        let scripts = package_json.get("scripts").and_then(Value::as_object);

        let mut facts = Map::new();
        let checks = [
            ("lint", "lint_ok"),
            ("test", "tests_ok"),
            ("build", "build_ok"),
            ("typecheck", "typecheck_ok"),
        ];

        for (script, fact) in checks {
            if scripts.is_some_and(|entries| entries.contains_key(script)) {
                facts.insert(fact.to_string(), Value::Bool(run_npm_script(repo, script)));
            }
        }

        let package_lock_path = Path::new(repo).join("package-lock.json");
        if package_lock_path.exists() {
            facts.insert("audit_ok".to_string(), Value::Bool(run_npm_audit(repo)));
        }

        json!(facts)
    }
}
