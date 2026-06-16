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

# SPEC: MBT Production Commenting

## 1. Identification

Spec slug: `mbt_production_commenting`

Repository: `/home/tia/_DEV/MATHILDE/metamorphic-binary-transport`

Research brief:

- `docs/reviews/mbt_production_commenting/mbt_production_commenting_research_brief.md`

Required future artifacts:

- peer audit:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_peer_audit.md`
- implementation plan:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_implementation_plan.md`
- implementation plan peer audit:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_implementation_plan_peer_audit.md`
- result review:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_result_review.md`

Old MBT reference artifact:

- `/home/tia/_DEV/MATHILDE/experiments/docs/specs/mathilde_binary_transport_production_commenting_SPEC.md`

Related current MBT artifacts:

- `docs/architecture/repository_structure.md`
- `docs/specs/mbt_workspace_architecture_SPEC.md`
- `docs/specs/mbt_core_runtime_migration_SPEC.md`
- `docs/specs/mbt_codegen_migration_SPEC.md`
- `docs/specs/mbt_schema_core_generation_SPEC.md`
- `docs/specs/mbt_projection_direct_writer_SPEC.md`
- `docs/specs/mbt_metamorphose_migration_SPEC.md`

## 2. Status

Status: `DRAFT_AWAITING_PEER_AUDIT`

This spec authorizes no source-code change.

A separate peer audit is required before implementation planning.

An approved implementation plan and implementation-plan peer audit are required
before any code file is changed.

## 3. Purpose

Add sparse, production-grade comments to the split MBT workspace where comments
make the system easier to audit.

The comments must explain non-obvious production boundaries:

- checked versus trusted access;
- schema/archive identity;
- cap-aware boundary output;
- explicit copy and allocation boundaries;
- MBT-to-MBT projection before boundary conversion;
- metamorphose format dispatch;
- transponding validity and offset semantics;
- adapter boundary ownership;
- codegen descriptor and emitter dispatch;
- benchmark timing and evidence ownership.

This is a reviewability and maintenance task. It must not change runtime
behavior.

## 4. Non-goals

- No MBT wire format change.
- No archive layout change.
- No schema option change.
- No public API change.
- No private API behavior change.
- No dependency change.
- No feature flag change.
- No benchmark logic change.
- No test behavior change.
- No generated schema file hand edits.
- No generated output change from codegen string literals.
- No warning cleanup unrelated to comments.
- No formatting rewrite unrelated to comments.
- No performance claim.
- No compile-time claim.
- No README rewrite.

## 5. Measured object

The measured object is comment coverage and reviewability of hand-owned MBT
source files in the production workspace.

In scope:

- core runtime source;
- metamorphose runtime source;
- transponding runtime source;
- adapter runtime source;
- codegen source;
- benchmark/evidence support source when a comment clarifies timing,
  baseline, or report ownership.

Out of scope:

- generated schema crate source files;
- generated adapter/source output;
- tests unless a later implementation plan proves a test helper comment is
  required;
- dependency documentation;
- external SDK documentation.

## 6. Schema source contract

No schema source changes are allowed.

The following schema inputs must remain unchanged:

- `proto/mathilde/options.proto`
- `proto/mathilde/binary_transport/v1/bars.proto`
- `proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto`

Generated schema modules remain codegen-owned. Manual comment edits are
forbidden in:

- `crates/schemas/bars_core/src/*.rs`
- `crates/schemas/test_compatibility_core/src/*.rs`

If generated API documentation is later needed, it must be emitted through
`crates/codegen/src/rust_emit.rs` and regenerated through approved codegen
commands. This spec does not authorize changing generated output.

## 7. Wire and archive contract

The existing wire and archive contract remains unchanged:

```text
.proto + MBT options
  -> deterministic generated schema code
  -> checked MBT envelope
  -> rkyv archive payload
  -> checked access or unsafe trusted access
  -> optional boundary adapter
```

Comments may explain this contract. Comments must not redefine it.

