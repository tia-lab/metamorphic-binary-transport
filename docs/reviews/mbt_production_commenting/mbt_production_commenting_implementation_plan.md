# MBT Production Commenting Implementation Plan

## Status

Status: implementation plan.

This plan authorizes no source-code change until:

1. this plan receives a separate implementation-plan peer audit;
2. the user approves implementation after that audit.

## Source Artifacts

Approved source chain:

- research brief:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_research_brief.md`
- spec:
  `docs/specs/mbt_production_commenting_SPEC.md`
- peer audit:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_peer_audit.md`

Required future artifacts:

- implementation-plan peer audit:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_implementation_plan_peer_audit.md`
- result review:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_result_review.md`

## Goal

Add sparse, local comments across the hand-owned MBT workspace source so the
production boundaries are easier to audit.

The implementation must be comment-only for source files. It must not change
behavior, generated output, tests, benchmarks, manifests, or dependencies.

## Non-goals

- No code behavior change.
- No generated schema file edits.
- No generated-output string edits in `crates/codegen/src/rust_emit.rs`.
- No Cargo manifest edits.
- No dependency changes.
- No feature flag changes.
- No test changes.
- No benchmark measured-logic changes.
- No README changes.
- No performance or compile-time claim.

## Comment Style Rules

The implementation must follow these rules:

- Use short `//` comments for private implementation boundaries.
- Use `///` only when documenting public API behavior or safety boundaries is
  useful.
- Use `//!` only for module ownership boundaries already present or clearly
  missing.
- Do not comment obvious Rust syntax.
- Do not duplicate nearby comments.
- Do not add speculative comments.
- Do not use comments to hide unclear code.
- Do not claim speed, readiness, finality, repair, or safety beyond the
  existing contract.
- Prefer one-line comments. Use two lines only when needed for clarity.

## Files To Edit

Only the files in this section may be edited, and only by adding, refining, or
removing comments.

### Core

`crates/core/src/codec.rs`

- Add one local comment near `response_checksum` stating that it is a
  deterministic evidence checksum, not a cryptographic integrity primitive.

`crates/core/src/envelope.rs`

- Refine or add a comment near `SchemaHeaderSpec` explaining the expected
  schema identity for envelope validation.
- Keep or refine the existing `TransportHeader` comment if needed.
- Keep or refine the existing `encode_header` comment if needed.
- Add or refine a comment near `decode_header` explaining header-only parsing
  before payload access.
- Add or refine a comment near `validate_header_for_schema` explaining checked
  payload checksum validation.
- Add or refine a comment near `trusted_payload_for_schema` explaining that
  trusted access validates envelope identity and length while payload integrity
  is the caller/storage contract.
- Add a short comment near `validate_identity_and_len` explaining the shared
  identity/length gate used by both checked and trusted paths.

`crates/core/src/output.rs`

- Keep or refine the existing `CheckedBytes` doc comment if needed.
- Add a local comment near `encode_protobuf` explaining direct append followed
  by cap validation.
- Add a local comment near `fmt_error` explaining conversion from formatting
  failure back to response-cap error.
- Add a local comment near `checked_len_add` explaining protobuf length
  preflight.
- Add a local comment near `write_utc` explaining allocation-free UTC output
  into `CheckedBytes`.
- Add a local comment near `utc_bytes` explaining stack-buffer UTC output for
  borrowed byte writers.
- Add a local comment near `write_base64` explaining bytes-to-text boundary
  encoding.
- Add a local comment near `civil_from_days` explaining local date conversion
  without a time dependency.

`crates/core/src/runtime.rs`

- Add a doc or local comment near `BinaryInspection` explaining inspect
  evidence without DTO decode.
- Add a doc or local comment near `MbtSchema` explaining generated schema
  marker dispatch.

### Metamorphose

`crates/metamorphose/src/lib.rs`

