use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, CodegenError>;

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("descriptor error: {0}")]
    Descriptor(String),
    #[error("missing descriptor: {0}")]
    MissingDescriptor(&'static str),
    #[error("missing option: {0}")]
    MissingOption(&'static str),
    #[error("invalid option {name}: {reason}")]
    InvalidOption { name: &'static str, reason: String },
    #[error("invalid schema: {0}")]
    InvalidSchema(String),
    #[error("rustfmt failed: {0}")]
    Rustfmt(String),
    #[error("unsupported argument: {0}")]
    UnsupportedArgument(String),
    #[error("generated output differs: {0}")]
    GeneratedDiff(PathBuf),
}
