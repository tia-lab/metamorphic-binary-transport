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

# SPEC: MBT Metamorphose Migration

## 1. Identification

Slug: `mbt_metamorphose_migration`

Repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Task class: spec authoring.

Research brief:

```text
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_research_brief.md
```

Source experiment crate:

```text
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport
```

## 2. Status

Status: `PEER_AUDITED_AWAITING_IMPLEMENTATION_PLAN`

This spec does not authorize code changes.

Implementation may start only after:

1. this spec passes a separate peer audit;
2. an implementation plan binds exact edits and validation commands;
3. the implementation plan passes its own audit if requested;
4. the implementation plan is explicitly approved.

## 3. Purpose

Migrate the old MBT metamorphose and hidden transponding behavior into the new
split workspace without changing the architecture that was already proved in
the experiment crate.

The public user concept remains:

```rust
metamorphose::json::<S>(bytes, max_response_bytes)
metamorphose::protobuf::<S>(bytes, max_response_bytes)
metamorphose::csv::<S>(bytes, max_response_bytes)
metamorphose::arrow::<S>(bytes, max_response_bytes)
metamorphose::arrow_ipc::<S>(bytes, max_response_bytes)
metamorphose::parquet::<S>(bytes, max_response_bytes)
```

The trusted serving concept is explicit and unsafe:

```rust
unsafe { metamorphose::json_trusted_unchecked::<S>(bytes, max_response_bytes) }
unsafe { metamorphose::protobuf_trusted_unchecked::<S>(bytes, max_response_bytes) }
unsafe { metamorphose::csv_trusted_unchecked::<S>(bytes, max_response_bytes) }
unsafe { metamorphose::arrow_trusted_unchecked::<S>(bytes, max_response_bytes) }
unsafe { metamorphose::arrow_ipc_trusted_unchecked::<S>(bytes, max_response_bytes) }
unsafe { metamorphose::parquet_trusted_unchecked::<S>(bytes, max_response_bytes) }
```

Trusted functions are for bytes that were already checked once for the same
schema and then stored or transported under an immutable cache contract.

Transponding is not a public user API. If a requested format needs row-to-column
conversion, the generated metamorphose implementation selects the hidden
transponding kernel automatically.

The migrated design must keep the compile surface smaller than the old
monolith:

```text
schema core default build
  -> core + rkyv only

schema with json feature
  -> core + rkyv + metamorphose + json adapter only

schema with arrow/parquet feature
  -> core + rkyv + metamorphose + transponding + selected columnar adapter only
```

## 4. Non-goals

This spec does not:

- change the MBT envelope;
- change rkyv archive layout;
- change projection semantics;
- expose transponding as a stable public API;
- implement compression;
- implement Arrow Flight;
- implement generic protobuf reflection;
- implement serde JSON DTO paths;
- implement prost DTO materialization paths;
- implement all possible adapters in core schema output;
- require every schema crate to compile every adapter;
- change MDB, cache DB, lookup DB, serving, or SDK policy;
- claim performance parity before benchmark evidence.

## 5. Measured object

Measured row-format public paths:

```text
MBT bytes
  -> checked schema access
  -> generated direct writer
  -> JSON/protobuf/CSV bytes
```

Measured row-format trusted paths:

```text
MBT bytes previously accepted by checked access
  -> trusted schema access
  -> generated direct writer
  -> JSON/protobuf/CSV bytes
```

Measured columnar public paths:

```text
MBT bytes
  -> checked schema access
  -> hidden generated transponding
  -> selected adapter writer
  -> Arrow RecordBatch / Arrow IPC bytes / Parquet bytes
```

Measured columnar trusted paths:

```text
MBT bytes previously accepted by checked access
  -> trusted schema access
  -> hidden generated transponding
  -> selected adapter writer
  -> Arrow RecordBatch / Arrow IPC bytes / Parquet bytes
```

The first benchmark schemas are:

```text
crates/schemas/bars_core
crates/schemas/test_compatibility_core
```

Bars is required for parity with old MBT result labels. Test compatibility is
required to prove scalar, bytes, raw string, arrays, nullable arrays, and
projection shapes are not Bars-specific.

## 6. Schema source contract

The source of truth remains:

```text
.proto files + proto/mathilde/options.proto
```

The schema root and module remain explicit codegen inputs.

No metamorphose or transponding behavior may be inferred from Rust source names,
benchmark labels, or handwritten schema-specific branches.

Projection order remains:

