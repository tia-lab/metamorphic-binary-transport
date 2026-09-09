# SPEC: MBT Derived UTC Metamorphose Codegen

## 1. Identification

Slug: `mbt_derived_utc_metamorphose_codegen`

Repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Research brief:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_research_brief.md
```

Related blocked result:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

## 2. Status

Status: `DRAFT_AMENDED_AFTER_PEER_AUDIT_AWAITING_PEER_AUDIT_V2`

This spec does not authorize code changes. Implementation may start only after:

1. this spec passes a separate peer audit;
2. a separate implementation plan is written;
3. the implementation plan passes its own peer audit if required by the active
   workflow;
4. the implementation plan is explicitly approved.

## 3. Purpose

Restore schema-derived UTC row-format output in generated metamorphose adapters
and restore protobuf row-format emission to the schema message tree where nested
messages exist.

The proto contract already declares ignored UTC string fields using:

```text
(mathilde.ignored) = true
(mathilde.derived_utc_from) = "source_ms"
```

The generated archive must continue to store only the millisecond source field.
Generated JSON, protobuf, and CSV adapters must emit the derived UTC string at
the boundary when the schema declares it.

JSON and CSV boundary outputs remain flattened by logical path. Protobuf
boundary output must follow protobuf message structure. For Bars, top-level
`open_utc` and `close_utc` are row message fields, while `metadata.*_utc`
fields are emitted inside protobuf field `22`, the metadata child message.

This is required before the Bars old-vs-new regression benchmark can become an
accepted apple-to-apple comparison for row-format outputs.

## 4. Non-goals

This spec does not:

- add a new protobuf option;
- change the MBT envelope, header, payload archive, schema ID, or payload bytes;
- store UTC strings in MBT;
- change MBT-to-MBT projection semantics;
- emit derived UTC columns in transponding, Arrow, Arrow IPC, or Parquet by
  default;
- hand-edit generated files;
- change old experiment crate code;
- change benchmark fixture rows;
- add dependencies;
- change core runtime APIs;
- change JSON or CSV flattening into nested objects;
- claim performance parity before the benchmark is rerun after implementation.

## 5. Measured object

The measured object is generated row-format boundary output for schemas with
ignored derived UTC fields:

```text
MBT archived row
  -> generated JSON writer using flattened logical-path fields
  -> generated CSV writer using flattened logical-path columns
  -> generated protobuf writer using protobuf message-tree fields
```

The immediate concrete schema is Bars:

```text
crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto
```

The generator behavior must be schema-generic. No Bars-specific code is allowed
outside generated Bars artifacts produced by `mbt_codegen`.

## 6. Schema source contract

The source of truth is:

```text
.proto files + proto/mathilde/options.proto
```

### Valid derived UTC declaration

A field is a derived UTC output field only when all are true:

- the field has `(mathilde.ignored) = true`;
- the field has a non-empty `(mathilde.derived_utc_from)`;
- the field kind is protobuf `string`;
- the field is not repeated;
- the source field resolves to one physical MBT field of kind `I64`;
- the source field belongs to the same row message tree.

### Source resolution

The `derived_utc_from` value is resolved as follows:

1. if the value contains `.`, it is an exact flattened row logical path;
2. otherwise it is resolved relative to the derived field parent path.

Examples:

```text
open_utc derived_utc_from "open_ms"
  -> open_ms

metadata.ingested_at_utc derived_utc_from "ingested_at_ms"
  -> metadata.ingested_at_ms

metadata.ingested_at_utc derived_utc_from "metadata.ingested_at_ms"
  -> metadata.ingested_at_ms
```

This prevents a nested derived field from accidentally binding to a top-level
field with the same name.

### Optional source semantics

If the source timestamp field has a presence bit:

- JSON omits the derived UTC field when the source timestamp is absent;
- protobuf omits the derived UTC field when the source timestamp is absent;
- CSV keeps the column and writes an empty cell when the source timestamp is
  absent.

If the source timestamp field is required, JSON/protobuf/CSV always emit the
derived UTC value.

### Row-format output shape by adapter

JSON and CSV use flattened row logical paths:

```text
metadata.ingested_at_utc
```

Protobuf uses the protobuf message tree:

```text
row field 22 -> metadata message
  tag 6 -> ingested_at_utc
