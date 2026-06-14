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

# SPEC: MBT Projection Migration

## 1. Identification

Slug: `mbt_projection_migration`

Repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Task class: spec authoring.

Research brief:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_research_brief.md
```

Blocking audits addressed by this amendment:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v2.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v3.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v4.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v5.md
```

Primary schema fixture:

```text
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

Generated primary fixture:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

## 2. Status

Status: `DRAFT_AWAITING_PEER_AUDIT_V6`

This spec does not authorize implementation.

Implementation may start only after:

1. this amended spec passes a separate peer audit;
2. an implementation plan binds exact edits, generated artifacts, tests, and
   validation commands;
3. the implementation plan is explicitly approved.

## 3. Purpose

Migrate the proven experiment MBT-to-MBT projection behavior into the split
`metamorphic-binary-transport` workspace.

Projection derives narrower MBT payloads from one canonical MBT payload without
writing separate DTOs and without crossing into JSON, protobuf, CSV, Arrow,
Arrow IPC, Parquet, or transponding.

The intended chain is:

```text
source .proto + mathilde.projection declarations
  -> codegen projection models
  -> generated source schema + generated projected schemas
  -> source MBT bytes
  -> projected MBT bytes
  -> projected schema checked/trusted access
```

This phase restores only the generated MBT-to-MBT projection layer.
Projection generation is opt-in through the new `--surface projection` codegen
surface. The existing `--surface core` surface remains core-only.

## 4. Non-goals

This spec does not:

- change the MBT envelope layout;
- change checked access semantics;
- change trusted access semantics;
- change source schema wire bytes;
- add new protobuf options;
- hand-write generated projection artifacts;
- implement JSON, protobuf, CSV, Arrow, Arrow IPC, Parquet, or transponding
  projection behavior;
- add MDB, cache, lookup, MLDB, SQLite, Postgres, or service integration;
- benchmark runtime throughput;
- claim performance parity with the old experiment crate before running bound
  benchmark evidence;
- redesign workspace architecture;
- require schema crates to depend on adapter crates;
- require schema crates to depend on `metamorphic_binary_transport_projection`
  unless a later audited implementation plan proves a shared helper is needed.

## 5. Measured object

The measured object is generated MBT projection behavior:

```text
TestCompatibilityV1 source MBT bytes
  -> TestCompatibilityV1::project_no_optional(...)
  -> TestCompatibilityV1NoOptional projected MBT bytes
  -> TestCompatibilityV1NoOptional::access/inspect(...)

TestCompatibilityV1 source MBT bytes
  -> TestCompatibilityV1::project_numeric_only(...)
  -> TestCompatibilityV1NumericOnly projected MBT bytes
  -> TestCompatibilityV1NumericOnly::access/inspect(...)
```

The first fixture projections are declared in:

```text
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

Projection declarations:

```text
name = "no_optional"
rust_marker = "TestCompatibilityV1NoOptional"
exclude_group = "optional"

name = "numeric_only"
rust_marker = "TestCompatibilityV1NumericOnly"
include_field = "schema_version"
include_field = "tenant"
include_field = "entity"
include_field = "close_ms"
include_field = "required_i64"
include_field = "optional_i64"
include_field = "required_i32"
include_field = "optional_i32"
include_field = "required_u32"
include_field = "optional_u32"
include_field = "required_f64"
include_field = "optional_f64"
include_field = "required_f32"
include_field = "optional_f32"
include_field = "required_i64_array"
include_field = "nullable_i64_array"
include_field = "required_i32_array"
include_field = "nullable_i32_array"
include_field = "required_u32_array"
include_field = "nullable_u32_array"
include_field = "required_f64_array"
include_field = "nullable_f64_array"
include_field = "required_f32_array"
include_field = "nullable_f32_array"
```

Measured by this spec:

- projection options are parsed from the source root message;
- projected models are generated from the source model;
- projected schemas have distinct marker types and schema hashes;
- projected rows contain exactly the selected physical fields plus mandatory
  schema/key fields;
- projected bytes validate only under the projected marker schema;
- checked and trusted projection outputs are byte-identical for immutable
  validated source bytes;
- generated code is reproducible by codegen.

Not measured by this spec:

- external application schemas;
- production Primitives wide-row compile time;
- adapter conversion speed;
- storage or service throughput.

## 6. Schema source contract

