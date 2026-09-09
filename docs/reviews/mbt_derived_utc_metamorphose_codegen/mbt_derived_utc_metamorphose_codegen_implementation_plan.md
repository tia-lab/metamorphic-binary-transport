# Implementation Plan: MBT Derived UTC Metamorphose Codegen

Slug: `mbt_derived_utc_metamorphose_codegen`

Status: `IMPLEMENTATION_PLAN_DRAFT_AWAITING_PEER_AUDIT`

This plan does not authorize code changes. Code may start only after this plan
passes a separate implementation-plan peer audit and is explicitly approved.

## Required Reads

Completed for this plan:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/specs/mbt_derived_utc_metamorphose_codegen_SPEC.md`
- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_research_brief.md`
- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_peer_audit.md`
- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_peer_audit_v2.md`
- `docs/specs/mbt_bars_regression_benchmark_SPEC.md`
- `crates/codegen/src/model.rs`
- `crates/codegen/src/descriptor.rs`
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/codegen/src/tests/test_descriptor.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`
- `crates/schemas/bars_core/src/lib.rs`
- `crates/schemas/bars_core/tests/test_bars_shape.rs`
- `crates/schemas/bars_core/tests/test_bars_projection.rs`
- `crates/schemas/bars_core/Cargo.toml`
- `crates/benches/src/bin/mbt_bars_regression_bench.rs`
- `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`

## Goal

Implement the approved spec exactly:

- preserve MBT archive/core/projection/columnar behavior;
- add schema-derived UTC row-format metadata to the codegen model;
- emit derived UTC in JSON, protobuf, and CSV only;
- keep JSON and CSV flattened by logical path;
- restore protobuf output to the schema message tree, including Bars metadata
  under row field `22`;
- regenerate only the allowed Bars row-format adapter files through
  `mbt_codegen`.

## Non-Goals

This implementation will not:

- edit generated files by hand;
- change `crates/core`;
- change adapter crates;
- change `crates/metamorphose`;
- change `crates/transponding`;
- change benchmark code;
- add dependencies;
- change `Cargo.lock`;
- emit UTC fields in archive, projection, transponding, Arrow, Arrow IPC, or
  Parquet output;
- claim performance before correctness and benchmark evidence exist.

## File Bindings

### Code files to edit

```text
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
crates/schemas/bars_core/tests/test_bars_shape.rs
```

### Code files conditionally allowed

Use only if the generated Bars test organization is cleaner with a separate
file:

```text
crates/schemas/bars_core/tests/test_bars_metamorphose.rs
crates/schemas/bars_core/tests/mod.rs
```

### Generated files to write only with `mbt_codegen --write`

```text
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
```

### Files that must not change

```text
Cargo.toml
Cargo.lock
crates/core/src/*
crates/adapters/json/src/lib.rs
crates/adapters/protobuf/src/lib.rs
crates/adapters/csv/src/lib.rs
crates/metamorphose/src/*
crates/transponding/src/*
crates/benches/src/*
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/bars_core/src/bars_v1_transponding.rs
crates/schemas/bars_core/src/bars_v1_arrow.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

If any forbidden file changes, implementation stops before validation.

## Implementation Steps

### 1. Model additions

Edit `crates/codegen/src/model.rs`.

Add:

```rust
pub struct DerivedUtcField { ... }
pub enum JsonCsvOutputField { ... }
pub struct ProtobufMessageModel { ... }
pub enum ProtobufOutputField { ... }
```

Add these fields to `SchemaModel`:

```rust
pub derived_utc_fields: Vec<DerivedUtcField>,
pub json_csv_output_fields: Vec<JsonCsvOutputField>,
pub protobuf_messages: Vec<ProtobufMessageModel>,
```

Required constraints:

- keep `model.fields` unchanged as physical archive fields;
- do not alter `PhysicalField`, `FieldKind`, projection structures, or schema
  hash semantics except where compilation requires new field initialization;
- initialize projection-only `SchemaModel` values with no derived UTC output
  metadata because projection remains MBT-to-MBT and not row-format output.

### 2. Descriptor derived-UTC model construction

Edit `crates/codegen/src/descriptor.rs`.

Add internal descriptor helper types if needed, for example:

```rust
struct DerivedUtcCandidate { ... }
struct OutputTraversalItem { ... }
```

Required behavior:

- while traversing the row message tree, collect ignored fields with
  `derived_utc_from`;
- keep ignored derived UTC fields out of `fields`;
- keep ignored non-derived fields out of all output models;
- fail when `derived_utc_from` appears without `ignored`;
- fail when derived UTC field is not `string`;
- fail when derived UTC field is repeated;
- resolve source names after all physical fields are known;
- resolve bare source names relative to the derived field parent path;
- resolve dotted source names as exact flattened row logical paths;
- require the source field to be physical `I64`;
- copy source presence information from the resolved physical source field;
- fail on missing source, ambiguous source, source outside the row tree,
  duplicate JSON/CSV output path, duplicate protobuf helper stem, or duplicate
  protobuf tag inside one message model.

Build:

- `derived_utc_fields` in deterministic traversal order;
- `json_csv_output_fields` in flattened proto traversal order;
- `protobuf_messages` from the protobuf message tree, with row message at
  index `0` and child message entries inserted deterministically.

Bars-specific expected descriptor shape:

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

No Bars-specific branch is allowed. Bars can appear only as generated output
from the schema.

### 3. JSON emitter update

Edit `crates/codegen/src/rust_emit.rs`.

Replace JSON field constant and row write iteration over `model.fields` with
iteration over `model.json_csv_output_fields`.

Required helper shape:

- physical JSON fields reuse the existing field constant naming and value
  writer behavior;
- derived JSON fields emit a static field constant using the derived logical
  path;
- derived JSON field write uses:

```rust
writer.utc_value(row.<source_rust_name>.to_native())?;
```

- if the source field is optional, guard both field prefix and UTC value with
  the same archived source presence predicate used for physical optional
  fields;
- no DTO allocation, no string allocation outside the response buffer, and no
  extra archive validation pass.

### 4. Protobuf emitter update

Edit `crates/codegen/src/rust_emit.rs`.

Replace protobuf row length/write iteration over `model.fields` with
`model.protobuf_messages`.

Required helper shape:

```rust
fn encoded_len_row(...) -> Result<usize>;
fn write_protobuf_row(...) -> Result<()>;
fn encoded_len_<child_stem>(...) -> Result<usize>;
fn write_protobuf_<child_stem>(...) -> Result<()>;
```

Required behavior:

- physical fields emit through the existing protobuf length/write logic;
- derived UTC fields emit through:

```rust
output::encoded_len_message(tag, output::utc_len(row.<source>.to_native())?)
writer.utc(tag, row.<source>.to_native())?;
```

- optional derived UTC fields are guarded by the resolved source field
  presence predicate;
- parent message fields call the child length helper and emit:

```rust
writer.message_prefix(child_tag, child_len)?;
write_protobuf_<child>(row, writer)?;
```

- Bars generated protobuf must contain `writer.message_prefix(22` and
  `write_protobuf_metadata`;
- Bars metadata UTC writes must be inside `write_protobuf_metadata`, including
  `writer.utc(6, row.ingested_at_ms.to_native())`.

The implementation must not flatten nested protobuf message fields into the row
message.

### 5. CSV emitter update

Edit `crates/codegen/src/rust_emit.rs`.

Replace CSV header and row write iteration over `model.fields` with
`model.json_csv_output_fields`.

Required behavior:

- header includes derived UTC logical paths in flattened traversal order;
- physical cells reuse existing cell writers;
- required derived UTC sources write:

```rust
writer.utc_cell(row.<source>.to_native())?;
```

- optional derived UTC sources leave an empty cell when absent and write the
  UTC cell only when present;
- comma placement remains controlled by output field index.

### 6. Columnar exclusion check

Do not edit Arrow, Arrow IPC, Parquet, or transponding generator logic except
if compilation requires import or type signature updates caused by model field
additions.

After generation, verify the generated columnar Bars files are unchanged:

```text
crates/schemas/bars_core/src/bars_v1_transponding.rs
crates/schemas/bars_core/src/bars_v1_arrow.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
```

### 7. Codegen tests

Edit `crates/codegen/src/tests/mod.rs`.

Add or extend fixtures:

- `valid_nested_derived_utc_proto()`;
- invalid derived UTC without ignored;
- invalid derived UTC with missing source;
- invalid derived UTC with non-I64 source;
- invalid repeated derived UTC;
- invalid duplicate protobuf output tag inside one message model;
- invalid duplicate protobuf helper stem, unless descriptor construction proves
  helper-stem duplication is structurally impossible and the validation point is
  bound in `test_descriptor.rs`.

Edit `crates/codegen/src/tests/test_descriptor.rs`.

Add tests proving:

- ignored derived UTC is absent from `model.fields`;
- ignored derived UTC is present in `model.derived_utc_fields`;
- `json_csv_output_fields` contains physical and derived fields in proto order;
- `protobuf_messages` contains row and nested child message models;
- nested physical fields are assigned to the owning protobuf message model;
- nested derived UTC fields are assigned to the owning protobuf message model;
- source resolution works for top-level and nested relative source names;
- source presence bit is copied to derived UTC metadata;
- invalid declarations fail before emission.

Edit `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`.

Add tests proving:

- JSON generated source includes a derived UTC field constant;
- JSON generated source calls `writer.utc_value(row.close_ms.to_native())`;
- protobuf generated source uses `output::utc_len` and `writer.utc`;
- protobuf generated source emits nested helper functions for a child message;
- protobuf generated source writes child message fields only inside the child
  helper;
- CSV generated source includes the derived field in `CSV_HEADER`;
- CSV generated source calls `writer.utc_cell(row.close_ms.to_native())`;
- Arrow, Arrow IPC, Parquet, and transponding generated sources do not contain
  the derived UTC field name for the fixture.

### 8. Generated Bars tests

Edit `crates/schemas/bars_core/tests/test_bars_shape.rs` or add
`crates/schemas/bars_core/tests/test_bars_metamorphose.rs`.

Required tests:

- encode a small Bars fixture and call public JSON metamorphose output;
- assert JSON output contains `open_utc` and `close_utc`;
- call public CSV metamorphose output;
- assert CSV output contains `open_utc`, `close_utc`, and metadata UTC columns;
- inspect `include_str!("../src/bars_v1_protobuf.rs")`;
- assert generated source contains `writer.message_prefix(22` and
  `write_protobuf_metadata`;
- assert generated source contains
  `writer.utc(6, row.ingested_at_ms.to_native())`;
- keep existing core inspection assertions that UTC fields are absent from MBT
  inspect/core output.

The tests must use public metamorphose APIs for JSON/CSV unless a separately
approved spec changes module visibility.

## Generated Artifact Steps

Generated files must be written only with these commands:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs
```

Then prove reproducibility with:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs
```

If any generated artifact requires manual editing, stop.

## Review and Evidence Artifact Bindings

Implementation must create or amend these review artifacts:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_result_review.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

Compile-surface evidence must be written to:

```text
docs/evidence/mbt_derived_utc_metamorphose_codegen/codegen_source_wc.txt
docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_row_format_wc.txt
docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_json_protobuf_csv_check_time.txt
docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_arrow_ipc_parquet_check_time.txt
```

Benchmark evidence remains under:

```text
docs/evidence/mbt_bars_regression_benchmark/
```

## Validation Commands

Run in this order.

### Narrow tests

```text
cargo test -p metamorphic_binary_transport_codegen
cargo test -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv
```

### Feature-surface checks

```text
cargo check -p metamorphic_binary_transport_schema_bars --no-default-features
cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv
cargo check -p metamorphic_binary_transport_schema_bars --features arrow_ipc,parquet
```

### Reproducibility checks

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs
```

### Clippy

```text
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
```

### Compile-surface evidence

```text
mkdir -p docs/evidence/mbt_derived_utc_metamorphose_codegen
wc -l crates/codegen/src/model.rs crates/codegen/src/descriptor.rs crates/codegen/src/rust_emit.rs | tee docs/evidence/mbt_derived_utc_metamorphose_codegen/codegen_source_wc.txt
wc -l crates/schemas/bars_core/src/bars_v1_json.rs crates/schemas/bars_core/src/bars_v1_protobuf.rs crates/schemas/bars_core/src/bars_v1_csv.rs | tee docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_row_format_wc.txt
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv 2>&1 | tee docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_json_protobuf_csv_check_time.txt
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_bars --features arrow_ipc,parquet 2>&1 | tee docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_arrow_ipc_parquet_check_time.txt
```

### Benchmark command

Run only after correctness, reproducibility, and compile-surface checks pass:

```text
mkdir -p docs/evidence/mbt_bars_regression_benchmark
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

No speed claim is accepted until semantic checksum parity passes.

## Expected Outputs

Expected codegen test outcomes:

- descriptor tests prove derived UTC is model-only row-format metadata and not
  physical archive data;
- emitter tests prove JSON/CSV flattened output and protobuf nested output;
- invalid derived UTC declarations fail before generation.

Expected generated Bars output:

- `bars_v1_json.rs` contains derived UTC JSON field constants and UTC writes;
- `bars_v1_csv.rs` contains derived UTC columns in the header and UTC cells;
- `bars_v1_protobuf.rs` contains `writer.message_prefix(22`,
  `write_protobuf_metadata`, and metadata-local UTC writes;
- generated core/projection/columnar files remain unchanged.

Expected command outcomes:

- all listed tests and checks pass;
- codegen `--check` passes for JSON/protobuf/CSV;
- compile-surface evidence files exist under
  `docs/evidence/mbt_derived_utc_metamorphose_codegen/`;
- benchmark writes one JSON evidence file under
  `docs/evidence/mbt_bars_regression_benchmark/`.

Expected review outputs:

- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_result_review.md`
  records implementation, validation, compile-surface evidence, failures if any,
  and remaining limits;
- `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md`
  is amended after the benchmark rerun to state whether semantic checksum parity
  passed before any speed interpretation.

## Stop Conditions

Stop immediately if:

- implementation needs a file not bound by this plan;
- implementation requires a new dependency;
- `Cargo.lock` changes;
- a generated file needs manual editing;
- generated core/projection/columnar files change;
- JSON/CSV need nested objects to pass tests;
- protobuf remains flattened for nested message fields;
- invalid derived UTC declarations cannot be rejected before emission;
- validation fails and the failure is not understood.

Do not relax tests or tolerances to proceed.

## Rollback Boundary

Rollback scope is limited to:

```text
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
crates/schemas/bars_core/tests/test_bars_shape.rs
crates/schemas/bars_core/tests/test_bars_metamorphose.rs
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
docs/evidence/mbt_derived_utc_metamorphose_codegen/
docs/evidence/mbt_bars_regression_benchmark/
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_result_review.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

No destructive git command is authorized.

## Known Risks

1. The current research brief still says protobuf uses flattened traversal.
   Implementation must follow the amended spec and peer audit v2, not that old
   brief wording.
2. Optional parent-message presence is outside this spec. If implementation
   discovers an optional parent-message case that affects Bars or required
   tests, stop and amend the spec.
3. Generated protobuf source will grow because nested helpers are required for
   semantic parity. Compile-surface evidence must record the actual impact.
4. Old-vs-new benchmark parity may still fail for another semantic reason. If
   it does, record it in the result review and do not claim performance parity.

## Implementation-Plan Peer Audit Requirement

This plan must pass a separate peer audit before implementation approval.

The audit must verify:

- every code, generated, test, benchmark, and evidence file is bound here;
- no generated file is manually edited;
- protobuf follows schema message-tree output and does not reuse the JSON/CSV
  flattened output list;
- derived UTC remains excluded from archive, projection, transponding, Arrow,
  Arrow IPC, and Parquet;
- no dependency, `Cargo.lock`, benchmark-code, or MBT-core change is authorized;
- validation and benchmark commands match the approved spec.

## Approval Checklist

- Spec passed peer audit v2: yes.
- Exact code files bound: yes.
- Exact generated files bound: yes.
- Dependency changes bound as none: yes.
- Validation commands bound: yes.
- Benchmark command bound: yes.
- Rollback boundary bound: yes.
- Code changes authorized by this plan: no.
- Implementation-plan peer audit required: yes.
- Implementation approval still required after peer audit: yes.
