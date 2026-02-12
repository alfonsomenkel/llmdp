mod adapters;

use adapters::{LanguageAdapter, RustAdapter};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};
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
        language: String,
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
            language,
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

            let facts = match language.as_str() {
                "rust" => {
                    let adapter = RustAdapter;
                    adapter.run(&repo)
                }
                _ => {
                    eprintln!("Error: unsupported language: {language}");
                    process::exit(3);
                }
            };

            let facts_text = facts.to_string();
            let facts_path: PathBuf = match write_facts {
                Some(path) => PathBuf::from(path),
                None => Path::new(&repo).join(".llmdp_facts.json"),
            };

            if fs::write(&facts_path, &facts_text).is_err() {
                process::exit(3);
            }

            let llmc_status = Command::new("llmc")
                .arg("--contract")
                .arg(&contract)
                .arg("--output")
                .arg(&facts_path)
                .status();

            match llmc_status {
                Ok(status) => match status.code() {
                    Some(code) => process::exit(code),
                    None => process::exit(3),
                },
                Err(_) => process::exit(3),
            }
        }
    }
}
