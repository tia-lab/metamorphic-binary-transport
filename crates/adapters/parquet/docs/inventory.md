# `crates/adapters/parquet` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/adapters/parquet/src/arrow_bridge.rs`: Arrow array and record-batch bridge helpers reused before Parquet writing.
- `crates/adapters/parquet/src/lib.rs`: uncompressed Parquet writer and adapter exports for generated metamorphose modules.
- `crates/adapters/parquet/src/tests/mod.rs`: Parquet adapter tests for record-batch conversion and Parquet byte writing.
