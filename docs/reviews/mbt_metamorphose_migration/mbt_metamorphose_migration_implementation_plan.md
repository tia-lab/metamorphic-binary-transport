```
MATHILDE PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 MATHILDE. All Rights Reserved.

This document contains trade secrets and confidential information owned
exclusively by MATHILDE, protected under Swiss law (URG, UWG, Art. 162 StGB).

PROHIBITED: Reproduction, copying, distribution, disclosure, or derivative
works without prior written authorization from MATHILDE.

ACCESS REQUIREMENT: Executed NDA with MATHILDE required. Unauthorized access
or possession violates Swiss law. Violations subject to civil remedies,
injunctive relief, damages, and criminal prosecution.

Legal Contact: massimo.nicora@wnlegal.ch
```

# Implementation Plan: MBT Metamorphose Migration

Status: `AUDITED_AWAITING_OWNER_APPROVAL`
Date: 2026-06-15
Slug: `mbt_metamorphose_migration`

## 1. Scope

Approved spec:

```text
docs/specs/mbt_metamorphose_migration_SPEC.md
```

Passed spec audit:

```text
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_peer_audit.md
```

This plan migrates the old MBT metamorphose behavior into the split
`metamorphic-binary-transport` workspace:

```text
checked or trusted MBT bytes
  -> generated schema-local writer
  -> selected boundary format
```

For Arrow, Arrow IPC, and Parquet:

```text
checked or trusted MBT bytes
  -> generated schema-local hidden transponding
  -> one adapter boundary writer
```

No code change is authorized until this plan is audited or explicitly approved.