- Keep the module-level ownership comment if accurate; refine only if needed
  to state that exports live here and implementation lives in `runtime.rs`.

`crates/metamorphose/src/runtime.rs`

- Keep existing `TrustedUnchecked` safety documentation; refine only if needed.
- Add or refine comments near `MetamorphoseFormat` and `MetamorphoseOutput`
  explaining safe dispatch and borrowed MBT versus owned boundary output.
- Add a comment near the schema traits explaining generated adapter
  implementations are schema-specific and do not require hand-written DTOs.
- Add or refine a comment near `decode` explaining format selection delegates
  validation and writing to schema/adapters.

### Transponding

`crates/transponding/src/lib.rs`

- Keep or refine the module-level ownership comment only if needed.

`crates/transponding/src/runtime.rs`

- Add a comment near `ValidityBitmap` explaining one bit per row and optional
  null semantics.
- Add a comment near `DictionaryMeta` explaining metadata carried into
  columnar adapters.
- Add a comment near `ConstU16Column` explaining constant dictionary columns
  without per-row values.
- Add a comment near `required_numeric_column` explaining shared required
  primitive column behavior.
- Add a comment near `optional_numeric_column` explaining physical defaults
  plus validity bits.
- Add a comment near `numeric_list_column` explaining offsets, values, and
  optional row validity for null-versus-empty arrays.
- Add a comment near `BoolColumn` explaining bool size accounting before
  adapter packing.
- Add a comment near `Utf8Column` explaining concatenated bytes plus offsets.
- Add a comment near `BinaryColumn` explaining UTF-8-like byte layout without
  string validation.
- Add a comment near `ensure_columnar_size` explaining cap checking before
  boundary output.
- Add a comment near `checksum_seed` or the update helpers explaining
  deterministic evidence checksum composition.

### Row-format Adapters

`crates/adapters/json/src/lib.rs`

- Add or refine a comment near `JsonWriter` explaining direct generated JSON
  writing through `CheckedBytes`.
- Add a comment near string/bytes helpers explaining escaping and base64 happen
  at the JSON boundary.
- Add a comment near array helpers explaining direct array emission without
  row DTO allocation.

`crates/adapters/csv/src/lib.rs`

- Add or refine a comment near `CsvWriter` explaining direct capped CSV
  writing.
- Add a comment near string helpers explaining CSV escaping and quoting at the
  boundary.
- Add a comment near bytes helpers explaining base64 text representation.
- Add a comment near array helpers explaining JSON-style cell payloads for
  repeated values.

`crates/adapters/protobuf/src/lib.rs`

- Add or refine a comment near `ProtoWriter` explaining direct protobuf wire
  writing.
- Add comments near length-delimited and packed repeated helpers explaining
  boundary wire encoding and preflight length accounting.

### Columnar Adapters

`crates/adapters/arrow/src/lib.rs`

- Add comments near `schema_metadata` and `field_metadata` explaining MBT
  schema identity and dictionary/bitmask metadata propagation.
- Add one comment grouping primitive array helpers as transponded columns to
  Arrow arrays.
- Add one comment grouping list array helpers as offsets, values, and validity
  mapping.
- Add comments near `record_batch`, `record_batch_byte_len`,
  `record_batch_checksum`, `null_buffer`, `ensure_offsets`, and
  `checksum_array` explaining batch assembly, size accounting, evidence
  checksum, validity conversion, and offset validation.

`crates/adapters/arrow_ipc/src/arrow_bridge.rs`

- Add the same Arrow bridge comments as needed for metadata, array conversion,
  list mapping, null buffers, and offset validation.

`crates/adapters/arrow_ipc/src/lib.rs`

- Add comments near `write_ipc_stream`, `record_batch_from_ipc_stream`,
  `CheckedArrowIpcWriter`, `initial_capacity`, and
  `arrow_ipc_or_overflow_error`.

`crates/adapters/parquet/src/arrow_bridge.rs`

