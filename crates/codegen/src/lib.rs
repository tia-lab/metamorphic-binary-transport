#![forbid(unsafe_code)]
//! Codegen ownership boundary.
//!
//! This crate owns descriptor parsing and Rust emission for MBT schema surfaces.
//! Runtime transport code remains in `mbt_core`.

pub mod config;
pub mod descriptor;
pub mod emit;
pub mod error;
pub mod model;
pub mod options;
pub mod rust_emit;

#[cfg(test)]
mod tests;
