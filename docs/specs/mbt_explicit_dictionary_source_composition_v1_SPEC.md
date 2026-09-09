# SPEC: MBT Explicit Dictionary Source Composition V1

## 1. Identification

Slug: `mbt_explicit_dictionary_source_composition_v1`

Repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Research brief:

```text
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_research_brief.md
```

External consumer spec:

```text
/home/tia/_DEV/MATHILDE/mathilde-mbt-schemas/docs/specs/schema_registry_shared_instrument_dictionary_v1_SPEC.md
```

## 2. Status

Status: `PEER_AUDIT_PASSED_IMPLEMENTATION_PLAN_WRITTEN_AWAITING_APPROVAL`

This spec does not authorize code changes.

Implementation may start only after:

1. the implementation plan is explicitly approved;
2. implementation rereads the locked protocols and approved artifacts.

## 3. Purpose

Allow MBT codegen callers to provide explicit dictionary source proto files.

The intended schema pattern is:

```text
shared dictionary proto owns option (mathilde.dictionary_values)
domain schema imports the shared dictionary proto
domain field references (mathilde.dictionary) = "instrument"
caller passes --dictionary-source shared/dictionary.proto
```

This removes the need to duplicate dictionary value lists in every domain
schema while keeping dictionary availability explicit and deterministic.

## 4. Non-goals

This spec does not:

- add a new MBT protobuf option;
- change MBT runtime, envelope, archive bytes, checked access, or trusted
  access;
- change generated Rust runtime semantics except for schemas that now resolve
  dictionary fields through explicit sources;
- scan all transitive imports for dictionaries;
- allow duplicate dictionary owners;
- edit application schemas;
- regenerate generated schema files;
- change MBT-PG or MBT-Cache;
- add dependencies;
- make runtime performance claims.

## 5. Measured object

The measured object is `mbt_codegen` descriptor/model construction:

```text
CodegenConfig / SchemaRequest
  -> protoc descriptor set
  -> DescriptorPool
  -> composed dictionary list
  -> SchemaModel
  -> normalized schema hash
```

The concrete behavior is generic to MBT codegen. The immediate consumer is the
schema-registry shared instrument dictionary, but no schema-registry file is
edited by this spec.

## 6. Schema source contract

The schema source of truth remains:

```text
.proto files + approved MBT custom options
```

Dictionary sources are ordinary proto files that contain file-level:

```proto
option (mathilde.dictionary_values) = {
  name: "..."
  value: "..."
};
```

Rules:

- dictionary sources must be passed explicitly by the caller;
- dictionary source paths use the same proto-root-relative identity style as
  `--schema`;
- dictionary source files must be part of the same `protoc` descriptor set as
  the target schema;
- domain schemas still reference dictionaries with existing
  `(mathilde.dictionary)` or `(mathilde.bitmask_dictionary)` options;
- no new protobuf option is introduced.

## 7. Wire and archive contract

The wire/archive contract is unchanged.

This feature only changes how codegen finds dictionary values before generating
schema code. A field that resolves to `FieldKind::U16Dictionary` or
`FieldKind::U64BitmaskDictionary` keeps the existing physical encoding and
generated runtime contract.

## 8. Checked and trusted access contract

Checked and trusted access are unchanged.

Dictionary source composition happens at codegen time. It does not change
runtime archive validation, checked access, trusted access, unsafe boundaries,
or payload header validation.

## 9. Codegen contract

### Config and request surface

Add:

```rust
pub dictionary_sources: Vec<PathBuf>
```

to:

```text
crates/codegen/src/config.rs::CodegenConfig
crates/codegen/src/config.rs::SchemaRequest
```

`CodegenConfig::schema_request()` must clone this list into `SchemaRequest`.

Add repeatable CLI flag:

```text
--dictionary-source <proto-file>
```

Rules:

- allowed with `--inspect`, `--write`, and `--check`;
- allowed with `core`, `projection`, and `metamorphose` surfaces;
- repeatable;
- rejected when the value is empty;
- absent flag means an empty list and preserves existing behavior.

### Descriptor set construction

`run_protoc` must pass the target schema input path and every explicit
dictionary source input path to `protoc`.

The command must keep:

```text
--include_imports
--include_source_info
--experimental_allow_proto3_optional
```

Dictionary source paths must be resolved using the same proto-root search
policy as the target schema path.

### Dictionary composition

`load_schema_model` must load dictionaries in this order:

1. target source file;
2. explicit dictionary source files in request order.

The composed dictionary list is stored in:

```text
SchemaModel.dictionaries
```

Existing `require_dictionary` must then resolve against that composed list.

Existing `validate_dictionaries` must validate the composed list. Duplicate
dictionary names are rejected, including root-source plus explicit-source
duplicates and duplicate names across explicit sources.

Duplicate values inside one dictionary remain rejected.

### Hash behavior

`normalized_hash` already iterates `model.dictionaries`.

Because composed dictionaries must be stored in `SchemaModel.dictionaries`,
the normalized schema hash must include the composed dictionary values.

No separate hash algorithm is approved.

### Existing behavior preservation

When `dictionary_sources` is empty, behavior must remain source-file-local:

```text
target source file dictionaries only
```

Existing local dictionary fixtures must continue to pass.

## 10. Crate boundary contract

Only `crates/codegen` is in scope.

In scope:

```text
crates/codegen/src/config.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
```

Out of scope:

```text
crates/core/**
crates/metamorphose/**
crates/transponding/**
crates/compression/**
crates/adapters/**
crates/schemas/**
proto/**
```

No generated artifacts are produced by this MBT task.

## 11. Dependency contract

No dependency changes are approved.

Existing dependencies are sufficient:

- `prost-reflect` already provides descriptor pools and file descriptors;
- `protoc` is already used by `crates/codegen/src/descriptor.rs`;
- standard library path handling is already used in `config.rs` and
  `descriptor.rs`.

## 12. Determinism contract

Composition is deterministic because:

- root source dictionaries are loaded first;
- explicit dictionary source files are loaded in request order;
- duplicate dictionary names fail instead of being merged;
- values inside each dictionary preserve proto option order;
- normalized hash uses the resulting `SchemaModel.dictionaries` order.

No sorted or transitive import scan is approved.

## 13. Failure contract

Required failures:

- `--dictionary-source` without a value fails CLI parsing;
- empty `--dictionary-source ""` fails CLI parsing;
- dictionary source path missing from proto roots fails before model emission;
- dictionary source descriptor missing from the descriptor pool fails before
  model emission;
- duplicate dictionary name in root plus source fails;
- duplicate dictionary name across sources fails;
- duplicate dictionary value in one source fails;
- field referencing a dictionary absent from the composed list fails exactly as
  today.

No fallback to raw strings or unknown dictionary pass-through is approved.

## 14. Compile-surface budget

Compile-surface impact is bounded to `mbt_codegen`.

Expected impact:

- no new crates;
- no new dependencies;
- no runtime crates touched;
- no generated schema crates touched;
- no adapter crates touched.

Validation must include:

```text
cargo check -p mbt_codegen
cargo tree -p mbt_codegen --edges normal
```

The dependency tree is expected to remain unchanged.

## 15. Runtime performance budget

No runtime performance change is expected because this is codegen-only.

Codegen runtime may perform one descriptor lookup per explicit dictionary
source. This is acceptable because codegen is not a runtime hot path.

No performance claim is made by this spec.

## 16. Correctness oracle

Correctness is proved by descriptor/model tests:

1. a target schema with no local dictionary and an explicit imported dictionary
   source resolves a dictionary field successfully;
2. the same target schema without the explicit source fails with missing
   dictionary;
3. duplicate dictionary names across composed sources fail;
4. existing source-file-local dictionary fixtures still pass with an empty
   source list;
5. changing a dictionary value in an explicit source changes the normalized
   schema hash;
6. inspect/model output exposes the composed dictionary in
   `SchemaModel.dictionaries`.

## 17. Benchmark methodology

No benchmark is required.

Reason: the work is codegen descriptor/model construction, not runtime
transport, archive access, storage, or adapter conversion. Performance claims
are explicitly out of scope.

## 18. Test plan

Add or update tests only under:

```text
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
```

Required tests:

```text
cli_accepts_repeated_dictionary_source_flags
cli_rejects_empty_dictionary_source
imported_dictionary_source_satisfies_dictionary_field
missing_explicit_dictionary_source_fails
duplicate_dictionary_names_across_sources_fail
schema_hash_changes_when_explicit_dictionary_values_change
source_file_local_dictionary_behavior_still_works
```

The implementation may combine assertions only if the test name and failure
message still identify the failing contract.

## 19. Code bindings

Edit only:

```text
crates/codegen/src/config.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
```

Required `config.rs` changes:

- add `dictionary_sources: Vec<PathBuf>` to `CodegenConfig`;
- add `dictionary_sources: Vec<PathBuf>` to `SchemaRequest`;
- clone `dictionary_sources` in `schema_request`;
- parse repeated `--dictionary-source`;
- reject empty values.

Required `descriptor.rs` changes:

- resolve dictionary source input paths with the same helper behavior as
  target schema input paths;
- include dictionary source inputs in `run_protoc`;
- load dictionary source file descriptors from the descriptor pool;
- compose dictionaries in root-then-source order;
- call `validate_dictionaries` on the composed list before field traversal.

Required tests support changes:

- update test config helpers to set `dictionary_sources: Vec::new()`;
- add a helper to construct config with dictionary sources, or set the field in
  each new test explicitly.

## 20. Generated artifact bindings

No generated artifacts are approved.

No files under these paths may be edited by this task:

```text
crates/schemas/**
target/**
```

Generated schema artifacts in `mathilde-mbt-schemas` are explicitly out of
scope until this MBT task is released and consumed there.

## 21. Review artifact bindings

This chain owns:

```text
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_research_brief.md
docs/specs/mbt_explicit_dictionary_source_composition_v1_SPEC.md
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_peer_audit.md
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_implementation_plan.md
```

Later implementation must write:

```text
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_result_review.md
```

## 22. Implementation plan requirement

The implementation plan must bind:

- exact files to edit;
- no dependency changes;
- no generated files;
- exact CLI flag behavior;
- exact descriptor composition helper behavior;
- exact tests;
- exact validation commands and expected outputs;
- rollback boundary;
- known risks.

## 23. Approval checklist

Pre-audit closure:

- mandatory section order matches `docs/protocols/spec_protocol.md`;
- no prior approved spec is superseded except the schema-registry stop gate
  that this work resolves;
- exact command surface is bound as `--dictionary-source <proto-file>`;
- there are no generated artifacts;
- dispatch path is bound through `CodegenConfig::schema_request`,
  `load_schema_model`, `run_protoc`, and dictionary resolution;
- existing local dictionary tests are preserved;
- new imported dictionary behavior tests are bound;
- code paths are exact;
- compile-surface evidence commands are defined;
- no design decision is deferred to the implementation plan.

Implementation readiness after peer audit:

- peer audit must classify `PEER_AUDIT_PASSED`;
- implementation plan must be written and explicitly approved;
- code changes must stay inside the paths listed in this spec.

## 24. Open questions

None blocking.

Non-blocking implementation detail:

- helper function names may vary if they preserve the exact behavior and file
  boundaries in this spec.
