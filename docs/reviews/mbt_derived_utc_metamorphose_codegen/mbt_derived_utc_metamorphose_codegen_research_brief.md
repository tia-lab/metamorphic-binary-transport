# Research Brief: MBT Derived UTC Metamorphose Codegen

Slug: `mbt_derived_utc_metamorphose_codegen`

Status: `RESEARCH_BRIEF_COMPLETE_SPEC_REQUIRED`

## Source Materials

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/research_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/specs/mbt_bars_regression_benchmark_SPEC.md`
- `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md`
- `proto/mathilde/options.proto`
- `crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto`
- `crates/codegen/src/options.rs`
- `crates/codegen/src/descriptor.rs`
- `crates/codegen/src/model.rs`
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/codegen/src/tests/test_descriptor.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`
- `crates/core/src/output.rs`
- `crates/adapters/json/src/lib.rs`
- `crates/adapters/protobuf/src/lib.rs`
- `crates/adapters/csv/src/lib.rs`
- `crates/schemas/bars_core/src/bars_v1_csv.rs`
- `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`

## Measured Object

The measured object is the generated row-format metamorphose output contract for
schemas that declare ignored UTC string fields derived from millisecond fields:

```text
.proto field with (mathilde.ignored)=true and
(mathilde.derived_utc_from)="source_ms"
  -> not stored in MBT archive
  -> not emitted in MBT-to-MBT projection
  -> emitted only by generated JSON/protobuf/CSV boundary adapters
```

The immediate regression symptom is the blocked Bars old-vs-new benchmark
comparison. The accepted comparison cannot proceed until old and new row-format
outputs share one semantic output contract.

## Candidate Approach

Keep `derived_utc_from` as an existing MBT option and fix only the codegen
model/emitter path:

1. Preserve ignored derived UTC fields as row-format output metadata while still
   excluding them from physical archive fields.
2. Resolve each derived field to an existing physical source timestamp field.
3. Generate JSON/protobuf/CSV writes in flattened proto traversal order.
4. Leave Arrow, Arrow IPC, Parquet, transponding, projection, core wire, and
   archive code unchanged.

## MBT Binding Surface

This research binds:

- schema option parsing through `proto/mathilde/options.proto` and
  `crates/codegen/src/options.rs`;
- schema modeling in `crates/codegen/src/model.rs` and
  `crates/codegen/src/descriptor.rs`;
- row-format adapter emission in `crates/codegen/src/rust_emit.rs`;
- generated Bars row-format adapter files under
  `crates/schemas/bars_core/src/`;
- benchmark result review unblock path for
  `mbt_bars_regression_benchmark`.

## Evidence Table