```text
full MBT
  -> optional MBT-to-MBT projection
  -> metamorphose selected schema marker
```

Generated adapter modules must work for both source schemas and projected
schemas emitted by the projection surface.

## 7. Wire and archive contract

Metamorphose must not change MBT bytes.

For `metamorphose::mbt::<S>`:

```text
checked access validates requested schema
response is borrowed input bytes
```

For JSON, protobuf, CSV, Arrow, Arrow IPC, and Parquet:

```text
source MBT envelope and archive are checked or trusted according to the called path
format output is a new boundary artifact
source MBT bytes remain unchanged
```

Response-cap semantics:

- row byte formats count their output bytes;
- Arrow RecordBatch counts the generated physical columnar batch byte estimate
  plus adapter-owned output buffer sizes where applicable;
- Arrow IPC and Parquet count final output bytes;
- overflow must return `TransportError::ResponseTooLarge`;
- the error must report the observed size and cap.

## 8. Checked and trusted access contract

Checked public functions must validate:

- header identity;
- payload length;
- payload checksum;
- archive shape;
- row validity according to generated schema validation.

Trusted public functions must be visibly unsafe and must state this safety
contract:

```text
The caller guarantees the bytes were previously accepted by checked MBT access
for this schema and then stored or transported without mutation.
```

Generated trusted metamorphose functions may skip full source checksum and
archive validation only by calling the existing generated trusted access path.

No adapter may invent a separate trusted-access contract.

The public trusted helpers in `crates/metamorphose` must route to generated
schema trait methods. They must not call the checked public methods and must not
revalidate the source envelope before conversion.

Row-format trusted support is required for JSON, protobuf, and CSV. CSV trusted
support is required even if the old experiment exposed only a checked CSV public
function, because the new workspace must keep the trusted API shape consistent
across boundary row formats.

## 9. Codegen contract

Current codegen surfaces:

```text
core
projection
```

This spec adds adapter codegen surfaces without changing the meaning of the
existing surfaces.

Required new surface spelling:

```text
--surface metamorphose
```

Required adapter selector:

```text
--adapter json
--adapter protobuf
--adapter csv
--adapter transponding
--adapter arrow
--adapter arrow-ipc
--adapter parquet
```

`--surface metamorphose` must require exactly one `--adapter`.

Output policy:

```text
--surface core        -> one generated core schema file
--surface projection  -> one generated core+projection schema file
--surface metamorphose --adapter X -> one generated schema-local adapter module
```

The generated schema-local adapter module file names are:

```text
src/{module}_json.rs
src/{module}_protobuf.rs
src/{module}_csv.rs
src/{module}_transponding.rs
src/{module}_arrow.rs
src/{module}_arrow_ipc.rs
src/{module}_parquet.rs
```

Generated adapter modules must not be hand edited.

Generated row-format adapter modules must:

- implement the format-specific trait for the generated schema marker;
- call the generated schema checked or trusted accessors;
- write directly from archived rows;
- avoid serde DTOs;
- avoid prost DTO materialization;
- avoid `Vec<Row>` materialization;
- avoid per-field static name allocation;
- avoid `.to_string()`, `.to_vec()`, `.collect()`, and `format!` in row loops
  unless the implementation plan proves the call is outside the hot path.

Generated row-format writer modules must use static field fragments:

- JSON field names are emitted as generated `&'static [u8]` fragments such as
  `b"\"close_ms\":"`;
- CSV headers are emitted as generated static fragments;
- protobuf tags are emitted from generated static or constant tag metadata;
- field names must not be escaped, formatted, allocated, or copied per row.

Generated row-format writers must pre-size their declared output buffer before
the row loop:

- JSON and CSV use generated lower-bound or exact capacity estimates from row
  count, static fragments, fixed-width numeric estimates, and known string/byte
  lengths already available from archived rows;
- protobuf uses generated encoded-length accounting where message length
  framing requires it;
- capacity estimation must not add a hidden full semantic pass unless the
  implementation plan proves the pass is already required by that format;
- output cap checks remain mandatory and must return
  `TransportError::ResponseTooLarge` on overflow.

Generated row-format and transponding modules must use automatic private chunks
for wide schemas:

```text
GENERATED_FIELD_CHUNK_SIZE = 32
```

If a generated row-format adapter writer or transponding writer would emit more
than 32 field write/access statements in one function, codegen must split it
into private generated chunk helpers. Chunking is internal to the generated
module. Users must never call chunk helpers manually.

Required private chunk naming pattern:

```text
write_json_fields_000_031
write_json_fields_032_063
write_csv_fields_000_031
write_protobuf_fields_000_031
transpond_fields_000_031
```

The implementation plan may bind exact suffix formatting, but it may not change
the 32-field threshold without a new audited spec.

Generated transponding modules must:

- define schema-specific column batch types;
- define `pub(crate)` transponding helpers only;
- allocate declared column buffers once from row count;
- not expose a public `transpond` user API;
- not sort, group, or reorder rows;
- keep dictionary fields ordinal/mask physical values unless a later adapter
  spec proves expansion is required.

Generated Arrow, Arrow IPC, and Parquet modules must:

- call the hidden generated transponding helper automatically;
- not duplicate transponding code;
- not expose transponding as public API;
- not build row DTOs before columnar writing.

Columnar adapter modules must perform exactly one hidden transponding pass for a
requested output. They must not materialize a row DTO vector and must not run a
second row-to-column pass in the adapter.

Generated modules must compile only when their schema crate feature is enabled.

Schema crate `src/lib.rs` must gate modules like:

```rust
pub mod bars_v1;

#[cfg(feature = "json")]
mod bars_v1_json;

#[cfg(feature = "protobuf")]
mod bars_v1_protobuf;

#[cfg(feature = "csv")]
mod bars_v1_csv;

#[cfg(any(feature = "arrow", feature = "arrow_ipc", feature = "parquet"))]
mod bars_v1_transponding;

#[cfg(feature = "arrow")]
mod bars_v1_arrow;

#[cfg(feature = "arrow_ipc")]
mod bars_v1_arrow_ipc;

#[cfg(feature = "parquet")]
mod bars_v1_parquet;
```

The exact module name must match the generated module argument.

## 10. Crate boundary contract

`crates/core` remains adapter-free.

`crates/core` owns dependency-free checked output helpers used by boundary
adapters:

```text
crates/core/src/output.rs
```

This module may contain only schema-agnostic byte-output support:

- `CheckedBytes`;
- response-cap checked length helpers;
- protobuf wire length helpers;
- UTC byte formatting helpers;
- base64 byte writing helpers.

It must not depend on `metamorphose`, adapter crates, generated schemas, Arrow,
Parquet, prost DTOs, serde, or any format-specific crate.

`crates/projection` remains boundary-format-free.

`crates/metamorphose` owns only:

- public traits;
- public helper functions;
- `MetamorphoseFormat`;
- `MetamorphoseOutput`.

`crates/metamorphose` must not own shared byte-output helpers. Those helpers
belong to `crates/core/src/output.rs` so adapters do not depend on
`metamorphose`.

`crates/metamorphose` must not contain concrete writer modules:

```text
crates/metamorphose/src/json.rs
crates/metamorphose/src/protobuf.rs
crates/metamorphose/src/csv.rs
```

`crates/transponding` owns shared column buffer contracts and size accounting.
It must not depend on Arrow, Arrow IPC, or Parquet.

`crates/adapters/json` owns JSON writer helpers only.

`crates/adapters/protobuf` owns protobuf wire writer helpers only.

`crates/adapters/csv` owns CSV writer helpers only.

`crates/adapters/arrow` owns Arrow RecordBatch construction helpers only.

`crates/adapters/arrow_ipc` owns Arrow IPC stream helpers only.

`crates/adapters/parquet` owns Parquet byte writer helpers only.

Schema crates may add optional feature-gated dependencies on:

- `metamorphic_binary_transport_metamorphose`;
- the selected adapter crate;
- `metamorphic_binary_transport_transponding` for columnar formats only.

This locally supersedes the core-generation phase rule that schema crates do not
depend on adapters, but only for explicitly enabled adapter features. Default
schema builds must remain adapter-free.

## 11. Dependency contract

No new dependency may enter:

- `crates/core`, except no-external-dependency module additions explicitly
  bound by this spec;
- `crates/projection`;
- default schema crate dependency sets.

Allowed dependencies by crate:

| Crate | Allowed dependency class |
| --- | --- |
| `crates/core` | no new external dependency for output helpers |
| `crates/metamorphose` | core only, for error/result types and trait helper dispatch |
| `crates/transponding` | core only, if required for error/result types |
| `crates/adapters/json` | core only; no dependency on `metamorphose`; no serde JSON dependency unless implementation plan proves need |
| `crates/adapters/protobuf` | core plus low-level protobuf wire helpers only; no dependency on `metamorphose`; no prost DTO dependency |
| `crates/adapters/csv` | core plus integer formatting only; no dependency on `metamorphose`; no CSV crate dependency unless implementation plan proves need |
| `crates/adapters/arrow` | core plus Arrow RecordBatch dependencies only; no dependency on `metamorphose` |
| `crates/adapters/arrow_ipc` | core plus Arrow IPC dependencies only; no dependency on `metamorphose` |
| `crates/adapters/parquet` | core plus Parquet writer dependencies only; no dependency on `metamorphose` |

