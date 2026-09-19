# metamorphic-binary-transport (MBT)

`metamorphic-binary-transport` is the Rust workspace for MBT, the schema-driven
binary transport used to move validated row payloads without hand-written DTOs
on the hot path.

The default artifact is MBT bytes. JSON, protobuf, CSV, Arrow, Arrow IPC, and
Parquet are explicit boundary formats, generated and compiled only when the
schema crate opts into the matching adapter surface.

## Index

- [What This Is](#what-this-is)
- [What This Is Not](#what-this-is-not)
- [Supported Surfaces](#supported-surfaces)
- [Core Conventions](#core-conventions)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Which Surface To Use](#which-surface-to-use)
- [Examples](#examples)
- [Codegen](#codegen)
- [Adapter Features](#adapter-features)
- [Compression](#compression)
- [Trusted Access](#trusted-access)
- [Current Limits](#current-limits)
- [What Not To Infer](#what-not-to-infer)
- [Further Reading](#further-reading)

## What This Is

MBT is a transport format generated from `.proto` schemas plus
`proto/mbt/options.proto`.

It provides:

- schema-specific binary encode and checked access;
- explicit trusted access for bytes already validated for the same schema;
- MBT-to-MBT projections;
- boundary conversion through metamorphose adapters;
- row-to-column transponding for columnar adapters;
- reproducible codegen commands for schema-owned Rust modules.

The workspace is split so a consumer can depend on core transport without
compiling every schema, adapter, or benchmark.

## What This Is Not

This repository is not a feed server, database, cache policy, finality contract,
mandatory compression layer, or general protobuf runtime.

It does not claim:

- row existence means application-level validation has passed;
- all schemas or adapters must compile into every user binary;
- compression is part of the MBT wire format;
- generated files may be edited by hand;
- benchmark speed without recorded run evidence.

## Supported Surfaces

### Core

**What it is:**
Envelope, checksum, response caps, checked validation, trusted payload access,
and schema runtime traits.

**Use it when:**
You need to encode MBT bytes, inspect MBT bytes, or access validated MBT rows.

### Codegen

**What it is:**
The generator that reads `.proto` schemas and writes Rust schema modules.

**Use it when:**
You add or change a schema, projection, or adapter surface.

### Schema Crates

**What it is:**
Generated schema-specific Rust code. Current in-repo examples are:

- `mbt_schema_telemetry`
- `mbt_schema_measurement`
- `mbt_schema_test_compatibility`

**Use it when:**
Your application needs concrete row types, schema markers, projections, and
selected adapter entrypoints.

### Projections

**What it is:**
Generated MBT-to-MBT conversion from a full schema to a projected schema.

**Use it when:**
You want a smaller logical MBT payload before any JSON, protobuf, CSV, Arrow,
Arrow IPC, or Parquet conversion.

### Metamorphose

**What it is:**
Boundary conversion from MBT bytes to an explicit output format.

**Use it when:**
The consumer asks for JSON, protobuf, CSV, Arrow, Arrow IPC, or Parquet at the
serving boundary.

### Transponding

**What it is:**
The row-to-column mechanism used internally by columnar metamorphose adapters.

**Use it when:**
You are working on Arrow, Arrow IPC, or Parquet adapter internals. Normal users
call metamorphose.

### Compression

**What it is:**
An opt-in zstd wrapper for completed MBT bytes or projected MBT bytes.

**Use it when:**
You need to ship or store MBT payloads with fewer bytes while keeping MBT itself
unchanged.

## Core Conventions

- MBT bytes are the canonical internal transport artifact.
- `.proto` plus MBT options are the schema source of truth.
- Generated Rust is codegen-owned and not hand edited.
- Projections happen as MBT-to-MBT before boundary conversion.
- Checked access validates bytes before archive access.
- Trusted access is explicit and only valid after a prior checked validation
  boundary for the same schema.
- Adapter compilation is opt-in per schema crate feature.
- Compression is outside the MBT wire contract and wraps completed MBT bytes.

## Installation

This repository is not documented here as a crates.io package. Use path or git
dependencies from your workspace.

Core-only dependency:

```toml
[dependencies]
mbt_core = { path = "../metamorphic-binary-transport/crates/core" }
```

Generated measurement schema without adapters:

```toml
[dependencies]
mbt_schema_measurement = { path = "../metamorphic-binary-transport/crates/schemas/measurement_core" }
```

Generated measurement schema with selected boundary adapters:

```toml
[dependencies]
mbt_schema_measurement = { path = "../metamorphic-binary-transport/crates/schemas/measurement_core", features = ["json", "protobuf", "csv"] }
mbt_metamorphose = { path = "../metamorphic-binary-transport/crates/metamorphose" }
```

Optional compression dependency:

```toml
[dependencies]
mbt_compression = { path = "../metamorphic-binary-transport/crates/compression" }
```

## Quick Start

Encode and read MBT bytes with a generated schema:

```rust
use mbt_schema_measurement::measurement_v1::{
    MeasurementV1, MeasurementRowV1,
};

fn encode_and_read(rows: Vec<MeasurementRowV1>) -> mbt_core::Result<()> {
    let max_response_bytes = 64 * 1024 * 1024;

    let bytes = MeasurementV1::encode_owned(rows, max_response_bytes)?;
    let view = MeasurementV1::access(&bytes)?;
    let inspection = MeasurementV1::inspect(&bytes)?;

    assert_eq!(view.len(), inspection.row_count);
    Ok(())
}
```

Use generic core helpers when the schema marker is already known:

```rust
use mbt_core::{access, encode_owned, inspect};
use mbt_schema_measurement::measurement_v1::{MeasurementV1, MeasurementRowV1};

fn via_core_helpers(rows: Vec<MeasurementRowV1>) -> mbt_core::Result<()> {
    let max_response_bytes = 64 * 1024 * 1024;

    let bytes = encode_owned::<MeasurementV1>(rows, max_response_bytes)?;
    let _view = access::<MeasurementV1>(&bytes)?;
    let _inspection = inspect::<MeasurementV1>(&bytes)?;

    Ok(())
}
```

## Which Surface To Use

Use core only when the application stores, moves, or serves MBT bytes directly.

Use a generated schema crate when the application needs concrete row types,
projection entrypoints, or schema-specific accessors.

Use projection when the requested logical payload is smaller than the full MBT
schema and should remain MBT.

Use metamorphose when the caller explicitly asks for a boundary format.

Use transponding only when implementing or auditing columnar adapter internals.

Use compression only after MBT bytes have already been produced. Compressing a
projection is valid because a projection is also completed MBT bytes.

Use benches only for measurement. Bench code is not part of the runtime surface.

## Examples

### Inspect MBT Bytes

```rust
use mbt_schema_measurement::measurement_v1::MeasurementV1;

fn inspect_bytes(bytes: &[u8]) -> mbt_core::Result<usize> {
    let inspection = MeasurementV1::inspect(bytes)?;
    Ok(inspection.row_count)
}
```

### Project MBT To MBT

```rust
use mbt_schema_measurement::measurement_v1::MeasurementV1;

fn project_values(bytes: &[u8]) -> mbt_core::Result<Vec<u8>> {
    let max_response_bytes = 64 * 1024 * 1024;
    MeasurementV1::project_values_only(bytes, max_response_bytes)
}
```

### Convert MBT To JSON

Requires the schema crate `json` feature.

```rust
use mbt_metamorphose as metamorphose;
use mbt_schema_measurement::measurement_v1::MeasurementV1;

fn as_json(bytes: &[u8]) -> mbt_core::Result<Vec<u8>> {
    let max_response_bytes = 64 * 1024 * 1024;
    metamorphose::json::<MeasurementV1>(bytes, max_response_bytes)
}
```

### Convert MBT To Protobuf

Requires the schema crate `protobuf` feature.

```rust
use mbt_metamorphose as metamorphose;
use mbt_schema_measurement::measurement_v1::MeasurementV1;

fn as_protobuf(bytes: &[u8]) -> mbt_core::Result<Vec<u8>> {
    let max_response_bytes = 64 * 1024 * 1024;
    metamorphose::protobuf::<MeasurementV1>(bytes, max_response_bytes)
}
```

### Convert MBT To Arrow IPC

Requires the schema crate `arrow_ipc` feature.

```rust
use mbt_metamorphose as metamorphose;
use mbt_schema_measurement::measurement_v1::MeasurementV1;

fn as_arrow_ipc(bytes: &[u8]) -> mbt_core::Result<Vec<u8>> {
    let max_response_bytes = 64 * 1024 * 1024;
    metamorphose::arrow_ipc::<MeasurementV1>(bytes, max_response_bytes)
}
```

### Trusted Boundary Conversion

Use trusted entrypoints only after the bytes were previously accepted by checked
access for the same schema and then stored or transported without mutation.

```rust
use mbt_metamorphose as metamorphose;
use mbt_schema_measurement::measurement_v1::MeasurementV1;

fn trusted_json(bytes: &[u8]) -> mbt_core::Result<Vec<u8>> {
    let max_response_bytes = 64 * 1024 * 1024;

    MeasurementV1::access(bytes)?;
    // SAFETY: checked access above validated these same immutable bytes.
    unsafe { metamorphose::json_trusted_unchecked::<MeasurementV1>(bytes, max_response_bytes) }
}
```

## Compression

Compression is opt-in and works on completed MBT bytes. It does not change the
MBT header, schema hash, projection contract, or trusted-access contract.

```rust
use mbt_compression::{compress, decompress};

fn compress_for_pipeline(
    bytes: &[u8],
    max_compressed_bytes: usize,
) -> mbt_core::Result<Vec<u8>> {
    compress(bytes, max_compressed_bytes)
}

fn decompress_from_pipeline(
    bytes: &[u8],
    max_decompressed_bytes: usize,
) -> mbt_core::Result<Vec<u8>> {
    decompress(bytes, max_decompressed_bytes)
}
```

The default codec is zstd level 3. Machine-bound compression results and byte-equality
checks are recorded in [benchmark evidence](docs/README.md).

## Codegen

Codegen takes explicit inputs. It does not infer schemas from Rust modules.
The generator and workspace generation driver are Rust binaries. Generation
requires `protoc` and `rustfmt`; CI uses Rust 1.90.0.

Generate, check, or inspect all 24 committed modules across the three example schemas:

```sh
cargo run -p mbt_codegen --bin schema_codegen -- --write
cargo run -p mbt_codegen --bin schema_codegen -- --check
cargo run -p mbt_codegen --bin schema_codegen -- --inspect
```

Validate the workspace:

```sh
cargo test --workspace --all-features
cargo check --all-targets --all-features
cargo build --workspace --all-features
```

Core generation example:

```bash
cargo run -p mbt_codegen --bin mbt_codegen -- \
  --write \
  --proto-root crates/schemas/measurement_core/proto \
  --proto-root proto \
  --schema mbt/example/measurement/v1/measurement.proto \
  --root mbt.example.measurement.v1.MeasurementResponseV1 \
  --module measurement_v1 \
  --surface core \
  --out /tmp/measurement_v1_core.rs
```

The committed main module includes projections; use the projection surface to regenerate it:

```bash
cargo run -p mbt_codegen --bin mbt_codegen -- \
  --write \
  --proto-root crates/schemas/measurement_core/proto \
  --proto-root proto \
  --schema mbt/example/measurement/v1/measurement.proto \
  --root mbt.example.measurement.v1.MeasurementResponseV1 \
  --module measurement_v1 \
  --surface projection \
  --out crates/schemas/measurement_core/src/measurement_v1.rs
```

Adapter generation writes one adapter module per selected adapter:

```bash
cargo run -p mbt_codegen --bin mbt_codegen -- \
  --write \
  --proto-root crates/schemas/measurement_core/proto \
  --proto-root proto \
  --schema mbt/example/measurement/v1/measurement.proto \
  --root mbt.example.measurement.v1.MeasurementResponseV1 \
  --module measurement_v1 \
  --surface metamorphose \
  --adapter json \
  --out crates/schemas/measurement_core/src/measurement_v1_json.rs
```

Use `--check` with the same arguments to verify committed generated files.

The generated file location is whatever `--out` specifies. A schema crate may
use `src/measurement_v1.rs`, `src/generated/measurement_v1.rs`, or another approved module
path. The codegen contract is the `--out` path, not a fixed folder name.

## Adapter Features

The in-repo measurement schema crate uses opt-in adapter features:

```toml
mbt_schema_measurement = { path = "../metamorphic-binary-transport/crates/schemas/measurement_core", features = ["json", "protobuf", "csv", "arrow_ipc"] }
```

Available feature names are `json`, `protobuf`, `csv`, `arrow`, `arrow_ipc`,
and `parquet`. Each feature enables only the required adapter dependency and
generated module. Core schema use does not compile adapter crates.

## Trusted Access

Checked access:

```text
MBT bytes -> header validation -> archive validation -> view
```

Trusted access:

```text
MBT bytes already validated for this schema -> trusted payload access -> view
```

Trusted access is unsafe because correctness depends on the caller preserving
the validation boundary. It is intended for immutable cache/storage boundaries
where bytes were validated before commit and never mutated in place afterward.

## Current Limits

- The repository is still a workspace implementation, not a published package
  contract.
- Adapter parity must be read from recorded benchmark evidence, not inferred
  from the existence of adapter code.
- Schema-specific generated code is expected. MBT avoids hand-written DTOs, not
  schema-specific codegen.
- External schemas must import `proto/mbt/options.proto` and run codegen
  explicitly.

## What Not To Infer

Do not infer source-data finality from MBT validation.

Do not infer that a projected payload can be read by the full schema marker.

Do not infer that a trusted entrypoint is safe for arbitrary bytes.

Do not infer that enabling one adapter validates or benchmarks another adapter.

Do not infer performance from architecture. Use the recorded benchmark evidence.

## Further Reading

- [Validation and benchmark evidence](docs/README.md)
- [Measurement schema](crates/schemas/measurement_core/proto/mbt/example/measurement/v1/measurement.proto)
- [Telemetry schema](crates/schemas/telemetry_core/proto/mbt/example/telemetry/v1/telemetry.proto)
- [MBT schema options](proto/mbt/options.proto)