| Evidence type      | Surface                                                                  | Observation                                                                                                                           |
| ------------------ | ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------- |
| Protocol evidence  | `AGENTS.md`                                                              | Generated code must come only from approved codegen; no code change is allowed before approved spec and approved implementation plan. |
| Protocol evidence  | `docs/invariants/core_invariants.md`                                     | Adapter output must preserve semantic equivalence with the MBT schema; generated code must not emit unrelated adapters by default.    |
| Protocol evidence  | `docs/protocols/codegen_protocol.md`                                     | Generated code must be deterministic, reject unsupported schema shapes, and not require manual generated edits.                       |
| Schema evidence    | `proto/mathilde/options.proto`                                           | `optional string derived_utc_from = 50211;` already exists.                                                                           |
| Schema evidence    | `crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto` | Bars declares `open_utc`, `close_utc`, and metadata UTC fields as ignored strings derived from `*_ms` fields.                         |
| Code-read evidence | `crates/codegen/src/options.rs`                                          | `derived_utc_from` is already loaded into `MbtExtensions`.                                                                            |
| Code-read evidence | `crates/codegen/src/descriptor.rs`                                       | `collect_physical_fields` skips ignored fields before preserving derived UTC metadata.                                                |
| Code-read evidence | `crates/codegen/src/descriptor.rs`                                       | `physical_field` references `extensions.derived_utc_from` only as an unused binding, so no derived model is produced.                 |
| Code-read evidence | `crates/codegen/src/model.rs`                                            | `SchemaModel` has `fields` and `projections`, but no row-format derived-output field model.                                           |
| Code-read evidence | `crates/codegen/src/rust_emit.rs`                                        | JSON, protobuf, and CSV emitters iterate `model.fields`, so ignored derived UTC fields cannot be emitted.                             |
| Code-read evidence | `crates/core/src/output.rs`                                              | `utc_len`, `utc_bytes`, and `write_utc` already exist and write UTC without heap allocation beyond the output buffer.                 |
| Code-read evidence | `crates/adapters/json/src/lib.rs`                                        | `JsonWriter::utc_value(ms)` already exists.                                                                                           |
| Code-read evidence | `crates/adapters/protobuf/src/lib.rs`                                    | `ProtoWriter::utc(tag, ms)` already exists.                                                                                           |
| Code-read evidence | `crates/adapters/csv/src/lib.rs`                                         | `CsvWriter::utc_cell(ms)` already exists.                                                                                             |
| Code-read evidence | `crates/schemas/bars_core/src/bars_v1_csv.rs`                            | Current new Bars CSV header lacks `open_utc`, `close_utc`, and metadata UTC columns.                                                  |
| Code-read evidence | Old experiment generated Bars                                            | Old Bars CSV header includes `open_utc`, `close_utc`, and metadata UTC columns in proto traversal order.                              |
| Result evidence    | `mbt_bars_regression_benchmark_corrective_result_review.md`              | Old/new semantic checksum parity failed structurally; UTC output differences are one diagnosed cause.                                 |

## Hypotheses

- Adding a row-format output model for derived UTC fields should restore the
  missing JSON/protobuf/CSV field contract for Bars. This is unproved until
  generated output tests and the Bars regression benchmark are rerun.
- Runtime throughput should remain close to the previous row-format adapter
  path because UTC helper functions already existed and the output bytes are
  semantically required. This is unproved until benchmark evidence exists.

## Unknowns

- The final old-vs-new performance ratio after semantic output parity is not
  known.
- The exact generated line-count increase is not known until implementation.
- Whether the remaining old/new checksum mismatch has additional causes beyond
  row-format UTC output remains unproved.

## Risks

1. Emitting derived UTC fields in archive/core/projection/columnar surfaces would
   violate the binary transport contract.
2. Appending derived fields after all physical fields would preserve values but
   break deterministic CSV/header and checksum parity with the old row-format
   contract.
3. Resolving `derived_utc_from` by bare field name globally could choose the
   wrong field in nested schemas.
4. Optional source timestamps need presence-aware derived output. Emitting UTC
   for absent optional timestamps would create false data.
5. Hand-editing generated Bars files would violate the generated-code contract.

## Required Decisions Before Spec

The research supports these decisions:

- derived UTC output is row-format adapter behavior only;
- derived UTC fields stay absent from archive fields and projection fields;
- row-format output order is flattened proto traversal order containing physical
  output fields and derived UTC output fields;
- source timestamp resolution is relative to the derived field's parent path
  unless `derived_utc_from` is an explicit dotted row path;
- optional source timestamp presence controls optional derived output;
- columnar adapters do not emit derived UTC by default.

## Evidence Required Before Coding

Before code implementation:

1. a spec must bind exact model, descriptor, emitter, test, generated-file, and
   benchmark surfaces;
2. a separate peer audit must pass;
3. an implementation plan must bind exact edits and validation commands;
4. the implementation plan must be approved.

## Recommended Next Phase

Write:

```text
docs/specs/mbt_derived_utc_metamorphose_codegen_SPEC.md
```

Then request a separate peer audit:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_peer_audit.md
```