No change is allowed to:

- header size;
- magic bytes;
- transport version;
- encoding kind;
- schema ID;
- schema version;
- schema hash;
- normalized proto hash;
- payload checksum;
- row count;
- row ordering;
- projection schema identity.

## 8. Checked and trusted access contract

Comments may clarify:

- checked access validates bytes before archive access;
- trusted access is for immutable bytes already accepted by checked access;
- trusted public helpers are unsafe;
- generated trusted implementations receive `TrustedUnchecked` only through
  unsafe public helper paths.

Comments must not weaken or hide trusted-access requirements.

Existing generated safety documentation may be referenced by source comments,
but this spec does not authorize changing generated safety documentation.

## 9. Codegen contract

Codegen source files may receive local comments where they explain descriptor
flow, CLI dispatch, option handling, generated surface dispatch, projection
emission, metamorphose adapter emission, transponding emission, and
compile-surface ownership.

Allowed codegen source files are listed in Section 19.

Forbidden codegen changes:

- changing generated output strings;
- changing generated artifact headers;
- changing schema hash logic;
- changing command-line behavior;
- changing write/check/inspect behavior;
- changing generated APIs;
- adding schema-specific hardcoding;
- editing generated files by hand.

Comment changes inside `crates/codegen/src/rust_emit.rs` must be comments in
the generator source itself, not changes to emitted generated source text.

## 10. Crate boundary contract

The existing split workspace boundary remains unchanged:

- `crates/core` owns envelope, errors, output caps, checksums, and runtime
  traits.
- `crates/metamorphose` owns public boundary dispatch traits and trusted token.
- `crates/transponding` owns shared row-to-column buffer contracts.
- `crates/adapters/*` own format-specific boundary writers.
- `crates/codegen` owns descriptor parsing and Rust emission.
- `crates/benches` owns benchmark and evidence harnesses.
- `crates/schemas/*` are generated schema crates and are not manually edited.

No crate may gain a new dependency or feature from this task.

## 11. Dependency contract

No dependency may be added, removed, or version-changed.

No Cargo manifest may be edited by this task.

No new environment variable, config knob, CLI flag, or workspace member may be
added.

## 12. Determinism contract

The implementation must be comment-only for source files and docs-only for
review artifacts.

It must not change:

- generated source bytes;
- MBT encoded bytes;
- projected MBT bytes;
- JSON bytes;
- protobuf bytes;
- CSV bytes;
- Arrow batches;
- Arrow IPC bytes;
- Parquet bytes;
- response checksums;
- semantic checksums;
- schema hashes;
- normalized proto hashes;
- benchmark reports;
- test expectations.

If any non-comment source diff appears, implementation must stop for diagnosis
or a new approved amendment.

## 13. Failure contract

Existing failure behavior must remain unchanged:

- corrupt payload rejection;
- wrong schema rejection;
- wrong version rejection;
- payload length mismatch rejection;
- payload checksum mismatch rejection;
- response cap rejection;
- non-finite numeric rejection;
- invalid dictionary rejection;
- invalid bitmask rejection;
- invalid row order rejection;
- invalid projection rejection;
- invalid adapter output cap handling.

No `unwrap`, `expect`, `panic!`, `todo!`, `unreachable!`, or hidden defaulting
may be introduced.

## 14. Compile-surface budget

The intended compile-surface delta is zero.

Manual comments must not:

- add code;
- add dependencies;
- add features;
- add generated output;
- force unrelated adapters or schemas to compile.

Minimum compile-surface validation after implementation:

```bash
cargo fmt --check
cargo check -p metamorphic_binary_transport_core --all-targets
cargo check -p metamorphic_binary_transport_metamorphose --all-targets
cargo check -p metamorphic_binary_transport_transponding --all-targets
cargo check -p metamorphic_binary_transport_codegen --all-targets
```

Adapter checks may be added by the implementation plan if adapter files are
edited:

