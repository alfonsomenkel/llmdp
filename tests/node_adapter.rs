use serde_json::{Value, json};
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

fn path_with_fake_bin(bin_dir: &Path) -> OsString {
    let existing = std::env::var_os("PATH").unwrap_or_default();
    let mut paths = vec![bin_dir.to_path_buf()];
    paths.extend(std::env::split_paths(&existing));
    std::env::join_paths(paths).unwrap()
}

fn write_fake_npm(bin_dir: &Path) {
    if cfg!(windows) {
        let npm_cmd = bin_dir.join("npm.cmd");
        fs::write(
            npm_cmd,
            r#"@echo off
if "%1"=="run" (
  if "%2"=="test" exit /b 0
  if "%2"=="lint" exit /b 1
  if "%2"=="build" exit /b 0
  if "%2"=="typecheck" exit /b 0
  exit /b 1
)
if "%1"=="audit" exit /b 0
exit /b 1
"#,
        )
        .unwrap();
    } else {
        let npm = bin_dir.join("npm");
        fs::write(
            &npm,
            r#"#!/bin/sh
if [ "$1" = "run" ]; then
  case "$2" in
    test) exit 0 ;;
    lint) exit 1 ;;
    build) exit 0 ;;
    typecheck) exit 0 ;;
    *) exit 1 ;;
  esac
fi

if [ "$1" = "audit" ]; then
  exit 0
fi

exit 1
"#,
        )
        .unwrap();
        Command::new("chmod").arg("+x").arg(&npm).status().unwrap();
    }
}

fn write_contract(path: &Path, rules: Value) {
    let contract = json!({
      "contract": "node_quality_gate",
      "version": 3,
      "inputs": ["repo"],
      "output_type": "object",
      "rules": rules
    });
    fs::write(path, serde_json::to_string_pretty(&contract).unwrap()).unwrap();
}

fn run_llmdp(repo_path: &Path, contract_path: &Path, facts_path: &Path, path_env: &OsString) -> i32 {
    let status = Command::new(env!("CARGO_BIN_EXE_llmdp"))
        .arg("run")
        .arg("--repo")
        .arg(repo_path)
        .arg("--language")
        .arg("node")
        .arg("--contract")
        .arg(contract_path)
        .arg("--write-facts")
        .arg(facts_path)
        .env("PATH", path_env)
        .status()
        .unwrap();
    status.code().unwrap()
}

fn read_facts(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

#[test]
fn node_adapter_without_package_json_passes_empty_contract_and_fails_required_field() {
    let temp_dir = tempdir().unwrap();
    let repo_path = temp_dir.path().join("repo");
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&repo_path).unwrap();
    fs::create_dir_all(&bin_dir).unwrap();
    write_fake_npm(&bin_dir);
    let path_env = path_with_fake_bin(&bin_dir);

    let pass_contract_path = temp_dir.path().join("contract_pass.txt");
    let pass_facts_path = temp_dir.path().join("facts_pass.json");
    write_contract(&pass_contract_path, json!([]));

    let pass_code = run_llmdp(&repo_path, &pass_contract_path, &pass_facts_path, &path_env);
    assert_eq!(pass_code, 0);
    assert_eq!(read_facts(&pass_facts_path), json!({}));

    let fail_contract_path = temp_dir.path().join("contract_fail.txt");
    let fail_facts_path = temp_dir.path().join("facts_fail.json");
    write_contract(
        &fail_contract_path,
        json!([{ "rule": "required_field", "field": "tests_ok" }]),
    );

    let fail_code = run_llmdp(&repo_path, &fail_contract_path, &fail_facts_path, &path_env);
    assert_eq!(fail_code, 1);
    assert_eq!(read_facts(&fail_facts_path), json!({}));
}

#[test]
fn node_adapter_sets_tests_ok_true_when_test_script_exists_and_succeeds() {
    let temp_dir = tempdir().unwrap();
    let repo_path = temp_dir.path().join("repo");
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&repo_path).unwrap();
    fs::create_dir_all(&bin_dir).unwrap();
    write_fake_npm(&bin_dir);
    let path_env = path_with_fake_bin(&bin_dir);

    fs::write(
        repo_path.join("package.json"),
        r#"{
  "name": "sample-node",
  "version": "1.0.0",
  "scripts": {
    "test": "exit 0"
  }
}
"#,
    )
    .unwrap();

    let contract_path = temp_dir.path().join("contract_test_ok.txt");
    let facts_path = temp_dir.path().join("facts_test_ok.json");
    write_contract(
        &contract_path,
        json!([{ "rule": "allowed_values", "field": "tests_ok", "values": [true] }]),
    );

    let code = run_llmdp(&repo_path, &contract_path, &facts_path, &path_env);
    assert_eq!(code, 0);
    assert_eq!(read_facts(&facts_path), json!({ "tests_ok": true }));
}

#[test]
fn node_adapter_sets_lint_ok_false_when_lint_script_fails() {
    let temp_dir = tempdir().unwrap();
    let repo_path = temp_dir.path().join("repo");
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&repo_path).unwrap();
    fs::create_dir_all(&bin_dir).unwrap();
    write_fake_npm(&bin_dir);
    let path_env = path_with_fake_bin(&bin_dir);

    fs::write(
        repo_path.join("package.json"),
        r#"{
  "name": "sample-node",
  "version": "1.0.0",
  "scripts": {
    "lint": "exit 1"
  }
}
"#,
    )
    .unwrap();

    let contract_path = temp_dir.path().join("contract_lint_fail.txt");
    let facts_path = temp_dir.path().join("facts_lint_fail.json");
    write_contract(
        &contract_path,
        json!([{ "rule": "allowed_values", "field": "lint_ok", "values": [false] }]),
    );

    let code = run_llmdp(&repo_path, &contract_path, &facts_path, &path_env);
    assert_eq!(code, 0);
    assert_eq!(read_facts(&facts_path), json!({ "lint_ok": false }));
}
