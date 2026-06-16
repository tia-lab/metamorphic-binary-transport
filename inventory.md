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

# `metamorphic-binary-transport` - Global Inventory (GENERATED; DO NOT EDIT)

Generated: 2026-06-16T13:03:16Z
Protocol: code-only inventory; docs are excluded from source inventory.

This file is generated from per-component inventories under `crates/*/docs/inventory.md` and `services/*/docs/inventory.md`.
Nested crates under `crates/adapters/*` and `crates/schemas/*` are discovered by their `Cargo.toml` files.
Docs, target directories, and vendored dependency directories are excluded from source-file inventory.
If a file purpose is missing in a component inventory, this file will mark it as `INVENTORY GAP`.

## Components

- `crate::adapters/arrow`
- `crate::adapters/arrow_ipc`
- `crate::adapters/csv`
- `crate::adapters/json`
- `crate::adapters/parquet`
- `crate::adapters/protobuf`
- `crate::benches`
- `crate::codegen`
- `crate::compression`
- `crate::core`
- `crate::metamorphose`
- `crate::schemas/bars_core`
- `crate::schemas/test_compatibility_core`
- `crate::transponding`

---

## `crates/adapters/arrow`

### Source Files

- `crates/adapters/arrow/src/lib.rs`: Arrow RecordBatch builders, field metadata helpers, array conversion helpers, and columnar checksum support.
- `crates/adapters/arrow/src/tests/mod.rs`: Arrow adapter unit tests for metadata, primitive arrays, list arrays, optional validity, and record batches.

---

## `crates/adapters/arrow_ipc`

### Source Files

- `crates/adapters/arrow_ipc/src/arrow_bridge.rs`: Arrow array and record-batch bridge helpers shared by Arrow IPC writer code.
- `crates/adapters/arrow_ipc/src/lib.rs`: Arrow IPC stream writer and adapter exports for generated metamorphose modules.
- `crates/adapters/arrow_ipc/src/tests/mod.rs`: Arrow IPC adapter tests for record-batch construction and IPC stream writing.

---

## `crates/adapters/csv`

### Source Files

- `crates/adapters/csv/src/lib.rs`: cap-aware CSV writer helpers for generated row-format metamorphose modules.
- `crates/adapters/csv/src/tests/mod.rs`: CSV adapter unit tests for quoting, scalar cells, bytes, row breaks, and caps.

---

## `crates/adapters/json`

### Source Files

- `crates/adapters/json/src/lib.rs`: cap-aware JSON writer helpers used by generated metamorphose JSON modules.
- `crates/adapters/json/src/tests/mod.rs`: JSON adapter unit tests for escaping, scalar output, and response-cap behavior.

---

## `crates/adapters/parquet`

### Source Files

- `crates/adapters/parquet/src/arrow_bridge.rs`: Arrow array and record-batch bridge helpers reused before Parquet writing.
- `crates/adapters/parquet/src/lib.rs`: uncompressed Parquet writer and adapter exports for generated metamorphose modules.
- `crates/adapters/parquet/src/tests/mod.rs`: Parquet adapter tests for record-batch conversion and Parquet byte writing.

---

## `crates/adapters/protobuf`

### Source Files

- `crates/adapters/protobuf/src/lib.rs`: protobuf wire writer helpers and encoded-length utilities used by generated protobuf metamorphose modules.
- `crates/adapters/protobuf/src/tests/mod.rs`: protobuf adapter unit tests for varints, fixed-width fields, bytes, strings, messages, and caps.

---

## `crates/benches`

### Source Files

