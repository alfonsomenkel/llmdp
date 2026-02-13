use crate::adapters::LanguageAdapter;
use serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct NodeAdapter;

fn run_npm_script(repo: &str, script: &str) -> Result<bool, String> {
    let status = Command::new("npm")
        .arg("run")
        .arg(script)
        .current_dir(repo)
        .status()
        .map_err(|err| format!("failed to run npm script '{script}': {err}"))?;

    Ok(status.code() == Some(0))
}

fn run_npm_audit(repo: &str) -> Result<bool, String> {
    let status = Command::new("npm")
        .arg("audit")
        .arg("--audit-level=high")
        .current_dir(repo)
        .status()
        .map_err(|err| format!("failed to run npm audit: {err}"))?;

    Ok(status.code() == Some(0))
}

impl LanguageAdapter for NodeAdapter {
    fn run(&self, repo: &str) -> Result<Value, String> {
        let package_json_path = Path::new(repo).join("package.json");
        if !package_json_path.exists() {
            return Ok(json!({}));
        }

        let package_json_text = match fs::read_to_string(&package_json_path) {
            Ok(text) => text,
            Err(_) => return Ok(json!({})),
        };

        let package_json: Value = match serde_json::from_str(&package_json_text) {
            Ok(value) => value,
            Err(_) => return Ok(json!({})),
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
                let check_ok = run_npm_script(repo, script)?;
                facts.insert(fact.to_string(), Value::Bool(check_ok));
            }
        }

        let package_lock_path = Path::new(repo).join("package-lock.json");
        if package_lock_path.exists() {
            let audit_ok = run_npm_audit(repo)?;
            facts.insert("audit_ok".to_string(), Value::Bool(audit_ok));
        }

        Ok(json!(facts))
    }
}
