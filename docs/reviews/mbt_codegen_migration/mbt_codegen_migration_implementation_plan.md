# Implementation Plan: MBT Codegen Migration

Status: `DRAFT_AWAITING_APPROVAL`

Slug: `mbt_codegen_migration`

This plan does not authorize implementation. Code changes may start only after
this plan is explicitly approved.

## 1. Approved Inputs

Spec:

```text
docs/specs/mbt_codegen_migration_SPEC.md
```

Passing peer audit:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_peer_audit_v3.md
```

Research brief:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_research_brief.md
```

Required protocol reads completed for planning:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/implementation_protocol.md
docs/protocols/code_style_protocol.md
docs/protocols/codegen_protocol.md
```

Code-read evidence used for this plan:

```text
crates/codegen/src/lib.rs
crates/codegen/src/main.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/model.rs
crates/codegen/src/options.rs
crates/codegen/src/rust_emit.rs
crates/codegen/Cargo.toml
proto/mathilde/options.proto
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/descriptor.rs
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/emit.rs
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/error.rs
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/model.rs
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/rust_emit.rs
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/bin/mbt_codegen.rs
```

## 2. Implementation Goal

Implement `crates/codegen` as a production MBT core-schema generator:

```text
explicit proto roots + schema proto + root message + module + surface core
  -> descriptor set through protoc
  -> prost-reflect descriptor model
  -> validated MBT schema model
  -> deterministic core-only generated Rust
  -> optional write/check/inspect CLI
  -> mandatory temporary smoke compile
```

The implementation must not emit projection, metamorphose, adapter,
transponding, prost DTO, DB, cache, lookup, or MLDB code.

## 3. Files To Edit

Edit only:

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

`Cargo.lock` may change only as the Cargo-generated result of the dependency
changes below.

## 4. Files To Create

Create only:

```text
crates/codegen/src/config.rs
crates/codegen/src/emit.rs
crates/codegen/src/error.rs
crates/codegen/src/tests/test_options.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/test_model.rs
crates/codegen/src/tests/test_rust_emit_core.rs
crates/codegen/src/tests/test_cli.rs
```

Use existing:

```text
crates/codegen/src/tests/mod.rs
```

for test module declarations and shared test helpers. Do not create additional
test support files unless a later approved plan amends this binding.

## 5. Files Not To Edit

Do not edit:

```text
crates/core/src/*
crates/projection/src/*
crates/metamorphose/src/*
crates/transponding/src/*
crates/adapters/**/*
crates/benches/**/*
```

Do not commit generated schema files.

## 6. Dependency Changes

Add to `crates/codegen/Cargo.toml`:

```toml
prost-reflect = { version = "=0.16.4", default-features = false }
thiserror = "=2.0.17"
```

Do not add:

```text
prost-build
rkyv
serde
serde_json
arrow-array
arrow-buffer
arrow-ipc
arrow-schema
parquet
zstd
```

`rkyv` appears only in generated source text and in the temporary smoke crate
`Cargo.toml`.

## 7. Module Plan

### `proto/mathilde/options.proto`

Replace the current placeholder with the exact proto2 MBT-only option file from
the spec.

Mandatory properties:

- package is `mathilde`;
- imports only `google/protobuf/descriptor.proto`;
- contains `DictionaryAlias`, `Dictionary`, and `ProjectionDefinition`;
- contains MBT-only extensions with the exact numbers in the spec;
- contains no DB, cache, lookup, MLDB, Postgres, SQLite, or serving options.

### `crates/codegen/src/lib.rs`

Expose only codegen modules:

```rust
pub mod config;
pub mod descriptor;
pub mod emit;
pub mod error;
pub mod model;
pub mod options;
pub mod rust_emit;
```

Keep `#![forbid(unsafe_code)]`.

### `crates/codegen/src/error.rs`

Define:

```rust
pub type Result<T> = std::result::Result<T, CodegenError>;

pub enum CodegenError {
    Io(std::io::Error),
    Descriptor(String),
    MissingDescriptor(&'static str),
    MissingOption(&'static str),
    InvalidOption { name: &'static str, reason: String },
    InvalidSchema(String),
    Rustfmt(String),
    UnsupportedArgument(String),
    GeneratedDiff(PathBuf),
}
```