```

The codegen model must preserve both:

- flattened logical row order for JSON and CSV;
- protobuf parent message path and field tag ownership for protobuf.

### Ignored non-derived fields

An ignored field without `derived_utc_from` remains fully ignored. It is not a
physical archive field and not a row-format output field.

## 7. Wire and archive contract

The wire/archive contract is unchanged.

Derived UTC fields:

- are not included in owned row structs;
- are not included in archived row structs;
- are not included in archived payloads;
- are not included in physical field arrays;
- are not included in MBT-to-MBT projection payloads;
- are not included in core schema hash input beyond the existing normalized
  archive schema behavior.

The core normalized schema hash remains archive-only. This spec does not add an
adapter-output hash.

## 8. Checked and trusted access contract

Checked and trusted access are unchanged.

Derived UTC generation happens after archive access, inside the generated
boundary adapter writer:

```text
checked bytes -> checked archived access -> derived UTC output
trusted bytes -> trusted archived access -> derived UTC output
```

Trusted generated functions keep the same safety contract: the caller must
guarantee the bytes were already validated for the schema before immutable
storage or transport handoff.

## 9. Codegen contract

### Model additions

`crates/codegen/src/model.rs` must add explicit row-format output metadata.

Approved model shape:

```rust
pub struct DerivedUtcField {
    pub proto_path: String,
    pub logical_path: String,
    pub parent_proto_path: String,
    pub parent_logical_path: String,
    pub proto_name: String,
    pub rust_name: String,
    pub proto_number: u32,
    pub source_field_index: usize,
    pub source_logical_path: String,
    pub source_rust_name: String,
    pub source_presence_bit: Option<u32>,
}

pub enum JsonCsvOutputField {
    Physical { field_index: usize },
    DerivedUtc { derived_index: usize },
}

pub struct ProtobufMessageModel {
    pub logical_path: String,
    pub proto_path: String,
    pub rust_helper_stem: String,
    pub enclosing_proto_number: Option<u32>,
    pub fields: Vec<ProtobufOutputField>,
}

pub enum ProtobufOutputField {
    Physical { field_index: usize },
    DerivedUtc { derived_index: usize },
    Message { message_index: usize },
}
```

`SchemaModel` must contain:

```rust
pub derived_utc_fields: Vec<DerivedUtcField>,
pub json_csv_output_fields: Vec<JsonCsvOutputField>,
pub protobuf_messages: Vec<ProtobufMessageModel>,
```

The model names may change only if the implementation plan justifies a smaller
equivalent shape and the peer audit accepts it. The behavior must not change.

### Descriptor behavior

`crates/codegen/src/descriptor.rs` must:

- collect ignored derived UTC declarations instead of discarding them;
- continue to exclude them from physical archive fields;
- resolve source fields after physical fields are known;
- reject invalid derived UTC declarations before emission;
- build `json_csv_output_fields` in flattened proto traversal order;
- build `protobuf_messages` from the protobuf row message tree.

Flattened proto traversal order means the same field order currently used for
physical fields, with ignored derived UTC output fields inserted at their proto
declaration position.

Protobuf message-tree behavior:

- `protobuf_messages[0]` is the row message itself;
- each non-repeated child message that owns row-format protobuf fields gets a
  `ProtobufMessageModel`;
- the parent message contains `Message { message_index }` at the child field's
  declaration position;
- physical and derived fields emit in their owning protobuf message, not
  flattened to the row message;
- protobuf tags are local to the owning protobuf message.

For Bars, the model must represent at least:

```text
row message:
  tag 4 open_ms
  tag 5 close_ms
  tag 6 open_utc
  tag 7 close_utc
  tag 22 metadata message

metadata message:
  tag 5 ingested_at_ms
  tag 6 ingested_at_utc
  tag 7 target_ingested_at_ms
  tag 8 target_ingested_at_utc
```

### Row-format emitter behavior

`crates/codegen/src/rust_emit.rs` must use `json_csv_output_fields` for:

- JSON field constants;
- JSON row writing;
- CSV header;
- CSV row writing.

It must use `protobuf_messages` for:

- protobuf encoded row length;
- protobuf row writing;
- nested protobuf message length helpers;
- nested protobuf message writer helpers.

`model.fields` remains the source for:

- archive structs;
- row validation;
- schema hash;
- projections;
- transponding;
- Arrow;
- Arrow IPC;
- Parquet.

### JSON emission

For a derived UTC field:

```rust
writer.raw_static(JSON_FIELD_<DERIVED>)?;
writer.utc_value(row.<source>.to_native())?;
```

If the source has a presence bit, the generated code must guard both field
prefix and value with the source presence condition.

JSON object field order must follow `json_csv_output_fields`.

### Protobuf emission

Protobuf emission must be generated from `protobuf_messages`, not from the
flattened JSON/CSV output list.

For every `ProtobufMessageModel`, codegen must emit deterministic helpers:

```rust
fn encoded_len_<message_stem>(
    row: &<RowType as Archive>::Archived,
    max_response_bytes: usize,
) -> Result<usize>;

