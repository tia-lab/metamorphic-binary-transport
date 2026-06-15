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

# Peer Audit: MBT Derived UTC Metamorphose Codegen

Slug: `mbt_derived_utc_metamorphose_codegen`

Classification: `BLOCKED`

## Required Reads

Completed:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_research_brief.md`
- `docs/specs/mbt_derived_utc_metamorphose_codegen_SPEC.md`
- `docs/specs/mbt_bars_regression_benchmark_SPEC.md`
- `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md`
- `crates/codegen/src/model.rs`
- `crates/codegen/src/descriptor.rs`
- `crates/codegen/src/rust_emit.rs`
- `crates/schemas/bars_core/src/lib.rs`
- `crates/schemas/bars_core/src/bars_v1_json.rs`
- `crates/schemas/bars_core/src/bars_v1_protobuf.rs`
- `crates/schemas/bars_core/src/bars_v1_csv.rs`
- `crates/benches/src/bin/mbt_bars_regression_bench.rs`
- `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`

## Findings

### 1. Blocker: protobuf derived UTC is specified as a flat row-format field, but Bars metadata UTC fields are nested protobuf fields

Severity: `BLOCKER`

Evidence:

- Spec lines 223-233 define `RowFormatField` as a flat `Physical` or
  `DerivedUtc` item.
- Spec lines 247-260 require a single flattened `row_format_fields` sequence
  for protobuf encoded row length and protobuf row writing.
- Spec lines 293-304 define derived protobuf output as
  `writer.utc(tag, row.<source>.to_native())`.
- Old generated Bars writes metadata as a nested protobuf message:
  `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`
  lines 3902-3905 write `message_prefix(22, child_len)` and then
  `write_protobuf_metadata`.
- Old generated Bars writes nested UTC fields inside that child message:
  the same file lines 4139-4149 write `ingested_at_ms` tag 5,
  `ingested_at_utc` tag 6, `target_ingested_at_ms` tag 7, and
  `target_ingested_at_utc` tag 8 inside `write_protobuf_metadata`.
- Current new generated protobuf already writes flattened metadata physical
  fields at row level in `crates/schemas/bars_core/src/bars_v1_protobuf.rs`.

Why this blocks:

The spec is correct for JSON and CSV flattening, but protobuf field numbers are
scoped by message. A flat `row_format_fields` sequence cannot correctly encode
`metadata.ingested_at_utc` as protobuf tag 6 at the row level, because row-level
tag 6 is `open_utc`. For Bars apple-to-apple parity, nested metadata fields
must be emitted inside the metadata message field, not flattened into the row.

Required amendment:

The spec must distinguish:

- flattened row-format output order for JSON and CSV;
- protobuf output tree/message structure for protobuf.

The amended protobuf contract must define a schema-derived message tree, nested
encoded-length helpers, nested writer helpers, and optional child-message
presence rules. For Bars, the generated protobuf path must be capable of:

```text
row field 22 -> metadata message
  -> metadata.ingested_at_ms tag 5
  -> metadata.ingested_at_utc tag 6
```

The implementation plan must not proceed until this protobuf nesting contract is
specified.

### 2. Blocker: benchmark command in this spec is not executable as written

Severity: `BLOCKER`

Evidence:

- Spec lines 476-480 bind this command:

```text
cargo run -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench --release
```

- `crates/benches/src/bin/mbt_bars_regression_bench.rs` lines 36-40 require:

```text
mbt_bars_regression_bench --report-dir <path>
```

- `docs/specs/mbt_bars_regression_benchmark_SPEC.md` binds the executable
  command with `--report-dir docs/evidence/mbt_bars_regression_benchmark`.

Why this blocks:

The spec protocol requires exact command surfaces before peer audit. The bound
benchmark command will fail because it does not pass `--report-dir`, so it
cannot produce the evidence artifact the spec later references.

Required amendment:

Replace the command with the already approved command shape:

```text
cd /home/tia/_DEV/MATHILDE/metamorphic-binary-transport
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

If this spec is not supposed to rerun benchmarks directly, then Section 17 must
state that no benchmark command is owned by this spec and must point only to a
later result-review or bars-regression plan. It must not include a broken
command.

## Non-Blocking Observations

### 1. CSV test wording should prefer public output over private constants

`crates/schemas/bars_core/src/lib.rs` imports `bars_v1_csv` as a private module
behind the `csv` feature. The spec says tests may assert
`bars_v1_csv::CSV_HEADER` or CSV output. The public-output option is acceptable.
The implementation plan should use `BarsV1::metamorphose_csv` output unless it
also changes module visibility in a separately approved spec.

### 2. The spec correctly excludes columnar UTC expansion

The spec preserves the current boundary that Arrow, Arrow IPC, Parquet, and
transponding remain physical-field only. That is aligned with the prior
architecture discussion and avoids widening columnar output without a separate
derived-column policy.

## Pass Criteria For Next Audit

The next spec version must:

1. define separate JSON/CSV flattened output and protobuf nested output
   contracts;
2. bind exact protobuf model additions needed for nested messages;
3. bind exact protobuf emitter responsibilities for child message length and
   write helpers;
4. add tests proving nested metadata UTC is emitted inside message field 22 for
   Bars or an equivalent generated fixture;
5. fix or remove the broken benchmark command;
6. keep all existing archive/projection/columnar exclusions intact.

## Final Classification

`BLOCKED`

No implementation plan may be written from the current spec.
