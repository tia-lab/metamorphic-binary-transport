# Result Review: MBT Derived UTC Metamorphose Codegen

Slug: `mbt_derived_utc_metamorphose_codegen`

Status: `IMPLEMENTED_WITH_VALIDATION_EVIDENCE`

## Scope Implemented

Implemented the approved row-format-only derived UTC model and codegen path.

Code-read evidence:

- `crates/codegen/src/model.rs` now separates physical MBT fields from
  `derived_utc_fields`, `json_csv_output_fields`, and `protobuf_messages`.
- `crates/codegen/src/descriptor.rs` keeps ignored derived UTC fields out of
  `model.fields`, validates declarations before emission, resolves source
  fields, and builds JSON/CSV flattened output plus protobuf message-tree
  output.
- `crates/codegen/src/rust_emit.rs` emits JSON/CSV derived UTC directly from
  archived source `*_ms` fields and emits protobuf nested message helpers.
- `crates/schemas/bars_core/src/bars_v1_json.rs`,
  `crates/schemas/bars_core/src/bars_v1_csv.rs`, and
  `crates/schemas/bars_core/src/bars_v1_protobuf.rs` were regenerated with
  `mbt_codegen --write`.
- `crates/schemas/bars_core/src/bars_v1_transponding.rs`,
  `crates/schemas/bars_core/src/bars_v1_arrow.rs`,
  `crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs`, and
  `crates/schemas/bars_core/src/bars_v1_parquet.rs` were not changed.

## Correctness Evidence

Run evidence:

- `cargo fmt --check`: passed.
- `cargo test -p metamorphic_binary_transport_codegen -- --nocapture`: passed,
  29 tests.
- `cargo test -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv -- --nocapture`:
  passed, 7 integration tests.
- `cargo check -p metamorphic_binary_transport_schema_bars --no-default-features`:
  passed.
- `cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv`:
  passed.
- `cargo check -p metamorphic_binary_transport_schema_bars --features arrow_ipc,parquet`:
  passed.
- `cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings`:
  passed.
- `mbt_codegen --check` for Bars JSON, protobuf, and CSV adapters: passed for
  all three generated files.

Test coverage added:

- valid nested derived UTC descriptor model;
- invalid derived UTC without `ignored`;
- invalid missing source;
- invalid non-I64 source;
- invalid non-string derived UTC field;
- invalid repeated derived UTC field;
- invalid duplicate protobuf output tag;
- invalid duplicate protobuf helper stem;
- JSON/CSV/protobuf generated source assertions;
- generated transponding/Arrow/Arrow IPC/Parquet exclusion assertions;
- public Bars JSON/CSV metamorphose output assertions;
- generated Bars protobuf nested metadata source assertions.

## Generated Output Evidence

Observed generated shape:

- JSON contains `open_utc`, `close_utc`, and metadata UTC constants and writes
  with `writer.utc_value(row.<source_ms>.to_native())`.
- CSV header contains flattened UTC columns and writes with
  `writer.utc_cell(row.<source_ms>.to_native())`.
- Protobuf contains `encoded_len_metadata`, `write_protobuf_metadata`,
  `writer.message_prefix(22, message_len)?`, and metadata-local UTC writes such
  as `writer.utc(6, row.ingested_at_ms.to_native())?`.

Evidence files:

- `docs/evidence/mbt_derived_utc_metamorphose_codegen/codegen_source_wc.txt`
- `docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_row_format_wc.txt`
- `docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_json_protobuf_csv_check_time.txt`
- `docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_arrow_ipc_parquet_check_time.txt`

Recorded line counts:

- codegen model/descriptor/emitter total: 4,975 lines.
- Bars generated JSON/protobuf/CSV total: 1,242 lines.

Recorded compile-surface checks:

- JSON/protobuf/CSV check evidence: exit status 0, wall time 0.10s, max RSS
  37,624 KB.
- Arrow IPC/parquet check evidence: exit status 0, wall time 0.12s, max RSS
  42,792 KB.

## Performance Evidence

Benchmark command:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

Run evidence:

- benchmark completed successfully;
- output file:
  `docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_7.json`.

Important limitation:

- `old_crate_comparison` is `null` in the benchmark output rows.
- A manual check against
  `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_3.json`
  showed semantic checksum mismatch for representative shared labels and row
  counts.
- Therefore old-vs-new speed parity is not proved by this result review.

## Invariants Proved

Proved by code and test evidence:

- derived UTC is not physical MBT archive data;
- JSON and CSV emit derived UTC in flattened logical-path output;
- protobuf emits derived UTC according to the protobuf message tree;
- optional derived UTC output follows the source field presence bit;
- generated row-format files are reproducible by `mbt_codegen --check`;
- transponding, Arrow, Arrow IPC, and Parquet generation did not receive derived
  UTC fields in this implementation.

## Remaining Limits

- Old-crate performance parity is not proved because current benchmark evidence
  did not attach comparable old rows and the checked old run is not
  semantically comparable.
- No claim is made that row-format performance matches or exceeds the old MBT
  crate.