```bash
cargo check -p metamorphic_binary_transport_adapter_json --all-targets
cargo check -p metamorphic_binary_transport_adapter_csv --all-targets
cargo check -p metamorphic_binary_transport_adapter_protobuf --all-targets
cargo check -p metamorphic_binary_transport_adapter_arrow --all-targets
cargo check -p metamorphic_binary_transport_adapter_arrow_ipc --all-targets
cargo check -p metamorphic_binary_transport_adapter_parquet --all-targets
```

Benchmark crate check may be added if benchmark support files are edited:

```bash
cargo check -p metamorphic_binary_transport_benches --all-targets
```

No compile-time improvement claim is allowed.

## 15. Runtime performance budget

The intended runtime-performance delta is zero.

No benchmark is required because comment-only changes do not change runtime
code. A later implementation plan may run existing benchmarks as regression
guards, but no speed claim may be made from this task.

## 16. Correctness oracle

The correctness oracle is behavior preservation through diff shape:

1. Review the source diff.
2. Confirm every production source change is comment-only.
3. Confirm no generated schema source file changed.
4. Confirm no Cargo manifest changed.
5. Run formatting and narrow cargo checks from Section 14.

Diff inspection must be recorded in the result review.

Allowed source diff forms:

- adding `//`, `///`, or `//!` comments;
- refining an existing comment;
- removing a redundant or misleading comment.

Forbidden source diff forms:

- changing Rust expressions;
- changing imports;
- changing type definitions;
- changing function signatures;
- changing generated string literals in codegen;
- changing tests;
- changing benchmark measured logic;
- changing manifests.

## 17. Benchmark methodology

No benchmark is bound by this spec.

If implementation later runs a benchmark, it is only a regression guard and
must not be interpreted as an improvement or performance claim.

## 18. Test plan

Minimum checks after implementation:

```bash
git diff --check
cargo fmt --check
cargo check -p metamorphic_binary_transport_core --all-targets
cargo check -p metamorphic_binary_transport_metamorphose --all-targets
cargo check -p metamorphic_binary_transport_transponding --all-targets
cargo check -p metamorphic_binary_transport_codegen --all-targets
```

If adapter files are edited, the corresponding adapter crate checks from
Section 14 must be run.

If benchmark support files are edited,
`cargo check -p metamorphic_binary_transport_benches --all-targets` must be
run.

No generated-code check command is required because this spec does not
authorize generated-output changes.

## 19. Code bindings

Only the files in this section may be edited, and only through comments.

The implementation plan must bind the exact subset it will touch. It may omit
any anchor when local code is already clear or a nearby existing comment already
covers the concept.

### `crates/core/src/codec.rs`

Allowed comment anchors:

- `response_checksum`: explain this is deterministic evidence checksum logic,
  not a cryptographic integrity primitive.

### `crates/core/src/envelope.rs`

Allowed comment anchors:

- `SchemaHeaderSpec`: explain the schema identity expected by envelope checks.
- `TransportHeader`: explain that headers bind schema identity, row count,
  payload length, and payload checksum.
- `normalized_proto_hash`: explain line-ending normalization for stable hashes.
- `encode_header`: explain the fixed 128-byte little-endian envelope.
- `decode_header`: explain header-only parsing before payload access.
- `validate_header_for_schema`: explain checked payload checksum validation.
- `trusted_payload_for_schema`: explain envelope/schema/length validation with
  payload integrity delegated to the trusted caller contract.
- `validate_identity_and_len`: explain shared checked/trusted identity and
  length gate.

### `crates/core/src/output.rs`

Allowed comment anchors:

- `CheckedBytes`: explain capped owned output for generated boundary writers.
- `encode_protobuf`: explain direct prost append followed by cap validation.
- `fmt_error`: explain conversion from `fmt::Error` back to cap error.
- `checked_len_add`: explain protobuf length preflight.
- `write_utc`: explain UTC output into `CheckedBytes` without an intermediate
  `String`.