Use `thiserror::Error`. Do not use `unwrap`, `expect`, `panic!`, `todo!`, or
`unreachable!`.

### `crates/codegen/src/config.rs`

Own CLI/config types:

```rust
pub enum Action { Inspect, Write, Check }
pub enum Surface { Core }
pub struct CodegenConfig { ... }
pub struct SchemaRequest { ... }
```

Required fields:

```text
action
proto_roots: Vec<PathBuf>
schema: PathBuf
root: String
module: String
surface: Surface
out: Option<PathBuf>
```

Validation:

- exactly one action;
- `--proto-root` non-empty and repeatable;
- `--schema` required;
- `--root` required;
- `--module` required and snake_case;
- `--surface core` required;
- `--out` required for `--write` and `--check`;
- `--out` forbidden for `--inspect`;
- unknown flags fail with `UnsupportedArgument`;
- duplicate action flags fail.

### `crates/codegen/src/main.rs`

Implement only:

```text
parse args
run selected action through emit module
print inspect output to stdout
print errors to stderr
exit non-zero on failure
```

No workspace discovery, no `--all`, no canonical requests, no generated
production module writing.

Exact actions:

```text
--inspect
--write
--check
```

### `crates/codegen/src/options.rs`

Own MBT extension lookup and option extraction.

Implement:

```rust
pub struct MbtExtensions { ... }
pub fn extensions(pool: &DescriptorPool) -> Result<MbtExtensions>;
pub fn required_u32(...);
pub fn optional_u32(...);
pub fn required_bool(...);
pub fn optional_bool(...);
pub fn required_string(...);
pub fn optional_string(...);
pub fn dictionary_from_value(...);
```

Projection extensions are resolved only so that the core surface recognizes
valid options. They must not create projection output models.

Dictionary aliases:

- parsed as valid option syntax;
- excluded from dictionary ordinals;
- excluded from core schema hash;
- not emitted in core generated Rust.

### `crates/codegen/src/model.rs`

Define the core-only schema model.

Required types:

```rust
pub struct Dictionary { name: String, values: Vec<String> }
pub struct SchemaModel { ... }
pub struct PhysicalField { ... }
pub enum FieldKind { ... }
pub struct KeyPart { ... }
```

Required field kinds:

```text
ConstU16
U16Dictionary required/optional
U64BitmaskDictionary
I32
U32
I64
F32
F64
Bool
Bytes
RawString
I64Array
I32Array
U32Array
F64Array
F32Array
```

Do not define target/adaptor models:

```text
TargetModel
TargetField
TargetParent
TargetFieldKind
ProjectionModel
```

Required helpers:

```text
rust_type_name
module_marker_type
payload_type_from_root
const_name
dict_prefix
helper_stem
field_kind_hash_name
validate_module_name
```

No hardcoded Bars, Primitives, compatibility, or wide-presence canonical
requests.

### `crates/codegen/src/descriptor.rs`

Implement descriptor loading and schema model construction.

Required behavior:

```text
run protoc with repeated --proto_path in CLI order
include imports
decode FileDescriptorSet with prost-reflect
resolve MBT extensions by full name
load explicit root message
validate payload_root
find exactly one repeated_payload row field
collect physical fields from row message
skip ignored fields before physical mapping
reject unsupported or ambiguous schema shapes
validate dictionaries
validate presence bits
validate key order
compute schema hash from spec normal form
```

Required public entrypoint:

```rust
pub fn load_schema_model(request: &SchemaRequest) -> Result<SchemaModel>;
```

Required internal helper groups:

```text
row_payload_field
collect_physical_fields
physical_field
rust_field_name
join_path
dictionaries_from_file
validate_dictionaries
validate_physical_fields
key_parts
require_dictionary
dictionary_by_name
raw_descriptor_set
run_protoc
temp_dir
normalized_hash
fnv1a64
```

`normalized_hash` must implement the exact normal form in the spec, not the
old projection-aware normal form.

### `crates/codegen/src/rust_emit.rs`

Implement deterministic core-only Rust generation.

Required public entrypoint:

```rust
pub fn generated_schema(model: &SchemaModel) -> Result<String>;
```

Required output groups:

```text
deterministic header
core imports only
schema constants
dictionary constants
presence constants
owned row and payload structs
row validation
semantic/minimal checksum functions
dictionary helpers
runtime marker API
MbtSchema implementation
view / rows iterator / archived row wrapper
decode and trusted access helpers
```

