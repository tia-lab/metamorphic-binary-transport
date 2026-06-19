# SPEC: MBT Projection Metamorphose Adapter

## 1. Identification

Slug: `mbt_projection_metamorphose_adapter`

Repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Task class: corrected spec authoring.

Research brief:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_research_brief.md
```

Prior JSON-only artifacts for this slug are superseded by this corrected spec.

Downstream blocker this spec may unblock after implementation and result
review:

```text
/home/tia/_DEV/MATHILDE/experiments/docs/specs/serving_new_mbt_cache_rebuild_SPEC.md
```

## 2. Status

Status: draft amended after implementation-plan peer audit v3, awaiting peer
audit v6.

This spec does not authorize code changes.

Implementation may start only after:

1. this corrected spec passes a separate peer audit;
2. an implementation plan is written;
3. the implementation plan passes its own audit;
4. the implementation plan is explicitly approved.

## 3. Purpose

Add generic generated adapter support for proto-declared MBT projection
markers across every currently supported metamorphose adapter.

The required architecture is:

```text
source MBT bytes
  -> generated MBT-to-MBT projection
  -> projected MBT bytes
  -> generated adapter for the projected marker
  -> requested boundary output
```

Projection remains transport-level MBT-to-MBT. Adapters never own projection
logic. A projected marker must behave like a normal schema marker for every
adapter that the schema crate explicitly generates and enables.

Required adapter selectors:

```text
json
protobuf
csv
transponding
arrow
arrow-ipc
parquet
```

Transponding remains hidden. It is included because Arrow, Arrow IPC, and
Parquet require the schema-local row-to-column kernel for projected markers.

## 4. Non-goals

This spec does not:

- change the MBT envelope;
- change rkyv archive layout;
- change projection semantics;
- change compression;
- change MBT core;
- change adapter crate runtime behavior unless a later implementation plan
  proves a helper gap, which this spec does not currently identify;
- change serving code;
- add serving-owned generated artifacts to this repository;
- add schema-specific branches for Bars, no-metadata, or serving;
- add generated adapter support by hand-editing generated files;
- expose transponding as a public user API;
- add dependencies;
- claim runtime or compile-time improvement before evidence.

## 5. Measured object

The measured object is generated code for:

```text
--surface metamorphose --adapter <adapter>
```

For every selected adapter, the generated artifact must contain:

- source schema marker adapter APIs;
- projection marker adapter APIs for every proto-declared projection;
- checked and trusted projection marker paths;
- trait implementations for the source marker and each projection marker where
  the adapter has a public metamorphose trait;
- private projected transponding support for columnar adapters.

Measured checked projection-marker flow:

```text
ProjectionMarker::metamorphose_<format>(projected_mbt_bytes, max_response_bytes)
  -> ProjectionMarker::access_archived(projected_mbt_bytes)
  -> generated adapter writer for ProjectionMarker
  -> boundary output
```

Measured trusted projection-marker flow:

```text
unsafe {
  ProjectionMarker::metamorphose_<format>_trusted_unchecked(
    projected_mbt_bytes,
    max_response_bytes,
  )
}
  -> ProjectionMarker::access_archived_trusted_unchecked(projected_mbt_bytes)
  -> generated adapter writer for ProjectionMarker
  -> boundary output
```

Columnar checked flow:

```text
ProjectionMarker::metamorphose_arrow/projected_ipc/projected_parquet(...)
  -> ProjectionMarker::access_archived(...)
  -> ProjectionMarker::transpond_archived(...)
  -> selected columnar adapter writer
```

The spec measures generated adapter availability and correctness, not HTTP
serving performance.

## 6. Schema source contract

The source of truth remains:

```text
.proto files + proto/mathilde/options.proto
```

Projection definitions remain proto-derived:

```proto
option (mathilde.projection) = {
  name: "..."
  rust_marker: "..."
  include_group: "..."
  exclude_group: "..."
  include_field: "..."
  exclude_field: "..."
};
```

Codegen must use descriptor/model data derived from proto annotations. It must
not infer projection behavior from:

- Rust type names;
- serving paths;
- benchmark labels;
- schema crate names;
- handwritten Bars logic.

Required generic proof fixture:

```text
crates/codegen/src/tests/mod.rs::valid_alias_and_projection_ignored_proto()
```

Required runtime proof schemas:

```text
crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

