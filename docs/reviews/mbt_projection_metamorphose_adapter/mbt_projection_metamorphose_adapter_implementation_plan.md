# MBT Projection Metamorphose Adapter Implementation Plan

Slug: `mbt_projection_metamorphose_adapter`

Status: `AMENDED_AFTER_IMPLEMENTATION_PLAN_PEER_AUDIT_V3_AWAITING_V4`

Spec:

```text
docs/specs/mbt_projection_metamorphose_adapter_SPEC.md
```

Passing spec peer audit:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit_v6.md
```

This plan does not authorize implementation. Code changes may start only after
this plan passes implementation-plan peer audit and the user explicitly
approves implementation.

## Goal

Generate projection-marker metamorphose adapter support for every supported
adapter selector:

```text
json
protobuf
csv
transponding
arrow
arrow-ipc
parquet
```

Required runtime shape:

```text
source MBT bytes
  -> generated MBT-to-MBT projection
  -> projected MBT bytes
  -> generated adapter for the projected marker
  -> requested boundary output
```

Adapters do not perform projection. Serving schemas stay outside the MBT repo.

## Required Reads Before Code

Before editing, reread:

1. `AGENTS.md`
2. `docs/invariants/core_invariants.md`
3. `docs/protocols/lifecycle_protocol.md`
4. `docs/protocols/spec_protocol.md`
5. `docs/protocols/implementation_protocol.md`
6. `docs/protocols/code_style_protocol.md`
7. `docs/protocols/codegen_protocol.md`
8. `docs/protocols/testing_benchmark_protocol.md`
9. `docs/specs/mbt_projection_metamorphose_adapter_SPEC.md`
10. `docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit_v6.md`
11. `crates/codegen/src/rust_emit.rs`
12. `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`
13. `crates/codegen/src/tests/test_rust_emit_projection_direct_writer.rs`
14. `crates/codegen/src/tests/mod.rs`
15. `crates/schemas/bars_core/tests/test_metamorphose.rs`
16. `crates/schemas/test_compatibility_core/tests/test_metamorphose.rs`

If any read invalidates this plan, stop and amend the spec or plan before
code.

## Files To Edit

Edit only these existing source and test files:

```text
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
```

## Files To Create

Create these projection-specific runtime test files:

```text
crates/schemas/bars_core/tests/test_projection_metamorphose.rs
crates/schemas/test_compatibility_core/tests/test_projection_metamorphose.rs
```

Create this result review after validation:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_result_review.md
```

## Generated Files

Regenerate these files only by approved codegen commands:

```text
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
crates/schemas/bars_core/src/bars_v1_transponding.rs
crates/schemas/bars_core/src/bars_v1_arrow.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs
```

Do not hand-edit generated files.

## Forbidden Edits

Do not edit:

```text
crates/core/**
crates/metamorphose/**
crates/transponding/**
crates/adapters/**
crates/compression/**
proto/**
Cargo.toml
Cargo.lock
```

If any forbidden edit appears necessary, stop and return to spec amendment.

## Dependency Changes

No dependency change is approved.

## Implementation Steps

### 1. Add projection adapter model derivation

File:

```text
crates/codegen/src/rust_emit.rs
```

Add private helper logic that derives an adapter-ready projection `SchemaModel`
from the source `SchemaModel` and `ProjectionModel`.

Required helper behavior:

1. Start from the existing `projection_schema_model(source, projection)` so
   physical fields, dictionaries, key parts, marker names, schema hash, and
   archive names remain projection-surface compatible.
2. Build a deterministic source-field-index to projected-field-index map from
   `projection.field_mappings`.
3. Rebuild `derived_utc_fields`:
   - retain only source derived UTC fields whose source physical field is in
     the projection map;
   - rewrite `source_field_index` to the projected field index;
   - rewrite `source_rust_name` to the projected field Rust name;
   - rewrite `source_presence_bit` to the projected field presence bit.
4. Rebuild `json_csv_output_fields`:
   - retain source physical outputs whose field index maps to a projected field;
   - retain source derived UTC outputs whose derived UTC field was retained;
   - rewrite retained indexes to projected indexes.
5. Rebuild `protobuf_messages` recursively:
   - retain physical outputs whose source field index maps to a projected field;
   - retain derived UTC outputs whose source derived UTC index maps to a
     projected derived UTC field;
   - recurse into nested message outputs;
   - drop empty nested messages;
   - drop empty parent messages;
   - preserve retained message names, tags, and field order.
6. Return `CodegenError::InvalidSchema` if an adapter needs row-format output
   metadata and the projected metadata is empty.

Do not change core projection output semantics for `--surface projection`
unless this helper proves the current shared helper can be safely extended
without changing generated projection bytes.

### 2. Refactor adapter emitters into source-plus-projection sections

File:

```text
crates/codegen/src/rust_emit.rs
```

Update `generated_metamorphose_adapter_schema(model, adapter)` so every
adapter emits:

1. import/header prelude once;
2. source marker section first;
3. projection marker sections in proto projection order.

Each emitter may return `Result<()>` so projection-model validation errors
propagate through the existing `Result<String>` return.

Required section helpers:

```text
emit_json_adapter_section(out, scope)
emit_protobuf_adapter_section(out, scope)
emit_csv_adapter_section(out, scope)
emit_transponding_adapter_section(out, scope)
emit_arrow_adapter_section(out, scope)
emit_arrow_ipc_adapter_section(out, scope)
emit_parquet_adapter_section(out, scope)
```

The exact helper names may differ, but their responsibilities must match this
plan and remain private to `rust_emit.rs`.

### 3. Scope row-format helper names

File:

```text
crates/codegen/src/rust_emit.rs
```

Make JSON, protobuf, and CSV helper functions/constants scope-aware.

Source marker generated names remain unprefixed where they exist today:

```text
JSON_SCHEMA_VERSION_FIELD
JSON_ROWS_FIELD
JSON_FIELD_*
write_json_response
write_json_row
write_json_field_prefix
write_json_*_bitmask
CSV_HEADER
write_csv_response
write_csv_row
write_csv_*_bitmask
write_protobuf_response
encoded_len_*
write_*
```

Projection marker generated names use `EmitScope::projection`:

```text
{PROJECTION}_JSON_SCHEMA_VERSION_FIELD
{projection}_write_json_response
{PROJECTION}_CSV_HEADER
{projection}_write_csv_response
{projection}_write_protobuf_response
{projection}_encoded_len_*
{projection}_write_*
```

Optional physical fields and derived UTC fields must use the active
`EmitScope` for presence checks.

### 4. Scope columnar helper names

File:

```text
crates/codegen/src/rust_emit.rs
```

Transponding output:

- source and projected marker column batch names are already marker-specific;
- `transpond_archived` lives inside each marker impl and may keep that method
  name;
- emitted sections must coexist in the same `{module}_transponding.rs` file.

Arrow family output:

- free helper `arrow_record_batch` must be scoped for projection sections;
- source marker helper may remain `arrow_record_batch`;
- projection marker helpers must use projection-scoped names;
- Arrow, Arrow IPC, and Parquet projection marker APIs must call the projected
  helper and projected `Self::transpond_archived`.

### 5. Add codegen matrix tests

File:

```text
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
```

Add tests using:

```text
valid_alias_and_projection_ignored_proto()
```

Required tests:

1. JSON source plus projection marker test.
2. Protobuf source plus projection marker test.
3. CSV source plus projection marker test.
4. Transponding source plus projection marker test.
5. Arrow source plus projection marker test.
6. Arrow IPC source plus projection marker test.
7. Parquet source plus projection marker test.

Required negative assertions for generated codegen source:

```text
serde_json
prost::Message
crates/serving
BarsV1
BarsV1NoMetadata
NoMetadata
OhlcvOnly
```

Add one test for retained optional projected field presence scoping. It may use
an inline proto fixture in this test file. It must prove projection marker
presence checks reference projection-scoped presence constants.

### 6. Add Bars projection runtime tests

Files:

```text
crates/schemas/bars_core/tests/test_projection_metamorphose.rs
```

Keep existing source marker tests in `test_metamorphose.rs` unchanged.

In `test_projection_metamorphose.rs`, add projection marker tests for
`BarsV1NoMetadata`:

1. encode fixture rows as `BarsV1`;
2. project with `BarsV1::project_no_metadata`;
3. validate with `BarsV1NoMetadata::access`;
4. call checked and trusted projection marker adapters:
   - JSON;
   - protobuf;
   - CSV;
   - Arrow;
   - Arrow IPC;
   - Parquet;
5. assert row count/selected retained fields;
6. assert removed metadata fields are absent from row-format output and
   Arrow RecordBatch schema fields;
7. assert removed metadata fields are absent from Arrow IPC readback schema
   fields through:

```rust
mbt_adapter_arrow_ipc::record_batch_from_ipc_stream
```

8. assert Parquet checked/trusted bytes are deterministic and non-empty;
9. assert generated Parquet source uses projected field names and does not
   contain removed metadata field names for the projection marker section.

Use existing helpers where present. Do not add a JSON parser or protobuf DTO
dependency.

### 7. Add test-compatibility projection runtime tests

Files:

```text
crates/schemas/test_compatibility_core/tests/test_projection_metamorphose.rs
```

Keep existing source marker tests in `test_metamorphose.rs` unchanged.

In `test_projection_metamorphose.rs`, add projection marker tests for at least
one existing test-compatibility projection:

1. encode fixture rows as `TestCompatibilityV1`;
2. project with the existing generated projection API;
3. validate projected bytes with the projection marker access API;
4. call checked and trusted projection marker adapters for every selected
   adapter;
5. assert retained representative scalar, dictionary, string, bytes, array,
   and nullable-array fields survive where the projection retains them;
