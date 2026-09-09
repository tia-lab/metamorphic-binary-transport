# Implementation Plan Peer Audit: MBT Derived UTC Metamorphose Codegen

Slug: `mbt_derived_utc_metamorphose_codegen`

Target plan:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_implementation_plan.md
```

Classification: `BLOCKED`

## Required Reads

Completed:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`
- `docs/protocols/review_documentation_protocol.md`
- `docs/specs/mbt_derived_utc_metamorphose_codegen_SPEC.md`
- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_peer_audit_v2.md`
- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_implementation_plan.md`
- `crates/codegen/src/model.rs`
- `crates/codegen/src/descriptor.rs`
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`
- `crates/benches/src/bin/mbt_bars_regression_bench.rs`
- `crates/schemas/bars_core/Cargo.toml`
- `crates/codegen/src/config.rs`

## Findings

### 1. Exact evidence and result-review artifacts are not fully bound

Severity: blocking.

Evidence:

- The spec requires compile-surface evidence after implementation:
  `wc -l` for codegen/generated files and two `/usr/bin/time -v cargo check`
  commands.
- The spec states compile-time claims require those outputs to be recorded in
  the result review.
- The spec binds:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_result_review.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
docs/evidence/mbt_derived_utc_metamorphose_codegen/
```

- The implementation plan lists compile-surface commands but does not bind
  exact files under `docs/evidence/mbt_derived_utc_metamorphose_codegen/` for
  those command outputs.
- The implementation plan names only the Bars benchmark evidence directory as a
  rollback/evidence path:

```text
docs/evidence/mbt_bars_regression_benchmark/
```

- The implementation plan mentions recording benchmark or parity failures in a
  result review, but does not bind either result-review file in the file,
  expected-output, or rollback sections.

Assessment:

This violates the spec requirement that the implementation plan bind exact
evidence files. It also leaves the compile-surface evidence without a durable
artifact path, even though generated-code compile surface is part of the
measured risk.

Required amendment:

- Add exact compile-surface evidence files under:

```text
docs/evidence/mbt_derived_utc_metamorphose_codegen/
```

Suggested minimum file set:

```text
docs/evidence/mbt_derived_utc_metamorphose_codegen/codegen_source_wc.txt
docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_row_format_wc.txt
docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_json_protobuf_csv_check_time.txt
docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_arrow_ipc_parquet_check_time.txt
```

- Bind creation or amendment of:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_result_review.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

- Add those evidence and review paths to the expected outputs and rollback
  boundary.

### 2. One failure-test binding is conditional where the spec requires explicit failure behavior

Severity: blocking.

Evidence:

- The spec requires codegen to fail before emission for duplicate protobuf
  helper stems and duplicate protobuf output field tags inside one protobuf
  message model.
- The plan binds descriptor behavior to fail on duplicate JSON/CSV output path,
  duplicate protobuf helper stem, and duplicate protobuf tag.
- The plan's test fixture list says:

```text
invalid duplicate protobuf tag or helper-stem case if needed to exercise the
model validation.
```

Assessment:

The phrase `if needed` leaves a spec-required failure surface optional. The
implementation plan can choose the exact fixture shape, but it must not make
coverage of required failure behavior optional.

Required amendment:

- Replace the conditional wording with an unconditional test binding for:
  - duplicate protobuf output field tag in one message model;
  - duplicate protobuf helper stem, or an explicit statement that helper-stem
    duplication is structurally impossible after deterministic stem derivation,
    with the validation point that proves it.

## Passing Observations

### 1. Code and generated file boundaries are mostly complete

Evidence:

- The plan binds production code edits to:

```text
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
```

- It binds tests under `crates/codegen/src/tests` and
  `crates/schemas/bars_core/tests`.
- It binds generated writes to only:

```text
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
```

- It explicitly forbids `Cargo.toml`, `Cargo.lock`, MBT core, adapters,
  metamorphose, transponding, benches, generated core, and generated columnar
  files.

Assessment:

The file boundary is acceptable after the evidence/result-review binding is
amended.

### 2. Protobuf message-tree behavior is bound

Evidence:

- The plan requires replacing protobuf row length/write iteration over
  `model.fields` with `model.protobuf_messages`.
- It requires child helpers:

```text
encoded_len_<child_stem>
write_protobuf_<child_stem>
```

- It requires Bars generated protobuf to contain `writer.message_prefix(22`,
  `write_protobuf_metadata`, and metadata-local
  `writer.utc(6, row.ingested_at_ms.to_native())`.

Assessment:

The plan does not repeat the previous flattened-protobuf mistake. This part is
consistent with the passed spec peer audit.

### 3. Command surface matches the current executable interfaces

Evidence:

- `crates/benches/src/bin/mbt_bars_regression_bench.rs` requires
  `--report-dir <path>`.
- `crates/codegen/src/config.rs` includes adapter variants for JSON,
  protobuf, CSV, transponding, Arrow, Arrow IPC, and Parquet.
- `crates/schemas/bars_core/Cargo.toml` defines the feature names used by the
  plan: `json`, `protobuf`, `csv`, `arrow_ipc`, and `parquet`.

Assessment:

The validation and benchmark command shapes are consistent with the observed
code interfaces.

## Required Plan Amendments

The plan must be amended before implementation approval to:

1. bind exact compile-surface evidence files under
   `docs/evidence/mbt_derived_utc_metamorphose_codegen/`;
2. bind the derived-UTC result review and Bars regression corrective result
   review as expected artifacts;
3. add the evidence and result-review paths to rollback scope;
4. make duplicate protobuf tag/helper-stem failure test coverage unconditional
   or prove one of those failure modes is structurally impossible and bind the
   validation point.

## Final Classification

`BLOCKED`

The implementation plan is close, but it is not ready for implementation
approval until the exact evidence/result-review artifact gap and conditional
failure-test wording are amended.
