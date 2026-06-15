# MBT Metamorphose Migration Compile Surface Evidence

Status: partial validation complete; benchmark evidence not produced.

Date: 2026-06-15

## Commands Run

All commands below completed successfully with no warnings in observed output.

```bash
cargo test -p metamorphic_binary_transport_core --all-targets
cargo test -p metamorphic_binary_transport_metamorphose --all-targets
cargo test -p metamorphic_binary_transport_transponding --all-targets
cargo test -p metamorphic_binary_transport_adapter_json --all-targets
cargo test -p metamorphic_binary_transport_adapter_protobuf --all-targets
cargo test -p metamorphic_binary_transport_adapter_csv --all-targets
cargo test -p metamorphic_binary_transport_adapter_arrow --all-targets
cargo test -p metamorphic_binary_transport_adapter_arrow_ipc --all-targets
cargo test -p metamorphic_binary_transport_adapter_parquet --all-targets
cargo test -p metamorphic_binary_transport_codegen --all-targets
cargo test -p metamorphic_binary_transport_schema_bars --all-targets --features json,protobuf,csv,arrow,arrow_ipc,parquet
cargo test -p metamorphic_binary_transport_schema_test_compatibility --all-targets --features json,protobuf,csv,arrow,arrow_ipc,parquet
cargo test -p metamorphic_binary_transport_benches --all-targets
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
cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow,arrow_ipc,parquet
cargo check -p metamorphic_binary_transport_schema_test_compatibility --no-default-features
cargo check -p metamorphic_binary_transport_schema_test_compatibility --features json,csv
cargo check -p metamorphic_binary_transport_schema_test_compatibility --features json,protobuf,csv,arrow,arrow_ipc,parquet
```

The generated adapter reproducibility checks also completed successfully for
both `bars_v1` and `test_compatibility_v1` across:

```text
json
protobuf
csv
transponding
arrow
arrow-ipc
parquet
```

## Observed Test Counts

| Surface | Observed tests |
| --- | ---: |
| core | 29 |
| metamorphose | 2 |
| transponding | 5 |
| json adapter | 3 |
| protobuf adapter | 4 |
| csv adapter | 3 |
| arrow adapter | 4 |
| arrow IPC adapter | 3 |
| parquet adapter | 3 |
| codegen | 27 |
| bars schema all-targets all-features | 5 |
| test compatibility schema all-targets all-features | 17 |
| benches support | 4 |

## Boundary Evidence

`cargo check -p metamorphic_binary_transport_schema_bars --features arrow_ipc`
compiled without enabling `metamorphic_binary_transport_adapter_arrow`.

`cargo check -p metamorphic_binary_transport_schema_bars --features parquet`
compiled without enabling `metamorphic_binary_transport_adapter_arrow`.

## Blocker

The benchmark runner required by the approved implementation plan is not
complete. The plan requires `transponding_full_public` and
`transponding_full_archived` labels, but generated transponding is intentionally
`pub(crate)` and hidden from the separate benches crate. Adding a public or
bench-only transponding exposure requires a spec/plan amendment because the
same plan forbids public transponding API.

No performance parity claim is made in this evidence file.