- `utc_bytes`: explain stack-buffer UTC output for writers that need a borrowed
  byte slice.
- `write_base64`: explain bytes-to-text boundary output.
- `civil_from_days`: explain local civil-date conversion and no time crate
  dependency.

### `crates/core/src/runtime.rs`

Allowed comment anchors:

- `BinaryInspection`: explain inspect evidence without decoding into DTOs.
- `MbtSchema`: explain generated schema marker dispatch for encode/access.
- `encode`, `encode_owned`, `access`, `inspect`: explain generic dispatch only
  when this would not duplicate trait method names.

### `crates/metamorphose/src/lib.rs`

Allowed comment anchors:

- module-level comment only, if needed, to state that public exports live here
  and implementation lives in `runtime.rs`.

### `crates/metamorphose/src/runtime.rs`

Allowed comment anchors:

- `TrustedUnchecked`: explain token construction as an unsafe trusted-path
  guard.
- `MetamorphoseFormat`: explain safe format dispatch.
- `MetamorphoseOutput`: explain borrowed MBT output versus owned boundary
  format outputs.
- `MbtMetamorphoseSchema`: explain MBT default path.
- adapter traits: explain each trait is implemented by generated schema adapter
  modules, not by hand-written DTOs.
- `decode`: explain format selection delegates validation and writing to the
  schema/adapters.
- unsafe trusted helper functions: retain safety docs and refine only if a
  comment is unclear.

### `crates/transponding/src/lib.rs`

Allowed comment anchors:

- module-level comment only, if needed, to state that shared column buffer
  contracts live in `runtime.rs`.

### `crates/transponding/src/runtime.rs`

Allowed comment anchors:

- `ValidityBitmap`: explain one bit per row and optional/null semantics.
- `DictionaryMeta`: explain metadata carried into columnar adapters.
- `ConstU16Column`: explain constant dictionary columns without per-row values.
- `required_numeric_column`: explain identical required primitive column
  behavior across numeric types.
- `optional_numeric_column`: explain physical default values plus validity
  bits.
- `numeric_list_column`: explain offsets, values, and optional row validity for
  null-versus-empty array semantics.
- `BoolColumn`: explain bool uses byte-count size accounting before adapter
  packing.
- `Utf8Column`: explain concatenated bytes plus offsets and optional validity.
- `BinaryColumn`: explain binary mirrors UTF-8 layout without string
  validation.
- `ensure_columnar_size`: explain cap checking before columnar boundary output.
- `checksum_seed` and update helpers: explain deterministic evidence checksum
  composition.

### `crates/adapters/json/src/lib.rs`

Allowed comment anchors:

- `JsonWriter`: explain direct archived-field JSON writing through
  `CheckedBytes`.
- string/bytes helpers: explain escaping and base64 happen only at the JSON
  boundary.
- array helpers: explain direct array emission without intermediate DTOs.

### `crates/adapters/csv/src/lib.rs`

Allowed comment anchors:

- `CsvWriter`: explain direct capped CSV writing.
- `string_cell`: explain CSV escaping and quoting at the boundary.
- bytes helper: explain base64 text representation.
- array helper: explain JSON-style cell payload for repeated values.

### `crates/adapters/protobuf/src/lib.rs`

Allowed comment anchors:

- `ProtoWriter`: explain direct protobuf wire writing.
- string/bytes helpers: explain direct length-delimited fields.
- packed repeated helpers: explain packed numeric encoding.
- encoded length helpers: explain preflight size calculation.

### `crates/adapters/arrow/src/lib.rs`

Allowed comment anchors:

- `schema_metadata`: explain Arrow metadata carries MBT schema identity.
- `field_metadata`: explain dictionary and bitmask metadata propagation.
- primitive array helpers: explain transponded columns become Arrow arrays.
- list array helpers: explain offsets, values, and validity mapping.
- `record_batch`: explain schema/array assembly and row-count validation.
- `record_batch_byte_len`: explain approximate in-memory response accounting.
- `record_batch_checksum`: explain deterministic evidence checksum.
- `null_buffer`: explain validity bitmap conversion.
- `ensure_offsets`: explain monotonic variable-width/list boundaries.
- `checksum_array`: explain evidence checksum, not cryptographic hashing.