6. assert removed optional fields are absent.

Use UTF-8 substring checks for row-format outputs. Use Arrow RecordBatch schema
inspection for direct Arrow output. Use
`mbt_adapter_arrow_ipc::record_batch_from_ipc_stream` for Arrow IPC readback.
For Parquet, assert deterministic non-empty bytes and generated-source
projection schema assertions. Do not add dependencies.

### 8. Regenerate generated artifacts

Run the exact write commands derived from the two adapter tables in the spec.

Bars write command pattern:

```bash
cargo run -p mbt_codegen --bin mbt_codegen -- \
  --write \
  --proto-root crates/schemas/bars_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/v1/bars.proto \
  --root mathilde.binary_transport.v1.MathildeTransportResponseV1 \
  --module bars_v1 \
  --surface metamorphose \
  --adapter <adapter> \
  --out <out>
```

Test-compatibility write command pattern:

```bash
cargo run -p mbt_codegen --bin mbt_codegen -- \
  --write \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface metamorphose \
  --adapter <adapter> \
  --out <out>
```

The `<adapter>` and `<out>` values must be exactly the rows from Section 20 of
the spec.

Execute all 14 write commands:

- 7 Bars adapter table rows;
- 7 test-compatibility adapter table rows.

The result review must record the concrete command list used.

### 9. Format

Run:

```bash
cargo fmt -p mbt_codegen -p mbt_schema_bars -p mbt_schema_test_compatibility
```

### 10. Validate generated checks

Run the exact `--check` command patterns from the spec for every adapter table
row.

No `--check` command may fail.

Execute all 14 check commands:

- 7 Bars adapter table rows;
- 7 test-compatibility adapter table rows.

The result review must record the concrete command list used.

### 11. Validate tests and compile surface

Run:

```bash
cargo test -p mbt_codegen --all-targets
cargo test -p mbt_schema_bars --all-targets --features json,protobuf,csv,arrow,arrow_ipc,parquet
cargo test -p mbt_schema_test_compatibility --all-targets --features json,protobuf,csv,arrow,arrow_ipc,parquet
cargo check -p mbt_schema_bars --no-default-features
cargo check -p mbt_schema_test_compatibility --no-default-features
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_codegen --all-targets
```

Run forbidden-source check:

```bash
rg -n 'crates/serving|BarsV1|BarsV1NoMetadata|NoMetadata|OhlcvOnly' crates/codegen/src --glob '!tests/**'
```

Expected result:

```text
no matches
```

### 12. Write result review

Create:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_result_review.md
```

Record:

- files changed;
- generated artifact commands run;
- all 28 concrete write/check commands or the exact generated command list;
- check commands run;
- test outputs;
- compile-surface output;
- forbidden-source output;
- what is proved;
- what remains unproved;
- any failed or unstable evidence.

## Adapter Tables

Bars adapter table:

| `<adapter>` | `<out>` |
| --- | --- |
| `json` | `crates/schemas/bars_core/src/bars_v1_json.rs` |
| `protobuf` | `crates/schemas/bars_core/src/bars_v1_protobuf.rs` |
| `csv` | `crates/schemas/bars_core/src/bars_v1_csv.rs` |
| `transponding` | `crates/schemas/bars_core/src/bars_v1_transponding.rs` |
| `arrow` | `crates/schemas/bars_core/src/bars_v1_arrow.rs` |
| `arrow-ipc` | `crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs` |
| `parquet` | `crates/schemas/bars_core/src/bars_v1_parquet.rs` |

Test-compatibility adapter table:

| `<adapter>` | `<out>` |
| --- | --- |
| `json` | `crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs` |
| `protobuf` | `crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs` |
| `csv` | `crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs` |
| `transponding` | `crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs` |
| `arrow` | `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs` |
| `arrow-ipc` | `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs` |
| `parquet` | `crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs` |

## Rollback Boundary

If validation fails, revert only files listed in:

- Files To Edit;
- Files To Create;
- Generated Files;
- Result Review.

Do not revert unrelated user or repository changes.

## Known Risks And Mitigations

Risk: projected protobuf metadata remapping emits empty nested messages.

Mitigation: recursive mapping must drop empty messages; codegen tests must
prove retained projection protobuf marker support.

Risk: helper names collide between source and projection sections.

Mitigation: all projection helper names must use `EmitScope::projection`.

Risk: source marker output behavior changes during helper refactor.

Mitigation: existing source marker codegen and runtime tests remain required.

Risk: columnar projection adapters duplicate transponding.

Mitigation: Arrow family adapters must import `{module}_transponding::*` and
must call the marker-local `Self::transpond_archived`.

Risk: generated files grow unexpectedly.

Mitigation: default no-feature schema checks and `mbt_codegen` timed compile
check are required result evidence.

## Approval State

This implementation plan is written and awaits implementation-plan peer audit.

Code remains forbidden until the implementation plan passes audit and receives
explicit implementation approval.