Dependency versions must be bound in the implementation plan before code. This
spec does not authorize an unpinned dependency addition by itself. If the
implementation plan cannot bind an exact version and owner crate, that adapter
is blocked for that implementation plan.

Dependency checks must prove:

```bash
cargo tree -p metamorphic_binary_transport_schema_bars --no-default-features
cargo tree -p metamorphic_binary_transport_schema_bars --features json
cargo tree -p metamorphic_binary_transport_schema_bars --features protobuf
cargo tree -p metamorphic_binary_transport_schema_bars --features csv
cargo tree -p metamorphic_binary_transport_schema_bars --features arrow
cargo tree -p metamorphic_binary_transport_schema_bars --features arrow_ipc
cargo tree -p metamorphic_binary_transport_schema_bars --features parquet
```

Each selected feature must pull only its selected adapter family.

## 12. Determinism contract

For identical input bytes and selected schema:

- JSON output must be byte-for-byte deterministic;
- protobuf output must be byte-for-byte deterministic;
- CSV output must be byte-for-byte deterministic;
- Arrow RecordBatch semantic checksum must be deterministic;
- Arrow IPC bytes must be deterministic under the selected writer settings;
- Parquet bytes must be deterministic under the selected writer settings;
- row order must be preserved;
- projection order remains full MBT, then projected MBT, then metamorphose.

If an upstream writer embeds nondeterministic metadata, the implementation plan
must either disable it or mark the format blocked.

## 13. Failure contract

Every public metamorphose function must return `Result`.

Required failures:

- wrong schema;
- corrupt header;
- corrupt archive;
- old or unsupported schema version;
- response too large;
- unsupported field kind for selected adapter;
- invalid UTF-8 for text output;
- invalid finite numeric value if the schema validation requires finite values;
- selected adapter feature missing at compile time.

Unsupported selected format must fail at codegen or compile time, not at runtime
through a generic fallback.

## 14. Compile-surface budget

Default schema crate builds must remain close to the current core/projection
surface.

Required compile evidence commands:

```bash
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
cargo check -p metamorphic_binary_transport_schema_test_compatibility --no-default-features
cargo check -p metamorphic_binary_transport_schema_test_compatibility --features json,csv
```

The result review must record elapsed time, max RSS where available, and line
counts for generated adapter modules.

The default schema crate dependency tree must not include:

```text
serde_json
prost
arrow-array
arrow-ipc
parquet
```

unless a later audited spec changes the dependency budget.

## 15. Runtime performance budget

The performance target is parity or improvement against the old MBT
implementation for identical logical payloads and comparable benchmark lanes.

Minimum required old baseline source:

```text
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md
```

Required large-row labels for Bars:

```text
metamorphose_json
metamorphose_protobuf
metamorphose_csv_full_public
metamorphose_csv_full_archived
transponding_full_public
transponding_full_archived
metamorphose_arrow_full_public
metamorphose_arrow_full_archived
metamorphose_arrow_ipc_full_public
metamorphose_arrow_ipc_full_archived
metamorphose_parquet_full_public
metamorphose_parquet_full_archived
```

The benchmark must run at least three release runs and report median, max/min
spread, rows/sec, MB/sec, output bytes, checksum, and selected feature set.

No stable speed claim is allowed for a lane whose max/min spread exceeds 7
percent. Small-row lanes may be reported but must not be used for stable
throughput claims unless the spread rule passes.

Optimization requirements:

- row formats must allocate only the output buffer;
- row formats must pre-size the output buffer before the timed row loop;
- MBT target must return borrowed bytes;
- columnar formats may allocate declared column/output buffers only;
- columnar formats must perform one hidden generated transponding pass;
- no row DTO materialization;
- no generic reflection over fields in hot paths;
- no per-row field-name formatting or escaping;
- no manually invoked public chunk API;
- no public transponding dispatch;
- no hidden projection inside metamorphose.

## 16. Correctness oracle

Correctness oracles:

- MBT target: borrowed output equals input bytes and schema checked access
  passes;