### `crates/adapters/arrow_ipc/src/arrow_bridge.rs`

Allowed comment anchors:

- same Arrow bridge concepts as `crates/adapters/arrow/src/lib.rs`, limited to
  anchors actually present in this bridge file.

### `crates/adapters/arrow_ipc/src/lib.rs`

Allowed comment anchors:

- `write_ipc_stream`: explain Arrow IPC as boundary encoding with cap
  enforcement.
- `record_batch_from_ipc_stream`: explain test/validation decode helper.
- `CheckedArrowIpcWriter`: explain sink-level cap enforcement.
- `initial_capacity`: explain sizing hint versus authoritative cap.
- `arrow_ipc_or_overflow_error`: explain overflow remapping.

### `crates/adapters/parquet/src/arrow_bridge.rs`

Allowed comment anchors:

- same Arrow bridge concepts as `crates/adapters/arrow/src/lib.rs`, limited to
  anchors actually present in this bridge file.

### `crates/adapters/parquet/src/lib.rs`

Allowed comment anchors:

- `write_uncompressed_parquet`: explain uncompressed Parquet boundary output
  and external compression policy.
- `CheckedParquetWriter`: explain cap-aware sink behavior.
- `parquet_or_overflow_error`: explain overflow remapping.

### `crates/codegen/src/config.rs`

Allowed comment anchors:

- `Action`: explain inspect/write/check command mode.
- `Surface`: explain core/projection/metamorphose generation split.
- `Adapter`: explain adapter generation is opt-in under metamorphose surface.
- `CodegenConfig`: explain normalized CLI contract.
- `SchemaRequest`: explain schema identity passed to descriptor loading.
- `parse_args`: explain explicit proto-root/schema/root/module/surface
  contract.

### `crates/codegen/src/descriptor.rs`

Allowed comment anchors:

- `load_schema_model`: explain descriptor options to `SchemaModel`.
- `row_payload_field`: explain exactly-one repeated row payload requirement.
- `collect_schema_fields`: explain physical and target traversal.
- `resolve_derived_utc_fields`: explain timestamp-derived target fields.
- `validate_row_format_outputs`: explain row-format output model checks.
- `physical_field`: explain MBT physical annotation mapping.
- `dictionaries_from_file`: explain file-level dictionary options.
- `validate_physical_fields`: explain unique names, presence bits, and key
  order.
- `projection_models`: explain root projection options to generated models.
- `build_projection_model`: explain projection presence rebasing and hash.
- `selected_projection_indices`: explain include/exclude selection.
- `validate_projection_fields`: explain mandatory keys and non-empty payload.
- `raw_descriptor_set`: explain temporary descriptor-set generation through
  `protoc`.
- `normalized_hash`: explain model-derived schema hash.

### `crates/codegen/src/emit.rs`

Allowed comment anchors:

- `inspect`: explain text inspection without writing files.
- `write`: explain generated output write path.
- `check`: explain temp regeneration and byte-for-byte comparison.
- `smoke_crate`: explain compile-surface probe crate.
- `format_rust`: explain rustfmt normalization before write/check.
- `write_if_changed`: explain unchanged generated artifacts are not rewritten.
- `generated_unformatted`: explain surface/adapter dispatch.

### `crates/codegen/src/model.rs`

Allowed comment anchors:

- `Dictionary`: explain dictionary option model.
- `SchemaModel`: explain physical archive, target output, projection, and
  derived field model.
- `DerivedUtcField`: explain target-only UTC derived output.
- `JsonCsvOutputField`: explain row-format output ordering.
- `ProtobufMessageModel`: explain nested protobuf target output.
- `ProjectionModel`: explain generated MBT-to-MBT projection model.
- `PhysicalField`: explain stored archive field.
- `FieldKind`: explain MBT physical primitive set.
- `validate_module_name`: explain stable Rust snake_case generated modules.