Bars proves parity with the real benchmark schema and no-metadata projections.
Test compatibility proves non-Bars field kinds.

## 7. Wire and archive contract

This spec does not alter MBT bytes.

Projection output remains a complete projected MBT payload:

```text
MBT envelope
projected rkyv payload bytes
```

Adapter output remains a boundary artifact:

```text
projected MBT bytes
  -> generated adapter writer
  -> JSON/protobuf/CSV/Arrow/Arrow IPC/Parquet output
```

Projection marker checked access must validate against the projected marker
schema, not the source marker schema.

The source marker adapter must continue to validate against the source marker
schema.

## 8. Checked and trusted access contract

Generated projection marker APIs must match source marker API shape.

Row-format examples:

```rust
impl ProjectionMarker {
    pub fn metamorphose_json(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> Result<Vec<u8>>;

    pub unsafe fn metamorphose_json_trusted_unchecked(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> Result<Vec<u8>>;
}
```

Columnar examples:

```rust
impl ProjectionMarker {
    pub fn metamorphose_arrow(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> Result<ArrowRecordBatch>;

    pub unsafe fn metamorphose_arrow_trusted_unchecked(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> Result<ArrowRecordBatch>;
}
```

Checked projection marker adapter access must call the projection marker
checked archive accessor.

Trusted projection marker adapter access must call the projection marker
trusted archive accessor and remain visibly unsafe.

No adapter may invent a separate trusted-access contract.

## 9. Codegen contract

The existing CLI contract remains:

```text
--surface metamorphose --adapter <adapter>
```

The existing dispatch path remains:

```text
crates/codegen/src/emit.rs
  Surface::Metamorphose
    -> generated_metamorphose_adapter_schema(model, adapter)
```

Required generated behavior:

1. `generated_metamorphose_adapter_schema(model, adapter)` emits source marker
   support exactly as before for the selected adapter.
2. The same generated artifact emits selected-adapter support for every
   projection in `model.projections`.
3. Projection support uses a projection `SchemaModel` derived from the source
   model and `ProjectionModel`.
4. Projection support completes adapter metadata that
   `projection_schema_model` currently clears:
   - `derived_utc_fields`;
   - `json_csv_output_fields`;
   - `protobuf_messages`.
5. Projection JSON and CSV outputs include only fields present in the projected
   schema, plus derived UTC outputs whose source physical field remains in the
   projected schema.
6. Projection protobuf outputs include only projected schema fields and
   retained derived UTC outputs, preserving message nesting semantics when the
   projected field path still belongs to a retained message path.
7. Projection transponding and columnar outputs include only physical fields
   present in the projected schema. Derived UTC row-format outputs are not
   columnar fields unless a future spec changes columnar semantics.
8. Projection output must not include removed metadata fields.
9. Source helper functions and constants keep their existing names where
   possible for source adapter stability.
10. Projection helper functions and constants must be scoped using existing
    `EmitScope::projection` naming so source and projection sections do not
    collide in one generated file.
11. Generated code must not contain schema-specific branches for Bars, serving,
    no-metadata, or ohlcv-only.
12. Generated code must not use serde JSON, prost DTO materialization, or owned
    row DTO materialization for projection adapter output.

Projected metadata derivation algorithm:

1. Build a deterministic source-to-projected physical field map from
   `ProjectionModel::field_mappings`.
2. Build a deterministic source-derived-UTC-to-projected-derived-UTC map by
   retaining each source derived UTC field only when its source physical field
   exists in the source-to-projected field map. The retained derived UTC field
   must point at the projected physical field index and projected rust field
   name.
