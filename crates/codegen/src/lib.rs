#![forbid(unsafe_code)]
//! Codegen ownership boundary.
//!
//! This crate owns descriptor parsing and core-only Rust emission for MBT
//! schemas. Runtime transport code remains in `metamorphic_binary_transport_core`.

pub mod config;
pub mod descriptor;
pub mod emit;
pub mod error;
pub mod model;
pub mod options;
pub mod rust_emit;

#[cfg(test)]
mod tests;
