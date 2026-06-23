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

# Implementation Plan: MBT Explicit Dictionary Source Composition V1

Slug: `mbt_explicit_dictionary_source_composition_v1`

Status: `PEER_AUDITED_IMPLEMENTATION_AWAITING_APPROVAL`

Spec:

```text
docs/specs/mbt_explicit_dictionary_source_composition_v1_SPEC.md
```

Peer audit:

```text
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_peer_audit.md
```

This plan does not authorize implementation until explicitly approved.

## Scope

Implement the approved MBT codegen extension:

```text
target source dictionaries
  + explicit dictionary source dictionaries
  -> composed deterministic dictionary set
```

No runtime, adapter, schema, generated artifact, dependency, benchmark, or
application repository change is in scope.

## Files To Edit

Edit only:

```text
crates/codegen/src/config.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_result_review.md
```

No other manually edited files are authorized.

## Dependency Changes

No dependency changes are approved.

Expected proof:

```text
git diff -- Cargo.toml Cargo.lock
```

Expected result:

```text
no diff
```

## Generated Files

No generated files are approved.

Do not edit:

```text
crates/schemas/**
proto/**
target/**
```

## Step 1: Config And CLI

Edit `crates/codegen/src/config.rs`.

Add to `CodegenConfig`:

```rust
pub dictionary_sources: Vec<PathBuf>,
```

Add to `SchemaRequest`:

```rust
pub dictionary_sources: Vec<PathBuf>,
```

Update `CodegenConfig::schema_request()` to clone the list.

Update `parse_args`:

- initialize `let mut dictionary_sources = Vec::new();`;
- support repeated `--dictionary-source`;
- use `next_arg(&args, idx, "--dictionary-source")?`;
- reject empty values with `UnsupportedArgument("--dictionary-source cannot be empty")`;
- push `PathBuf::from(value)`;
- include `dictionary_sources` in the returned `CodegenConfig`.

No other CLI flag behavior may change.

## Step 2: Descriptor Input Paths

Edit `crates/codegen/src/descriptor.rs`.

Add helper behavior equivalent to:

```rust
fn schema_input_paths(request: &SchemaRequest) -> Result<Vec<PathBuf>>
```

It must:

1. resolve `request.schema` using the existing `schema_input_path` behavior;
2. resolve every `request.dictionary_sources` entry using the same proto-root
   search behavior;
3. return target schema first, then dictionary source inputs in request order.

`run_protoc` must pass every returned input path to the same `protoc` command.

The existing `schema_input_path` helper may be kept and reused or generalized.

## Step 3: Dictionary Composition

Edit `crates/codegen/src/descriptor.rs`.

Replace the single-file dictionary load:

```rust
let dictionaries = dictionaries_from_file(&file.options(), &extensions)?;
validate_dictionaries(&dictionaries)?;
```

with helper behavior equivalent to:

```rust
let dictionaries = dictionaries_from_request(&pool, &extensions, request, &source_file)?;
validate_dictionaries(&dictionaries)?;
```

Required composition behavior:

1. load root source file by `source_file`;
2. append `dictionaries_from_file(root.options(), extensions)?`;
3. for each `request.dictionary_sources` path in order:
   - convert to descriptor file name using the same string identity rule as
     `source_file_name`;
   - load the file descriptor from the pool;
   - return `InvalidSchema("missing dictionary source descriptor ...")` if not
     found;
   - append `dictionaries_from_file(source.options(), extensions)?`;
4. call existing `validate_dictionaries` once on the composed list.

Do not change `dictionary_from_value`, `require_dictionary`, or
`normalized_hash`.

Reason: storing the composed list in `SchemaModel.dictionaries` already makes
hash normalization include the composed dictionaries.

## Step 4: Test Helper Updates

Edit `crates/codegen/src/tests/mod.rs`.

Update `config_with_surface` to set:

```rust
dictionary_sources: Vec::new(),
```

Add one helper if useful:

```rust
fn config_with_dictionary_sources(
    root: &Path,
    schema: &str,
    module: &str,
    dictionary_sources: Vec<&str>,
) -> CodegenConfig
```

The helper must convert each `&str` into `PathBuf` and must not hide errors.

Add fixture helper functions only if needed:

```text
imported_dictionary_root_proto()
shared_instrument_dictionary_proto(values: &[&str])
```

