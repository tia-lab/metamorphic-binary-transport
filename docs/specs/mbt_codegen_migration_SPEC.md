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

# SPEC: MBT Codegen Migration

## 1. Identification

Slug: `mbt_codegen_migration`

Repository: `/home/tia/_DEV/MATHILDE/metamorphic-binary-transport`

Task class: spec authoring.

Research brief:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_research_brief.md
```

Source experiment crate:

```text
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport
```

Target crate:

```text
crates/codegen
```

## 2. Status

Status: `DRAFT_AWAITING_PEER_AUDIT_V3`

This spec does not authorize implementation.

Implementation may start only after:

1. this spec passes a separate peer audit;
2. an implementation plan binds exact edits and validation commands;
3. the implementation plan is explicitly approved.

## 3. Purpose

Migrate the MBT schema code generator from the experiment crate into:

```text
crates/codegen
```

The migration must preserve the successful architecture:

```text
.proto + mathilde/options.proto
  -> descriptor model
  -> deterministic schema model
  -> deterministic generated Rust
```

The migration must not preserve the experiment monolith. The first generated
surface is core MBT binary transport only. Boundary adapters and columnar
surfaces are separate crate work and remain out of scope.

The production generator must support external schema locations. It must not
depend on hardcoded Bars, Primitives, compatibility, or benchmark schema lists.

## 4. Non-goals

This spec does not:

- commit generated production schema files;
- generate a Bars schema crate;
- generate Primitives or compatibility schemas;
- generate projection code;
- generate JSON, protobuf, CSV, Arrow, Arrow IPC, or Parquet adapters;
- generate transponding column batches;
- generate prost DTO sidecar modules;
- migrate MDB, cache, lookup, or MLDB annotations;
- run transport throughput benchmarks;
- claim runtime performance parity;
- redesign the core envelope;
- change `crates/core`;
- change `crates/projection`, `crates/metamorphose`, `crates/transponding`,
  adapter crates, or benchmark crate logic.

## 5. Measured Object

The measured object is codegen migration and isolation:

```text
experiment src/codegen/*
experiment src/bin/mbt_codegen.rs
experiment mathilde/options.proto MBT transport options
  -> crates/codegen/src/*
  -> proto/mathilde/options.proto
```

Measured by this spec:

- descriptor loading succeeds for fixture schemas;
- MBT option definitions are available from `proto/mathilde/options.proto`;
- unsupported schema shapes are rejected before runtime;
- generated Rust source is deterministic;
- generated source references `metamorphic_binary_transport_core`;
- generated source excludes adapter/transponding/projection surfaces;
- codegen dependencies remain inside `crates/codegen`;
- core dependency graph remains unchanged.

Not measured by this spec:

- full schema runtime encode/access throughput;
- adapter throughput;
- projection throughput;
- transponding throughput;
- wide production schema compile time.

## 6. Schema Source Contract

The schema source of truth is:

```text
.proto files + proto/mathilde/options.proto
```

Codegen input must be explicit:

```text
proto root directories
root proto file
fully qualified root message
generated Rust module name
selected output surface
```

The first selected output surface is:

```text
core
```

`core` means MBT binary encode/access/inspect only.

Allowed schema source locations:

```text
inside this repository
external schema repository
application crate proto directory
shared schema crate proto directory
```

Forbidden:

- discover root messages by scanning Rust code;
- infer schema intent from generated file names;
- hardcode Bars or Primitives as canonical requests;
- require schemas to live inside this repository;
- require a Rust dependency from a schema folder.

MBT option file contract:

```text
proto/mathilde/options.proto
```

The file must be MBT-only in this migration. It must not contain MDB, cache,
lookup, MLDB, Postgres, SQLite, or serving options.

Exact option file contents:

```proto
syntax = "proto2";

package mathilde;

import "google/protobuf/descriptor.proto";

message DictionaryAlias {
  required string value = 1;
  repeated string alias = 2;
}

message Dictionary {
  required string name = 1;
  repeated string value = 2;
  repeated DictionaryAlias alias = 3;
}

message ProjectionDefinition {
  required string name = 1;
  required string rust_marker = 2;
  repeated string include_group = 3;
  repeated string exclude_group = 4;
  repeated string include_field = 5;
  repeated string exclude_field = 6;
}

extend google.protobuf.FileOptions {
  repeated Dictionary dictionary_values = 50001;
}

extend google.protobuf.MessageOptions {
  optional uint32 schema_id = 50101;
  optional uint32 schema_version = 50102;
  optional string transport_name = 50103;
  optional bool payload_root = 50104;
  repeated ProjectionDefinition projection = 50105;
}

extend google.protobuf.FieldOptions {
  optional string dictionary = 50201;
  optional string bitmask_dictionary = 50202;
  optional uint32 presence_bit = 50203;
  optional bool key_part = 50204;
  optional uint32 key_order = 50205;
  optional uint32 const_u16 = 50206;
  optional bool repeated_payload = 50207;
  optional bool raw_string = 50208;
  optional bool ignored = 50209;
  optional string projection_group = 50210;
  optional string derived_utc_from = 50211;
}
```

Dictionary alias behavior in this migration:

- `Dictionary.alias` is accepted as valid MBT option syntax;
- aliases do not affect dictionary ordinal assignment;
- aliases do not affect generated archive fields;
- aliases do not affect core schema hash;
- aliases are not emitted as core-surface helper APIs;
- alias lookup behavior is deferred to a later adapter or application-schema
  spec.

Projection option behavior in this migration:

- `projection` and `projection_group` remain in the option file because they
  are MBT transport concepts;
- `--surface core` must parse these extensions only enough to recognize them
  as valid MBT options;
- `--surface core` must not construct projection output schemas;
- projection definitions and projection groups do not affect the core schema
  hash;
- projection definitions and projection groups do not affect generated core
  output;
- projection emission is deferred to a later projection codegen spec.

## 7. Wire and Archive Contract

Generated core schema output must use the existing `crates/core` contract:

```rust
metamorphic_binary_transport_core::envelope::SchemaHeaderSpec
metamorphic_binary_transport_core::envelope::TransportHeader
metamorphic_binary_transport_core::envelope::encode_header
metamorphic_binary_transport_core::envelope::decode_header
metamorphic_binary_transport_core::envelope::validate_header_for_schema
metamorphic_binary_transport_core::envelope::trusted_payload_for_schema
metamorphic_binary_transport_core::envelope::HEADER_LEN
metamorphic_binary_transport_core::envelope::fnv1a64
metamorphic_binary_transport_core::runtime::{BinaryInspection, MbtSchema}
metamorphic_binary_transport_core::error::{Result, TransportError}
```

Generated schema output must own schema-specific constants:

```rust
SCHEMA_ID
SCHEMA_VERSION
SCHEMA_VERSION_VALUE
TRANSPORT_NAME
GENERATED_SCHEMA_HASH
SCHEMA_HEADER
```

Generated schema output must use rkyv archive types for payload and rows.

Generated checked access must:

1. decode header;
2. validate schema identity, payload length, and payload checksum through core;
3. access the rkyv archive through checked rkyv access;
4. validate row count against header row count;
5. expose an archived view without materializing owned rows.

Generated trusted access must:

1. call `trusted_payload_for_schema`;
2. access rkyv bytes through the trusted unchecked archive path;
3. validate only the minimum postcondition needed to preserve the trusted
   immutable-cache contract, including row count when available;
4. be visibly unsafe and documented.

The wire envelope bytes are not changed by this spec.

## 8. Checked and Trusted Access Contract

Generated APIs must keep checked and trusted paths distinct.

Required public checked shape:

```rust
impl SchemaMarker {
    pub fn encode(rows: &[Row], max_response_bytes: usize) -> Result<Vec<u8>>;
    pub fn encode_owned(rows: Vec<Row>, max_response_bytes: usize) -> Result<Vec<u8>>;
    pub fn access(bytes: &[u8]) -> Result<View<'_>>;
    pub fn inspect(bytes: &[u8]) -> Result<BinaryInspection>;
}

impl MbtSchema for SchemaMarker { ... }
```

Required internal or explicitly documented trusted shape:

```rust
impl SchemaMarker {
    pub unsafe fn access_archived_trusted_unchecked(bytes: &[u8]) -> Result<&ArchivedPayload>;
}
```

Required generated core type surface:

```rust
pub struct RowType { ... }
pub struct PayloadType { ... }
pub struct SchemaMarker;
pub struct SchemaMarkerView<'a> { archived: &'a <PayloadType as rkyv::Archive>::Archived }
pub struct SchemaMarkerRows<'a> { inner: core::slice::Iter<'a, <RowType as rkyv::Archive>::Archived> }
pub struct ArchivedSchemaMarkerRow<'a> { row: &'a <RowType as rkyv::Archive>::Archived }
```

`SchemaMarker`, `RowType`, and `PayloadType` above are placeholders for the
generated marker, row, and payload type names. The concrete names are derived
from the root payload message and repeated row message. The
view, row iterator, and archived row wrapper are generated for every root
schema.

Required generated view methods:

```rust
impl<'a> <Marker>View<'a> {
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn rows(&self) -> <Marker>Rows<'a>;
}
```

Required generated row iterator behavior:

```rust
impl<'a> Iterator for <Marker>Rows<'a> {
    type Item = Archived<Marker>Row<'a>;

    fn next(&mut self) -> Option<Self::Item>;
}
```

The iterator must wrap archived row references. It must not materialize owned
rows.

Required generated archived-row wrapper behavior:

```rust
impl<'a> Archived<Marker>Row<'a> {
    pub fn <field>(&self) -> <borrowed_or_native_field_type>;
    pub fn has_<field>(&self) -> bool; // only for fields with presence bits
}
```

Accessor return policy:

- scalar numeric fields return native scalar values;
- `bool` fields return `bool`;
- bytes fields return `&[u8]`;
- raw string fields return `&str`;
- array fields return `impl Iterator<Item = native_scalar> + '_`;
- dictionary fields expose `<field>_ordinal() -> u16` and, when the original
  logical field is a string dictionary, `<field>() -> Result<&'static str>`;
- bitmask dictionary fields return `u64`;
- optional fields are represented by `has_<field>()` plus the physical backing
  getter; getters do not allocate and do not materialize `Option` wrappers;
- fields marked `(mathilde.ignored) = true` do not produce archive fields or
  archived-row getters.

Presence accessor policy:

- every field with `(mathilde.presence_bit)` must generate
  `has_<field>() -> bool` on `Archived<Marker>Row`;
- schemas with one presence word must generate
  `presence_bits() -> u64`;
- schemas with more than one presence word must generate
  `presence_words() -> impl Iterator<Item = u64> + '_`;
- presence accessors must read archived presence storage directly and must not
  allocate.

Trusted access safety contract:

```text
Caller guarantees bytes were previously accepted by checked MBT encode/access
for the same schema and then stored or transported immutably without mutation.
```

Generated trusted functions must not be callable through the generic safe
`MbtSchema` trait.

## 9. Codegen Contract

The production codegen crate owns:

```text
descriptor loading
MBT option extension resolution
option validation
schema model construction
schema hash normalization
core Rust emission
deterministic formatting/checking
CLI argument parsing
```

The generator must reject before emitting Rust when:

- root message is missing;
- root message is not marked `(mathilde.payload_root) = true`;
- schema id/version/transport name are missing;
- repeated payload field is missing;
- more than one repeated payload field exists;
- repeated payload field is not a repeated message;
- a nested row field is marked repeated payload;
- unannotated string appears;
- a string mixes incompatible MBT annotations;
- a raw string mixes another physical annotation;
- required raw string has a presence bit;
- optional raw string lacks a presence bit;
- optional scalar, bytes, raw string, dictionary, or nullable array lacks a
  presence bit;
- duplicate or non-contiguous presence bits are declared;
- duplicate or non-contiguous key order is declared;
- dictionary reference is missing;
- duplicate dictionary names exist;
- bitmask dictionary has more than 64 values;
- nullable bitmask dictionary is declared;
- unsupported repeated types are declared;
- unsupported protobuf kind is declared;
- duplicate generated Rust field names are produced.

Nullable array semantics:

- repeated numeric fields without `(mathilde.presence_bit)` are required
  non-null MBT arrays;
- empty required arrays are present empty arrays;
- repeated numeric fields with `(mathilde.presence_bit)` are nullable MBT
  arrays;
- absent nullable arrays require the generated owned-row backing `Vec<T>` to
  be empty;
- present nullable arrays may be empty;
- future boundary adapters must omit absent nullable arrays, but adapter
  behavior is not emitted by this migration.

The first migrated emitter must include only the `core` output surface.

Forbidden strings or APIs in core-surface generated output:

```text
ArrowMetamorphoseSchema
ParquetMetamorphoseSchema
CsvMetamorphoseSchema
JsonWriter
ProtoWriter
RecordBatch
arrow_array
arrow_schema
crate::arrow
crate::arrow_ipc
crate::parquet
transpond
ColumnBatch
metamorphose_json
metamorphose_protobuf
metamorphose_csv
metamorphose_arrow
metamorphose_arrow_ipc
metamorphose_parquet
project_
projection_
ProjectionDefinition
Projected
```

Generated output header must be deterministic:

```text
// This file is @generated by metamorphic_binary_transport mbt_codegen.
// schema_id=<id> schema_version=<version> schema_hash=<hash>
```

The schema hash must be derived from normalized schema model content, not raw
source file bytes.

Core schema hash normal form:

- hash algorithm is FNV-1a 64-bit using offset basis `0xcbf29ce484222325`
  and prime `0x00000100000001B3`;
- input is UTF-8 normalized text built by the generator;
- input uses `\n` line endings;
- raw source comments, source file paths, source locations, dictionary aliases,
  projection definitions, projection groups, and ignored fields are excluded;
- dictionary order follows declared `Dictionary` order;
- dictionary values follow declared `Dictionary.value` order;
- physical field order follows protobuf field number order after ignored
  fields are removed.

Exact core schema hash lines:

```text
schema:<schema_id>:<schema_version>:<schema_version_value>:<transport_name>:<payload_root>:<row_type>\n
dictionary:<dictionary_name>:<value_0>:<value_1>:...:<value_n>\n
field:<proto_path>:<rust_name>:<field_kind>:<presence_bit>:<key_order>\n
```

`schema:` appears once. `dictionary:` appears once per dictionary, in declared
order. `field:` appears once per physical archived field, in physical field
order.

Hash value formatting rules:

- numeric values are decimal without separators;
- `payload_root` is `true` or `false`;
- absent `presence_bit` and `key_order` are encoded as `none`;
- present `presence_bit` and `key_order` are decimal;
- `field_kind` is one of:

```text
const_u16(<value>)
u16_dictionary(<dictionary>,required)
u16_dictionary(<dictionary>,optional)
u64_bitmask_dictionary(<dictionary>)
i32
u32
i64
f32
f64
bool
bytes
raw_string
i64_array
i32_array
u32_array
f64_array
f32_array
```

Changing any included line changes `GENERATED_SCHEMA_HASH`. Changing only
comments, option aliases, projection declarations, projection groups, ignored
fields, source file path, or source location must not change
`GENERATED_SCHEMA_HASH` for `--surface core`.

## 10. Crate Boundary Contract

Implementation-owned target:

```text
crates/codegen
proto/mathilde/options.proto
```

Crates that must not change in this migration:

```text
crates/core
crates/projection
crates/metamorphose
crates/transponding
crates/adapters/json
crates/adapters/protobuf
crates/adapters/csv
crates/adapters/arrow
crates/adapters/arrow_ipc
crates/adapters/parquet
crates/benches
```

`crates/codegen` must not depend on generated schema crates.

`crates/core` must not depend on `prost-build`, `prost-reflect`, `rkyv`,
adapter crates, or codegen.

The codegen binary name must be:

```text
mbt_codegen
```

The codegen package remains:

```text
metamorphic_binary_transport_codegen
```

## 11. Dependency Contract

Allowed production dependencies for `crates/codegen` in this spec:

```toml
prost-reflect = { version = "=0.16.4", default-features = false }
thiserror = "=2.0.17"
```

Rationale:

- experiment codegen used `prost-reflect` to inspect descriptors and custom
  options;
- `thiserror` is already accepted in core and provides explicit codegen errors.
- descriptor-set creation is performed by the `protoc` command, not by
  `prost-build`, in this core-only migration.

Forbidden production dependencies for `crates/codegen` in this spec:

```text
arrow-array
arrow-buffer
arrow-ipc
arrow-schema
parquet
serde
serde_json
prost-build
zstd
```

`rkyv` may appear only in generated source text, generated schema consumer
crates, or the mandatory temporary smoke crate. It must not be a production
dependency of `crates/codegen` in this migration.

`Cargo.lock` may change only as a Cargo-generated dependency artifact after
the approved implementation plan.

Required host tools:

```text
cargo
protoc
rustfmt
```

The result review must record:

```text
cargo --version
protoc --version
rustfmt --version
```

Generated core-schema consumer dependencies are separate from codegen
dependencies. A generated core schema crate or the temporary smoke crate must
depend on:

```toml
metamorphic_binary_transport_core = { path = "../../../crates/core" }
rkyv = { version = "=0.8.16", features = ["unaligned"] }
```

The relative `metamorphic_binary_transport_core` path above is the required
path for the mandatory smoke crate under:

```text
target/mbt-codegen-check/smoke/
```

Actual generated schema crates may use their own repository-relative path or a
published dependency after a later schema-crate spec. That does not make
`rkyv` or generated-schema dependencies production dependencies of
`crates/codegen`.

## 12. Determinism Contract

Determinism requirements:

- descriptor input order is explicit;
- root message is explicit;
- output module name is explicit;
- dictionaries preserve declared value order;
- generated field order follows protobuf field number order;
- key parts are sorted by declared key order;
- presence bits must be contiguous from zero;
- generated source is rustfmt-normalized;
- repeated generation over the same input produces byte-identical output;
- inspect output is sorted and deterministic enough for tests.

Temporary test directories must be process-unique and deleted after use where
possible.

## 13. Failure Contract

Codegen failures must return typed errors, not panic.

Required error classes:

```text
I/O error
descriptor/protoc error
missing descriptor
missing option
invalid option
invalid schema
rustfmt failure
unsupported argument
generated diff
```

The CLI must exit non-zero and print the error text on failure.

The implementation must not use `unwrap`, `expect`, or `panic!` in codegen
library or CLI code. Test code may use local helpers only if they are outside
production paths and do not hide validation failures.

## 14. Compile-Surface Budget

This migration must prove compile-surface isolation before any runtime speed
claim.

Required compile-surface checks:

```text
cargo check -p metamorphic_binary_transport_codegen
cargo test -p metamorphic_binary_transport_codegen
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
cargo check --workspace
cargo tree -p metamorphic_binary_transport_codegen
cargo tree -p metamorphic_binary_transport_core
cargo check --manifest-path target/mbt-codegen-check/smoke/Cargo.toml
```

Required budget:

- `metamorphic_binary_transport_core` dependency tree remains unchanged from
  the core migration result review;
- codegen dependencies do not appear in `metamorphic_binary_transport_core`;
- adapter dependencies do not appear in `metamorphic_binary_transport_codegen`;
- no generated production schema is committed by this spec;
- generated fixture source line count is recorded in the result review;
- generated fixture source must exclude adapter/transponding/projection strings
  listed in section 9;
- generated fixture source must compile in the mandatory smoke crate.

No compile-time improvement claim is allowed until timings are recorded.

## 15. Runtime Performance Budget

No runtime throughput claim is made by this spec.

The generated core-only API must preserve the algorithmic shape of the
experiment core path:

```text
validate rows
archive payload
write envelope
checked access validates envelope + archive
trusted access uses trusted payload identity path
```

Runtime performance parity with the experiment MBT generated schema is deferred
until a later generated-schema migration spec commits a concrete schema and
runs the existing benchmark methodology.

## 16. Correctness Oracle

Correctness for this migration is proved by:

1. fixture proto descriptor loads successfully;
2. fixture schema model contains expected schema id/version/hash, row type,
   field kinds, dictionaries, key order, and presence bits;
3. invalid fixture schemas fail before Rust emission;
4. generated source is byte-identical across two runs;
5. generated source uses `metamorphic_binary_transport_core` imports;
6. generated source excludes forbidden adapter/projection/transponding symbols;
7. optional/wide presence behavior is represented in generated source;
8. generated output compiles against `metamorphic_binary_transport_core` and
   `rkyv` in the mandatory temporary smoke crate;
9. generated view, row iterator, archived-row wrapper, and field accessors
   match the API surface defined in section 8;
10. schema hash tests prove that comments, source path, aliases, projection
    definitions, projection groups, and ignored fields do not affect the
    `--surface core` hash.

The oracle is source and build correctness, not runtime benchmark speed.

## 17. Benchmark Methodology

No benchmark is required for this migration.

If the implementation plan chooses to record codegen execution time, it must be
reported as build/tool evidence only, not runtime performance evidence.

Required non-benchmark evidence:

- command;
- profile;
- dependency tree;
- generated fixture line count;
- generated fixture forbidden-string scan;
- deterministic generation comparison result;
- smoke crate `cargo check` result.

## 18. Test Plan

Required tests under `crates/codegen/src/tests/`:

```text
test_options.rs
test_descriptor.rs
test_model.rs
test_rust_emit_core.rs
test_cli.rs
```

Required test coverage:

- `proto/mathilde/options.proto` exposes all MBT transport extensions;
- valid generic non-Bars fixture loads and emits;
- scalar fixture covers `i32`, `u32`, `i64`, `f32`, `f64`, `bool`, `bytes`;
- raw string fixture covers required and optional raw string rules;
- array fixture covers repeated numeric arrays and nullable array presence;
- wide presence fixture covers more than 64 optional fields;
- invalid schemas reject unsupported repeated string/bytes/bool arrays;
- invalid schemas reject unannotated string;
- invalid schemas reject missing presence bit on optional values;
- invalid schemas reject duplicate/gapped presence bits;
- invalid schemas reject duplicate/gapped key order;
- invalid schemas reject nullable bitmask dictionary;
- emitter determinism test compares two generations byte-for-byte;
- forbidden-surface test scans generated core output for adapter/transponding
  and projection strings listed in section 9;
- schema hash normal-form tests cover included field changes and excluded
  comments, source paths, dictionary aliases, projections, projection groups,
  and ignored fields;
- generated API tests assert the generated view, row iterator, archived-row
  wrapper, field accessors, and presence accessors compile in the smoke crate;
- smoke compile test builds the generated fixture through
  `target/mbt-codegen-check/smoke/Cargo.toml`;
- CLI parse tests cover single schema request, missing action, duplicate
  action, invalid module name, `--write`, `--check`, and `--inspect`.

Tests must use temporary proto roots and temporary outputs. No generated
production schema output is committed by this migration.

## 19. Code Bindings

Allowed implementation files to edit:

```text
crates/codegen/Cargo.toml
crates/codegen/src/lib.rs
crates/codegen/src/main.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/model.rs
crates/codegen/src/options.rs
crates/codegen/src/rust_emit.rs
proto/mathilde/options.proto
Cargo.lock
```

Allowed implementation files to create:

```text
crates/codegen/src/config.rs
crates/codegen/src/emit.rs
crates/codegen/src/error.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_options.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/test_model.rs
crates/codegen/src/tests/test_rust_emit_core.rs
crates/codegen/src/tests/test_cli.rs
```

Implementation must not edit:

```text
crates/core/src/*
crates/projection/src/*
crates/metamorphose/src/*
crates/transponding/src/*
crates/adapters/**/*
crates/benches/**/*
```

Implementation may edit root `Cargo.toml` only if the peer-audited
implementation plan proves a workspace-level dependency entry is cleaner than
crate-local dependencies. Default policy is crate-local dependencies in
`crates/codegen/Cargo.toml`.

## 20. Generated Artifact Bindings

Committed generated artifacts:

```text
none
```

Codegen-owned source option file:

```text
proto/mathilde/options.proto
```

Temporary generated artifacts allowed only under:

```text
target/mbt-codegen-fixtures/
target/mbt-codegen-check/
```

Temporary artifacts must not be checked into git.

The codegen CLI must support these exact actions:

```text
--inspect
--write
--check
```

Required common arguments for every action:

```text
--proto-root <path>
--schema <relative.proto>
--root <fully.qualified.Message>
--module <snake_case_module>
--surface core
```

Required arguments for `--write` and `--check`:

```text
--out <path>
```

`--proto-root` is repeatable. Proto roots are passed to `protoc` in CLI order.
When two roots contain the same import path, the first root wins because it is
passed first to `protoc`. The implementation must reject an empty root list.

Exact inspect command shape:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --inspect \
  --proto-root <path> \
  --schema <relative.proto> \
  --root <fully.qualified.Message> \
  --module <snake_case_module> \
  --surface core
```