3. Build projected JSON/CSV output fields by iterating source
   `json_csv_output_fields` in order:
   - retain `Physical` outputs whose source field index exists in the
     source-to-projected map and rewrite them to the projected field index;
   - retain `DerivedUtc` outputs whose source derived UTC index exists in the
     derived UTC map and rewrite them to the projected derived UTC index;
   - drop all other outputs.
4. Build projected protobuf messages recursively from source
   `protobuf_messages`:
   - retain `Physical` outputs whose source field index exists in the
     source-to-projected map and rewrite them to the projected field index;
   - retain `DerivedUtc` outputs whose source derived UTC index exists in the
     derived UTC map and rewrite them to the projected derived UTC index;
   - recurse into nested `Message` outputs;
   - drop nested messages with no retained child fields;
   - drop parent messages with no retained child fields;
   - preserve protobuf field numbers, message names, and ordering for retained
     outputs.

The transponding generated artifact for a module owns source plus all
projection marker transponding sections. For example:

```text
crates/schemas/bars_core/src/bars_v1_transponding.rs
```

must contain:

- `BarsV1ColumnBatch`;
- `BarsV1::transpond_archived`;
- `BarsV1NoMetadataColumnBatch`;
- `BarsV1NoMetadata::transpond_archived`;
- every other projection marker column batch and private transponding method.

Arrow, Arrow IPC, and Parquet adapter files must continue importing:

```rust
use crate::{module}_transponding::*;
```

They must not duplicate transponding logic.

Required adapter support matrix:

| Adapter selector | Generated projection marker requirement |
| --- | --- |
| `json` | `impl JsonMetamorphoseSchema for ProjectionMarker` and inherent checked/trusted JSON APIs |
| `protobuf` | `impl ProtobufMetamorphoseSchema for ProjectionMarker` and inherent checked/trusted protobuf APIs |
| `csv` | `impl CsvMetamorphoseSchema for ProjectionMarker` and inherent checked/trusted CSV APIs |
| `transponding` | private `ProjectionMarker::transpond_archived` and projected column batch types |
| `arrow` | `impl ArrowMetamorphoseSchema for ProjectionMarker`, calling projected private transponding |
| `arrow-ipc` | `impl ArrowIpcMetamorphoseSchema for ProjectionMarker`, calling projected private transponding |
| `parquet` | `impl ParquetMetamorphoseSchema for ProjectionMarker`, calling projected private transponding |

The implementation may refactor existing emitter helpers to accept an
`EmitScope`. That refactor must preserve source marker output behavior.

## 10. Crate boundary contract

Allowed production crate boundaries:

- `mbt_core` remains unchanged;
- `mbt_metamorphose` remains unchanged unless implementation proves a trait
  gap, which this spec does not identify;
- adapter crates remain unchanged unless implementation proves a helper gap,
  which this spec does not identify;
- `mbt_codegen` owns generator changes;
- schema crates own regenerated generated artifacts.

The serving crate remains outside this repository and must generate its own
schema artifacts after this generic codegen support exists.

## 11. Dependency contract

No new dependency is allowed.

Projection marker adapter generation must use the same dependency surface as
the selected source marker adapter.

No protobuf, Arrow, Parquet, SQLite, Heed, HTTP, or serving dependency may be
introduced unless it is already owned by the selected adapter feature.

Default schema builds must remain adapter-free.

## 12. Determinism contract

Generated adapter output must be deterministic for the same proto input,
module name, root message, and selected adapter.

Emission order:

1. source marker adapter section;
2. projection marker adapter sections in proto projection order.

Within each marker:

- fields are emitted in generated schema field order;
- row-format output order is deterministic;
- derived UTC fields keep deterministic order after their source fields;
- helper names are deterministic from projection name and field names.

Regenerated files must pass `--check` with no diff after `--write`.

## 13. Failure contract

Generated projection marker checked access must return existing MBT errors for:

- wrong schema id;
- wrong schema hash;
- wrong schema version;
- corrupt payload bytes;
- invalid archive shape;
- response cap overflow or excess;
- unsupported field kind for the selected adapter.

Generated projection marker trusted access keeps the existing unsafe contract:

```text
caller guarantees bytes were previously accepted by checked access for the same
projection marker and then stored or transported without mutation
```

If implementation discovers that projection metadata for an adapter cannot be
derived safely from the current model, work must stop and the spec must be
amended before code continues.

## 14. Compile-surface budget

Default schema builds must remain adapter-free.

Required compile-surface checks:

```bash
cargo check -p mbt_schema_bars --no-default-features
cargo check -p mbt_schema_test_compatibility --no-default-features
```

Feature builds may grow by `O(number_of_projections * selected_fields)` inside
the selected generated adapter file only.

Required adapter feature checks:

```bash
cargo check -p mbt_schema_bars --features json
cargo check -p mbt_schema_bars --features protobuf
cargo check -p mbt_schema_bars --features csv
cargo check -p mbt_schema_bars --features arrow
cargo check -p mbt_schema_bars --features arrow_ipc
cargo check -p mbt_schema_bars --features parquet
cargo check -p mbt_schema_test_compatibility --features json
cargo check -p mbt_schema_test_compatibility --features protobuf
cargo check -p mbt_schema_test_compatibility --features csv
cargo check -p mbt_schema_test_compatibility --features arrow
cargo check -p mbt_schema_test_compatibility --features arrow_ipc
cargo check -p mbt_schema_test_compatibility --features parquet
```

