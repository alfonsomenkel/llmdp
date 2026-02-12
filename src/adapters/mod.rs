use serde_json::Value;

mod rust;

pub trait LanguageAdapter {
    fn run(&self, repo: &str) -> Value;
}

pub use rust::RustAdapter;
