# Implementation Plan: MBT Bars Regression Benchmark

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15
Status: `IMPLEMENTATION_PLAN_AMENDED_AWAITING_APPROVAL`
Repository: `/home/tia/_DEV/MATHILDE/metamorphic-binary-transport`

## Source Chain

| Artifact       | Path                                                                                         | Status                                        |
| -------------- | -------------------------------------------------------------------------------------------- | --------------------------------------------- |
| Research brief | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_research_brief.md` | complete                                      |
| Spec           | `docs/specs/mbt_bars_regression_benchmark_SPEC.md`                                           | amended with Cargo-generated lockfile binding |
| Peer audit     | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_peer_audit.md`     | blocked                                       |
| Peer audit v2  | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_peer_audit_v2.md`  | passed                                        |

This plan does not authorize code changes. Code may change only after this plan
is explicitly approved.

## Goal

Implement one dedicated Bars regression benchmark in `crates/benches` that
measures:

- full Bars MBT encode plus checked inspect;
- Bars checked metamorphose JSON, protobuf, and CSV;
- Bars trusted metamorphose JSON, protobuf, CSV, Arrow IPC, and Parquet;
- a bench-only Rust DTO to `serde_json::to_vec` baseline;
- old tracked Bars baseline comparisons where the spec binds an old label.

Projection remains owned by the existing projection benchmark.

## Files To Edit

Only these hand-edited files are approved:

```text
crates/benches/Cargo.toml
crates/benches/src/lib.rs
crates/benches/src/tests/mod.rs
```

No production crate source file may be edited.

## Files To Create

Only these new files are approved:

```text
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
docs/evidence/mbt_bars_regression_benchmark/.gitkeep
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
```

## Cargo-Generated Artifact

`Cargo.lock` may be changed only by Cargo as a dependency-resolution artifact
from the approved `crates/benches/Cargo.toml` edits.

Manual edits to `Cargo.lock` are not allowed.

The implementation must stop if the lockfile change introduces dependencies
not explained by:

- `serde = "=1.0.228"` with `derive`;
- `serde_json = "=1.0.145"`;
- their required transitive crates.

The amended spec now binds this lockfile behavior in its dependency and code
binding sections. A follow-up implementation-plan peer audit must verify this
before code starts.

## Dependency Edits

Edit `crates/benches/Cargo.toml` exactly as follows.

Replace the current Bars dependency:

```toml
metamorphic_binary_transport_schema_bars = { path = "../schemas/bars_core" }
```

with:

```toml
metamorphic_binary_transport_schema_bars = {
    path = "../schemas/bars_core",
    features = ["json", "protobuf", "csv", "arrow_ipc", "parquet"]
}
```

Add:

```toml
serde = { version = "=1.0.228", features = ["derive"] }
serde_json = "=1.0.145"
```

No other dependency edit is approved.

## Module Wiring

Edit `crates/benches/src/lib.rs`:

```rust
pub mod bars_regression;
pub mod projection;
```

Preserve the existing crate-level `#![forbid(unsafe_code)]`.

Edit `crates/benches/src/tests/mod.rs`:

```rust
mod test_bars_regression_bench_output;
mod test_projection_bench_output;
```

Remove the existing skeleton test if it is still present, because it adds no
contract coverage.

## New Support Module

Create `crates/benches/src/bars_regression.rs`.

Responsibilities:

1. Constants:

```text
MAX_RESPONSE_BYTES = 1_073_741_824
ROW_COUNTS = [1, 100, 500, 1_000, 10_000, 100_000]
OLD_BENCH_RESULTS = /home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md
REQUIRED_SCHEMA_FEATURES = ["json", "protobuf", "csv", "arrow_ipc", "parquet"]
```

2. Current labels:

```text
bars_mbt_full_encode_inspect_checked
bars_metamorphose_json_checked
bars_metamorphose_protobuf_checked
bars_metamorphose_csv_checked
bars_metamorphose_json_trusted
bars_metamorphose_protobuf_trusted
bars_metamorphose_csv_trusted
bars_metamorphose_arrow_ipc_trusted
bars_metamorphose_parquet_trusted
bars_serde_json_baseline
```

3. Old labels:

```text
mathilde_binary_generated
metamorphose_json
metamorphose_protobuf
metamorphose_csv_full_archived
metamorphose_arrow_ipc_full_archived
metamorphose_parquet_full_archived
```

4. Label mapping:

| Current label                          | Old label                              |
| -------------------------------------- | -------------------------------------- |
| `bars_mbt_full_encode_inspect_checked` | `mathilde_binary_generated`            |
| `bars_metamorphose_json_checked`       | `metamorphose_json`                    |
| `bars_metamorphose_protobuf_checked`   | `metamorphose_protobuf`                |
| `bars_metamorphose_csv_trusted`        | `metamorphose_csv_full_archived`       |
| `bars_metamorphose_arrow_ipc_trusted`  | `metamorphose_arrow_ipc_full_archived` |
| `bars_metamorphose_parquet_trusted`    | `metamorphose_parquet_full_archived`   |

5. Data types:

```text
BenchResult<T>
Comparison
BarsRegressionRow
BarsRegressionMetadata
OldBaselineEntry
SerdeBarRow
```

`SerdeBarRow` must contain every field from `MathildeBarRowV1` so the serde JSON
baseline is the same logical row payload. The conversion from
`MathildeBarRowV1` to `SerdeBarRow` happens outside measured timing.

6. Required helpers:

```text
bars_rows(row_count) -> Vec<MathildeBarRowV1>
serde_rows_from_bars(rows) -> Vec<SerdeBarRow>
next_bars_run_path(report_dir) -> PathBuf
parse_old_bars_baselines(path) -> Vec<OldBaselineEntry>
verify_required_old_baselines(entries)
comparison_for(entries, old_label, row_count, observed_rows_per_second)
measured_rates(row_count, output_bytes, milliseconds)
response_checksum(bytes)
metadata_for_run(command, report_path)
write_report(path, metadata, rows)
```

`bars_rows` may delegate to the existing deterministic projection fixture:

```rust
crate::projection::bars_rows(row_count)
```

The old baseline parser must parse the full benchmark tables where:

```text
row_count = markdown column 1
format/label = markdown column 2
rows/sec = markdown column 5
MB/sec = markdown column 6
```

It must fail if any required old label is missing for any required row count.

7. JSON writer:

The report writer must be explicit and deterministic. It may follow the
existing manual writer style in `projection.rs`. It must not use `serde_json`
for report serialization; `serde_json` is reserved for the measured serde JSON
baseline.

## Benchmark Binary

Create:

```text
crates/benches/src/bin/mbt_bars_regression_bench.rs
```

Responsibilities:

1. Parse exactly:

```text
mbt_bars_regression_bench --report-dir <path>
```

Any other CLI shape is an error.

2. Load and verify old baselines from:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md
```

3. For each row count:

- generate Bars rows outside measured timing;
- prebuild serde DTO rows outside measured timing;
- pre-encode MBT bytes for metamorphose lanes outside metamorphose timing;
- measure full MBT encode plus inspect as one lane;
- measure checked JSON/protobuf/CSV metamorphose over pre-encoded MBT bytes;
- measure trusted JSON/protobuf/CSV/Arrow IPC/Parquet metamorphose over
  pre-encoded MBT bytes;
- measure serde JSON output over prebuilt DTO rows.

4. Trusted calls must be visibly unsafe in the binary only. The safety contract
   is:

```text
bytes were produced by BarsV1::encode in the same benchmark iteration and are
not mutated before trusted access.
```

5. Write the next available report:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_N.json
```

The binary must refuse to overwrite an existing run file.

## Report JSON Contract

Top-level object:

```text
metadata
rows
```

Required `metadata` fields:

```text
slug
timestamp_utc
operator
command
build_profile
dirty_state
rustc
os_kernel
cpu
ram
dataset_identity
row_counts
enabled_schema_features
max_response_bytes
old_baseline_path
projection_evidence_glob
report_path
cache_mode
```

`cache_mode` must be `"not_applicable"` because this benchmark does not touch
storage.

Required `rows[]` fields:

```text
label
row_count
output_bytes
total_milliseconds
rows_per_second
mb_per_second
response_checksum
semantic_checksum
minimal_projection_checksum
old_baseline_label
old_crate_comparison
serde_json_comparison
```

`old_crate_comparison` is either `null` or:

```text
rows_per_second
mb_per_second
ratio_rows_per_second
```

`serde_json_comparison` is either `null` or:

```text
rows_per_second
mb_per_second
ratio_rows_per_second
```

Only `bars_metamorphose_json_checked` at the same row count needs
`serde_json_comparison`.

Every numeric timing and throughput field must be finite. Non-finite numeric
values are benchmark failures.

Expected row count per report:

```text
10 labels * 6 row counts = 60 rows
```

## Test File

Create:

```text
crates/benches/src/tests/test_bars_regression_bench_output.rs
```

Required tests:

1. `required_old_baselines_are_found`
   - parse the old baseline file;
   - verify all six old labels exist for all six row counts.

2. `next_run_path_refuses_overwrite`
   - create `bars_regression_run_1.json`;
   - verify by returning an error unless the next path is
     `bars_regression_run_2.json`;
   - verify direct write to an existing path returns an error.

3. `required_labels_are_exact`
   - verify by returning an error unless the current labels list contains
     exactly the ten labels in this plan.

4. `serde_rows_preserve_selected_fields`
   - convert deterministic Bars rows to serde DTO rows;
   - verify row count and selected fields:
     - `schema_version`;
     - `pair_ordinal`;
     - `tf_ordinal`;
     - `close_ms`;
     - `c`;
     - `presence_bits`.

5. `report_contains_required_fields`
   - write a minimal report into a temporary directory;
   - read it as text;
   - verify all required metadata and row field names are present.

Tests must return `Result<(), Box<dyn std::error::Error>>` and avoid
`unwrap`, `expect`, `panic!`, `assert!`, `assert_eq!`, and `assert_ne!`.

## Result Review Stub