### `crates/codegen/src/options.rs`

Allowed comment anchors:

- `MbtExtensions`: explain centralized MBT protobuf options.
- `extensions`: explain descriptor-pool lookup of approved options.
- required/optional option helpers: explain strict option type validation.
- `dictionary_from_value`: explain dictionary option conversion.

### `crates/codegen/src/rust_emit.rs`

Allowed comment anchors:

- `generated_schema`: explain generated core schema module orchestration.
- `generated_projection_schema`: explain projection surface generation.
- `generated_metamorphose_adapter_schema`: explain adapter-specific generation.
- `emit_metamorphose_json`, `emit_metamorphose_protobuf`, `emit_metamorphose_csv`:
  explain direct boundary writer generation.
- `emit_metamorphose_transponding`, `emit_metamorphose_arrow`,
  `emit_metamorphose_arrow_ipc`, `emit_metamorphose_parquet`: explain
  columnar adapter generation path.
- `emit_column_batch`: explain generated row-to-column batch bridge.
- `emit_runtime_api`: explain generated encode/access/project API.
- `emit_direct_projection_wrappers`: explain direct MBT-to-MBT projection.
- `emit_projection_presence_repack`: explain presence-bit repacking.
- `presence_words`, `presence_is_wide`, `presence_storage_type`: explain
  narrow versus wide presence storage generation.
- `emit_header`: explain deterministic generated header.
- `emit_imports`: explain core imports shared by generated schema modules.
- `emit_validation`: explain generated byte-admission validation.
- `emit_order_validation`: explain deterministic row-order validation.
- `emit_checksums`: explain semantic and minimal checksum emission.
- `emit_view_types`: explain borrowed archived row view generation.
- `emit_decode_helpers`: explain checked and trusted archive access helpers.
- helper functions that map field kinds to output writers, Arrow arrays, or
  column types when a local comment prevents audit ambiguity.

### `crates/codegen/src/lib.rs`

Allowed comment anchors:

- existing module-level comment may be refined only if it is inaccurate after
  current workspace migration.

### `crates/codegen/src/main.rs`

Allowed comment anchors:

- `run`: explain CLI exits through typed codegen errors rather than panics.

### `crates/benches/src/lib.rs`

Allowed comment anchors:

- existing module-level comment may be refined only if it is inaccurate.

### `crates/benches/src/bars_regression.rs`

Allowed comment anchors:

- `OLD_PARITY_EVIDENCE_GLOB`: explain historical oracle source.
- `REQUIRED_SCHEMA_FEATURES`: explain adapter feature set for regression
  lanes.
- `BarsRegressionRow`: explain one measured lane result.
- `SerdeBarRow`: explain direct baseline DTO used only by the benchmark crate.
- `metadata_for_run`: explain recorded environment and dirty-state evidence.
- `write_report`: explain JSON evidence artifact ownership.
- parser helpers: explain old evidence parsing only if local code is unclear.

### `crates/benches/src/projection.rs`

Allowed comment anchors:

- `OLD_BENCH_RESULTS`: explain old-crate baseline table source.
- `SchemaName`: explain benchmark schema lane selection.
- `BenchRow`: explain one projection lane result.
- `parse_old_crate_projection_baselines`: explain old evidence normalization.
- `parse_current_owned_baselines`: explain owned-row baseline comparison.
- `inspection_checksums`: explain semantic/minimal checksum extraction.

### `crates/benches/src/bin/mbt_bars_regression_bench.rs`

Allowed comment anchors:

- `measure_row_count`: explain setup outside timing lanes.
- `measure_full_mbt`: explain checked encode and inspect timing boundary.
- `measure_output`: explain trusted adapter lane timing boundary.
- `measure_serde_json`: explain direct JSON baseline.

### `crates/benches/src/bin/mbt_projection_bench.rs`

