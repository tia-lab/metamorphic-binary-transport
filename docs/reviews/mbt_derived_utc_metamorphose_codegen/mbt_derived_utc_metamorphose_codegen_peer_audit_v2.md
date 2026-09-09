# Peer Audit V2: MBT Derived UTC Metamorphose Codegen

Slug: `mbt_derived_utc_metamorphose_codegen`

Classification: `PEER_AUDIT_PASSED`

## Required Reads

Completed:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_research_brief.md`
- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_peer_audit.md`
- `docs/specs/mbt_derived_utc_metamorphose_codegen_SPEC.md`
- `docs/specs/mbt_bars_regression_benchmark_SPEC.md`
- `crates/schemas/bars_core/Cargo.toml`
- `crates/codegen/Cargo.toml`
- `Cargo.toml`
- `crates/codegen/src/model.rs`
- `crates/codegen/src/descriptor.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/codegen/src/tests/test_descriptor.rs`
- `crates/schemas/bars_core/src/bars_v1_protobuf.rs`
- `crates/schemas/bars_core/tests/test_bars_shape.rs`
- `crates/benches/src/bin/mbt_bars_regression_bench.rs`
- `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`

## Findings

No blocking findings.

## Audit Evidence

### 1. Prior blocker on flattened protobuf output is resolved

Evidence:

- Spec lines 102-107 define the measured object as JSON and CSV flattened
  logical-path output, but protobuf message-tree output.
- Spec lines 172-187 define the adapter output split: JSON/CSV flattened row
  logical paths, protobuf row field `22` to metadata child message.
- Spec lines 253-278 add separate model contracts for
  `JsonCsvOutputField`, `ProtobufMessageModel`, and `ProtobufOutputField`.
- Spec lines 292-308 require descriptor construction for both
  `json_csv_output_fields` and `protobuf_messages`, and require physical and
  derived fields to emit in their owning protobuf message.
- Spec lines 310-325 bind the concrete Bars model shape: row tags `4`, `5`,
  `6`, `7`, and metadata message tag `22`; metadata tags `5`, `6`, `7`, and
  `8`.
- Spec lines 370-447 require protobuf emission from `protobuf_messages`, with
  deterministic `encoded_len_<message_stem>` and
  `write_protobuf_<message_stem>` helpers, child message length handling, and
  `message_prefix(child_tag, child_len)`.

Old-crate evidence:

- `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`
  lines 3902-3905 write row field `22` with `message_prefix` and then call
  `write_protobuf_metadata`.
- The same old file lines 4139-4149 writes metadata-local fields
  `ingested_at_ms` tag `5`, `ingested_at_utc` tag `6`,
  `target_ingested_at_ms` tag `7`, and `target_ingested_at_utc` tag `8`
  inside `write_protobuf_metadata`.

Assessment:

The amended spec no longer models protobuf as one flat row-format list. It now
binds the schema message tree and the nested Bars metadata oracle required by
the first audit.

### 2. Prior blocker on benchmark command is resolved

Evidence:

- `crates/benches/src/bin/mbt_bars_regression_bench.rs` lines 36-42 require
  exactly `--report-dir <path>`.
- `docs/specs/mbt_bars_regression_benchmark_SPEC.md` line 587 binds the same
  new-MBT benchmark command shape with `--report-dir`.
- The amended spec lines 627-632 now bind:

```text
cd /home/tia/_DEV/MATHILDE/metamorphic-binary-transport
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

Assessment:

The command surface is executable as written with the required report
directory argument.

### 3. Archive, projection, and columnar exclusions remain intact

Evidence:

- Spec lines 79-95 list non-goals including unchanged MBT envelope/payload,
  no UTC storage in MBT, no projection semantic change, and no derived UTC in
  transponding, Arrow, Arrow IPC, or Parquet by default.
- Spec lines 197-211 state derived UTC fields are absent from owned rows,
  archived rows, archived payloads, physical field arrays, MBT-to-MBT
  projection payloads, and core schema hash input.
- Spec lines 343-352 keep `model.fields` as the source for archive structs,
  validation, schema hash, projections, transponding, Arrow, Arrow IPC, and
  Parquet.
- Spec lines 471-480 explicitly keep columnar emitters physical-field only.
- Spec lines 761-777 allow only generated JSON, protobuf, and CSV Bars files
  to change and explicitly forbid changes to generated core, transponding,
  Arrow, Arrow IPC, Parquet, and test compatibility generated files.

Assessment:

The amended spec does not widen MBT archive/projection/columnar semantics.

### 4. Correctness and compile-surface gates are defined

Evidence:

- Spec lines 529-543 define codegen failure cases, including duplicate
  JSON/CSV output paths, duplicate protobuf helper stems, and duplicate
  protobuf tags within one protobuf message model.
- Spec lines 551-574 define compile-surface evidence commands for codegen and
  Bars generated row-format files.
- Spec lines 596-617 define correctness oracles including nested protobuf
  helper placement and Bars metadata UTC writes inside
  `write_protobuf_metadata`.
- Spec lines 640-720 bind descriptor tests, emitter tests, generated Bars
  tests, and validation commands.
- `crates/schemas/bars_core/Cargo.toml` defines the required feature names
  `json`, `protobuf`, `csv`, `arrow_ipc`, and `parquet`.

Assessment:

The spec contains enough exact validation surface for implementation planning.

## Non-Blocking Observations

### 1. Research brief still describes protobuf as flattened

The research brief predates the first peer audit and still states that
JSON/protobuf/CSV writes are generated in flattened proto traversal order. The
amended spec supersedes that candidate approach for protobuf. This is not a
blocker because the spec is the source of truth, and the amended spec now binds
protobuf message-tree output.

Implementation planning should avoid copying the older research-brief wording.

### 2. Optional parent-message presence remains outside the immediate proof

The current descriptor recursively flattens non-repeated child messages in
`crates/codegen/src/descriptor.rs` lines 150-160 and does not model parent
message presence separately. The amended spec follows the existing supported
row-tree model and proves the Bars metadata case, where `metadata` is a
non-optional child message.

This is not a blocker for this spec. If a future schema needs optional parent
message presence, that support should be specified separately or rejected
before emission.

### 3. Generated-source inspection is acceptable for protobuf nesting

The spec uses generated-source inspection for `bars_v1_protobuf.rs` to prove
`writer.message_prefix(22` and `write_protobuf_metadata`. That is acceptable
for this stage because the target behavior is generated code shape and the
old-vs-new benchmark later checks semantic output parity.

## Implementation Plan Requirements

The implementation plan may now be written, but it must preserve these audit
constraints:

1. no code changes outside the spec-bound files;
2. no generated-file hand edits;
3. no changes to MBT core, projection, transponding, Arrow, Arrow IPC, or
   Parquet generated output;
4. no benchmark code changes;
5. protobuf generated code must use message-tree helpers, not flattened
   JSON/CSV output order;
6. validation must run the exact commands bound by the spec or record failures
   without relaxing expectations.

## Final Classification

`PEER_AUDIT_PASSED`

The spec is ready for a separate implementation plan. No code is authorized by
this peer audit.