- `crates/benches/src/bars_regression.rs`: Bars regression fixtures, timing rows, report metadata, old-evidence parsing, and report writing helpers.
- `crates/benches/src/bin/mbt_bars_regression_bench.rs`: executable Bars regression benchmark timing MBT, projection, and metamorphose lanes.
- `crates/benches/src/bin/mbt_compression_bench.rs`: executable compression benchmark for full and projected Bars MBT bytes.
- `crates/benches/src/bin/mbt_projection_bench.rs`: executable projection benchmark for Bars and test-compatibility schemas.
- `crates/benches/src/compression.rs`: compression benchmark source construction, zstd timing, report writing, environment writing, and summary helpers.
- `crates/benches/src/lib.rs`: benchmark crate entrypoint and module exports.
- `crates/benches/src/projection.rs`: projection benchmark fixtures, baseline parsing, measurement rows, and report helpers.
- `crates/benches/src/tests/mod.rs`: benchmark test module registration.
- `crates/benches/src/tests/test_bars_regression_bench_output.rs`: Bars regression report output tests.
- `crates/benches/src/tests/test_compression_bench_output.rs`: compression benchmark report shape and row-count binding tests.
- `crates/benches/src/tests/test_projection_bench_output.rs`: projection benchmark report output tests.

---

## `crates/codegen`

### Source Files

