//! Schema-agnostic MBT core runtime.
//!
//! This crate owns only the envelope, checksum, error, and dispatch contracts.

pub mod codec;
pub mod envelope;
pub mod error;
pub mod output;
pub mod runtime;

pub use error::Result;
pub use runtime::{BinaryInspection, MbtSchema, access, encode, encode_owned, inspect};

#[cfg(test)]
mod tests;
