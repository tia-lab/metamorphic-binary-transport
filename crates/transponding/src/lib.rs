#![forbid(unsafe_code)]
//! Transponding ownership boundary.
//!
//! Shared column buffer contracts for generated columnar adapters.

pub mod runtime;

pub use runtime::{
    BinaryColumn, BoolColumn, ConstU16Column, DictionaryMeta, F32Column, F32ListColumn, F64Column,
    F64ListColumn, I32Column, I32ListColumn, I64Column, I64ListColumn, OptionalF32Column,
    OptionalF64Column, OptionalI32Column, OptionalI64Column, OptionalU16Column, OptionalU32Column,
    OptionalU64Column, U16Column, U32Column, U32ListColumn, U64Column, Utf8Column, ValidityBitmap,
    checksum_seed, ensure_columnar_size, update_bool, update_bytes, update_f32, update_f64,
    update_i32, update_i64, update_str, update_u16, update_u32, update_u64, update_usize,
};

#[cfg(test)]
mod tests;