## Step 5: CLI Tests

Edit `crates/codegen/src/tests/test_cli.rs`.

Add:

```text
cli_accepts_repeated_dictionary_source_flags
cli_rejects_empty_dictionary_source
```

Expected assertions:

- repeated flags preserve order;
- parsed `dictionary_sources` equals:
  ```text
  ["shared/instruments_v1.proto", "shared/venues_v1.proto"]
  ```
- empty value is rejected.

Existing CLI tests must still pass.

## Step 6: Descriptor Tests

Edit `crates/codegen/src/tests/test_descriptor.rs`.

Add:

```text
imported_dictionary_source_satisfies_dictionary_field
missing_explicit_dictionary_source_fails
duplicate_dictionary_names_across_sources_fail
schema_hash_changes_when_explicit_dictionary_values_change
source_file_local_dictionary_behavior_still_works
```

Test design:

1. create temp root;
2. write `mathilde/options.proto`;
3. write target schema file without local `instrument` dictionary values;
4. write shared dictionary source file with `name: "instrument"`;
5. call `load_schema_model` with `dictionary_sources` containing the shared
   file path.

Required success assertions:

- `model.dictionaries` contains exactly the composed `instrument` dictionary;
- the target `instrument` field resolves as `FieldKind::U16Dictionary`;
- `model.normalized_schema_hash` differs when dictionary values differ.

Required failure assertions:

- without `dictionary_sources`, the target schema fails with missing
  dictionary;
- duplicate dictionary names across root plus source or source plus source
  fail before emission.

Existing `valid_fixture_schemas_load` continues to prove source-file-local
dictionary behavior. The added `source_file_local_dictionary_behavior_still_works`
test may assert this directly for readability.

## Step 7: Validation Commands

Run in `/home/tia/_DEV/MATHILDE/metamorphic-binary-transport`, in order:

```text
cargo fmt --check
cargo check -p mbt_codegen
cargo test -p mbt_codegen cli_accepts_repeated_dictionary_source_flags -- --nocapture
cargo test -p mbt_codegen cli_rejects_empty_dictionary_source -- --nocapture
cargo test -p mbt_codegen imported_dictionary_source_satisfies_dictionary_field -- --nocapture
cargo test -p mbt_codegen missing_explicit_dictionary_source_fails -- --nocapture
cargo test -p mbt_codegen duplicate_dictionary_names_across_sources_fail -- --nocapture
cargo test -p mbt_codegen schema_hash_changes_when_explicit_dictionary_values_change -- --nocapture
cargo test -p mbt_codegen source_file_local_dictionary_behavior_still_works -- --nocapture
cargo test -p mbt_codegen dictionary -- --nocapture
cargo test -p mbt_codegen -- --nocapture
cargo tree -p mbt_codegen --edges normal
git diff -- Cargo.toml Cargo.lock
```

Expected outputs:

- `cargo fmt --check`: success;
- `cargo check -p mbt_codegen`: success;
- focused tests: success;
- `cargo test -p mbt_codegen dictionary -- --nocapture`: success;
- full `mbt_codegen` tests: success;
- dependency tree command: completes and shows no new dependency introduced by
  this work;
- `git diff -- Cargo.toml Cargo.lock`: no diff.

If any command fails, stop and write the result review as blocked with the
first failing command.

## Result Review

After validation, write:

```text
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_result_review.md
```

It must record:

- files changed;
- validation command outputs;
- no dependency-change proof;
- no generated-file proof;
- whether the schema-registry stop gate is satisfied;
- remaining limitations.

## Rollback Boundary

Rollback is limited to:

```text
crates/codegen/src/config.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_result_review.md
```

No runtime, generated, dependency, or schema rollback is required because those
surfaces are out of scope.

## Known Risks

1. Descriptor file-name identity for dictionary sources must match the path
   identity used by `protoc` and `DescriptorPool::get_file_by_name`.
2. Missing dictionary source paths may surface as `protoc` descriptor errors.
   This is acceptable if the focused failure tests remain clear.
3. If external callers manually construct `CodegenConfig`, they must add
   `dictionary_sources: Vec::new()`. Current in-repo search found only the
   codegen test helper and parser return paths.
4. Schema-registry implementation remains blocked until this plan is approved,
   implemented, validated, reviewed, and released by version bump.