Required codegen compile check:

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_codegen --all-targets
```

The result review must record output. It may not claim compile-time
improvement unless compared with a recorded baseline.

## 15. Runtime performance budget

This spec does not set a speed-improvement target.

Runtime requirements:

- source adapter behavior must not intentionally add a new allocation or copy
  point;
- projection adapter may allocate the final boundary output, as all non-MBT
  boundary outputs do;
- projection adapter must read archived projected rows directly through
  generated accessors;
- no owned row DTO path is allowed;
- no serde JSON DTO materialization is allowed;
- no prost DTO materialization is allowed;
- columnar formats may allocate declared column/output buffers only;
- columnar formats must perform exactly one projected transponding pass.

Performance claims require a later result review with run evidence.

## 16. Correctness oracle

Correctness is proved by five oracles.

### 16.1 Generic codegen matrix oracle

For `valid_alias_and_projection_ignored_proto()`, every adapter selector must
emit source marker support and projection marker support.

The generated source must contain projection marker names and selected trait or
hidden helper surfaces, for example:

```text
impl SmallProjection
SmallProjection::access_archived
SmallProjection::access_archived_trusted_unchecked
impl JsonMetamorphoseSchema for SmallProjection
impl ProtobufMetamorphoseSchema for SmallProjection
impl CsvMetamorphoseSchema for SmallProjection
SmallProjectionColumnBatch
SmallProjection::transpond_archived
impl ArrowMetamorphoseSchema for SmallProjection
impl ArrowIpcMetamorphoseSchema for SmallProjection
impl ParquetMetamorphoseSchema for SmallProjection
```

The generated source must not contain:

```text
serde_json
prost::Message
crates/serving
BarsV1
BarsV1NoMetadata
```

### 16.2 Bars runtime oracle

Bars tests must:

1. encode fixture rows as `BarsV1`;
2. project with `BarsV1::project_no_metadata`;
3. run no-metadata projected bytes through each enabled adapter marker:
   - `BarsV1NoMetadata::metamorphose_json`;
   - `BarsV1NoMetadata::metamorphose_protobuf`;
   - `BarsV1NoMetadata::metamorphose_csv`;
   - `BarsV1NoMetadata::metamorphose_arrow`;
   - `BarsV1NoMetadata::metamorphose_arrow_ipc`;
   - `BarsV1NoMetadata::metamorphose_parquet`;
4. validate checked and trusted conversion paths where the adapter exposes both;
5. assert row count and selected fields are retained;
6. assert metadata fields removed by projection are absent from row-format
   output;
7. assert metadata fields removed by projection are absent from Arrow
   RecordBatch schema fields;
8. assert metadata fields removed by projection are absent from Arrow IPC
   readback schema fields through `mbt_adapter_arrow_ipc::record_batch_from_ipc_stream`;
9. for Parquet, assert checked/trusted bytes are deterministic and non-empty,
   and use generated-source assertions to prove the Parquet adapter builds its
   Arrow schema from the projected field list. This scope must not add a new
   public Parquet readback helper.

### 16.3 Test-compatibility runtime oracle

Test-compatibility tests must:

1. encode rows using the generated test-compatibility schema;
2. project to at least one existing projection;
3. call the projection marker adapter for every selector;
4. assert projected output contains only fields retained by the projection;
5. assert removed optional fields are absent;
6. assert arrays and nullable arrays remain representable in adapters that
   support them.

### 16.4 Generated-artifact oracle

Every regenerated adapter file must pass its exact `--check` command.

### 16.5 Source stability oracle

Source marker adapter tests must still pass. The implementation may refactor
emitter helpers, but source marker generated behavior must remain
semantically equivalent.

## 17. Benchmark methodology

No benchmark is required before implementation because this scope is a
correctness and architecture prerequisite for serving.

If benchmarks are added later, they must:

- compare identical projected MBT bytes;
- separate projection cost from adapter conversion cost;
- separate checked and trusted access;
- record command, profile, row count, payload size, and output bytes.

## 18. Test plan

Required codegen tests:

```bash
cargo test -p mbt_codegen --all-targets
```

Required schema tests:

```bash
cargo test -p mbt_schema_bars --all-targets --features json,protobuf,csv,arrow,arrow_ipc,parquet
cargo test -p mbt_schema_test_compatibility --all-targets --features json,protobuf,csv,arrow,arrow_ipc,parquet
```

Required compile checks:

```bash
cargo check -p mbt_schema_bars --no-default-features
cargo check -p mbt_schema_test_compatibility --no-default-features
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_codegen --all-targets
```

Required forbidden-source check:

```bash
rg -n 'crates/serving|BarsV1|BarsV1NoMetadata|NoMetadata|OhlcvOnly' crates/codegen/src --glob '!tests/**'
```

Expected forbidden-source result:

```text
no matches
```

## 19. Code bindings

Allowed codegen implementation source files:

```text
crates/codegen/src/rust_emit.rs
```

Allowed codegen unit test files:

```text
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
```

Allowed schema runtime test files:

```text
crates/schemas/bars_core/tests/test_metamorphose.rs
crates/schemas/test_compatibility_core/tests/test_metamorphose.rs
crates/schemas/bars_core/tests/test_projection_metamorphose.rs
crates/schemas/test_compatibility_core/tests/test_projection_metamorphose.rs
```

Test ownership:

- existing `test_metamorphose.rs` files remain the source marker adapter test
  owners;
- new `test_projection_metamorphose.rs` files own projection marker adapter
  tests;
- the implementation plan may create the new projection test files;
- source marker tests must not be moved or weakened.

Generated source files are bound in Section 20 and must be changed only by
approved codegen commands.

No other code file may be edited unless a future spec amendment binds it
before peer audit.

Required codegen behavior to bind in implementation:

- `generated_metamorphose_adapter_schema` emits source plus projection
  sections for each selected adapter;
- projection marker sections use scoped helper names;
- projected row-format output fields are computed from projected schema fields;
- projected protobuf message metadata is computed from projected schema fields;
- projected columnar metadata is computed from projected schema fields;
- projected derived UTC fields are retained only when their source physical
  field remains in the projection;
- source marker adapter behavior remains source-scoped and API-compatible;
- generic fixture proves projection marker adapter generation for every
  selector.

Forbidden implementation files in this scope:

```text
crates/core/**
crates/metamorphose/**
crates/transponding/**
crates/adapters/**
crates/compression/**
proto/**
```

## 20. Generated artifact bindings

Generated Bars adapter artifacts to update by codegen only:

```text
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
crates/schemas/bars_core/src/bars_v1_transponding.rs
crates/schemas/bars_core/src/bars_v1_arrow.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
```

Generated test-compatibility adapter artifacts to update by codegen only:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs
```

Bars generation command pattern, where `<adapter>` and `<out>` are exactly one
row from the Bars adapter table below:

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

Bars check command pattern, where `<adapter>` and `<out>` are exactly the same
row from the Bars adapter table below:

```bash
cargo run -p mbt_codegen --bin mbt_codegen -- \
  --check \
  --proto-root crates/schemas/bars_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/v1/bars.proto \
  --root mathilde.binary_transport.v1.MathildeTransportResponseV1 \
  --module bars_v1 \
  --surface metamorphose \
  --adapter <adapter> \
  --out <out>
```

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

Test-compatibility generation command pattern, where `<adapter>` and `<out>`
are exactly one row from the test-compatibility adapter table below:

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

Test-compatibility check command pattern, where `<adapter>` and `<out>` are
exactly the same row from the test-compatibility adapter table below:

```bash
cargo run -p mbt_codegen --bin mbt_codegen -- \
  --check \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface metamorphose \
  --adapter <adapter> \
  --out <out>
```

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

For every table row, literal substitution of `<adapter>` and `<out>` into the
generation or check command pattern is the exact command. The implementation
plan must not add, remove, or alter command flags.

Generated files remain non-hand-edited.

## 21. Review artifact bindings

Research brief:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_research_brief.md
```

Spec:

```text
docs/specs/mbt_projection_metamorphose_adapter_SPEC.md
```

Required next peer audit:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit_v6.md
```

Required implementation plan after peer audit passes:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_implementation_plan.md
```

Required implementation-plan peer audit:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_implementation_plan_peer_audit_v3.md
```

Required result review after implementation:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_result_review.md
```

The serving rebuild may not proceed past its MBT prerequisite gate until the
result review exists and records passing validation for this spec.

## 22. Implementation plan requirement

The implementation plan must bind:

- exact code edits in `crates/codegen/src/rust_emit.rs`;
- exact test edits in `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`;
- exact schema runtime tests;
- exact generated artifacts;
- exact codegen write/check commands for every adapter and schema;
- exact validation commands;
- rollback boundary;
- compile-surface evidence command output required in result review;
- no dependency changes;
- no serving changes.

The implementation plan must be audited before implementation.

## 23. Approval checklist

### Pre-audit closure checklist

- Mandatory section order matches `docs/protocols/spec_protocol.md`.
- Prior approved specs searched:
  - `mbt_projection_direct_writer_SPEC.md` is preserved because projection
    remains MBT-to-MBT before adapter conversion.
  - `mbt_metamorphose_migration_SPEC.md` is preserved and completed for
    projection markers because it already requires adapter modules to work for
    projected schemas.
  - `serving_new_mbt_cache_rebuild_SPEC.md` is cited as downstream blocker and
    is not implemented here.
- Command surfaces are exact for `--write`, `--check`, tests, compile checks,
  and forbidden-source search.
- Every generated artifact has one owner and one reproducibility command
  pattern plus exact adapter table row.
- No generated artifact is bound to two incompatible command surfaces.
- Runtime/codegen dispatch path to change is listed:
  `generated_metamorphose_adapter_schema(model, adapter)`.
- Tests encoding old behavior are preserved and extended; source marker
  adapter tests remain required and projection marker tests are explicitly
  bound.
- Exact code paths needed for the change are bound.
- No design decision is deferred to the implementation plan.
- Generated-code compile-surface evidence commands are defined.

### Readiness checklist

- Required reads complete.
- Measured object precise.
- Correctness oracle defined.
- Benchmark method defined as not required for this correctness prerequisite.
- Compile-surface budget defined.
- Code bindings exact.
- Generated artifact bindings exact.
- Test and review bindings exact.
- Peer audit still required.
- Implementation plan still required.

## 24. Open questions

No open question blocks peer audit.

Implementation-planning detail still required:

- exact projected protobuf metadata helper implementation;
- exact names of any private shared emitter helper functions;
- exact test assertions for adapter-specific generated source strings.

These are implementation details under the approved design, not open design
questions.
