use clap::{Parser, Subcommand};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::{self, Command};

#[derive(Parser)]
#[command(name = "llmdp")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(long, required = true)]
        repo: String,
        #[arg(long, required = true)]
        contract: String,
        #[arg(long)]
        write_facts: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            repo,
            contract,
            write_facts,
        } => {
            if !Path::new(&repo).exists() {
                eprintln!("Error: repo path does not exist: {repo}");
                process::exit(3);
            }

            if !Path::new(&contract).exists() {
                eprintln!("Error: contract path does not exist: {contract}");
                process::exit(3);
            }

            let fmt_status = Command::new("cargo")
                .arg("fmt")
                .arg("--")
                .arg("--check")
                .current_dir(&repo)
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
                .current_dir(&repo)
                .status();

            let clippy_ok = match clippy_status {
                Ok(status) => status.code() == Some(0),
                Err(_) => false,
            };

            let tests_status = Command::new("cargo")
                .arg("test")
                .current_dir(&repo)
                .status();

            let tests_ok = match tests_status {
                Ok(status) => status.code() == Some(0),
                Err(_) => false,
            };

            let facts = json!({
                "fmt_ok": fmt_ok,
                "clippy_ok": clippy_ok,
                "tests_ok": tests_ok
            });

            let facts_text = facts.to_string();
            println!("{facts_text}");

            if let Some(path) = write_facts {
                let _ = fs::write(path, &facts_text);
            }
        }
    }
}