- Add the same Arrow bridge comments as needed for metadata, array conversion,
  list mapping, null buffers, and offset validation.

`crates/adapters/parquet/src/lib.rs`

- Add comments near `write_uncompressed_parquet`, `CheckedParquetWriter`, and
  `parquet_or_overflow_error`.

### Codegen

`crates/codegen/src/config.rs`

- Add comments near `Action`, `Surface`, `Adapter`, `CodegenConfig`,
  `SchemaRequest`, and `parse_args` explaining command mode, surface split,
  adapter opt-in, normalized CLI contract, descriptor request identity, and
  explicit proto-root/schema/root/module/surface requirements.

`crates/codegen/src/descriptor.rs`

- Add comments near `load_schema_model`, `row_payload_field`,
  `collect_schema_fields`, `resolve_derived_utc_fields`,
  `validate_row_format_outputs`, `physical_field`,
  `dictionaries_from_file`, `validate_physical_fields`,
  `projection_models`, `build_projection_model`,
  `selected_projection_indices`, `validate_projection_fields`,
  `raw_descriptor_set`, and `normalized_hash`.

`crates/codegen/src/emit.rs`

- Add comments near `inspect`, `write`, `check`, `smoke_crate`,
  `format_rust`, `write_if_changed`, and `generated_unformatted`.

`crates/codegen/src/model.rs`

- Add comments near `Dictionary`, `SchemaModel`, `DerivedUtcField`,
  `JsonCsvOutputField`, `ProtobufMessageModel`, `ProjectionModel`,
  `PhysicalField`, `FieldKind`, and `validate_module_name`.

`crates/codegen/src/options.rs`

- Add comments near `MbtExtensions`, `extensions`, required/optional option
  helpers, and `dictionary_from_value`.

`crates/codegen/src/rust_emit.rs`

- Add comments in generator source only. Do not change emitted generated-source
  string literals.
- Add comments near `generated_schema`, `generated_projection_schema`,
  `generated_metamorphose_adapter_schema`, row-format adapter emitters,
  columnar adapter emitters, `emit_column_batch`, `emit_runtime_api`,
  `emit_direct_projection_wrappers`, `emit_projection_presence_repack`,
  presence-storage helpers, `emit_header`, `emit_imports`, `emit_validation`,
  `emit_order_validation`, `emit_checksums`, `emit_view_types`,
  `emit_decode_helpers`, and selected field-kind mapping helpers.

`crates/codegen/src/lib.rs`

- Refine the module-level ownership comment only if inaccurate.

`crates/codegen/src/main.rs`

- Add a short comment near `run` explaining typed error propagation through the
  CLI.

### Benchmark And Evidence Support

`crates/benches/src/lib.rs`

- Keep or refine the module-level ownership comment only if inaccurate.

`crates/benches/src/bars_regression.rs`

- Add comments near `OLD_PARITY_EVIDENCE_GLOB`,
  `REQUIRED_SCHEMA_FEATURES`, `BarsRegressionRow`, `SerdeBarRow`,
  `metadata_for_run`, and `write_report`.

`crates/benches/src/projection.rs`

- Add comments near `OLD_BENCH_RESULTS`, `SchemaName`, `BenchRow`,
  `parse_old_crate_projection_baselines`, `parse_current_owned_baselines`,
  and `inspection_checksums`.

`crates/benches/src/bin/mbt_bars_regression_bench.rs`

- Add comments near `measure_row_count`, `measure_full_mbt`,
  `measure_output`, and `measure_serde_json`.

`crates/benches/src/bin/mbt_projection_bench.rs`

- Add comments near `verify_old_baselines`, `measure_bars`,
  `measure_compatibility`, `measure_public`, `measure_archived`, and
  `measure_inspect`.

## Files Intentionally Left Untouched

The implementation must not edit:

