#![deny(unsafe_op_in_unsafe_fn)]
//! Public metamorphose ownership boundary.

pub mod runtime;

pub use runtime::{
    ArrowIpcMetamorphoseSchema, ArrowMetamorphoseSchema, CsvMetamorphoseSchema,
    JsonMetamorphoseSchema, MbtMetamorphoseSchema, MetamorphoseFormat, MetamorphoseOutput,
    ParquetMetamorphoseSchema, ProtobufMetamorphoseSchema, arrow, arrow_ipc,
    arrow_ipc_trusted_unchecked, arrow_trusted_unchecked, csv, csv_trusted_unchecked, decode, json,
    json_trusted_unchecked, mbt, mbt_trusted_unchecked, parquet, parquet_trusted_unchecked,
    protobuf, protobuf_trusted_unchecked,
};

#[cfg(test)]
mod tests;