fn write_protobuf_<message_stem>(
    row: &<RowType as Archive>::Archived,
    writer: &mut ProtoWriter,
) -> Result<()>;
```

The row message may keep the existing names:

```text
encoded_len_row
write_protobuf_row
```

Child messages use deterministic stems derived from the protobuf path, for
example:

```text
encoded_len_metadata
write_protobuf_metadata
```

For a derived UTC field, encoded length must use the existing UTC length helper
inside the owning message helper:

```rust
len = output::checked_len_add(
    len,
    output::encoded_len_message(tag, output::utc_len(row.<source>.to_native())?),
    max_response_bytes,
)?;
```

Write path:

```rust
writer.utc(tag, row.<source>.to_native())?;
```

If the source has a presence bit, both length and write paths must be guarded by
the source presence condition.

For a child protobuf message, parent encoded length must use the child helper:

```rust
let child_len = encoded_len_<child>(row, max_response_bytes)?;
if child_len > 0 {
    len = output::checked_len_add(
        len,
        output::encoded_len_message(child_tag, child_len),
        max_response_bytes,
    )?;
}
```

Parent write path:

```rust
let child_len = encoded_len_<child>(row, usize::MAX)?;
if child_len > 0 {
    writer.message_prefix(child_tag, child_len)?;
    write_protobuf_<child>(row, writer)?;
}
```

For Bars this restores the old row field `22` metadata message behavior. The
metadata physical fields and metadata derived UTC fields must be emitted inside
`write_protobuf_metadata`, not at the row level.

### CSV emission

CSV header must include the derived field logical path in
`json_csv_output_fields` order.

For a required source:

```rust
writer.utc_cell(row.<source>.to_native())?;
```

For an optional source:

```rust
if source_present {
    writer.utc_cell(row.<source>.to_native())?;
}
```

The comma layout remains controlled by field index, so absent optional sources
produce an empty cell.

### Columnar exclusion

The following emitters must continue using physical fields only:

- transponding;
- Arrow RecordBatch;
- Arrow IPC;
- Parquet.

Generated columnar outputs must not contain `open_utc`, `close_utc`, or
metadata UTC columns unless a later approved spec adds a derived-column policy.

## 10. Crate boundary contract

Allowed crates to edit:

```text
crates/codegen
crates/schemas/bars_core
```

`crates/schemas/bars_core` may change only through generated adapter artifacts
and tests explicitly bound by this spec.

Forbidden crate changes:

```text
crates/core
crates/metamorphose
crates/transponding
crates/adapters/*
crates/benches
```

Exception: later implementation plan may add a result-review-only benchmark
rerun command or evidence artifact under `docs/`, but no benchmark code change
is authorized by this spec.

## 11. Dependency contract

No new dependency is allowed.

Existing UTC writer helpers in `crates/core` and adapter crates must be reused.

`Cargo.lock` must not change. If Cargo changes `Cargo.lock`, implementation
must stop and explain why before proceeding.

## 12. Determinism contract

Given identical proto inputs and codegen command:

- generated source must be byte-for-byte stable after rustfmt;
- row-format output field order must be deterministic;
- CSV header order must be deterministic;
- JSON writer order must be deterministic for benchmark checksum stability;
- protobuf writer order must be deterministic even though protobuf semantic
  order is not significant.

## 13. Failure contract

Codegen must fail before emission for:

- `derived_utc_from` on a field without `ignored`;
- `derived_utc_from` with an empty source string;
- derived field kind not `string`;
- repeated derived field;
- missing source field;
- ambiguous relative source resolution;
- source field not physical MBT `I64`;
- derived source outside the row message tree;
- duplicate output logical path in `json_csv_output_fields`;
- duplicate protobuf helper stem in `protobuf_messages`;
- duplicate protobuf output field tag inside one protobuf message model.

Runtime failure behavior remains unchanged:

- invalid UTC timestamp formatting returns the existing transport error from
  `utc_len`, `utc_bytes`, or `write_utc`;
- response cap overflow returns the existing `ResponseTooLarge` error.

## 14. Compile-surface budget

The change is generated-code work and must record compile-surface evidence.

Expected compile-surface impact:

- `crates/codegen` gains small model, descriptor, and emitter logic;
- generated JSON/CSV Bars files gain only the derived UTC writes declared in
  the schema;
- generated protobuf Bars files gain the schema-tree message helpers and
  derived UTC writes required to match the protobuf contract;
- core, projection, transponding, Arrow, Arrow IPC, and Parquet generated files
  are not expected to grow from this change.

Required evidence after implementation:

```text
wc -l crates/codegen/src/model.rs crates/codegen/src/descriptor.rs crates/codegen/src/rust_emit.rs
wc -l crates/schemas/bars_core/src/bars_v1_json.rs crates/schemas/bars_core/src/bars_v1_protobuf.rs crates/schemas/bars_core/src/bars_v1_csv.rs
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_bars --features arrow_ipc,parquet
```

Compile-time claims require those outputs to be recorded in the result review.

## 15. Runtime performance budget

No speed claim is authorized by this spec alone.

Runtime budget after implementation:

- JSON/protobuf/CSV output bytes may increase because schema-declared UTC
  fields are emitted;
- per-row added work is one UTC format operation per emitted derived field;
- protobuf may move existing nested physical fields from row-level output into
  schema-correct child messages; output bytes change, but MBT archive bytes do
  not change;
- no heap allocation is allowed outside the existing response `Vec<u8>` output
  buffer;
- no extra archive validation pass is allowed;
- no DTO materialization is allowed.

Performance must be assessed only by the Bars regression benchmark after
semantic parity checks pass.

## 16. Correctness oracle

Correctness is proved by:

1. descriptor tests proving derived UTC fields are absent from physical fields
   and present in row-format metadata;
2. emitter tests proving JSON/protobuf/CSV generated source writes derived UTC
   using the existing UTC writer helpers;
3. emitter tests proving protobuf nested physical and derived fields are emitted
   inside their owning message helper;
4. emitter tests proving columnar generated source does not include derived UTC
   fields;
5. generated Bars row-format adapter tests proving:
   - JSON output contains `open_utc` and `close_utc`;
   - CSV header contains `open_utc`, `close_utc`, and metadata UTC columns;
   - protobuf generated source contains `writer.message_prefix(22` and
     `write_protobuf_metadata`;
   - protobuf metadata helper writes metadata UTC fields, including
     `writer.utc(6, row.ingested_at_ms.to_native())`;
6. codegen `--check` commands proving generated files are reproducible;
7. Bars regression benchmark semantic checksum parity before speed ratios are
   accepted.

## 17. Benchmark methodology

The benchmark methodology is inherited from:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md
```

After implementation, rerun only after correctness checks pass:

```text
cd /home/tia/_DEV/MATHILDE/metamorphic-binary-transport
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

Old parity evidence is not regenerated by this spec. The result review must
state whether existing old parity evidence is reused or whether a later
approved benchmark plan reruns old evidence.

Speed ratios remain invalid until semantic checksum parity passes.

## 18. Test plan

### Codegen descriptor tests

Update:

```text
crates/codegen/src/tests/test_descriptor.rs
```

Required assertions:

- ignored derived UTC is absent from `model.fields`;
- ignored derived UTC is present in `model.derived_utc_fields`;
- `json_csv_output_fields` contains physical and derived fields in proto order;
- `protobuf_messages` contains row and nested child message models;
- nested physical fields are assigned to the owning protobuf message model;
- nested derived UTC fields are assigned to the owning protobuf message model;
- source resolution works for top-level and nested relative source names;
- source presence bit is copied to the derived UTC metadata;
- invalid derived declarations fail before emission.

### Codegen emitter tests

Update:

```text
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
```

Required assertions:

- JSON generated source includes a field constant for `ignored_utc`;
- JSON generated source calls `writer.utc_value(row.close_ms.to_native())`;
- protobuf generated source uses `output::utc_len` and `writer.utc`;
- protobuf generated source emits nested helper functions for a fixture with a
  child message;
- protobuf generated source writes child message fields only inside the child
  helper;
- CSV generated source includes `ignored_utc` in `CSV_HEADER`;
- CSV generated source calls `writer.utc_cell(row.close_ms.to_native())`;
- Arrow, Arrow IPC, Parquet, and transponding generated sources do not contain
  the derived UTC field name for the fixture.

### Generated Bars tests

Update or add tests under:

```text
crates/schemas/bars_core/tests
```

Required assertions:

- CSV output contains `open_utc` and `close_utc`; tests should use public
  metamorphose output unless module visibility is changed by a separately
  approved spec;
- JSON output contains `open_utc` and `close_utc`;
- `include_str!` or equivalent generated-source inspection for
  `bars_v1_protobuf.rs` contains `writer.message_prefix(22` and
  `write_protobuf_metadata`;
- generated-source inspection for `bars_v1_protobuf.rs` contains metadata UTC
  writes inside the metadata helper, including
  `writer.utc(6, row.ingested_at_ms.to_native())`;
- existing core inspection tests are migrated so they still assert UTC fields
  are absent from MBT inspect/core output.

### Validation commands

Required commands:

```text
cargo test -p metamorphic_binary_transport_codegen
cargo test -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv
cargo check -p metamorphic_binary_transport_schema_bars --no-default-features
cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv
cargo check -p metamorphic_binary_transport_schema_bars --features arrow_ipc,parquet
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
```

If any command fails, implementation stops and the result review records the
failure.

## 19. Code bindings

Allowed code files:

```text
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
crates/schemas/bars_core/tests/test_bars_shape.rs
```

Conditionally allowed if required by test organization:

```text
crates/schemas/bars_core/tests/test_bars_metamorphose.rs
crates/schemas/bars_core/tests/mod.rs
```

Forbidden code files:

```text
crates/core/src/output.rs
crates/adapters/json/src/lib.rs
crates/adapters/protobuf/src/lib.rs
crates/adapters/csv/src/lib.rs
crates/transponding/src/*
crates/metamorphose/src/*
crates/benches/src/*
```

## 20. Generated artifact bindings

Generated files may be changed only by `mbt_codegen --write`.

Allowed generated files:

```text
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
```

Generated files that must not change for this spec:

```text
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/bars_core/src/bars_v1_transponding.rs
crates/schemas/bars_core/src/bars_v1_arrow.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Reproducibility commands:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs
```

No generated file may be edited by hand.

## 21. Review artifact bindings

Research brief:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_research_brief.md
```

Peer audit:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_peer_audit.md
```

Implementation plan:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_implementation_plan.md
```

Result review:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_result_review.md
```

Bars regression result review to amend after benchmark rerun:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

Evidence directory:

```text
docs/evidence/mbt_derived_utc_metamorphose_codegen/
```

## 22. Implementation plan requirement

The implementation plan must bind:

- exact code edits;
- exact generated writes;
- exact tests;
- exact validation commands;
- exact benchmark rerun command if benchmark rerun is included;
- exact evidence files;
- rollback boundary;
- known risks.

The plan must state that generated files are written only by `mbt_codegen`.

## 23. Approval checklist

- Required reads complete: yes.
- Research brief exists: yes.
- Measured object precise: yes.
- Source schema contract explicit: yes.
- Wire/archive contract explicit: yes.
- Checked/trusted access contract explicit: yes.
- Codegen contract explicit: yes.
- Dependency contract explicit: yes.
- Failure contract explicit: yes.
- Compile-surface budget explicit: yes.
- Runtime performance budget explicit: yes.
- Correctness oracle explicit: yes.
- Benchmark methodology explicit: yes.
- Code bindings exact: yes.
- Generated artifact bindings exact: yes.
- Peer audit passed: no.
- Implementation plan written: no.
- Implementation plan approved: no.

### Pre-audit closure checklist

- Mandatory section order matches `docs/protocols/spec_protocol.md`: yes.
- Prior approved specs searched:
  - `mbt_bars_regression_benchmark_SPEC.md` is preserved; this spec unblocks a
    diagnosed semantic mismatch before speed ratios can be accepted.
  - `mbt_metamorphose_migration_SPEC.md` is preserved; this spec uses its
    existing row-format writer helper contracts.
  - `mbt_projection_migration_SPEC.md` is preserved; derived UTC remains outside
    MBT-to-MBT projection.
- Every command surface is exact: yes.
- Every generated artifact has one owner and one reproducibility command: yes.
- No generated artifact is bound to incompatible command surfaces: yes.
- Runtime/codegen dispatch paths that must change are listed: yes.
- Tests that encode old behavior are migrated or preserved:
  - core inspect tests preserve UTC absence from MBT core output;
  - row-format tests migrate to UTC presence in JSON/protobuf/CSV output;
  - protobuf tests migrate from flattened nested-message output to
    schema-tree nested-message output.
- Exact code paths needed for the change are bound: yes.
- No design decision is deferred to the implementation plan: yes.
- Compile-surface evidence commands are defined: yes.

## 24. Open questions

None.

If the peer audit finds an ambiguity, this spec must be amended before an
implementation plan is written.