Create:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
```

Initial status:

```text
RESULT_REVIEW_STUB_AWAITING_RUN_EVIDENCE
```

The stub must state:

- implementation exists but run evidence is not yet recorded;
- performance claims remain unproved until validation commands and three
  release benchmark runs are recorded;
- projection stability is a separate evidence surface.

After validation, the result review must be updated with command outputs and
benchmark tables.

## Implementation Sequence

1. Update `crates/benches/Cargo.toml`.
2. Add `crates/benches/src/bars_regression.rs`.
3. Add `crates/benches/src/bin/mbt_bars_regression_bench.rs`.
4. Update `crates/benches/src/lib.rs`.
5. Add `crates/benches/src/tests/test_bars_regression_bench_output.rs`.
6. Update `crates/benches/src/tests/mod.rs`.
7. Add `docs/evidence/mbt_bars_regression_benchmark/.gitkeep`.
8. Add the result review stub.
9. Run pre-test audit commands.
10. Run validation commands.
11. Run benchmark commands only after correctness checks pass.
12. Update result review with evidence.

## Pre-Test Audit Commands

Run before tests and benchmarks:

```text
git diff -- crates/benches/Cargo.toml crates/benches/src/lib.rs crates/benches/src/tests/mod.rs crates/benches/src/bars_regression.rs crates/benches/src/bin/mbt_bars_regression_bench.rs crates/benches/src/tests/test_bars_regression_bench_output.rs docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md Cargo.lock
```

```text
rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!|unwrap_or\\(|unwrap_or_default\\(|assert!|assert_eq!|assert_ne!" crates/benches/src/bars_regression.rs crates/benches/src/bin/mbt_bars_regression_bench.rs crates/benches/src/tests/test_bars_regression_bench_output.rs
```

Expected result for the `rg` command:

```text
no matches
```

```text
cargo tree -p metamorphic_binary_transport_core
```

Expected result:

```text
core dependency tree has no serde, serde_json, adapter, schema, benchmark, Arrow, Parquet, or prost dependency.
```

## Validation Commands

Run after implementation.

Formatting:

```text
cargo fmt --all --check
```

Generated artifact checks:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter transponding --out crates/schemas/bars_core/src/bars_v1_transponding.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter arrow-ipc --out crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter parquet --out crates/schemas/bars_core/src/bars_v1_parquet.rs
```

Compile-surface checks:

```text
/usr/bin/time -v cargo check -p metamorphic_binary_transport_benches --all-targets
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
/usr/bin/time -v cargo check -p metamorphic_binary_transport_core
cargo tree -p metamorphic_binary_transport_benches
```

If `/usr/bin/time` is unavailable, run the same `cargo check` commands without
it and record that max RSS was not collected.

Tests:

```text
cargo test -p metamorphic_binary_transport_benches --lib
```

Benchmark commands:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

Companion projection command:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer
```

## Expected Outputs

Validation expected outputs:

- all codegen `--check` commands exit 0;
- all compile checks exit 0;
- `cargo test -p metamorphic_binary_transport_benches --lib` exits 0;
- `cargo tree -p metamorphic_binary_transport_benches` shows serde and
  serde_json only under the benches crate dependency graph;
- `cargo check -p metamorphic_binary_transport_core` still does not compile
  serde, serde_json, schema, adapter, benchmark, Arrow, Parquet, or prost
  dependencies.

Benchmark expected outputs:

- each Bars regression run prints one
  `docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_N.json`
  path;
- each Bars regression JSON report contains exactly 60 benchmark rows;
- old comparisons are present for the six labels with old baselines;
- `serde_json_comparison` is present only for
  `bars_metamorphose_json_checked`;
- no benchmark row has zero output bytes;
- no benchmark row has non-finite timing or throughput;
- the companion projection command prints one projection run path.

Result review expected output:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
```

must record:

- commands run;
- run file paths;
- compile-surface timing and max RSS if available;
- three-run benchmark table;
- old baseline comparison table;
- serde JSON comparison table;
- correctness results;
- limitations.

## Rollback Boundary

If implementation fails before benchmark evidence is produced, rollback is
limited to:

```text
crates/benches/Cargo.toml
crates/benches/src/lib.rs
crates/benches/src/tests/mod.rs
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
docs/evidence/mbt_bars_regression_benchmark/.gitkeep
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
Cargo.lock
```

If benchmark runs produce failed evidence, do not delete that evidence unless
explicitly instructed. Failed and unstable runs are evidence and must be
reported.

## Known Risks

1. `Cargo.lock` may change because `serde` and `serde_json` are new to this
   workspace lockfile. The amended spec binds this as a Cargo-generated
   dependency artifact.
2. The serde JSON baseline is current-workspace-only. It is not an old-crate
   direct JSON baseline and must not be described as one.
3. The old baseline parser depends on the tracked markdown table shape. The
   benchmark must fail if the required labels and row counts are not parsed.
4. Three run files are required before any stability claim. A single run is
   evidence only for that run.
5. Projection evidence remains separate from the new Bars regression evidence.
   The result review must not merge their stability claims.

## Approval Gate

Implementation may start only after:

1. this implementation plan is peer-audited or otherwise explicitly accepted;
2. the user approves implementation against this exact plan.