The schema source of truth is `.proto + mathilde/options.proto`.

The projection migration must use the existing option file:

```text
proto/mathilde/options.proto
```

No new option file is allowed in this phase.

Projection declarations are read from the root payload message option:

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

Field projection groups are read from physical row field options:

```proto
[(mathilde.projection_group) = "..."]
```

The primary fixture source is:

```text
package = mathilde.binary_transport.test_compatibility.v1
root = mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1
module = test_compatibility_v1
schema = mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

Fixture schema identity:

```text
schema_id = 40001
schema_version = 1
transport_name = "mathilde.test_compatibility.v1"
payload_root = true
```

Fixture mandatory key fields:

```text
schema_version
tenant
entity
close_ms
```

Projection does not define finality, closed-bar, watermark, hole, repair, or
safe-serving semantics. Those are application data contracts and remain outside
MBT.

## 7. Wire and archive contract

Every projected schema is a normal generated MBT schema.

Projected schemas inherit from the source schema:

```text
schema_id
schema_version
schema_version_value
dictionaries used by selected fields
row payload field name
```

Projected schemas derive:

```text
transport_name = "{source_transport_name}.{projection_name}"
marker_type = projection rust_marker
row_type = source row type + projection suffix
payload_type = source payload type + projection suffix
view_type = projection marker + "View"
rows_iter_type = projection marker + "Rows"
archived_row_type = "Archived" + projected row type
schema_hash = projected hash defined in Section 9
```

Projection suffix is derived by removing the source marker prefix from the
projection marker when it is present. Example:

```text
source marker = TestCompatibilityV1
projection marker = TestCompatibilityV1NumericOnly
suffix = NumericOnly
source row = TestCompatibilityRowV1
projected row = TestCompatibilityRowV1NumericOnly
```

Projected bytes must use the projected schema hash in the MBT header.

Wrong-marker behavior:

- source bytes accessed by a projected marker must fail;
- projected bytes accessed by the source marker must fail;
- projection A bytes accessed by projection B marker must fail.

The expected failure class is schema header mismatch through the core envelope
validation path.

Projection must not change source MBT wire bytes or source schema hash.

## 8. Checked and trusted access contract

For each projection named `projection_name`, codegen must emit on the source
marker:

```rust
pub fn project_<projection_name>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<Vec<u8>>;

