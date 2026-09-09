# Result Review: MBT Explicit Dictionary Source Composition V1

## Classification

PASSED

## Implemented Scope

Edited files:

```text
crates/codegen/src/config.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
```

Created review artifact:

```text
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_result_review.md
```

Implemented changes:

- added `CodegenConfig.dictionary_sources`;
- added `SchemaRequest.dictionary_sources`;
- added repeatable `--dictionary-source <proto-file>` CLI parsing;
- rejected empty `--dictionary-source` values;
- included explicit dictionary source proto files in the `protoc` descriptor
  request;
- composed dictionaries from the target source file first, then explicit
  dictionary source files in request order;
- preserved existing source-file-local dictionary behavior when no explicit
  dictionary source is provided;
- reused existing duplicate dictionary-name validation;
- added focused CLI and descriptor tests for the approved behavior.

## Validation Environment

Working directory:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Validation commands were run in the exact implementation-plan order.

## Validation Results

### 1. `cargo fmt --check`

Result: passed.

Observed output:

```text
<no output>
```

### 2. `cargo check -p mbt_codegen`

Result: passed.

Observed output:

```text
Checking mbt_codegen v0.1.1 (.../metamorphic-binary-transport/crates/codegen)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
```

### 3. `cargo test -p mbt_codegen cli_accepts_repeated_dictionary_source_flags -- --nocapture`

Result: passed.

Observed output:

```text
test tests::test_cli::cli_accepts_repeated_dictionary_source_flags ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 43 filtered out
```

### 4. `cargo test -p mbt_codegen cli_rejects_empty_dictionary_source -- --nocapture`

Result: passed.

Observed output:

```text
test tests::test_cli::cli_rejects_empty_dictionary_source ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 43 filtered out
```

### 5. `cargo test -p mbt_codegen imported_dictionary_source_satisfies_dictionary_field -- --nocapture`

Result: passed.

Observed output:

```text
test tests::test_descriptor::imported_dictionary_source_satisfies_dictionary_field ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 43 filtered out
```

### 6. `cargo test -p mbt_codegen missing_explicit_dictionary_source_fails -- --nocapture`

Result: passed.

Observed output:

```text
test tests::test_descriptor::missing_explicit_dictionary_source_fails ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 43 filtered out
```

### 7. `cargo test -p mbt_codegen duplicate_dictionary_names_across_sources_fail -- --nocapture`

Result: passed.

Observed output:

```text
test tests::test_descriptor::duplicate_dictionary_names_across_sources_fail ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 43 filtered out
```

### 8. `cargo test -p mbt_codegen schema_hash_changes_when_explicit_dictionary_values_change -- --nocapture`

Result: passed.

Observed output:

```text
test tests::test_descriptor::schema_hash_changes_when_explicit_dictionary_values_change ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 43 filtered out
```

### 9. `cargo test -p mbt_codegen source_file_local_dictionary_behavior_still_works -- --nocapture`

Result: passed.

Observed output:

```text
test tests::test_descriptor::source_file_local_dictionary_behavior_still_works ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 43 filtered out
```

### 10. `cargo test -p mbt_codegen dictionary -- --nocapture`

Result: passed.

Observed output:

```text
running 7 tests
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out
```

### 11. `cargo test -p mbt_codegen -- --nocapture`

Result: passed.

Observed output:

```text
running 44 tests
test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
Doc-tests mbt_codegen
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 12. `cargo tree -p mbt_codegen --edges normal`

Result: passed.

Observed output:

```text
mbt_codegen v0.1.1 (.../metamorphic-binary-transport/crates/codegen)
├── prost-reflect v0.16.4
└── thiserror v2.0.17
```

The full tree output showed only the existing normal dependency surface:
`prost-reflect`, `prost`, `prost-types`, `bytes`, `thiserror`, and their
existing proc-macro dependencies.

### 13. `git diff -- Cargo.toml Cargo.lock`

Result: passed.

Observed output:

```text
<no output>
```

## Dependency-Change Proof

No dependency manifest changes were introduced.

Evidence:

```text
git diff -- Cargo.toml Cargo.lock
```

produced no output.

## Generated-File Proof

No generated files, schema files, runtime crates, adapters, MBT-PG, MBT-Cache,
or benchmarks were intentionally edited. The approved implementation touched
only the listed `crates/codegen` source/test files and this review artifact.

## Invariants Proved

- Explicit dictionary sources can satisfy dictionary fields without copying
  dictionary definitions into the root source file.
- Existing source-file-local dictionary behavior remains valid.
- Duplicate dictionary names across explicit sources fail before emission.
- Explicit dictionary values participate in normalized schema hashing.
- The implementation compiles in `mbt_codegen`.
- The full `mbt_codegen` unit and doc-test suite passes.
- No dependency changes were introduced.

## Remaining Claims

This result review does not claim downstream schema-registry adoption. The next
schema-registry step must use `--dictionary-source` explicitly and validate its
own generated artifacts under its own protocol chain.
