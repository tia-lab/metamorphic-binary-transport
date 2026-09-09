#![forbid(unsafe_code)]
//! Benchmark ownership boundary.
//!
//! Benchmark fixtures and report writers live here so binaries only own timing
//! boundaries and any required trusted-access calls.

pub mod compression;
pub mod projection;
pub mod telemetry_regression;

#[cfg(test)]
mod tests;