- `crates/codegen/src/config.rs`: explicit CLI argument parser and validated codegen configuration model.
- `crates/codegen/src/descriptor.rs`: protobuf descriptor loading and MBT schema-model construction from custom options.
- `crates/codegen/src/emit.rs`: codegen action dispatcher, rustfmt integration, write/check handling, and inspect output.
- `crates/codegen/src/error.rs`: typed codegen error surface and shared `Result` alias.
- `crates/codegen/src/lib.rs`: crate entrypoint and module ownership boundary for descriptor parsing and Rust emission.
- `crates/codegen/src/main.rs`: `mbt_codegen` binary entrypoint and process-boundary error handling.
- `crates/codegen/src/model.rs`: resolved schema, field, dictionary, projection, and output-tree model types consumed by emitters.
- `crates/codegen/src/options.rs`: MBT custom option descriptor resolution and typed option extraction helpers.
- `crates/codegen/src/rust_emit.rs`: Rust code emitters for core schemas, projections, metamorphose adapters, and transponding surfaces.
- `crates/codegen/src/tests/mod.rs`: shared codegen fixtures, temporary output helpers, and test module registration.
- `crates/codegen/src/tests/test_cli.rs`: CLI parser validation tests for action, surface, adapter, and output argument rules.
- `crates/codegen/src/tests/test_descriptor.rs`: descriptor-to-model tests for supported schemas and rejected invalid schema shapes.
- `crates/codegen/src/tests/test_model.rs`: schema model helper tests for naming and deterministic model behavior.
- `crates/codegen/src/tests/test_options.rs`: MBT custom option parsing tests.
- `crates/codegen/src/tests/test_rust_emit_core.rs`: generated core schema smoke and check tests.
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`: generated metamorphose adapter smoke and check tests.
- `crates/codegen/src/tests/test_rust_emit_projection_direct_writer.rs`: generated direct projection writer smoke and check tests.

---

## `crates/compression`

### Source Files

- `crates/compression/src/lib.rs`: export-only compression crate entrypoint.
- `crates/compression/src/runtime.rs`: opt-in zstd compression and decompression helpers for completed MBT bytes.
- `crates/compression/src/tests/mod.rs`: compression test module registration.
- `crates/compression/src/tests/test_runtime.rs`: compression roundtrip, cap, corrupt-input, and caller-buffer tests.

---

## `crates/core`

### Source Files

- `crates/core/src/codec.rs`: deterministic response-checksum helper for tests, reports, and benchmark evidence.
- `crates/core/src/envelope.rs`: MBT envelope constants, header encode/decode, checksum, checked schema validation, and trusted payload slicing.
- `crates/core/src/error.rs`: typed transport error surface and shared `Result` alias.
- `crates/core/src/lib.rs`: crate entrypoint and public re-export surface for schema-agnostic runtime helpers.
- `crates/core/src/output.rs`: cap-aware byte buffer and encoding helpers shared by generated boundary writers.
- `crates/core/src/runtime.rs`: schema marker trait plus generic encode, access, and inspection dispatch helpers.
- `crates/core/src/tests/mod.rs`: core test module registration.
- `crates/core/src/tests/test_codec.rs`: checksum behavior tests for the core codec helper.
- `crates/core/src/tests/test_envelope.rs`: envelope validation, corruption, schema mismatch, and trusted-payload tests.
- `crates/core/src/tests/test_output.rs`: checked output buffer and response-cap behavior tests.
- `crates/core/src/tests/test_runtime.rs`: generic runtime trait dispatch smoke tests.

---

## `crates/metamorphose`

### Source Files

- `crates/metamorphose/src/lib.rs`: crate entrypoint and export-only surface for metamorphose runtime traits and helpers.
- `crates/metamorphose/src/runtime.rs`: public metamorphose format enum, output enum, schema traits, safe dispatch, and trusted-access dispatch helpers.
- `crates/metamorphose/src/tests/mod.rs`: metamorphose test module registration.
- `crates/metamorphose/src/tests/test_runtime.rs`: runtime dispatch and trusted-token behavior tests.

---

## `crates/schemas/bars_core`

### Source Files

- `crates/schemas/bars_core/src/bars_v1.rs`: generated Bars core schema, envelope access, validation, checksums, views, and MBT-to-MBT projections.
- `crates/schemas/bars_core/src/bars_v1_arrow.rs`: generated Bars Arrow RecordBatch metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs`: generated Bars Arrow IPC metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_csv.rs`: generated Bars CSV metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_json.rs`: generated Bars JSON metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_parquet.rs`: generated Bars Parquet metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_protobuf.rs`: generated Bars protobuf metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_transponding.rs`: generated Bars row-to-column transponding module for columnar adapters.
- `crates/schemas/bars_core/src/lib.rs`: schema crate entrypoint and feature-gated generated adapter module declarations.
- `crates/schemas/bars_core/tests/test_bars_metamorphose.rs`: Bars metamorphose tests for derived UTC row outputs and protobuf nesting.
- `crates/schemas/bars_core/tests/test_bars_projection.rs`: Bars projection tests for checked/trusted parity and projected schema identity.
- `crates/schemas/bars_core/tests/test_bars_shape.rs`: Bars generated-shape tests for schema constants and public type behavior.

---

## `crates/schemas/test_compatibility_core`

### Source Files

- `crates/schemas/test_compatibility_core/src/lib.rs`: schema crate entrypoint and feature-gated generated adapter module declarations.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`: generated all-fields compatibility core schema, validation, checksums, views, and projections.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs`: generated all-fields compatibility Arrow RecordBatch metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs`: generated all-fields compatibility Arrow IPC metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs`: generated all-fields compatibility CSV metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs`: generated all-fields compatibility JSON metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs`: generated all-fields compatibility Parquet metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs`: generated all-fields compatibility protobuf metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs`: generated all-fields compatibility row-to-column transponding module for columnar adapters.
- `crates/schemas/test_compatibility_core/tests/test_determinism.rs`: deterministic encoding, inspection, and row-order tests for the all-fields schema.
- `crates/schemas/test_compatibility_core/tests/test_failures.rs`: invalid input and corrupt payload failure tests for the all-fields schema.
- `crates/schemas/test_compatibility_core/tests/test_projection.rs`: all-fields projection correctness and schema identity tests.
- `crates/schemas/test_compatibility_core/tests/test_roundtrip.rs`: all-fields encode/access/inspect roundtrip tests across supported scalar and array field kinds.

---

## `crates/transponding`

### Source Files

- `crates/transponding/src/lib.rs`: crate entrypoint and export-only surface for shared column buffer contracts.
- `crates/transponding/src/runtime.rs`: transponded column structs, validity bitmap, size checks, and deterministic checksum helpers.
- `crates/transponding/src/tests/mod.rs`: transponding test module registration.
- `crates/transponding/src/tests/test_runtime.rs`: validity bitmap, column sizing, and checksum behavior tests.

---