Exact write command shape:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --write \
  --proto-root <path> \
  --schema <relative.proto> \
  --root <fully.qualified.Message> \
  --module <snake_case_module> \
  --surface core \
  --out <path>
```

Exact check command shape:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --check \
  --proto-root <path> \
  --schema <relative.proto> \
  --root <fully.qualified.Message> \
  --module <snake_case_module> \
  --surface core \
  --out <path>
```

`--check` regenerates into a temporary file under `target/mbt-codegen-check/`
and compares bytes with `--out`. It must fail with `generated diff` if the
bytes differ.

Mandatory generated smoke crate:

```text
target/mbt-codegen-check/smoke/Cargo.toml
target/mbt-codegen-check/smoke/src/lib.rs
target/mbt-codegen-check/smoke/src/generated_fixture.rs
```

The smoke crate must compile the generated fixture with:

```text
cargo check --manifest-path target/mbt-codegen-check/smoke/Cargo.toml
```

The implementation plan may define whether the smoke crate is created by the
test harness or by a codegen helper, but it may not omit the smoke compile.

## 21. Review Artifact Bindings

Research brief:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_research_brief.md
```

Spec:

```text
docs/specs/mbt_codegen_migration_SPEC.md
```

Required peer audit path:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_peer_audit.md
```

Required implementation plan path after peer audit passes:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_implementation_plan.md
```

Required next peer audit path for this amendment:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_peer_audit_v3.md
```

Required result review path after implementation:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_result_review.md
```

## 22. Implementation Plan Requirement

The implementation plan must bind:

- exact dependency additions and versions;
- exact CLI validation behavior for the CLI spelling defined in this spec;
- exact fixture proto contents or fixture writer helpers;
- exact temporary directories;
- exact validation commands;
- rollback boundary;
- known risks.

The implementation plan must also list every old experiment codegen function
that is migrated, changed, or intentionally rejected.

## 23. Approval Checklist

This spec is not implementation-ready until all are true:

- required reads complete;
- research brief exists;
- peer audit exists and passes;
- MBT option surface is exact;
- generated output surface is exact;
- forbidden adapter/projection/transponding surfaces are listed;
- dependency contract is exact;
- compile-surface budget is exact;
- code bindings are exact;
- generated artifact bindings are exact;
- implementation plan is written and approved.

Current state:

```text
required reads: complete for draft
research brief: complete
peer audit: blocked; v3 required after this amendment
implementation plan: missing
implementation approval: missing
```

## 24. Open Questions

1. The implementation plan must decide whether rustfmt failures include the
   unformatted source excerpt in errors or only stderr. Either is acceptable if
   deterministic failure is explicit.
