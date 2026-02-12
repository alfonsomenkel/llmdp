use std::fs;
use std::process::Command;
use tempfile::tempdir;

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
        "fmt_ok = true\nclippy_ok = true\ntests_ok = true\n",
    )
    .unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_llmdp"))
        .arg("run")
        .arg("--repo")
        .arg(repo_path)
        .arg("--contract")
        .arg(&contract_path)
        .status()
        .unwrap();

    assert_eq!(status.code(), Some(0));
}
