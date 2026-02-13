use serde_json::Value;

mod node;
mod rust;

pub trait LanguageAdapter {
    fn run(&self, repo: &str) -> Result<Value, String>;
}

pub use node::NodeAdapter;
pub use rust::RustAdapter;