- JSON: deterministic checksum plus field-for-field comparison against old
  metamorphose JSON for Bars where old output exists;
- protobuf: byte-for-byte or decoded field-for-field comparison against old
  metamorphose protobuf for Bars;
- CSV: deterministic checksum and row/cell count comparison against old CSV
  behavior;
- Arrow: row count, column count, schema, null counts, and deterministic
  semantic checksum;
- Arrow IPC: readback row count, column count, schema, and semantic checksum;
- Parquet: readback row count, column count, schema, and semantic checksum;
- projections: projected MBT checked access must pass before adapter conversion.

Test compatibility schema must prove non-Bars field kinds are either supported
or rejected by codegen with an explicit error.

## 17. Benchmark methodology

Benchmark implementation path:

```text
crates/benches/src/metamorphose.rs
crates/benches/src/bin/mbt_metamorphose_bench.rs
```

Benchmark command:

```bash
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_metamorphose_bench -- --report-dir docs/evidence/mbt_metamorphose_migration
```

The benchmark must:

- parse old MBT baseline labels from old `bench_results.md`;
- validate old required labels are present before running;
- measure public checked path separately from trusted path;
- measure setup outside the timed conversion loop;
- write one JSON run file per execution;
- refuse to overwrite existing run files;
- write a summary only after at least three runs are present;
- record benchmark environment in a companion metadata artifact.

Allowed benchmark helper direct-row baselines:

- direct JSON writer baseline;
- direct protobuf writer baseline;
- direct CSV writer baseline;
- direct Arrow/Parquet row baseline only for comparison.

Direct baselines are not production API.

## 18. Test plan

Required tests:

```bash
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
```

Source-level hot-path guard tests must reject generated adapter hot-path bodies
containing:

```text
Vec::with_capacity(archived.rows.len()) for row DTO materialization
.to_string()
.to_vec()
.collect()
serde_json
prost::Message
format!(
```

Exceptions require exact implementation-plan justification and must prove the
call is outside the row loop or not on the benchmarked hot path.

Additional generated-source guard tests must verify:

- JSON field names are emitted through generated static field fragments;
- CSV headers are generated static fragments;
- protobuf tags are generated static or constant metadata;
- trusted helpers call generated trusted access paths, not checked access paths;
- no generated adapter exposes `pub fn transpond` or a public transponding trait;
- no generated hot-path helper contains more than 32 field write/access
  statements;
- chunk helpers are private and schema-local;
- columnar adapters call exactly one generated transponding helper before
  adapter writing.

## 19. Code bindings

Allowed implementation files:

```text
crates/metamorphose/Cargo.toml
crates/metamorphose/src/lib.rs
crates/metamorphose/src/runtime.rs
crates/metamorphose/src/tests/mod.rs
crates/core/src/lib.rs
crates/core/src/output.rs
crates/core/src/tests/test_output.rs
crates/transponding/Cargo.toml
crates/transponding/src/lib.rs
crates/transponding/src/runtime.rs
crates/transponding/src/tests/mod.rs
crates/adapters/json/Cargo.toml
crates/adapters/json/src/lib.rs
crates/adapters/json/src/tests/mod.rs
crates/adapters/protobuf/Cargo.toml
crates/adapters/protobuf/src/lib.rs
crates/adapters/protobuf/src/tests/mod.rs
crates/adapters/csv/Cargo.toml
crates/adapters/csv/src/lib.rs
crates/adapters/csv/src/tests/mod.rs
crates/adapters/arrow/Cargo.toml
crates/adapters/arrow/src/lib.rs
crates/adapters/arrow/src/tests/mod.rs
crates/adapters/arrow_ipc/Cargo.toml
crates/adapters/arrow_ipc/src/lib.rs
crates/adapters/arrow_ipc/src/tests/mod.rs
crates/adapters/parquet/Cargo.toml
crates/adapters/parquet/src/lib.rs
crates/adapters/parquet/src/tests/mod.rs
crates/codegen/src/config.rs
crates/codegen/src/emit.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/model.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
crates/codegen/src/tests/test_rust_emit_transponding.rs
crates/schemas/bars_core/Cargo.toml
crates/schemas/bars_core/src/lib.rs
crates/schemas/bars_core/tests/test_metamorphose.rs
crates/schemas/bars_core/tests/test_transponding_hidden.rs
crates/schemas/test_compatibility_core/Cargo.toml
crates/schemas/test_compatibility_core/src/lib.rs
crates/schemas/test_compatibility_core/tests/test_metamorphose.rs
crates/schemas/test_compatibility_core/tests/test_transponding_hidden.rs
crates/benches/Cargo.toml
crates/benches/src/lib.rs
crates/benches/src/metamorphose.rs
crates/benches/src/bin/mbt_metamorphose_bench.rs
crates/benches/src/tests/test_metamorphose_bench_output.rs
```