Generated imports may reference only:

```text
core
rkyv
metamorphic_binary_transport_core
```

Forbidden generated output strings are those listed in spec section 9.

Rust formatting is handled by `emit.rs`, not inside `rust_emit.rs`.

### `crates/codegen/src/emit.rs`

Own orchestration:

```rust
pub fn inspect(config: &CodegenConfig) -> Result<String>;
pub fn write(config: &CodegenConfig) -> Result<()>;
pub fn check(config: &CodegenConfig) -> Result<()>;
```

Required helper behavior:

- `inspect` emits deterministic text over schema id/version/hash, root, row
  type, dictionaries, fields, key parts, presence bits, and generated line
  count estimate if available;
- `write` writes rustfmt-normalized generated source to `--out`;
- `check` regenerates under `target/mbt-codegen-check/`, compares bytes with
  `--out`, and fails with `GeneratedDiff` on mismatch;
- `format_rust` calls `rustfmt --edition 2021` through stdin/stdout;
- rustfmt failure records stderr only;
- `write_if_changed` avoids touching files when bytes are unchanged;
- temp directories are process-unique and under `target/mbt-codegen-check/`;
- no prost output is generated.

Smoke crate helper:

- create `target/mbt-codegen-check/smoke/Cargo.toml`;
- create `target/mbt-codegen-check/smoke/src/lib.rs`;
- create `target/mbt-codegen-check/smoke/src/generated_fixture.rs`;
- write generated fixture into `generated_fixture.rs`;
- run only through the validation command in section 12, not implicitly during
  normal `--write`.

## 8. Test Plan Binding

### `crates/codegen/src/tests/mod.rs`

Declare:

```rust
mod test_options;
mod test_descriptor;
mod test_model;
mod test_rust_emit_core;
mod test_cli;
```

Shared test helpers may live in this file only.

Helper groups:

```text
temp_root(label)
write_options_proto(root)
write_proto(root, relative_path, contents)
run_codegen_to_string(...)
assert_forbidden_absent(source)
smoke_crate(root, generated_source)
```

### Fixture Writer Helpers

Use generated temporary proto files, not committed fixture schemas.

Required fixture helpers:

```text
valid_scalar_proto()
valid_raw_string_proto()
valid_array_proto()
valid_wide_presence_proto()
valid_alias_and_projection_ignored_proto()
invalid_unannotated_string_proto()
invalid_missing_presence_proto()
invalid_duplicate_presence_proto()
invalid_gapped_presence_proto()
invalid_duplicate_key_order_proto()
invalid_gapped_key_order_proto()
invalid_nullable_bitmask_proto()
invalid_repeated_string_proto()
invalid_repeated_bytes_proto()
invalid_repeated_bool_proto()
invalid_duplicate_rust_field_proto()
```

All valid fixtures must:

- import `mathilde/options.proto`;
- define at least one dictionary when dictionary behavior is tested;
- define one payload root with schema id, schema version, transport name, and
  `payload_root = true`;
- define exactly one repeated row field with `repeated_payload = true`;
- avoid Bars-specific schema names.

The alias/projection fixture must prove:

- aliases are accepted;
- projection options are accepted;
- generated core output excludes projection strings;
- core hash is unchanged when only aliases/projection declarations change.

### Required Tests

`test_options.rs`:

- option extension lookup succeeds;
- MBT option file is proto2 and MBT-only;
- DB/cache/lookup/MLDB/Postgres/SQLite option names are absent.

`test_descriptor.rs`:

- valid scalar, raw string, array, and wide-presence schemas load;
- repeated `--proto-root` order is passed to protoc;
- invalid schemas fail before rust emission;
- dictionary aliases are accepted but absent from ordinals and hash inputs.

`test_model.rs`:

- `FieldKind` mapping covers every scalar and array kind;
- presence bits are contiguous;
- key order is contiguous;
- schema hash normal form changes for included field changes;
- schema hash normal form does not change for comments, source path, aliases,
  projections, projection groups, or ignored fields.

`test_rust_emit_core.rs`:

- generated source is byte-identical across two runs;
- generated source includes marker/view/rows/archived-row APIs;
- generated source includes checked and trusted access APIs;
- generated source includes presence accessors;
- generated source excludes all forbidden strings in spec section 9;
- smoke crate compiles generated fixture.

