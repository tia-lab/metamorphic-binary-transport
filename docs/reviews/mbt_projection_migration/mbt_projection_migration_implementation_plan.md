# Implementation Plan: MBT Projection Migration

Status: `AWAITING_IMPLEMENTATION_PLAN_PEER_AUDIT_V2`

Slug: `mbt_projection_migration`

This plan does not authorize implementation until the implementation-plan peer
audit passes and the user explicitly approves implementation.

## 1. Inputs

Approved spec:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Passed peer audit:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v6.md
```

Blocking implementation-plan audit addressed by this amendment:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan_peer_audit.md
```

Required passed implementation-plan audit before code:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan_peer_audit_v2.md
```

Primary schema input:

```text
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

Generated artifact:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

## 2. Required Reads Before Code

Before implementation, reread:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/lifecycle_protocol.md
docs/protocols/spec_protocol.md
docs/protocols/implementation_protocol.md
docs/protocols/code_style_protocol.md
docs/protocols/codegen_protocol.md
docs/protocols/testing_benchmark_protocol.md
docs/specs/mbt_projection_migration_SPEC.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v6.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan_peer_audit_v2.md
```

## 3. Files To Edit

Production/codegen files:

```text
crates/codegen/src/model.rs
crates/codegen/src/config.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/emit.rs
crates/codegen/src/lib.rs
crates/codegen/src/rust_emit.rs
```

Codegen test files:

```text
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_rust_emit_core.rs
```

Schema test files to create:

```text
crates/schemas/test_compatibility_core/tests/test_projection.rs
```

Generated file to update by codegen only:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

`crates/codegen/src/lib.rs` may only receive documentation text that removes or
narrows stale core-only wording. It must not add runtime behavior.

## 4. Files Not To Edit

These paths are out of scope:

```text
proto/mathilde/options.proto
crates/core/src/*
crates/projection/src/*
crates/metamorphose/src/*
crates/transponding/src/*
crates/adapters/*
```

If any of these paths become necessary, stop and request an amended plan before
editing.

## 5. Dependency Changes

No dependency changes are approved.

Forbidden effects:

- no adapter dependency enters `crates/schemas/test_compatibility_core`;
- no `prost` runtime dependency enters `crates/schemas/test_compatibility_core`;
- no `serde`, `serde_json`, Arrow, Parquet, SQLite, Postgres, cache, lookup, or
  service dependency enters `crates/schemas/test_compatibility_core`;
- no generated schema crate depends on `crates/codegen`;
- no schema crate dependency on `crates/projection` in this phase.

## 6. Implementation Phases

### Phase 0: Pre-Edit Evidence

Run and record output for the later result review:

```text
git status --short
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_core
```

No interpretation or readiness claim is made from these commands. They are
baseline evidence only.

### Phase 1: CLI Surface

Edit:

```text
crates/codegen/src/config.rs
crates/codegen/src/tests/test_cli.rs
```

Required changes:

- add `Surface::Projection`;
- parse `--surface projection`;
- keep `--surface core` behavior unchanged;
- keep unsupported surfaces rejected;
- keep `--inspect` forbidding `--out`;
- keep `--write` and `--check` requiring `--out`;
- add CLI tests proving:
  - `--surface projection` parses;
  - `--surface core` still parses;
  - unsupported surfaces still fail;
  - action/output validation remains unchanged.

### Phase 2: Projection Model

Edit:

```text
crates/codegen/src/model.rs
```

Required new model types:

```rust
pub struct ProjectionDefinitionModel {
    pub name: String,
    pub rust_marker: String,
    pub include_groups: Vec<String>,
    pub exclude_groups: Vec<String>,
    pub include_fields: Vec<String>,
    pub exclude_fields: Vec<String>,
}

pub struct ProjectionFieldMapping {
    pub source_index: usize,
    pub source_presence_bit: Option<u32>,
    pub projected_presence_bit: Option<u32>,
}

pub struct ProjectionModel {
    pub definition: ProjectionDefinitionModel,
    pub marker_type: String,
    pub payload_type: String,
    pub row_type: String,
    pub view_type: String,
    pub rows_iter_type: String,
    pub archived_row_type: String,
    pub transport_name: String,
    pub dictionaries: Vec<Dictionary>,
    pub fields: Vec<PhysicalField>,
    pub key_parts: Vec<KeyPart>,
    pub normalized_schema_hash: u64,
    pub field_mappings: Vec<ProjectionFieldMapping>,
}
```

Add:

```rust
pub projections: Vec<ProjectionModel>
```

to `SchemaModel`.

No generated schema crate may expose these codegen model types.

### Phase 3: Descriptor Projection Parsing And Selection

Edit:

```text
crates/codegen/src/descriptor.rs
crates/codegen/src/tests/test_descriptor.rs
```

Do not edit `crates/codegen/src/options.rs` in this phase.

Required descriptor behavior:

- parse repeated root message `mathilde.projection` values directly from
  `extensions.projection`;
- preserve current source `normalized_schema_hash` behavior:
  - `mathilde.projection` excluded;
  - `mathilde.projection_group` excluded;
  - ignored non-physical fields excluded;
- validate before emission:
  - duplicate projection name fails;
  - duplicate projection marker fails;
  - invalid projection name fails;
  - invalid Rust marker fails;
  - unknown include/exclude field fails;
  - unknown include/exclude group fails;
  - attempts to include ignored fields fail;
  - projected selection without schema-version fails;
  - projected selection without deterministic key fields fails when source has
    key fields;
- implement deterministic projection selection:
  1. start with mandatory fields;
  2. apply include groups/fields when any include exists;
  3. otherwise include every non-ignored physical field;
  4. apply excludes;
  5. re-add mandatory fields;
  6. preserve source physical field order;
- repack selected optional scalar and nullable-array presence bits from zero in
  projected physical order;
- include only dictionaries used by selected projected fields;
- compute projected schema hash from the exact Section 9 hash text contract;
- add descriptor tests proving all validation and hash rules above.

### Phase 4: Emission Dispatch

Edit:

```text
crates/codegen/src/emit.rs
crates/codegen/src/tests/test_cli.rs
```

Required behavior:

- `--surface core` dispatches to core-only generated source and emits no
  projection markers or projection functions;
- `--surface projection` dispatches to source schema plus declared projection
  output;
- `inspect --surface projection` includes line-based projection metadata:

```text
projection=no_optional marker=TestCompatibilityV1NoOptional
projection=numeric_only marker=TestCompatibilityV1NumericOnly
```

- `write --surface projection` writes only
  `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs` for
  the compatibility schema;
- `check --surface projection` checks that same generated file;
- inspect/write/check action semantics stay unchanged.

### Phase 5: Rust Projection Emission

Edit:

```text
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_rust_emit_core.rs
```

Required emitter shape:

- keep existing source schema generated names unchanged for `--surface core`;
- for `--surface projection`, emit the source schema first with unchanged
  source API names;
- emit one projected schema section per declared projection;
- projected internal constants and helpers must be projection-prefixed to avoid
  collisions, for example:
  - `NO_OPTIONAL_GENERATED_SCHEMA_HASH`;
  - `NO_OPTIONAL_SCHEMA_HEADER`;
  - `no_optional_validate_rows`;
  - `no_optional_validate_archived_payload`;
  - `no_optional_inspect_archived_rows`;
- projected public marker APIs use associated constants/functions on:
  - `TestCompatibilityV1NoOptional`;
  - `TestCompatibilityV1NumericOnly`;
- emit source marker projection functions:
  - `project_no_optional`;
  - `project_no_optional_trusted_unchecked`;
  - `project_numeric_only`;
  - `project_numeric_only_trusted_unchecked`;
- checked projection must call source checked archived access;
- trusted projection must call source trusted archived access and remain
  `unsafe`;
- projection row construction must allocate only:
  - projected owned row vector;
  - projected output bytes through projected `encode_owned`;
- projection runtime must not use reflection;
- static projection metadata must not allocate at runtime;
- do not emit JSON, protobuf, CSV, Arrow, Arrow IPC, Parquet, transponding, MDB,
  cache, lookup, or service code.

Expected generated type names:

```text
TestCompatibilityV1NoOptional
TestCompatibilityResponseV1PayloadNoOptional
TestCompatibilityRowV1NoOptional
TestCompatibilityV1NoOptionalView
TestCompatibilityV1NoOptionalRows
ArchivedTestCompatibilityRowV1NoOptional

TestCompatibilityV1NumericOnly
TestCompatibilityResponseV1PayloadNumericOnly
TestCompatibilityRowV1NumericOnly
TestCompatibilityV1NumericOnlyView
TestCompatibilityV1NumericOnlyRows
ArchivedTestCompatibilityRowV1NumericOnly
```

Required core regression test:

```text
cargo test -p metamorphic_binary_transport_codegen core_surface_ignores_projection_declarations_and_emits_no_projection_symbols
```

The test must live in:

```text
crates/codegen/src/tests/test_rust_emit_core.rs
```

It must prove:

- compatibility schema through `--surface core` keeps source hash stable;
- core output does not contain `TestCompatibilityV1NoOptional`;
- core output does not contain `TestCompatibilityV1NumericOnly`;
- core output does not contain `project_no_optional`;
- core output does not contain `project_numeric_only`;
- core output still contains the source marker, row, payload, view, and
  checked/trusted access surface.

### Phase 6: Schema Projection Tests

Create:

```text
crates/schemas/test_compatibility_core/tests/test_projection.rs
```

Required tests:

- checked `project_no_optional` output validates with
  `TestCompatibilityV1NoOptional`;
- trusted `project_no_optional_trusted_unchecked` output is byte-identical to
  checked output for the same immutable source bytes;
- checked `project_numeric_only` output validates with
  `TestCompatibilityV1NumericOnly`;
- trusted `project_numeric_only_trusted_unchecked` output is byte-identical to
  checked output for the same immutable source bytes;
- source marker rejects each projected payload;
- projected marker rejects source payload;
- no-optional projected row has no optional/nullable generated getters or
  presence constants;
- numeric-only projected row preserves:
  - schema version;
  - tenant/entity dictionaries;
  - close_ms;
  - required numeric scalars;
  - optional numeric scalars;
  - required numeric arrays;
  - nullable numeric arrays;
  - null versus present-empty nullable arrays;
- tests compare fields explicitly and do not require generated `PartialEq`.

## 7. Generated Artifact Commands

After implementation, run these in order.

Projection inspect:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --inspect --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection
```

Expected output contains at least:

```text
projection=no_optional marker=TestCompatibilityV1NoOptional
projection=numeric_only marker=TestCompatibilityV1NumericOnly
```

Projection write:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Expected output:

```text
exit code 0
```

Projection check:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Expected output:

```text
exit code 0
```

The generated file must not be manually edited.

## 8. Validation Commands

Run after generated artifact update:

```text
cargo test -p metamorphic_binary_transport_codegen core_surface_ignores_projection_declarations_and_emits_no_projection_symbols
cargo test -p metamorphic_binary_transport_codegen
cargo test -p metamorphic_binary_transport_schema_test_compatibility
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
cargo clippy -p metamorphic_binary_transport_schema_test_compatibility --all-targets -- -D warnings
cargo check --workspace
cargo fmt --all --check
```

Compile-surface evidence commands:

```text
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_core
```

Expected dependency evidence:

- `metamorphic_binary_transport_schema_test_compatibility` depends on
  `metamorphic_binary_transport_core` and `rkyv`;
- no adapter crates appear in the schema crate tree;
- `metamorphic_binary_transport_codegen` does not appear in the schema crate
  runtime dependency tree;
- `metamorphic_binary_transport_core` remains free of adapter dependencies.

No runtime throughput benchmark is required by this plan.

## 9. Pre-Test Audit

Before running validation, inspect the final patch against:

```text
docs/specs/mbt_projection_migration_SPEC.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan.md
docs/protocols/code_style_protocol.md
docs/protocols/codegen_protocol.md
```

The pre-test audit must confirm:

- only bound files changed;
- generated file was produced by `mbt_codegen --write --surface projection`;
- no generated file was edited by hand;
- no dependency changed;
- `--surface core` still emits no projection code;
- `--surface projection` emits only declared projections;
- wrong-marker rejection tests exist;
- checked/trusted byte equality tests exist;
- nullable array null versus present-empty tests exist;
- no `unwrap`, `expect`, `panic!`, `todo!`, or `unreachable!` was introduced in
  runtime, codegen, measurement, or reusable support logic.

## 10. Rollback Boundary

Rollback is limited to these paths:

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
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

No database, dataset, external schema repository, or service state is touched.

If validation fails because generated output requires manual edits, stop. Do
not hand-edit the generated file.

## 11. Known Risks

- Multiple schema sections in one generated module can collide if prefixed
  helper names are incomplete.
- Projection hash stability can drift if source hash inputs and projection hash
  inputs are not separated exactly.
- Presence repacking can corrupt nullable array semantics if absent and
  present-empty arrays are not tested separately.
- Trusted projection safety can be weakened if trusted access performs less
  than the existing trusted archived payload shape check.
- Generated line count and schema crate check time will increase; this plan
  records evidence but does not claim compile-time improvement.

## 12. Approval Gate

Implementation may start only after:

1. the implementation-plan peer audit passes in:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan_peer_audit_v2.md
```

2. the user explicitly approves:

```text
Approved: implement docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan.md
```
