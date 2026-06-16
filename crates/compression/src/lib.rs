#![forbid(unsafe_code)]
//! Public compression ownership boundary.

pub mod runtime;

pub use runtime::{
    CompressionConfig, DEFAULT_ZSTD_LEVEL, compress, compress_into, compress_with_config,
    decompress, decompress_into,
};

#[cfg(test)]
mod tests;