## 2. Required Reads Completed

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`
- `docs/specs/mbt_metamorphose_migration_SPEC.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_peer_audit.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan_peer_audit.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan_peer_audit_v2.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan_peer_audit_v3.md`
- old source oracle:
  `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/metamorphose/*`
- old source oracle:
  `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`
- old benchmark oracle:
  `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md`

## 3. Non-Negotiable Boundaries

Do not edit:

```text
crates/core/src/* except crates/core/src/output.rs, crates/core/src/lib.rs, and crates/core/src/tests/test_output.rs
crates/projection/src/*
proto/mathilde/options.proto
```

The only authorized core change is the dependency-free output helper module
required to keep adapter crates independent from `crates/metamorphose`.

Do not hand edit generated adapter files after codegen writes them.

Do not add:

- serde JSON writer paths;
- prost DTO materialization;
- CSV crate dependency;
- row DTO vectors in metamorphose hot paths;
- public transponding API;
- fallback runtime reflection over fields;
- adapter dependencies to default schema builds.

Stop and amend the spec before code if:

- adapter error behavior cannot use the exact mapping in section 6 without new
  `TransportError` variants in `crates/core/src/error.rs`;
- Arrow IPC or Parquet cannot be implemented without exposing transponding as
  public API;
- generated adapter output requires manual edits;
- the codegen surface needs a new proto option.

## 4. Implementation Slices

Implement in this order. Do not proceed to the next slice until the narrow tests
for the current slice pass.

### Slice A: Runtime and Adapter Writers

Create the shared dependency-free output helper surface in `crates/core`.

Create the public trait and dispatch helper surface in `crates/metamorphose`.

Port row-format writer helpers into adapter crates:

- JSON writer in `crates/adapters/json`;
- protobuf wire writer in `crates/adapters/protobuf`;
- CSV writer in `crates/adapters/csv`.

Port columnar support into:

- shared column buffer contracts in `crates/transponding`;
- Arrow RecordBatch construction helpers in `crates/adapters/arrow`;
- Arrow IPC byte writer in `crates/adapters/arrow_ipc`;
- Parquet byte writer in `crates/adapters/parquet`.

### Slice B: Codegen Surface

Extend codegen with:

```text
Surface::Metamorphose
Adapter::Json
Adapter::Protobuf
Adapter::Csv
Adapter::Transponding
Adapter::Arrow
Adapter::ArrowIpc
Adapter::Parquet
```

CLI contract:

```text
--surface metamorphose --adapter <adapter>
```

`--adapter` is required only for `--surface metamorphose` and forbidden for
`core` and `projection`.

### Slice C: Generated Schema Adapter Modules

Generate schema-local modules for Bars and test compatibility. The modules
must implement adapter traits for generated schema marker types and may add
inherent convenience methods on those markers.

Generated modules must use private chunks when a generated helper would exceed
32 field write/access statements.

### Slice D: Schema Feature Gates

Add optional adapter dependencies and feature-gated modules to the two schema
crates only.

Default schema builds remain core-only:

```bash
cargo check -p metamorphic_binary_transport_schema_bars --no-default-features
cargo check -p metamorphic_binary_transport_schema_test_compatibility --no-default-features
```

### Slice E: Tests, Benchmarks, and Evidence

Add correctness tests, source guard tests, compile-surface checks, and the
metamorphose benchmark runner.

No performance claim is allowed until the benchmark evidence files exist.

## 5. Dependency Changes

Exact dependency additions:

| File | Dependency |
| --- | --- |
| `crates/metamorphose/Cargo.toml` | `metamorphic_binary_transport_core = { path = "../core" }` |
| `crates/transponding/Cargo.toml` | `metamorphic_binary_transport_core = { path = "../core" }` |
| `crates/adapters/json/Cargo.toml` | `metamorphic_binary_transport_core = { path = "../../core" }` |
| `crates/adapters/protobuf/Cargo.toml` | `metamorphic_binary_transport_core = { path = "../../core" }` |
| `crates/adapters/protobuf/Cargo.toml` | `prost = "=0.14.4"` |
| `crates/adapters/csv/Cargo.toml` | `metamorphic_binary_transport_core = { path = "../../core" }` |
| `crates/adapters/csv/Cargo.toml` | `itoa = "=1.0.18"` |
| `crates/adapters/arrow/Cargo.toml` | `metamorphic_binary_transport_core = { path = "../../core" }` |
| `crates/adapters/arrow/Cargo.toml` | `metamorphic_binary_transport_transponding = { path = "../../transponding" }` |
| `crates/adapters/arrow/Cargo.toml` | `arrow-array = "=56.2.1"` |
| `crates/adapters/arrow/Cargo.toml` | `arrow-buffer = "=56.2.1"` |
| `crates/adapters/arrow/Cargo.toml` | `arrow-schema = "=56.2.1"` |
| `crates/adapters/arrow_ipc/Cargo.toml` | `metamorphic_binary_transport_core = { path = "../../core" }` |
| `crates/adapters/arrow_ipc/Cargo.toml` | `arrow-array = "=56.2.1"` |
| `crates/adapters/arrow_ipc/Cargo.toml` | `arrow-ipc = "=56.2.1"` |
| `crates/adapters/arrow_ipc/Cargo.toml` | `arrow-schema = "=56.2.1"` |
| `crates/adapters/parquet/Cargo.toml` | `metamorphic_binary_transport_core = { path = "../../core" }` |
| `crates/adapters/parquet/Cargo.toml` | `arrow-array = "=56.2.1"` |
| `crates/adapters/parquet/Cargo.toml` | `arrow-schema = "=56.2.1"` |
| `crates/adapters/parquet/Cargo.toml` | `parquet = "=56.2.1"` |
| `crates/benches/Cargo.toml` | `metamorphic_binary_transport_metamorphose = { path = "../metamorphose" }` |

`crates/core` receives no new external dependency for `crates/core/src/output.rs`.

Adapter crates must not depend on
`metamorphic_binary_transport_metamorphose`. They import checked output helpers
from `metamorphic_binary_transport_core::output` and expose only their selected
format writer API.

Arrow IPC and Parquet must not depend on
`metamorphic_binary_transport_adapter_arrow`. Their selected schema features
pull only their selected adapter crate plus `transponding`.

The RecordBatch bridge is generated schema-local per selected columnar adapter:

- `*_arrow.rs` builds a RecordBatch and returns it through the Arrow adapter
  trait;
- `*_arrow_ipc.rs` builds a RecordBatch and passes it directly to the Arrow IPC
  writer;
- `*_parquet.rs` builds a RecordBatch and passes it directly to the Parquet
  writer.

This duplicates only adapter-specific RecordBatch bridge statements. It must
not duplicate transponding code, must call exactly one generated hidden
transponding helper, and must not materialize row DTOs.

If `prost = "=0.14.4"` does not expose the required low-level encoding API,
stop and amend the plan. Do not downgrade or broaden the version without audit.

## 6. Failure Mapping

No edit to `crates/core/src/error.rs` is authorized by this plan.

All adapter failures must map to existing `TransportError` variants:

| Failure | Mapping |
| --- | --- |
| wrong schema | existing generated checked access error |
| corrupt header | existing generated checked access error |
| corrupt archive | existing generated checked access error |
| old or unsupported schema version | existing generated checked access error |
| response cap overflow | `TransportError::ResponseTooLarge { observed, cap }` |
| checked non-finite numeric field | `TransportError::NonFiniteNumeric(field)` |
| invalid time conversion for UTC text | `TransportError::InvalidTimeGrid(message)` |
| unsupported field kind for selected adapter | `CodegenError::InvalidSchema(message)` before generated runtime exists |
| selected adapter feature missing | Rust compile-time missing trait/module error |
| Arrow RecordBatch construction failure | `TransportError::MalformedArchive(message)` |
| Arrow IPC writer failure | `TransportError::MalformedArchive(message)` |
| Parquet writer failure | `TransportError::MalformedArchive(message)` |

Generated MBT raw strings are Rust strings after archive validation. Adapter
writers must not perform fallible UTF-8 conversion in the row loop. Bytes fields
are emitted as base64 for JSON and CSV and as bytes for protobuf. If an adapter
API later requires fallible UTF-8 conversion, stop and amend the spec before
code.

Adapter error messages are allowed to allocate on error paths only. They are
not part of the measured hot path.

If any dependency API cannot be mapped through this table without a new
`TransportError` variant, that adapter is blocked for this implementation plan.

## 7. Public Trait and Helper API

File:

```text
crates/metamorphose/src/lib.rs
```

Replace the placeholder with these public traits and helpers.

Trait names:

```rust
pub trait MbtMetamorphoseSchema
pub trait JsonMetamorphoseSchema
pub trait ProtobufMetamorphoseSchema
pub trait CsvMetamorphoseSchema
pub trait ArrowMetamorphoseSchema
pub trait ArrowIpcMetamorphoseSchema
pub trait ParquetMetamorphoseSchema
```

Required checked helpers:

```rust
pub fn mbt<S: MbtMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<&[u8]>;
pub fn json<S: JsonMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub fn protobuf<S: ProtobufMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub fn csv<S: CsvMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub fn arrow<S: ArrowMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<S::RecordBatch>;
pub fn arrow_ipc<S: ArrowIpcMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub fn parquet<S: ParquetMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
```

Required trusted helpers:

```rust
pub unsafe fn mbt_trusted_unchecked<S: MbtMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<&[u8]>;
pub unsafe fn json_trusted_unchecked<S: JsonMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub unsafe fn protobuf_trusted_unchecked<S: ProtobufMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub unsafe fn csv_trusted_unchecked<S: CsvMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub unsafe fn arrow_trusted_unchecked<S: ArrowMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<S::RecordBatch>;
pub unsafe fn arrow_ipc_trusted_unchecked<S: ArrowIpcMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub unsafe fn parquet_trusted_unchecked<S: ParquetMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
```

Safety documentation must state:

```text
The caller guarantees that bytes were previously accepted by checked access for
the same schema and then stored or transported without mutation.
```

`ArrowMetamorphoseSchema` uses an associated type to keep `crates/metamorphose`
free of Arrow dependencies:

```rust
type RecordBatch;
```

`MetamorphoseFormat` and `MetamorphoseOutput<'a>` are row-format only:

```rust
pub enum MetamorphoseFormat { Mbt, Json, Protobuf, Csv }
pub enum MetamorphoseOutput<'a> { Mbt(&'a [u8]), Json(Vec<u8>), Protobuf(Vec<u8>), Csv(Vec<u8>) }
```

The row-format `decode` helper may require:

```rust
S: MbtMetamorphoseSchema + JsonMetamorphoseSchema + ProtobufMetamorphoseSchema + CsvMetamorphoseSchema
```

Do not include Arrow in `MetamorphoseOutput`, because that would force Arrow
dependencies into `crates/metamorphose`.

## 8. Core Output Helper API

File:

```text
crates/core/src/output.rs
```

Implement:

```rust
pub struct CheckedBytes
```

Required methods:

```rust
pub fn new(cap: usize) -> Self;
pub fn with_capacity(cap: usize, requested_capacity: usize) -> Self;
pub fn push(&mut self, byte: u8) -> Result<()>;
pub fn push_str(&mut self, value: &str) -> Result<()>;
pub fn extend_from_slice(&mut self, value: &[u8]) -> Result<()>;
pub fn encode_protobuf<F>(&mut self, encode: F) -> Result<()> where F: FnOnce(&mut Vec<u8>);
pub fn finish(self) -> Vec<u8>;
pub fn fmt_error(&self) -> TransportError;
```

Additional helpers:

```rust
pub fn checked_len_add(lhs: usize, rhs: usize, cap: usize) -> Result<usize>;
pub fn encoded_len_string(tag: u32, value: &str) -> usize;
pub fn encoded_len_message(tag: u32, len: usize) -> usize;
pub fn utc_len(ms: i64) -> Result<usize>;
pub fn utc_bytes(ms: i64, out: &mut [u8; 64]) -> Result<&[u8]>;
pub fn write_utc(out: &mut CheckedBytes, ms: i64) -> Result<()>;
pub fn write_base64(out: &mut CheckedBytes, value: &[u8]) -> Result<()>;
```

`CheckedBytes` is adapter support API, not a stable user-facing API. It is
public only because adapter crates are separate crates.

Export the module from:

```text
crates/core/src/lib.rs
```

The implementation must not add external dependencies to `crates/core`.

`crates/metamorphose/src/runtime.rs` may remain a small internal module or be
empty if no dispatch-local helper is required. It must not contain duplicated
checked byte writer logic.

`crates/metamorphose/src/lib.rs` must not keep `#![forbid(unsafe_code)]`
because this plan requires public unsafe trusted functions. Use:

```rust
#![deny(unsafe_op_in_unsafe_fn)]
```

Do not introduce unsafe blocks in `crates/metamorphose`.

## 9. Adapter Writer APIs

### JSON

Files:

```text
crates/adapters/json/Cargo.toml
crates/adapters/json/src/lib.rs
crates/adapters/json/src/tests/mod.rs
```

Writer:

```rust
pub struct JsonWriter
```

Constructor:

```rust
pub fn with_capacity(max_response_bytes: usize, requested_capacity: usize) -> Self;
```

Field fragments:

```rust
pub fn raw_static(&mut self, value: &'static [u8]) -> Result<()>;
```

Scalar and array writer methods:

```rust
begin_object, end_object, begin_array, end_array, comma,
string_value, u32_value, i32_value, i64_value, f32_value, f64_value,
bool_value, bytes_value, utc_value,
i64_array_value, i32_array_value, u32_array_value, f64_array_value,
f32_array_value
```

### Protobuf

Files:

```text
crates/adapters/protobuf/Cargo.toml
crates/adapters/protobuf/src/lib.rs
crates/adapters/protobuf/src/tests/mod.rs
```

Writer:

```rust
pub struct ProtoWriter
```

Constructor:

```rust
pub fn with_capacity(max_response_bytes: usize, requested_capacity: usize) -> Self;
```

Methods:

```rust
uint32, int32, int64, float, double, bool, string, bytes, utc, message_prefix
```

No prost DTO or `prost::Message` usage is allowed.

### CSV

Files:

```text
crates/adapters/csv/Cargo.toml
crates/adapters/csv/src/lib.rs
crates/adapters/csv/src/tests/mod.rs
```

Writer:

```rust
pub struct CsvWriter
```

Constructor:

```rust
pub fn with_capacity(max_response_bytes: usize, requested_capacity: usize) -> Self;
```

Header fragments:

```rust
pub fn raw_static(&mut self, value: &'static [u8]) -> Result<()>;
```

Methods:

```rust
comma, newline,
string_cell, u32_cell, i32_cell, i64_cell, f32_cell, f64_cell,
bool_cell, bytes_cell, utc_cell,
i64_array_cell, i32_array_cell, u32_array_cell, f64_array_cell,
f32_array_cell
```

Repeated values remain JSON-style payloads inside a quoted CSV cell.

## 10. Transponding and Columnar APIs

Files:

```text
crates/transponding/Cargo.toml
crates/transponding/src/lib.rs
crates/transponding/src/runtime.rs
crates/transponding/src/tests/mod.rs
crates/adapters/arrow/Cargo.toml
crates/adapters/arrow/src/lib.rs
crates/adapters/arrow/src/tests/mod.rs
crates/adapters/arrow_ipc/Cargo.toml
crates/adapters/arrow_ipc/src/lib.rs
crates/adapters/arrow_ipc/src/tests/mod.rs
crates/adapters/parquet/Cargo.toml
crates/adapters/parquet/src/lib.rs
crates/adapters/parquet/src/tests/mod.rs
```

`crates/transponding` implements shared column buffer types:

```text
ValidityBitmap
ConstU16Column
U16Column, I32Column, U32Column, U64Column, I64Column, F32Column, F64Column
OptionalU16Column, OptionalI32Column, OptionalU32Column, OptionalU64Column,
OptionalI64Column, OptionalF32Column, OptionalF64Column
BoolColumn
Utf8Column
BinaryColumn
I64ListColumn, I32ListColumn, U32ListColumn, F64ListColumn, F32ListColumn
```

It also implements checksum and byte-size helpers:

```text
checksum_seed
update_bool, update_bytes, update_f32, update_f64, update_i32, update_i64
update_str, update_u16, update_u32, update_u64, update_usize
```

`crates/adapters/arrow` owns Arrow RecordBatch support for the Arrow adapter
feature and may re-export these Arrow types for generated schema-local
RecordBatch construction:

```rust
pub use arrow_array::{
    ArrayRef, BinaryArray, BooleanArray, Float32Array, Float64Array, Int32Array,
    Int64Array, RecordBatch, StringArray, UInt16Array, UInt32Array, UInt64Array,
};
pub use arrow_array::builder::{
    BinaryBuilder, BooleanBuilder, Float32Builder, Float64Builder, Int32Builder,
    Int64Builder, ListBuilder, StringBuilder, UInt16Builder, UInt32Builder,
    UInt64Builder,
};
pub use arrow_schema::{DataType, Field, Schema};
```

Helper API:

```rust
pub fn record_batch_byte_size(batch: &RecordBatch) -> usize;
pub fn ensure_record_batch_within_cap(batch: &RecordBatch, max_response_bytes: usize) -> Result<()>;
```

`crates/adapters/arrow_ipc` writes an Arrow IPC stream from a `RecordBatch`.
It must not depend on `crates/adapters/arrow`. Its allowed Arrow re-exports are
exactly the same list as `crates/adapters/arrow`, because generated
`*_arrow_ipc.rs` modules build the writer input locally before calling the IPC
writer.

Writer API:

```rust
pub use arrow_array::RecordBatch;
pub fn write_ipc_stream(batch: &RecordBatch, max_response_bytes: usize) -> Result<Vec<u8>>;
```

`crates/adapters/parquet` writes uncompressed Parquet bytes from a
`RecordBatch`. It must not depend on `crates/adapters/arrow`. It may re-export
exactly the same Arrow type list as `crates/adapters/arrow`, because generated
`*_parquet.rs` modules build the writer input locally before calling the
Parquet writer.

Writer API:

```rust
pub use arrow_array::RecordBatch;
pub fn write_uncompressed_parquet(batch: &RecordBatch, max_response_bytes: usize) -> Result<Vec<u8>>;
```

Compression remains out of scope.

## 11. Codegen Edits

Files:

```text
crates/codegen/src/config.rs
crates/codegen/src/emit.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/model.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
crates/codegen/src/tests/test_rust_emit_transponding.rs
```

`model.rs` may add helper methods only if they derive from existing
`SchemaModel` and `PhysicalField` values. Do not add schema-specific branches.

`rust_emit.rs` adds these functions:

```rust
pub fn generated_metamorphose_adapter_schema(model: &SchemaModel, adapter: Adapter) -> Result<String>;
```

Private emitter groups:

```text
emit_metamorphose_json
emit_metamorphose_protobuf
emit_metamorphose_csv
emit_metamorphose_transponding
emit_metamorphose_arrow
emit_metamorphose_arrow_ipc
emit_metamorphose_parquet
```

Chunking constant:

```rust
const GENERATED_FIELD_CHUNK_SIZE: usize = 32;
```

Private chunk helper naming:

```text
write_json_fields_000_031
write_json_fields_032_063
write_csv_fields_000_031
write_protobuf_fields_000_031
transpond_fields_000_031
```

If a range has fewer than 32 fields, the suffix still records inclusive start
and end indexes.

## 12. Generated Module APIs

Generated row adapter modules define:

```rust
impl JsonMetamorphoseSchema for BarsV1
impl ProtobufMetamorphoseSchema for BarsV1
impl CsvMetamorphoseSchema for BarsV1
```

and the equivalent impls for generated projection marker types and test
compatibility markers.

Generated row adapter modules also add inherent methods:

```rust
pub fn metamorphose_json(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub unsafe fn metamorphose_json_trusted_unchecked(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub fn metamorphose_protobuf(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub unsafe fn metamorphose_protobuf_trusted_unchecked(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub fn metamorphose_csv(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
pub unsafe fn metamorphose_csv_trusted_unchecked(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;
```

Generated columnar modules define:

```rust
pub(crate) struct BarsV1ColumnBatch
pub(crate) fn transpond_archived(...)
pub(crate) fn transpond_fields_000_031(...)
```

and:

```rust
impl ArrowMetamorphoseSchema for BarsV1
impl ArrowIpcMetamorphoseSchema for BarsV1
impl ParquetMetamorphoseSchema for BarsV1
```

plus trusted variants through the trait methods and public helpers.

No generated function named `pub fn transpond` is allowed.

Generated `*_arrow_ipc.rs` and `*_parquet.rs` modules must not import
`metamorphic_binary_transport_adapter_arrow`. They build their adapter-local
RecordBatch input from the hidden schema-local transponded columns and pass it
to their selected adapter writer.

## 13. Schema Feature Gates

Files:

```text
crates/schemas/bars_core/Cargo.toml
crates/schemas/bars_core/src/lib.rs
crates/schemas/test_compatibility_core/Cargo.toml
crates/schemas/test_compatibility_core/src/lib.rs
```

Add exact features to both schema crates:

```toml
[features]
default = []
json = [
    "dep:metamorphic_binary_transport_metamorphose",
    "dep:metamorphic_binary_transport_adapter_json",
]
protobuf = [
    "dep:metamorphic_binary_transport_metamorphose",
    "dep:metamorphic_binary_transport_adapter_protobuf",
]
csv = [
    "dep:metamorphic_binary_transport_metamorphose",
    "dep:metamorphic_binary_transport_adapter_csv",
]
arrow = [
    "dep:metamorphic_binary_transport_metamorphose",
    "dep:metamorphic_binary_transport_transponding",
    "dep:metamorphic_binary_transport_adapter_arrow",
]
arrow_ipc = [
    "dep:metamorphic_binary_transport_metamorphose",
    "dep:metamorphic_binary_transport_transponding",
    "dep:metamorphic_binary_transport_adapter_arrow_ipc",
]
parquet = [
    "dep:metamorphic_binary_transport_metamorphose",
    "dep:metamorphic_binary_transport_transponding",
    "dep:metamorphic_binary_transport_adapter_parquet",
]
```

Add optional dependencies matching those features. Do not add adapter
dependencies outside feature gates.

Add module gates:

```rust
pub mod bars_v1;

#[cfg(feature = "json")]
mod bars_v1_json;
#[cfg(feature = "protobuf")]
mod bars_v1_protobuf;
#[cfg(feature = "csv")]
mod bars_v1_csv;
#[cfg(any(feature = "arrow", feature = "arrow_ipc", feature = "parquet"))]
mod bars_v1_transponding;
#[cfg(feature = "arrow")]
mod bars_v1_arrow;
#[cfg(feature = "arrow_ipc")]
mod bars_v1_arrow_ipc;
#[cfg(feature = "parquet")]
mod bars_v1_parquet;
```

Repeat with `test_compatibility_v1` module names for the compatibility schema.

## 14. Generated File Commands

All generated files are produced by `mbt_codegen`. For every `--write` command
below, the matching reproducibility check command is identical except
`--write` is replaced with `--check`.

Bars:

```bash
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter transponding --out crates/schemas/bars_core/src/bars_v1_transponding.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter arrow --out crates/schemas/bars_core/src/bars_v1_arrow.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter arrow-ipc --out crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter parquet --out crates/schemas/bars_core/src/bars_v1_parquet.rs
```

Test compatibility:

```bash
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter json --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter protobuf --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter csv --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter transponding --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter arrow --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter arrow-ipc --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter parquet --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs
```

## 15. Test Files to Create

Create:

```text
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
crates/codegen/src/tests/test_rust_emit_transponding.rs
crates/schemas/bars_core/tests/test_metamorphose.rs
crates/schemas/bars_core/tests/test_transponding_hidden.rs
crates/schemas/test_compatibility_core/tests/test_metamorphose.rs
crates/schemas/test_compatibility_core/tests/test_transponding_hidden.rs
crates/benches/src/metamorphose.rs
crates/benches/src/bin/mbt_metamorphose_bench.rs
crates/benches/src/tests/test_metamorphose_bench_output.rs
```

Update:

```text
crates/codegen/src/tests/mod.rs
crates/benches/src/lib.rs
```

## 16. Source Guard Tests

Add source-level tests that fail if generated adapter code contains:

```text
Vec::with_capacity(archived.rows.len())
.to_string()
.to_vec()
.collect()
serde_json
prost::Message
format!(
pub fn transpond
pub trait Transpond
```

Guard tests must also verify:

- JSON field names use generated `&'static [u8]` fragments;
- CSV headers use generated `&'static [u8]` fragments;
- protobuf tags are generated constants;
- trusted methods call `access_archived_trusted_unchecked`, not checked access;
- each generated chunk helper has at most 32 field write/access statements;
- chunk helpers are private;
- Arrow IPC and Parquet modules call one generated transponding helper.
- Arrow IPC and Parquet generated modules do not import
  `metamorphic_binary_transport_adapter_arrow`.

## 17. Correctness Tests

Schema tests must cover:

- checked JSON/protobuf/CSV output for Bars and test compatibility;
- trusted JSON/protobuf/CSV output equals checked output;
- checked Arrow semantic checksum;
- trusted Arrow semantic checksum equals checked;
- Arrow IPC readback row count, column count, and semantic checksum;
- Parquet readback row count, column count, and semantic checksum;
- projected MBT can be metamorphosed by using the projected marker type;
- response cap errors for JSON, protobuf, CSV, Arrow IPC, and Parquet;
- wrong schema rejection before adapter writing.

Test compatibility rows must include:

- required and optional numeric scalars;
- bool;
- raw strings;
- bytes;
- repeated numeric arrays;
- nullable arrays;
- absent optional values;
- present empty nullable arrays.

## 18. Benchmark Implementation

Files:

```text
crates/benches/src/metamorphose.rs
crates/benches/src/bin/mbt_metamorphose_bench.rs
crates/benches/src/tests/test_metamorphose_bench_output.rs
```

Benchmark command:

```bash
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_metamorphose_bench -- --report-dir docs/evidence/mbt_metamorphose_migration
```

Required labels:

```text
metamorphose_json
metamorphose_protobuf
metamorphose_csv_full_public
metamorphose_csv_full_archived
transponding_full_public
transponding_full_archived
metamorphose_arrow_full_public
metamorphose_arrow_full_archived
metamorphose_arrow_ipc_full_public
metamorphose_arrow_ipc_full_archived
metamorphose_parquet_full_public
metamorphose_parquet_full_archived
```

The benchmark must:

- parse old labels from
  `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md`;
- fail before timing if any required old label is absent;
- time checked and trusted paths separately;
- measure setup outside the timed loop;
- emit one JSON run file per execution;
- refuse to overwrite run files;
- write summary only when three run files exist.

## 19. Evidence Files

Create after validation:

```text
docs/evidence/mbt_metamorphose_migration/benchmark_environment.md
docs/evidence/mbt_metamorphose_migration/dependency_trees.md
docs/evidence/mbt_metamorphose_migration/compile_surface.md
docs/evidence/mbt_metamorphose_migration/metamorphose_run_1.json
docs/evidence/mbt_metamorphose_migration/metamorphose_run_2.json
docs/evidence/mbt_metamorphose_migration/metamorphose_run_3.json
docs/evidence/mbt_metamorphose_migration/metamorphose_summary.md
```

Do not claim speed parity until all three run files exist and spread is within
the spec limit.

## 20. Validation Commands

Run narrow checks first:

```bash
cargo test -p metamorphic_binary_transport_core --all-targets
cargo test -p metamorphic_binary_transport_metamorphose --all-targets
cargo test -p metamorphic_binary_transport_adapter_json --all-targets
cargo test -p metamorphic_binary_transport_adapter_protobuf --all-targets
cargo test -p metamorphic_binary_transport_adapter_csv --all-targets
cargo test -p metamorphic_binary_transport_transponding --all-targets
cargo test -p metamorphic_binary_transport_adapter_arrow --all-targets
cargo test -p metamorphic_binary_transport_adapter_arrow_ipc --all-targets
cargo test -p metamorphic_binary_transport_adapter_parquet --all-targets
```

Run codegen tests:

```bash
cargo test -p metamorphic_binary_transport_codegen --all-targets
```

Run generated schema tests:

```bash
cargo test -p metamorphic_binary_transport_schema_bars --all-targets --features json,protobuf,csv,arrow,arrow_ipc,parquet
cargo test -p metamorphic_binary_transport_schema_test_compatibility --all-targets --features json,protobuf,csv,arrow,arrow_ipc,parquet
```

Run bench support tests:

```bash
cargo test -p metamorphic_binary_transport_benches --all-targets
```

Run generated reproducibility checks by replacing every command in section 14
with `--check`.

Run compile-surface checks:

```bash
cargo check -p metamorphic_binary_transport_core
cargo check -p metamorphic_binary_transport_projection
cargo check -p metamorphic_binary_transport_metamorphose
cargo check -p metamorphic_binary_transport_transponding
cargo check -p metamorphic_binary_transport_schema_bars --no-default-features
cargo check -p metamorphic_binary_transport_schema_bars --features json
cargo check -p metamorphic_binary_transport_schema_bars --features protobuf
cargo check -p metamorphic_binary_transport_schema_bars --features csv
cargo check -p metamorphic_binary_transport_schema_bars --features arrow
cargo check -p metamorphic_binary_transport_schema_bars --features arrow_ipc
cargo check -p metamorphic_binary_transport_schema_bars --features parquet
cargo check -p metamorphic_binary_transport_schema_test_compatibility --no-default-features
cargo check -p metamorphic_binary_transport_schema_test_compatibility --features json,csv
```

Run dependency-tree checks:

```bash
cargo tree -p metamorphic_binary_transport_schema_bars --no-default-features
cargo tree -p metamorphic_binary_transport_schema_bars --features json
cargo tree -p metamorphic_binary_transport_schema_bars --features protobuf
cargo tree -p metamorphic_binary_transport_schema_bars --features csv
cargo tree -p metamorphic_binary_transport_schema_bars --features arrow
cargo tree -p metamorphic_binary_transport_schema_bars --features arrow_ipc
cargo tree -p metamorphic_binary_transport_schema_bars --features parquet
```

Run benchmark three times:

```bash
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_metamorphose_bench -- --report-dir docs/evidence/mbt_metamorphose_migration
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_metamorphose_bench -- --report-dir docs/evidence/mbt_metamorphose_migration
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_metamorphose_bench -- --report-dir docs/evidence/mbt_metamorphose_migration
```

## 21. Expected Outputs

Expected source outputs:

- all generated files listed in section 14 exist;
- no generated file is manually edited after write;
- schema crates compile with no adapter deps by default;
- selected schema features compile only selected adapter surfaces;
- generated source guard tests pass.

Expected test outputs:

- all commands in section 20 pass before benchmark;
- generated check commands report no diffs;
- wrong-schema and response-cap tests fail with explicit `TransportError`.

Expected benchmark outputs:

- three JSON run files under `docs/evidence/mbt_metamorphose_migration`;
- `metamorphose_summary.md` with median, spread, rows/sec, MB/sec, output
  bytes, checksum, and feature set;
- no stable speed claim if max/min spread is greater than 7 percent.

## 22. Rollback Boundary

The rollback boundary is exactly the files listed in sections 5, 7, 8, 9, 10,
11, 13, 14, 15, and 18 plus `Cargo.lock`.

No rollback may touch:

```text
crates/core/src/* except crates/core/src/output.rs, crates/core/src/lib.rs, and crates/core/src/tests/test_output.rs
crates/projection/src/*
proto/mathilde/options.proto
```

unless a later audited spec authorizes those paths.

## 23. Known Risks

- Dependency APIs may expose error types that lose detail when mapped to
  `TransportError::MalformedArchive(message)`. This is accepted only on error
  paths and does not authorize core error changes.
- Arrow IPC and Parquet RecordBatch bridge code is generated per selected
  adapter module to preserve adapter dependency isolation. Source guards must
  prove it does not duplicate transponding code or import the Arrow adapter
  crate.
- Benchmark parity is unproved until the three-run evidence exists.
- Compile-surface budget is unproved until the feature-specific `cargo check`
  and `cargo tree` evidence exists.

## 24. Approval State

- spec: peer audited;
- implementation plan: amended after first implementation plan peer audit;
- implementation plan peer audit v2: required before code unless explicitly
  waived by the owner;
- code changes: not authorized by this amended plan.
