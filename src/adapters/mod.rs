use serde_json::{Map, Value};
use std::fmt;
use std::path::Path;

mod node;
mod rust;

pub type AdapterFacts = Map<String, Value>;

#[derive(Debug)]
pub enum AdapterError {
    Operational(String),
}

impl AdapterError {
    pub fn operational(message: impl Into<String>) -> Self {
        Self::Operational(message.into())
    }
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operational(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for AdapterError {}

/// Language adapters collect deterministic quality facts from a repository.
///
/// Contract requirements:
/// - A check command that exits non-zero must emit the corresponding fact as `false`.
/// - A non-applicable check must omit that fact key.
/// - A required check command that cannot execute must return `AdapterError::Operational`.
pub trait LanguageAdapter {
    fn run(&self, repo: &Path) -> Result<AdapterFacts, AdapterError>;
}

pub use node::NodeAdapter;
pub use rust::RustAdapter;
