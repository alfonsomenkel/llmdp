use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

fn path_with_only_bin(bin_dir: &Path) -> OsString {
    std::env::join_paths([bin_dir]).unwrap()
}

fn write_fake_llmc_success(bin_dir: &Path) {
    if cfg!(windows) {
        let llmc_cmd = bin_dir.join("llmc.cmd");
        fs::write(
            llmc_cmd,
            r#"@echo off
exit /b 0
"#,
        )
        .unwrap();
    } else {
        let llmc = bin_dir.join("llmc");
        fs::write(
            &llmc,
            r#"#!/bin/sh
exit 0
"#,
        )
        .unwrap();
        Command::new("chmod").arg("+x").arg(&llmc).status().unwrap();
    }
}

#[test]
fn rust_quality_gate_passes_for_minimal_crate() {
    let temp_dir = tempdir().unwrap();
    let repo_path = temp_dir.path();

    fs::create_dir_all(repo_path.join("src")).unwrap();

    fs::write(
        repo_path.join("Cargo.toml"),
        r#"[package]
name = "sample_crate"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
"#,
    )
    .unwrap();

    fs::write(
        repo_path.join("src/lib.rs"),
        r#"pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn adds_numbers() {
        assert_eq!(add(2, 2), 4);
    }
}
"#,
    )
    .unwrap();

    let contract_path = repo_path.join("contract.txt");
    fs::write(
        &contract_path,
        r#"{
  "contract": "rust_quality_gate",
  "version": 3,
  "inputs": ["repo"],
  "output_type": "object",
  "rules": [
    { "rule": "allowed_values", "field": "fmt_ok", "values": [true] },
    { "rule": "allowed_values", "field": "clippy_ok", "values": [true] },
    { "rule": "allowed_values", "field": "tests_ok", "values": [true] }
  ]
}
"#,
    )
    .unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_llmdp"))
        .arg("run")
        .arg("--repo")
        .arg(repo_path)
        .arg("--language")
        .arg("rust")
        .arg("--contract")
        .arg(&contract_path)
        .status()
        .unwrap();

    assert_eq!(status.code(), Some(0));
}

#[test]
fn rust_quality_gate_fails_when_fmt_fails() {
    let temp_dir = tempdir().unwrap();
    let repo_path = temp_dir.path();

    fs::create_dir_all(repo_path.join("src")).unwrap();

    fs::write(
        repo_path.join("Cargo.toml"),
        r#"[package]
name = "sample_crate"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
"#,
    )
    .unwrap();

    fs::write(
        repo_path.join("src/lib.rs"),
        r#"pub fn add(a:i32,b:i32)->i32{a+b}
#[cfg(test)]
mod tests{
use super::add;
#[test]
fn adds_numbers(){assert_eq!(add(2,2),4);}
}
"#,
    )
    .unwrap();

    let contract_path = repo_path.join("contract.txt");
    fs::write(
        &contract_path,
        r#"{
  "contract": "rust_quality_gate",
  "version": 3,
  "inputs": ["repo"],
  "output_type": "object",
  "rules": [
    { "rule": "allowed_values", "field": "fmt_ok", "values": [true] },
    { "rule": "allowed_values", "field": "clippy_ok", "values": [true] },
    { "rule": "allowed_values", "field": "tests_ok", "values": [true] }
  ]
}
"#,
    )
    .unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_llmdp"))
        .arg("run")
        .arg("--repo")
        .arg(repo_path)
        .arg("--language")
        .arg("rust")
        .arg("--contract")
        .arg(&contract_path)
        .status()
        .unwrap();

    assert_eq!(status.code(), Some(1));
}

#[test]
fn rust_adapter_exits_with_operational_error_when_cargo_is_missing() {
    let temp_dir = tempdir().unwrap();
    let repo_path = temp_dir.path().join("repo");
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir_all(repo_path.join("src")).unwrap();
    fs::create_dir_all(&bin_dir).unwrap();
    write_fake_llmc_success(&bin_dir);
    let path_env = path_with_only_bin(&bin_dir);

    fs::write(
        repo_path.join("Cargo.toml"),
        r#"[package]
name = "sample_crate"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
"#,
    )
    .unwrap();

    fs::write(
        repo_path.join("src/lib.rs"),
        r#"pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#,
    )
    .unwrap();

    let contract_path = temp_dir.path().join("contract.txt");
    fs::write(
        &contract_path,
        r#"{
  "contract": "rust_quality_gate",
  "version": 3,
  "inputs": ["repo"],
  "output_type": "object",
  "rules": []
}
"#,
    )
    .unwrap();

    let facts_path = temp_dir.path().join("facts_missing_cargo.json");
    let status = Command::new(env!("CARGO_BIN_EXE_llmdp"))
        .arg("run")
        .arg("--repo")
        .arg(repo_path)
        .arg("--language")
        .arg("rust")
        .arg("--contract")
        .arg(&contract_path)
        .arg("--write-facts")
        .arg(&facts_path)
        .env("PATH", &path_env)
        .status()
        .unwrap();

    assert_eq!(status.code(), Some(3));
    assert!(!facts_path.exists());
}
