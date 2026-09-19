# `crates/schemas/telemetry_core` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/schemas/telemetry_core/src/telemetry_v1.rs`: generated Telemetry core schema, envelope access, validation, checksums, views, and MBT-to-MBT projections.
- `crates/schemas/telemetry_core/src/telemetry_v1_arrow.rs`: generated Telemetry Arrow RecordBatch metamorphose adapter module.
- `crates/schemas/telemetry_core/src/telemetry_v1_arrow_ipc.rs`: generated Telemetry Arrow IPC metamorphose adapter module.
- `crates/schemas/telemetry_core/src/telemetry_v1_csv.rs`: generated Telemetry CSV metamorphose adapter module.
- `crates/schemas/telemetry_core/src/telemetry_v1_json.rs`: generated Telemetry JSON metamorphose adapter module.
- `crates/schemas/telemetry_core/src/telemetry_v1_parquet.rs`: generated Telemetry Parquet metamorphose adapter module.
- `crates/schemas/telemetry_core/src/telemetry_v1_protobuf.rs`: generated Telemetry protobuf metamorphose adapter module.
- `crates/schemas/telemetry_core/src/telemetry_v1_transponding.rs`: generated Telemetry row-to-column transponding module for columnar adapters.
- `crates/schemas/telemetry_core/src/lib.rs`: schema crate entrypoint and feature-gated generated adapter module declarations.
- `crates/schemas/telemetry_core/tests/test_telemetry_metamorphose.rs`: Telemetry metamorphose tests for derived UTC row outputs and logical field values.
- `crates/schemas/telemetry_core/tests/test_telemetry_projection.rs`: Telemetry projection tests for checked/trusted parity and projected schema identity.
- `crates/schemas/telemetry_core/tests/test_telemetry_shape.rs`: Telemetry generated-shape tests for schema constants and public type behavior.

- `crates/schemas/telemetry_core/tests/common/mod.rs`: independent test rows and row-format/columnar oracles.
- `crates/schemas/telemetry_core/tests/test_telemetry_roundtrip.rs`: field-for-field roundtrip and replay.
- `crates/schemas/telemetry_core/tests/test_telemetry_failures.rs`: malformed values, corruption and schema identity rejection.
- `crates/schemas/telemetry_core/tests/test_projection_metamorphose.rs`: projected adapter values and parity.