`test_cli.rs`:

- `--inspect` succeeds without `--out`;
- `--write` requires `--out`;
- `--check` requires `--out`;
- missing action fails;
- duplicate action fails;
- invalid module name fails;
- unknown argument fails;
- empty `--proto-root` list fails;
- `--surface` accepts only `core`;
- `--check` returns `GeneratedDiff` for changed output.

## 9. Old Experiment Codegen Function Disposition

### Binary CLI

Migrate with changes:

```text
main
run
Action
Command
parse_args
set_action
next_arg
```

Reject:

```text
workspace_root
--all
--inspect-options spelling
canonical schema defaults
```

The new CLI uses explicit `--proto-root`, `--schema`, `--root`, `--module`,
`--surface core`, and optional `--out` according to the spec.

### `error.rs`

Migrate with changes:

```text
CodegenError
Result
```

The new `CodegenError` keeps the old error classes but belongs to
`crates/codegen/src/error.rs`.

### `model.rs`

Migrate with changes:

```text
CodegenRequest -> SchemaRequest / CodegenConfig split
Dictionary
SchemaModel
PhysicalField
FieldKind
KeyPart
MbtExtensions
required_u32
optional_u32
required_bool
optional_bool
required_string
optional_string
dictionary_from_value
rust_type_name
module_marker_type
payload_type_from_root
const_name
dict_prefix
helper_stem
validate_module_name
```

Reject:

```text
BARS_MODULE
BARS_PROTO
BARS_ROOT
PRIMITIVES_MODULE
PRIMITIVES_PROTO
PRIMITIVES_ROOT
TEST_COMPATIBILITY_MODULE
TEST_COMPATIBILITY_PROTO
TEST_COMPATIBILITY_ROOT
WIDE_PRESENCE_MODULE
WIDE_PRESENCE_PROTO
WIDE_PRESENCE_ROOT
canonical_requests
TargetModel
TargetField
TargetParent
TargetFieldKind
ProjectionDefinitionModel
ProjectionModel
```

Projection option descriptors are recognized in `MbtExtensions`, but
projection output models are rejected in this migration.

### `descriptor.rs`

Migrate with changes:

```text
load_schema_model
RowPayload
row_payload_field
collect_physical_fields
physical_field
rust_field_name
join_path
dictionaries_from_file
validate_dictionaries
validate_physical_fields
key_parts
require_dictionary
dictionary_by_name
raw_descriptor_set
run_protoc
temp_dir
extensions
extension
normalized_hash
fnv1a64
```

Reject:

```text
projection_definitions
projection_string
projection_string_list
build_projection_models
build_projection_model
collect_target_model
TargetFieldCollector
target_field
target_kind
projection_target_model
projection_type_suffix
validate_projection_references
initial_projection_selection
apply_projection_includes
apply_projection_excludes
validate_projection_fields
validate_projection_target_fields
is_mandatory_projection_field
field_matches
```

`normalized_hash` must be rewritten to the core-only normal form in the spec.

### `emit.rs`

Migrate with changes:

```text
inspect
write_outputs -> write
check_outputs -> check
check_outputs_inner
format_rust
write_if_changed
temp_dir
inspect_text
```

Reject:

```text
RequestMode
GENERATED_DIR
SHARED_SCHEMA_PROTO_DIR
proto_root
write_proto_output
write_proto_output_to
compile_proto
root_package_output
mod_rs
modules_for_mod_rs
existing_generated_modules
prost sidecar generation
checked-in generated module management
```

### `rust_emit.rs`

Migrate with changes:

```text
generated_schema
validate_model
emit_header
emit_imports
emit_schema_constants
emit_dictionary_constants
emit_zero_based_dictionary
emit_one_based_dictionary
emit_bitmask_dictionary
presence_fields
presence_count
presence_words
presence_is_wide
presence_word
presence_mask
presence_storage_type
presence_zero_expr
presence_allowed_masks
presence_mask_const
presence_word_const
owned_presence_has_expr
archived_presence_has_expr
owned_presence_absent_expr
presence_set_line
presence_error_expr
emit_presence_constants
emit_structs
emit_validation
emit_absent_validation
emit_order_validation
emit_validation_helpers
emit_checksums
emit_checksum_helpers
emit_owned_checksum_line
emit_owned_checksum_line_indented
emit_archived_checksum_line
checksum_func
checksum_array_element_func
emit_dictionary_helpers
emit_required_symbol_helper
emit_optional_symbol_helper
emit_ordinal_helper
emit_runtime_api
emit_runtime_trait
emit_view_types
emit_archived_getter
dictionary_ordinal_getter_name
emit_decode_helpers
presence_helper_name
archived_presence_helper_name
presence_view_method_name
presence_helper_name_for_stem
archived_presence_helper_name_for_stem
rust_field_type
archived_value_access
archived_to_owned_access
minimal_fields
metadata_fields
has_metadata_checksum_fields
has_presence
needs_validate_enum
has_u16_checksum_field
has_u64_checksum_field
has_i32_field
has_u32_field
has_i64_field
has_f32_field
has_f64_field
has_bool_field
has_raw_string_or_bytes
has_array_field
has_i64_array_field
has_i32_array_field
has_u32_array_field
has_f64_array_field
has_f32_array_field
is_array_kind
field_by_name
dictionary_is_used
dictionary_is_key
dictionary_is_bitmask
dictionary_has_optional_use
last_dictionary_const
first_dictionary_const
validate_rows_fn
validate_row_fn
has_fn
semantic_checksum_fn
minimal_projection_checksum_fn
checksum_row_fn
checksum_archived_row_fn
minimal_archived_row_fn
decode_payload_fn
validate_archived_payload_fn
validate_archived_rows_fn
inspect_archived_rows_fn
row_from_archived_fn
presence_const
presence_words_const
presence_allowed_mask
```

Reject:

```text
transponding_imports
emit_projected_schema_constant
emit_projection_methods
emit_transponding_helpers
emit_arrow_batch_methods
emit_arrow_marker_methods
emit_parquet_marker_methods
emit_arrow_ipc_marker_methods
emit_transponding_batch_init
emit_column_push
column_batch_type
column_type
column_init
numeric_column_init
column_vector_init
archived_transpond_value
direct_transpond_value
arrow_data_type
arrow_array_helper
emit_metamorphose_methods
emit_metamorphose_trait
emit_metamorphose_helpers
emit_csv_helpers
emit_csv_row_cells
emit_csv_field
csv_numeric_access
csv_string_access
csv_bytes_access
csv_array_access
has_derived_utc_target
emit_json_helpers
emit_json_parent_field
emit_json_field
emit_json_static_field
emit_json_value
emit_protobuf_helpers
emit_protobuf_parent_helpers
emit_protobuf_len_field
emit_protobuf_write_field
protobuf_len_expr
protobuf_array_len_expr
target_parents
dictionary_helper_for_target
target_presence_expr
write_json_response_fn
write_json_row_fn
write_json_parent_fn
write_protobuf_response_fn
write_protobuf_row_fn
write_protobuf_parent_fn
write_csv_response_fn
write_csv_valid_rows_response_fn
write_csv_row_fn
write_csv_owned_row_fn
csv_header
csv_header_name
encoded_len_response_fn
encoded_len_row_fn
encoded_len_parent_fn
archived_project_value
owned_project_value
projection_stem
projection_presence_prefix
```

`has_open_close_ms` is rejected because it is Bars-specific. Ordering must use
declared key parts only.

## 10. Implementation Sequence

1. Update `proto/mathilde/options.proto`.
2. Add codegen dependencies to `crates/codegen/Cargo.toml`.
3. Create `error.rs` and wire `lib.rs`.
4. Create `config.rs` and implement CLI parsing in `main.rs`.
5. Implement `options.rs` extension lookup and option helpers.
6. Implement core-only `model.rs`.
7. Implement `descriptor.rs` with protoc descriptor loading and validation.
8. Implement `rust_emit.rs` core-only emission.
9. Implement `emit.rs` orchestration, rustfmt, write/check/inspect, and smoke
   crate helper.
10. Replace skeleton tests with the tests bound in section 8.
11. Run validation commands in section 12.
12. Write the result review only after implementation and validation.

Stop immediately if any step requires a behavior not present in the spec.

## 11. Generated Artifacts

Committed generated artifacts:

```text
none
```

Allowed temporary outputs:

```text
target/mbt-codegen-fixtures/
target/mbt-codegen-check/
target/mbt-codegen-check/smoke/Cargo.toml
target/mbt-codegen-check/smoke/src/lib.rs
target/mbt-codegen-check/smoke/src/generated_fixture.rs
```

Temporary outputs must not be checked into git.

## 12. Validation Commands

Run after implementation:

```text
cargo fmt --all --check
cargo check -p metamorphic_binary_transport_codegen
cargo test -p metamorphic_binary_transport_codegen
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
cargo check --workspace
cargo tree -p metamorphic_binary_transport_codegen
cargo tree -p metamorphic_binary_transport_core
cargo check --manifest-path target/mbt-codegen-check/smoke/Cargo.toml
cargo --version
protoc --version
rustfmt --version
```

Run a manual CLI smoke after tests create or the implementation creates a
temporary fixture:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --inspect \
  --proto-root target/mbt-codegen-fixtures/manual/proto \
  --proto-root proto \
  --schema test/scalar.proto \
  --root test.scalar.v1.ScalarPayloadV1 \
  --module scalar_v1 \
  --surface core
```

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --write \
  --proto-root target/mbt-codegen-fixtures/manual/proto \
  --proto-root proto \
  --schema test/scalar.proto \
  --root test.scalar.v1.ScalarPayloadV1 \
  --module scalar_v1 \
  --surface core \
  --out target/mbt-codegen-check/manual/scalar_v1.rs
```

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --check \
  --proto-root target/mbt-codegen-fixtures/manual/proto \
  --proto-root proto \
  --schema test/scalar.proto \
  --root test.scalar.v1.ScalarPayloadV1 \
  --module scalar_v1 \
  --surface core \
  --out target/mbt-codegen-check/manual/scalar_v1.rs
```

The manual fixture may be produced by a test helper or by a small temporary
script during validation. It must not be committed.

## 13. Expected Outputs

Expected validation results:

- formatting check passes;
- codegen crate check passes;
- codegen crate tests pass;
- codegen clippy passes with `-D warnings`;
- workspace check passes;
- codegen dependency tree contains `prost-reflect` and `thiserror`;
- core dependency tree does not contain `prost-reflect`, `prost-build`,
  `rkyv`, adapter crates, Arrow, Parquet, serde, serde_json, or zstd;
- smoke crate compiles generated fixture;
- generated fixture excludes all forbidden strings in spec section 9;
- generated fixture source line count is recorded in result review;
- host tool versions are recorded in result review.

No runtime throughput claim is expected from this work.

## 14. Result Review Binding

After approved implementation and validation, write:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_result_review.md
```

The result review must record:

- command evidence and outputs;
- dependency tree findings;
- generated fixture line count;
- forbidden-string scan result;
- smoke crate compile result;
- host tool versions;
- what old experiment functions were migrated, changed, or rejected;
- unproved claims, including runtime throughput and wide production schema
  compile time.

## 15. Rollback Boundary

Rollback consists of reverting only:

```text
crates/codegen/Cargo.toml
crates/codegen/src/lib.rs
crates/codegen/src/main.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/model.rs
crates/codegen/src/options.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/config.rs
crates/codegen/src/emit.rs
crates/codegen/src/error.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_options.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/test_model.rs
crates/codegen/src/tests/test_rust_emit_core.rs
crates/codegen/src/tests/test_cli.rs
proto/mathilde/options.proto
Cargo.lock
```

Temporary directories under `target/mbt-codegen-fixtures/` and
`target/mbt-codegen-check/` may be deleted after validation.

## 16. Known Risks

1. Core-only generated source may still be large for wide schemas. This plan
   records line count but does not claim compile-time improvement.
2. `prost-reflect` custom option decoding can expose descriptor-shape edge
   cases. Tests must cover invalid option types and missing options.
3. `rustfmt` availability is required. Failure is explicit through
   `CodegenError::Rustfmt`.
4. The old experiment emitter used Bars-specific open/close validation. This
   plan rejects that shortcut; ordering is declared-key based only.
5. The generated `encode(rows: &[Row])` API may require row cloning unless the
   implementation proves a no-clone serialization path while preserving the
   spec API. `encode_owned` remains the intended owned hot path.

## 17. Approval State

```text
spec: peer-audited and passed
implementation plan: draft awaiting approval
code implementation: not approved
```
