# Metamorphic Binary Transport

MBT is a schema-driven binary transport workspace for Rust. A protobuf schema
with MBT annotations generates row types, checked access, projections and
optional boundary adapters. The repository includes fictional telemetry, a wide measurement fixture, and an
all-fields compatibility fixture; applications can supply their own schemas.

MBT validates transport structure and schema identity. Application validation,
storage policy and source-data completeness remain the caller's responsibility.

## Workspace

| Surface | Purpose |
| --- | --- |
| `mbt_core` | Envelope, validation, checksums, response caps and runtime traits |
| `mbt_codegen` | Read annotated protobuf schemas and generate Rust modules |
| `mbt_schema_telemetry` | Synthetic device readings and a temperature projection |
| `mbt_schema_measurement` | Wide synthetic measurements, nested metadata and two projections |
| `mbt_schema_test_compatibility` | Scalar, array, dictionary and optional-field fixture |
| `mbt_metamorphose` | Dispatch boundary conversions |
| `mbt_transponding` | Internal row-to-column support |
| `crates/adapters/*` | JSON, protobuf, CSV, Arrow, Arrow IPC and Parquet writers |
| `mbt_compression` | Optional zstd wrapper around completed MBT bytes |
| `mbt_benches` | Synthetic fixtures and measurement commands |

Projection produces another MBT payload with its own schema identity. Boundary
conversion produces a requested external format. Compression wraps completed
bytes; it is outside the MBT envelope.

## Build and validate

CI uses Rust 1.90.0. Generation also requires `rustfmt`, `protoc`, and Python 3.
Install the protobuf compiler through your system package manager.

```sh
rustup toolchain install 1.90.0 --component rustfmt
rustup run 1.90.0 python3 scripts/schema_codegen.py --check
cargo +1.90.0 test --workspace --all-features
cargo +1.90.0 check --all-targets --all-features
cargo +1.90.0 build --all-features
```

The generation check compares all committed schema modules with freshly
generated output. The tests use Python's standard CSV and JSON readers for
independent string-array and row-format validation.

## Use the telemetry example

Use a path dependency from your application workspace:

```toml
[dependencies]
mbt_core = { path = "../metamorphic-binary-transport/crates/core" }
mbt_schema_telemetry = { path = "../metamorphic-binary-transport/crates/schemas/telemetry_core", features = ["json"] }
```

Encode and inspect a fictional reading:

```rust
use mbt_schema_telemetry::telemetry_v1::{
    TelemetryRowV1, TelemetryV1, DEVICE_SENSOR_A, STATUS_ACTIVE,
};

fn example() -> mbt_core::Result<()> {
    let cap = 1024 * 1024;
    let rows = vec![TelemetryRowV1 {
        schema_version: 1,
        device_ordinal: DEVICE_SENSOR_A,
        recorded_at_ms: 1_700_000_000_000,
        temperature_c: 21.5,
        battery_percent: 0.0,
        status_ordinal: STATUS_ACTIVE,
        tags_mask: 0,
        presence_bits: 0,
    }];
    let bytes = TelemetryV1::encode_owned(rows, cap)?;
    let view = TelemetryV1::access(&bytes)?;
    assert_eq!(view.len(), 1);
    let json = TelemetryV1::metamorphose_json(&bytes, cap)?;
    assert!(!json.is_empty());
    Ok(())
}
```

The example has no battery value because its presence bit is unset. A present
zero is distinct from an absent value. The schema source is
[telemetry.proto](crates/schemas/telemetry_core/proto/mbt/example/telemetry/v1/telemetry.proto).

## Projection and adapters

```rust
use mbt_schema_telemetry::telemetry_v1::{TelemetryV1, TelemetryV1TemperatureOnly};

fn project(bytes: &[u8]) -> mbt_core::Result<Vec<u8>> {
    let projected = TelemetryV1::project_temperature_only(bytes, 1024 * 1024)?;
    TelemetryV1TemperatureOnly::access(&projected)?;
    Ok(projected)
}
```

Schema default features are empty. Enable `json`, `protobuf`, `csv`, `arrow`,
`arrow_ipc` or `parquet` as needed. Columnar formats activate their supporting
dependencies. A core-only consumer does not need all adapters.

Use checked entrypoints for unvalidated input. Trusted entrypoints are unsafe
and require prior checked validation for the same schema plus immutable bytes.
A safe wrapper must establish that condition itself:

```rust
use mbt_schema_telemetry::telemetry_v1::TelemetryV1;

fn validated_json(bytes: &[u8]) -> mbt_core::Result<Vec<u8>> {
    TelemetryV1::access(bytes)?;
    // SAFETY: checked access validated these same immutable bytes above.
    unsafe { TelemetryV1::metamorphose_json_trusted_unchecked(bytes, 1024 * 1024) }
}
```

## Schema generation

The source contract is `.proto` plus [MBT options](proto/mbt/options.proto).
The workspace wrapper owns a finite matrix: three schemas and eight modules per
schema. The main module includes projections; each adapter has a separate file.

```sh
python3 scripts/schema_codegen.py --write
python3 scripts/schema_codegen.py --check
python3 scripts/schema_codegen.py --inspect
```

Never edit generated Rust by hand. Custom schemas use the generator CLI with
explicit roots, root message, output module and surface. Example:

```sh
cargo run -p mbt_codegen --bin mbt_codegen -- \
  --check \
  --proto-root crates/schemas/telemetry_core/proto \
  --proto-root proto \
  --schema mbt/example/telemetry/v1/telemetry.proto \
  --root mbt.example.telemetry.v1.TelemetryResponseV1 \
  --module telemetry_v1 \
  --surface projection \
  --out crates/schemas/telemetry_core/src/telemetry_v1.rs
```

## Measurements

These commands use deterministic synthetic inputs. Results describe only the
recorded dataset, build, machine and timing boundaries. They are not general
performance guarantees.

```sh
cargo run --release -p mbt_benches --bin mbt_telemetry_regression_bench -- --report-dir /tmp/mbt-benchmarks/regression
cargo run --release -p mbt_benches --bin mbt_projection_bench -- --report-dir /tmp/mbt-benchmarks/projection
cargo run --release -p mbt_benches --bin mbt_compression_bench -- --smoke --report-dir /tmp/mbt-benchmarks/compression
```

The wide measurement suite preserves the original ten regression lanes, twelve
projection lanes (including compatibility), and three compression lanes. It uses
six input sizes from 1 to 100,000 rows. Run the full suite with:

```sh
cargo run --release -p mbt_benches --bin mbt_measurement_regression_bench -- --report-dir /tmp/mbt-benchmarks/measurement-regression
cargo run --release -p mbt_benches --bin mbt_measurement_projection_bench -- --report-dir /tmp/mbt-benchmarks/measurement-projection
cargo run --release -p mbt_benches --bin mbt_measurement_compression_bench -- --report-dir /tmp/mbt-benchmarks/measurement-compression
```

The regression serde baseline serializes a flat physical-row DTO; its JSON shape
differs from the generated logical JSON. Projection also measures a reconstructed
owned-row reference on the same inputs. External implementation comparisons remain
unavailable unless a matching reference report is supplied. Regression and
projection use one timed sample per lane/input size; these are not distributions.

Every recorded run includes its command, machine, toolchain, source identity and
validation outcome. See [test and benchmark evidence](docs/README.md).

## Project status

This repository is under development and is not declared ready for public release.
Passing local checks and measured benchmark results do not establish release readiness.