Forbidden implementation files in this spec:

```text
crates/projection/src/*
proto/mathilde/options.proto
```

No projection/proto edit is allowed by this spec. Core edits are limited to
adding `crates/core/src/output.rs`, exporting it from `crates/core/src/lib.rs`,
and testing it through `crates/core/src/tests/test_output.rs`.

## 20. Generated artifact bindings

Required generated adapter files for Bars:

```text
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
crates/schemas/bars_core/src/bars_v1_transponding.rs
crates/schemas/bars_core/src/bars_v1_arrow.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
```

Required generated adapter files for test compatibility:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs
```

Each file has exactly one owner:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --surface metamorphose --adapter <adapter> ...
```

Example Bars JSON command:

```bash
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs
```

Example check command:

```bash
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs
```

The implementation plan must list every adapter generation command for both
schemas.

## 21. Review artifact bindings

Required review artifacts:

```text
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_research_brief.md
docs/specs/mbt_metamorphose_migration_SPEC.md
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_peer_audit.md
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_result_review.md
```

Required evidence directory:

```text
docs/evidence/mbt_metamorphose_migration/
```

Required evidence files after implementation:

```text
docs/evidence/mbt_metamorphose_migration/benchmark_environment.md
docs/evidence/mbt_metamorphose_migration/dependency_trees.md
docs/evidence/mbt_metamorphose_migration/compile_surface.md
docs/evidence/mbt_metamorphose_migration/metamorphose_run_1.json
docs/evidence/mbt_metamorphose_migration/metamorphose_run_2.json
docs/evidence/mbt_metamorphose_migration/metamorphose_run_3.json
docs/evidence/mbt_metamorphose_migration/metamorphose_summary.md
```

## 22. Implementation plan requirement

The implementation plan must bind:

- exact dependency versions;
- exact generated file commands for every adapter and schema;
- exact feature names in each schema crate;
- exact public trait names in `crates/metamorphose`;
- exact neutral output helper APIs in `crates/core/src/output.rs`;
- exact writer helper APIs in each adapter crate;
- exact trusted helper names and safety documentation;
- exact output pre-sizing helper APIs;
- exact static field fragment representation;
- exact private chunk helper names and 32-field enforcement tests;
- exact hidden transponding function names;
- exact tests for no public transponding API;
- exact benchmark labels;
- exact old baseline parser behavior;
- exact rollback boundary.

The plan must not introduce implementation decisions absent from this spec.

## 23. Approval checklist

Pre-audit closure checklist:

- mandatory section order matches `docs/protocols/spec_protocol.md`;
- prior specs searched: workspace architecture, codegen migration, schema core
  generation, projection migration, projection direct writer;
- prior conflict resolved: schema crates remain adapter-free by default, but
  adapter features locally permit selected optional adapter dependencies;
- command surfaces are exact for `--surface metamorphose --adapter <adapter>`;
- generated artifact ownership is one codegen command per generated adapter
  file;
- runtime dispatch path is bound: public `metamorphose::format::<S>` calls
  schema-specific generated trait impls;
- trusted dispatch path is bound: public unsafe
  `metamorphose::format_trusted_unchecked::<S>` calls generated trusted trait
  impls and generated trusted access;
- columnar dispatch path is bound: generated format impl calls hidden generated
  transponding helper automatically;
- row writer hot path is bound: static field fragments, output pre-sizing, and
  private automatic 32-field chunking;
- tests that could encode old monolithic behavior are migrated to feature-gated
  adapter modules;
- exact code paths are listed under code bindings;
- compile-surface evidence commands are defined;
- performance benchmark methodology is defined;
- no design decision is deferred to implementation planning except exact
  dependency versions and exact implementation ordering.

Approval status:

- research brief: written;
- spec: peer audited;
- peer audit: passed in
  `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_peer_audit.md`;
- implementation plan: required after peer audit;
- code changes: forbidden until implementation plan approval.

## 24. Open questions

No design question blocks peer audit.

Implementation planning must still decide whether to implement all formats in
one code pass or split row formats and columnar formats into separate approved
plans. If split, each implementation plan must remain consistent with this
spec and must not expose transponding as a public API.