pub unsafe fn project_<projection_name>_trusted_unchecked(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<Vec<u8>>;
```

Checked public projection must:

1. validate source bytes through the source checked archived accessor;
2. project from the archived source payload;
3. encode projected owned rows through the projected marker `encode_owned`.

Trusted projection must:

1. use the source trusted archived accessor;
2. still validate the archived payload shape against header row count as the
   existing trusted access method does;
3. project from the archived source payload;
4. encode projected owned rows through the projected marker `encode_owned`.

The trusted projection function must remain `unsafe`. Its safety contract is
the same trusted-access contract as generated core access: the caller guarantees
that bytes were previously accepted by checked MBT access for this schema and
then stored or transported without mutation.

Codegen may emit private helper functions for archived projection. Those
helpers must not be public unless a later audited implementation plan proves a
public archived API is required.

For each projected marker, codegen must emit the same checked/trusted core
schema API already emitted for source schemas:

```rust
pub fn access(bytes: &[u8]) -> Result<ProjectedView<'_>>;
pub(crate) fn access_archived(bytes: &[u8]) -> Result<&ArchivedProjectedPayload>;
pub unsafe fn access_archived_trusted_unchecked(bytes: &[u8]) -> Result<&ArchivedProjectedPayload>;
```

## 9. Codegen contract

Projection is generated code. Generated projection artifacts must come only
from approved codegen.

### Projection surface

This spec introduces a new codegen surface:

```text
--surface projection
```

The new surface emits:

- the same generated source schema API emitted by `--surface core`;
- only the MBT-to-MBT projections declared by `mathilde.projection`;
- no boundary adapter code.

This spec does not supersede the earlier `--surface core` no-projection
contract. The following earlier clauses remain active for `--surface core`:

```text
docs/specs/mbt_codegen_migration_SPEC.md:
  --surface core must parse projection extensions only enough to recognize them
  --surface core must not construct projection output schemas
  projection definitions and projection groups do not affect generated core output

docs/specs/mbt_schema_core_generation_SPEC.md:
  generated core schema output emits no projection code
```

The projection surface is the only surface authorized by this spec to emit
declared projected schema markers and projection methods.

Generated artifact ownership for the compatibility schema:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
  -> owned by --surface projection after this migration
```

This locally supersedes the earlier schema-core reproducibility contract for
that generated file only. The old schema-core proof remains valid historical
evidence for the core-only phase, but after this migration the committed
compatibility schema module is checked with `--surface projection`.

`--surface core` remains active and core-only. It must still be tested through
codegen unit tests or temporary generated output, but it no longer owns the
committed compatibility schema artifact after this migration.

Implementation of this surface requires updating the CLI/config model and the
emit dispatch layer. That is why this spec binds `crates/codegen/src/config.rs`,
`crates/codegen/src/emit.rs`, and CLI tests in Section 19.

### Projection selection

Projection selection is deterministic:

1. Start with mandatory fields.
2. If at least one `include_group` or `include_field` is declared, add only
   fields matched by those includes.
3. If no includes are declared, add every non-ignored source physical field.
4. Apply `exclude_group` and `exclude_field`.
5. Re-add mandatory fields.
6. Preserve source physical field order in the projected row.

Mandatory fields are always included:

- source `const_u16` schema-version field;
- every field with `key_part = true`;
- every field with `key_order`.

Validation rules:

- unknown included field names fail before emission;
- unknown excluded field names fail before emission;
- unknown included group names fail before emission;
- unknown excluded group names fail before emission;
- duplicate projection names fail before emission;
- duplicate projection marker types fail before emission;
- projection names must be valid snake-case API suffixes;
- projection marker names must be valid Rust type names;
- selected fields must not create duplicate Rust field names;
- selected fields must not reference ignored fields.

Ignored fields remain non-physical and cannot appear in projected rows.

### Presence generation

Projected presence storage is rebuilt for the selected projected row:

- selected optional scalar fields receive projected presence bits in projected
  physical order;
- selected nullable arrays receive projected presence bits in projected
  physical order;
- unselected optional fields do not reserve presence bits;
- source presence bit numbers are not preserved in projected rows;
- projected presence uses the same narrow/wide presence implementation as core
  schema generation;
- there is no 64-field projection cap;
- absent nullable arrays remain absent/null in projected output;
- present empty nullable arrays remain present empty arrays in projected output.

The `no_optional` fixture projection should have no optional/nullable fields
after selection.

The `numeric_only` fixture projection should preserve optional numeric scalars
and nullable numeric arrays, with projected presence bits repacked from zero.

### Projected schema hash

Source core schema hash behavior:

- source `normalized_schema_hash` excludes `mathilde.projection`
  declarations;
- source `normalized_schema_hash` excludes `mathilde.projection_group`;
- adding, removing, or renaming projection declarations does not change source
  MBT bytes or source schema hash;
- ignored non-physical fields remain excluded from source and projection
  hashes.

Projected schema hash behavior:

Projected schema hash is computed from a deterministic projection hash text,
then hashed with the same FNV-1a 64-bit function used by core codegen.

Projection hash text ordering:

```text
projection_schema:
  source_schema_hash
  source_schema_id
  source_schema_version
  source_schema_version_value
  source_transport_name
  projection_name
  projection_rust_marker
  projected_transport_name
  projected_row_type

projection_dictionary entries:
  for each dictionary used by selected fields, in source dictionary order:
    dictionary name
    dictionary values in declared order

projection_field entries:
  for each selected physical field, in projected physical order:
    source proto_path
    logical_path
    proto_name
    projected rust_name
    field kind hash name
    dictionary name when field kind uses a dictionary
    projected presence bit or none
    projected presence word and mask when present
    key order or none
```

Unselected source fields do not affect a projected schema hash.

Changing any of the following must change the projected schema hash:

- source schema hash;
- projection name;
- projection rust marker;
- projected transport name;
- projected row type;
- selected field set;
- selected field order;
- selected field kind;
- selected dictionary value list;
- projected presence layout;
- key order of a selected field.

Changing a projection group assignment changes projected hash only when it
changes the selected field set.

### Generated API

For each projected marker, codegen must emit:

```rust
pub const SCHEMA_ID: u32;
pub const SCHEMA_VERSION: u16;
pub const SCHEMA_HASH: u64;
pub const TRANSPORT_NAME: &'static str;
pub fn header_spec() -> SchemaHeaderSpec;
pub fn encode(rows: &[ProjectedRow], max_response_bytes: usize) -> Result<Vec<u8>>;
pub fn encode_owned(rows: Vec<ProjectedRow>, max_response_bytes: usize) -> Result<Vec<u8>>;
pub fn access(bytes: &[u8]) -> Result<ProjectedView<'_>>;
pub(crate) fn access_archived(bytes: &[u8]) -> Result<&ArchivedProjectedPayload>;
pub unsafe fn access_archived_trusted_unchecked(bytes: &[u8]) -> Result<&ArchivedProjectedPayload>;
pub fn inspect(bytes: &[u8]) -> Result<BinaryInspection>;
impl MbtSchema for ProjectedMarker;
```

Generated projection APIs must not require `metamorphose`, `transponding`, or
adapter crates.

### Codegen commands

Projection inspect command:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --inspect \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface projection
```

Projection write command:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --write \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface projection \
  --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Projection check command:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --check \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface projection \
  --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Command ownership:

- `--inspect --surface projection` reports projection-surface model metadata
  for the compatibility schema and must include declared projection metadata in
  inspect output after implementation;
- `--write --surface projection` is the only command in this spec allowed to
  update `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`;
- `--check --surface projection` is the generated-code reproducibility
  contract for that committed generated file;
- `--surface core` must not claim ownership of
  `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs` after
  this migration.

Core-surface regression proof:

```text
cargo test -p metamorphic_binary_transport_codegen core_surface_ignores_projection_declarations_and_emits_no_projection_symbols
```

The required test file is:

```text
crates/codegen/src/tests/test_rust_emit_core.rs
```

The required test behavior is:

- build the compatibility schema through the core surface;
- assert source schema hash stability when projection declarations are present;
- assert generated core output does not contain projected marker names;
- assert generated core output does not contain generated projection methods;
- assert generated core output still contains the source marker, row, payload,
  view, and checked/trusted access surface.

## 10. Crate boundary contract

Projection migration must preserve split workspace boundaries.

Allowed production crates touched by a later implementation plan:

```text
crates/codegen
crates/schemas/test_compatibility_core
```

Expected not to change in this phase:

```text
proto/mathilde/options.proto
crates/core/src/*
crates/projection/src/*
crates/metamorphose/src/*
crates/transponding/src/*
crates/adapters/*
```

`crates/projection` remains a reserved ownership boundary. This spec does not
require schema crates to depend on `metamorphic_binary_transport_projection`.

If peer audit or implementation finds an unavoidable shared helper, the
implementation plan must be amended and re-approved before code changes. The
amendment must bind:

- helper signature;
- why generated local code is worse;
- why the helper does not enlarge adapter compile surfaces;
- tests proving the helper does not change wire bytes.

## 11. Dependency contract

Allowed dependencies for generated schema crates remain:

```text
metamorphic_binary_transport_core
rkyv
```

No dependency may be added to the generated compatibility schema crate unless
an amended implementation plan proves it is required and is approved before
code changes.

Forbidden dependency effects:

- no adapter crate dependency in `test_compatibility_core`;
- no `prost` runtime dependency in `test_compatibility_core`;
- no `serde`, `serde_json`, Arrow, Parquet, SQLite, Postgres, cache, lookup,
  or service dependency in `test_compatibility_core`;
- no codegen dependency in runtime schema crates.

## 12. Determinism contract

Projection output must be deterministic:

- same source bytes;
- same projection declaration;
- same response cap;
- same generated code;
- same projected bytes.

Field order in projected rows is always source physical field order after
selection. Projection declarations do not reorder fields.

Dictionary ordinal mapping is inherited from source dictionaries and must not
be re-sorted or re-numbered.

Repeated codegen runs over identical inputs must produce identical generated
Rust bytes.

## 13. Failure contract

Projection must return explicit `Result` errors.

Generation-time failures:

- duplicate projection name;
- duplicate projected marker;
- invalid projection name;
- invalid Rust marker;
- unknown include/exclude field;
- unknown include/exclude group;
- projected field selection attempts to include an ignored field;
- projected model has no schema-version field;
- projected model has no deterministic key fields when the source has keys.

Runtime failures:

- source bytes fail source header validation;
- source bytes fail source archive validation;
- projected response exceeds `max_response_bytes`;
- projected bytes fail projected validation;
- wrong marker access fails through header validation;
- trusted path is called on bytes not matching the source schema contract.

No projection path may use `unwrap`, `expect`, or `panic!`.

## 14. Compile-surface budget

This spec does not claim compile-time improvement.

The implementation plan must record compile-surface evidence:

```text
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_core
```

Required evidence interpretation:

- generated line count before and after projection generation is recorded;
- schema crate check timing and maximum resident set size are recorded;
- dependency tree proves no adapter crates enter the schema crate;
- dependency tree proves codegen is not a runtime dependency of the schema
  crate;
- no compile-time improvement is claimed unless a measured before/after result
  proves it.

Generated code constraints:

- do not derive `Debug`, `Clone`, `PartialEq`, or `RkyvDeserialize` unless an
  approved implementation plan proves a measured need;
- generate projection code only for declared projections;
- do not emit adapter code in projected schema modules;
- do not emit all possible projection surfaces for schemas without projection
  declarations.

## 15. Runtime performance budget

This spec does not make a runtime speed claim.

Runtime constraints for implementation:

- checked projection validates source bytes once;
- trusted projection must not revalidate the full source byte envelope beyond
  the existing trusted header/payload identity and archived payload shape
  checks;
- projection loops allocate only the projected owned row vector and projected
  output bytes;
- no JSON/protobuf/intermediate DTO is allowed;
- no reflection is allowed in generated projection runtime code;
- no heap allocation is allowed for static field-selection metadata at runtime.

Any speed claim requires later benchmark evidence and a result review.

## 16. Correctness oracle

The correctness oracle is field-for-field semantic equality between source rows
and projected rows for selected fields, plus byte-level equality between
checked and trusted projection outputs.

Oracle rules:

- source rows are the semantic reference for selected projected fields;
- projected bytes must validate through the projected marker schema;
- wrong-marker rejection proves schema-hash isolation;
- checked and trusted projection equality is byte-for-byte for identical
  immutable validated source bytes;
- omitted fields must not be present in projected row structs, projected view
  getters, projected checksums, or projected inspect semantics;
- field comparisons are explicit and do not require generated `PartialEq`;
- null nullable arrays are distinct from present empty nullable arrays;
- present empty nullable arrays must remain present empty after projection;
- absent nullable arrays must remain absent after projection.

The oracle does not accept row-count-only tests.

## 17. Benchmark methodology

No benchmark is required to approve this migration spec.

The implementation plan must run correctness and compile-surface checks before
any benchmark.

If a projection benchmark is added later, it is non-authoritative until
correctness evidence passes. Runtime projection performance parity with the
experiment crate requires a later benchmark/result review that binds:

- dataset;
- row count;
- payload bytes;
- checked projection timing;
- trusted projection timing;
- projected access/inspect timing;
- comparison to old experiment evidence;
- machine and build profile.

## 18. Test plan

The implementation plan must include tests proving:

1. codegen parses projection definitions from the root message;
2. core source schema hash remains unchanged when projection declarations are
   added or removed;
3. projected schemas have their own hashes;
4. projected hash changes when selected field set, field kind, dictionary
   values, projected presence layout, projection name, or projected marker
   changes;
5. generated source includes projected marker types for declared projections;
6. `project_no_optional` emits bytes accepted by
   `TestCompatibilityV1NoOptional`;
7. `project_numeric_only` emits bytes accepted by
   `TestCompatibilityV1NumericOnly`;
8. projected bytes are rejected by the source marker;
9. source bytes are rejected by each projected marker;
10. checked and trusted projection outputs are byte-identical for the same
    immutable validated source bytes;
11. selected required scalar, dictionary, raw string, bytes, numeric array, and
    nullable array fields preserve values;
12. omitted fields have no generated projected getter or row field;
13. `no_optional` emits no optional/nullable presence constants;
14. `numeric_only` repacks optional scalar and nullable array presence bits;
15. null versus present-empty nullable arrays survive projection;
16. the existing projection-ignored codegen test contract is migrated from
    "projection declarations are ignored by core output" to:
    - `--surface core` still ignores projection output and keeps source hash
      stable;
    - `--surface projection` emits projected models/output while keeping the
      source hash stable;
    - the committed compatibility generated file is projection-surface output
      after this migration;
    - the exact core-surface regression test is
      `core_surface_ignores_projection_declarations_and_emits_no_projection_symbols`
      in `crates/codegen/src/tests/test_rust_emit_core.rs`;
17. generated files are reproducible by the bound `mbt_codegen --check`
    command;
18. generated files pass formatting and clippy cleanliness required by the
    repository.

Tests must compare fields explicitly. They must not require generated row
`PartialEq`.

Minimum validation command set:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --inspect --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
cargo test -p metamorphic_binary_transport_codegen core_surface_ignores_projection_declarations_and_emits_no_projection_symbols
cargo test -p metamorphic_binary_transport_codegen
cargo test -p metamorphic_binary_transport_schema_test_compatibility
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
cargo clippy -p metamorphic_binary_transport_schema_test_compatibility --all-targets -- -D warnings
cargo check --workspace
cargo fmt --all --check
```

## 19. Code bindings

Allowed code files for a future implementation plan:

```text
crates/codegen/src/model.rs
crates/codegen/src/config.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/emit.rs
crates/codegen/src/lib.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_rust_emit_core.rs
crates/schemas/test_compatibility_core/tests/test_projection.rs
```

Generated file:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

`crates/codegen/src/lib.rs` is allowed only for documentation text that removes
or narrows the stale "core-only" crate description after adding the projection
surface. It must not add runtime behavior.

Conditionally allowed only if peer audit proves a missing option-binding
defect:

```text
crates/codegen/src/options.rs
```

Expected not to change in this phase:

```text
proto/mathilde/options.proto
crates/core/src/*
crates/projection/src/*
crates/metamorphose/src/*
crates/transponding/src/*
crates/adapters/*
```

If implementation requires any expected-not-to-change file, the implementation
plan must be amended and re-approved before code changes.

## 20. Generated artifact bindings

Codegen input files:

```text
proto/mathilde/options.proto
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

Generated Rust artifact:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Generated artifact ownership rules:

- `src/test_compatibility_v1.rs` is generated only by `mbt_codegen`;
- after this migration, `src/test_compatibility_v1.rs` is generated by
  `--surface projection`, not `--surface core`;
- this locally supersedes the schema-core generated-file reproducibility
  contract for this compatibility schema artifact;
- `--surface core` remains core-only and must be tested by
  `core_surface_ignores_projection_declarations_and_emits_no_projection_symbols`
  in `crates/codegen/src/tests/test_rust_emit_core.rs`;
- the generated file must not be hand-edited;
- the generated file must remain reproducible by the spec-level `--check`
  command;
- generated output must keep the existing generated file header.

## 21. Review artifact bindings

Research brief:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_research_brief.md
```

Spec:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Blocking peer audit:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit.md
```

Required next peer audit:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v6.md
```

Required later implementation plan:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan.md
```

Required later result review:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_result_review.md
```

## 22. Implementation plan requirement

Implementation may not begin from this spec alone.

The implementation plan must bind:

- exact code files to edit;
- execution of the spec-bound projection `--inspect`, `--write`, and
  `--check` commands;
- exact `--surface projection` parser/test changes;
- exact `emit.rs` surface dispatch changes for inspect, write, and check;
- exact implementation of
  `core_surface_ignores_projection_declarations_and_emits_no_projection_symbols`
  in `crates/codegen/src/tests/test_rust_emit_core.rs`;
- exact test file creation;
- exact validation commands;
- expected generated marker/API names;
- expected failure tests;
- compile-surface evidence commands;
- dependency tree evidence commands;
- rollback boundary;
- known risks.

The plan must explicitly state that manual generated-file edits are forbidden.

## 23. Approval checklist

### Pre-audit closure checklist

This spec satisfies the pre-audit closure gate from
`docs/protocols/spec_protocol.md` as follows:

- Mandatory section order is complete and follows
  `docs/protocols/spec_protocol.md` sections 1 through 24 exactly.
- Prior approved or active specs that may conflict have been searched:
  - `docs/specs/mbt_workspace_architecture_SPEC.md`;
  - `docs/specs/mbt_core_runtime_migration_SPEC.md`;
  - `docs/specs/mbt_codegen_migration_SPEC.md`;
  - `docs/specs/mbt_schema_core_generation_SPEC.md`.
- Prior-spec conflict resolution is explicit:
  - `--surface core` remains core-only and must not emit projection code;
  - `--surface projection` is the only surface introduced by this spec for
    declared MBT-to-MBT projection output;
  - `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`
    becomes projection-surface output after this migration;
  - that generated-file ownership locally supersedes the schema-core
    reproducibility contract for this compatibility schema artifact only;
  - the schema-core no-projection contract remains active and must be proved
    through
    `core_surface_ignores_projection_declarations_and_emits_no_projection_symbols`
    in `crates/codegen/src/tests/test_rust_emit_core.rs`.
- Exact command surface is closed:
  - the projection `mbt_codegen --inspect`, `--write`, and `--check` commands
    are defined in Section 9 and Section 18;
  - the commands use `--surface projection`;
  - `--inspect` has no `--out`;
  - `--write` and `--check` use the committed compatibility generated file
    path as `--out`;
  - `--surface core` must not claim ownership of the committed compatibility
    generated file after this migration;
  - the core-surface no-projection regression proof is the exact named test
    `core_surface_ignores_projection_declarations_and_emits_no_projection_symbols`
    in `crates/codegen/src/tests/test_rust_emit_core.rs`;
  - the implementation plan may only bind command execution ordering and
    expected outputs.
- Generated artifact ownership is closed:
  - `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`
    has one owner after this migration: `--surface projection`;
  - no generated artifact in this spec is bound to two incompatible command
    surfaces;
  - hand edits to that file remain forbidden.
- Runtime and codegen dispatch bindings are closed:
  - `crates/codegen/src/config.rs` owns the parsed `Surface::Projection`
    configuration;
  - `crates/codegen/src/emit.rs` owns inspect, write, and check dispatch for
    `--surface core` versus `--surface projection`;
  - `crates/codegen/src/descriptor.rs` and `crates/codegen/src/model.rs` own
    projection-definition loading and deterministic projection models;
  - `crates/codegen/src/rust_emit.rs` owns projected marker, row, payload,
    view, accessor, projection, and inspect emission;
  - `crates/codegen/src/tests/test_cli.rs`,
    `crates/codegen/src/tests/test_descriptor.rs`,
    `crates/codegen/src/tests/test_rust_emit_core.rs`, and
    `crates/schemas/test_compatibility_core/tests/test_projection.rs` own the
    required behavior tests;
  - `crates/codegen/src/lib.rs` may change documentation text only.
- Test migration from old behavior is closed:
  - the old core-only assertion that projection declarations are ignored by
    core output is preserved for `--surface core`;
  - the new `--surface projection` behavior must prove projected models and
    projected schema output are emitted while the source schema hash remains
    stable;
  - this migrated contract is bound in Section 18.
- Compile-surface evidence is closed before audit because Section 14 binds:
  - generated line-count evidence with `wc -l`;
  - schema crate build evidence with `/usr/bin/time -v cargo check`;
  - dependency isolation evidence with `cargo tree`;
  - the rule that no compile-time improvement is claimed without build
    evidence.
- No design decision is deferred to the implementation plan:
  - surface choice, artifact ownership, schema-hash contract, dispatch files,
    generated API shape, command surfaces, failure behavior, test migration,
    and compile-surface evidence commands are all specified here;
  - the implementation plan may bind execution order, expected outputs, and
    rollback boundaries only after this spec passes peer audit.

This spec is implementation-ready only when all are true:

- required reads are complete;
- pre-audit closure checklist is complete;
- this amended spec passes peer audit v6;
- measured object is precise;
- schema source contract is explicit;
- wire/archive contract is explicit;
- checked/trusted access contract is explicit;
- codegen contract is explicit;
- exact codegen-check command is defined;
- `--surface projection` is defined without weakening `--surface core`;
- generated artifact ownership between `--surface core` and
  `--surface projection` is explicit;
- projected schema-hash input is deterministic;
- compile-surface budget is measurable;
- runtime performance budget is explicit;
- correctness oracle is separate from test plan;
- code bindings are exact;
- generated artifact bindings are exact;
- review artifact bindings are exact;
- implementation plan is written and approved.

## 24. Open questions

No implementation-blocking open questions are intended to remain after this
amendment.

Non-blocking future question:

- whether `crates/projection` should later expose a shared projection trait or
  helper API. This phase does not need that dependency based on current
  code-read evidence.

Next required command:

```text
Approved: write docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v6.md
```