- `Cargo.toml`
- `Cargo.lock`
- `proto/**`
- `crates/schemas/bars_core/src/*.rs`
- `crates/schemas/test_compatibility_core/src/*.rs`
- `crates/**/src/tests/**`
- `crates/**/tests/**`
- existing benchmark measured logic beyond comments listed above
- generated-output string literals in `crates/codegen/src/rust_emit.rs`

## Dependency Changes

No dependency change is allowed.

No manifest change is allowed.

No feature change is allowed.

## Generated Files

Generated schema files must not change.

No codegen write command is part of this plan because generated output must not
change.

If generated files change during implementation, the work must stop and the
change must be reverted or a new amendment must be approved.

## Implementation Steps

1. Reread locked protocols and source artifacts before editing:
   - `AGENTS.md`
   - `docs/invariants/core_invariants.md`
   - `docs/protocols/lifecycle_protocol.md`
   - `docs/protocols/spec_protocol.md`
   - `docs/protocols/implementation_protocol.md`
   - `docs/protocols/code_style_protocol.md`
   - `docs/protocols/codegen_protocol.md`
   - `docs/protocols/testing_benchmark_protocol.md`
   - `docs/specs/mbt_production_commenting_SPEC.md`
   - this implementation plan
2. Add comments in the order:
   - core;
   - metamorphose;
   - transponding;
   - adapters;
   - codegen;
   - benchmark/evidence support.
3. Skip any planned comment that duplicates an existing nearby comment.
4. Do not change generated strings, Rust expressions, imports, signatures,
   tests, manifests, or benchmark measured logic.
5. Run validation commands.
6. Inspect the diff and confirm source changes are comment-only.
7. Write the result review.

## Validation Commands

Run from repository root:

```bash
git diff --check
cargo fmt --check
cargo check -p metamorphic_binary_transport_core --all-targets
cargo check -p metamorphic_binary_transport_metamorphose --all-targets
cargo check -p metamorphic_binary_transport_transponding --all-targets
cargo check -p metamorphic_binary_transport_adapter_json --all-targets
cargo check -p metamorphic_binary_transport_adapter_csv --all-targets
cargo check -p metamorphic_binary_transport_adapter_protobuf --all-targets
cargo check -p metamorphic_binary_transport_adapter_arrow --all-targets
cargo check -p metamorphic_binary_transport_adapter_arrow_ipc --all-targets
cargo check -p metamorphic_binary_transport_adapter_parquet --all-targets
cargo check -p metamorphic_binary_transport_codegen --all-targets
cargo check -p metamorphic_binary_transport_benches --all-targets
```

Expected output:

- every command exits with status 0;
- no generated schema file appears in `git diff --name-only`;
- no manifest appears in `git diff --name-only`;
- source diffs are comment-only.

No benchmark command is required.

## Diff Audit

After edits, inspect:

```bash
git diff --name-only
git diff -- crates/core crates/metamorphose crates/transponding crates/adapters crates/codegen crates/benches
```

The result review must state:

- source diff is comment-only;
- generated schema files are unchanged;
- manifests are unchanged;
- tests are unchanged;
- benchmark measured logic is unchanged.

## Rollback Boundary

Rollback is limited to:

- comment edits in files listed under `Files To Edit`;
- `docs/reviews/mbt_production_commenting/mbt_production_commenting_result_review.md`
  if validation fails before final result review.

No rollback of unrelated existing workspace changes is allowed.

## Known Risks

- A planned comment may duplicate local context; implementation must omit it.
- Some cargo checks may expose pre-existing unrelated failures. If that
  happens, the result review must report the failure without claiming this
  task caused or fixed it.
- Adding too many comments would reduce reviewability. The implementation must
  keep comments sparse even though this plan covers the full hand-owned
  workspace.

## Approval Status

Status: not approved for implementation.

Required next command:

```text
Approved: write docs/reviews/mbt_production_commenting/mbt_production_commenting_implementation_plan_peer_audit.md
```