Allowed comment anchors:

- `verify_old_baselines`: explain old-baseline sanity gate.
- `measure_bars`: explain Bars projection lane grouping.
- `measure_compatibility`: explain all-fields projection lane grouping.
- `measure_public`: explain checked public projection path.
- `measure_archived`: explain trusted archived projection path.
- `measure_inspect`: explain inspect-only evidence lane.

## 20. Generated artifact bindings

No generated artifact may be edited by hand.

Generated schema files explicitly out of scope:

- `crates/schemas/bars_core/src/bars_v1.rs`
- `crates/schemas/bars_core/src/bars_v1_json.rs`
- `crates/schemas/bars_core/src/bars_v1_protobuf.rs`
- `crates/schemas/bars_core/src/bars_v1_csv.rs`
- `crates/schemas/bars_core/src/bars_v1_transponding.rs`
- `crates/schemas/bars_core/src/bars_v1_arrow.rs`
- `crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs`
- `crates/schemas/bars_core/src/bars_v1_parquet.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs`

No codegen write or check command is required by this spec because generated
output is not allowed to change.

If a later amendment allows generated documentation changes, it must bind:

- exact `rust_emit.rs` string changes;
- exact generated files;
- exact regeneration commands;
- exact codegen check commands;
- generated-output diff inspection proving comments only.

## 21. Review artifact bindings

Already created or required:

- research brief:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_research_brief.md`
- spec:
  `docs/specs/mbt_production_commenting_SPEC.md`

Required next artifacts:

- peer audit:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_peer_audit.md`
- implementation plan:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_implementation_plan.md`
- implementation plan peer audit:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_implementation_plan_peer_audit.md`
- result review:
  `docs/reviews/mbt_production_commenting/mbt_production_commenting_result_review.md`

No evidence artifact is required unless implementation validation produces
command output that must be preserved.

## 22. Implementation plan requirement

Before implementation, the plan must bind:

- exact files to edit;
- exact comment anchors to touch;
- exact wording class for each comment, not necessarily full prose;
- exact files intentionally left untouched;
- validation commands;
- rollback boundary;
- result-review requirements;
- implementation-plan peer audit requirement.

The plan must prove:

- all source diffs are intended to be comment-only;
- generated schema files are untouched;
- generated output string literals are untouched;
- no manifest changes are planned;
- no benchmark behavior changes are planned.

## 23. Approval checklist

Required before peer audit:

- Mandatory section order matches `docs/protocols/spec_protocol.md`: yes.
- Prior approved specs searched and preserved: yes; workspace, core,
  codegen, schema generation, projection, and metamorphose specs were checked
  for boundary conflicts.
- Command surfaces exact: yes; validation commands are listed in Sections 14
  and 18.
- Generated artifacts have one owner: yes; schema files remain codegen-owned
  and out of manual-edit scope.
- No generated artifact has two incompatible command surfaces: yes; no
  generated-output command is bound because output must not change.
- Runtime/codegen dispatch paths listed: yes; allowed source anchors are listed
  in Section 19.
- Tests that encode old behavior handled: yes; tests are out of scope because
  behavior must not change.
- Exact code paths bound before audit: yes; Section 19 binds allowed files and
  anchors.
- No design decision deferred to implementation plan: yes; the plan only
  selects the approved subset of anchors.
- Generated-code compile-surface commands defined if generated code touched:
  not applicable; generated output must not change.

Approval state:

- Research brief written: yes.
- Spec written: yes.
- Peer audit passed: no.
- Implementation plan written: no.
- Implementation plan peer audit passed: no.
- Approved for code implementation: no.

Required next command:

```text
Approved: write docs/reviews/mbt_production_commenting/mbt_production_commenting_peer_audit.md
```

## 24. Open questions

No blocking open question remains for peer audit.

The implementation pass may omit a listed anchor when the local code is already
clear or when adding a comment would duplicate a nearby comment. The result
review must record this as a scope decision, not as missing implementation.
