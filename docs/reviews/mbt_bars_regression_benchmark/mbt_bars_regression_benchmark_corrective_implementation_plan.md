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

# Corrective Implementation Plan: MBT Bars Regression Benchmark

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15
Status: `BLOCKED_BEFORE_CODE`
Repository: `/home/tia/_DEV/MATHILDE/metamorphic-binary-transport`

## Source Chain

| Artifact | Path | Status |
|---|---|---|
| Research brief | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_research_brief.md` | complete |
| Spec | `docs/specs/mbt_bars_regression_benchmark_SPEC.md` | corrective amendment present |
| Corrective peer audit | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_peer_audit.md` | passed |
| Existing implementation plan | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md` | superseded for old-baseline parity claims |
| Existing result review | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md` | non-authoritative for old-baseline parity |

This plan does not authorize code changes. It identifies one blocker that must
be resolved before implementation can start.

## Goal

Correct the Bars regression benchmark so old-baseline comparisons are
apple-to-apple against the old tracked benchmark evidence.

Required corrections:

1. Replace the current projection Bars fixture with an exact bench-only port of
   the old Bars fixture.
2. Replace single-pass timing with the old row-label and iteration table.
3. Parse old semantic checksums from the old markdown baseline and require
   fixture semantic checksum parity before speed comparison.
4. Match old timing boundaries for each old-baseline comparison lane.
5. Mark prior `bars_regression_run_1..3` evidence as method-invalid for old
   parity claims.

## Blocking Finding

Exact old archived adapter timing for CSV, Arrow IPC, and Parquet cannot be
implemented with benchmark-crate-only edits.

### Evidence

Old archived lanes excluded archive access from measured total time:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/bench.rs
```

Code-read evidence:

- `metamorphose_arrow_ipc_full_archived` encodes bytes, performs
  `BarsV1::access_archived(&bytes)` before `total_start`, then times
  transponding, Arrow, and IPC writing.
- `metamorphose_parquet_full_archived` encodes bytes, performs
  `BarsV1::access_archived(&bytes)` before `total_start`, then times
  transponding, Arrow, and Parquet writing.
- `metamorphose_csv_full_archived` encodes bytes, performs
  `BarsV1::access_archived(&bytes)` before `total_start`, then times CSV
  writing.

Current new workspace public trusted adapter APIs take MBT bytes and perform
trusted access inside the public call:

```text
crates/schemas/bars_core/src/bars_v1_csv.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
```

Code-read evidence:

- `BarsV1::metamorphose_csv_trusted_unchecked(bytes, ...)` calls
  `Self::access_archived_trusted_unchecked(bytes)?` inside the function.
- `BarsV1::metamorphose_arrow_ipc_trusted_unchecked(bytes, ...)` calls
  `Self::access_archived_trusted_unchecked(bytes)?` inside the function.
- `BarsV1::metamorphose_parquet_trusted_unchecked(bytes, ...)` calls
  `Self::access_archived_trusted_unchecked(bytes)?` inside the function.

The direct archived conversion helpers are not public to the benchmark crate:

```text
crates/schemas/bars_core/src/bars_v1_transponding.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
```

Observed visibility:

- `BarsV1::transpond_archived(...)` is `pub(crate)`;
- `BarsV1ColumnBatch` is `pub(crate)`;
- `arrow_record_batch(...)` is private;
- `write_csv_response(...)` is private.

Therefore the benchmark crate cannot reproduce the old archived timing boundary
without either:

1. changing generated schema or adapter API surface; or
2. changing the spec to compare the current trusted bytes API boundary instead
   of the old archived boundary.

## Decision

Implementation is blocked until the spec chooses one of these two paths:

### Path A: Exact old archived boundary

Amend the spec to authorize generated archived adapter entrypoints.

Required generated/public shape, names subject to codegen review:

```rust
impl BarsV1 {
    pub fn metamorphose_csv_archived(
        archived: &ArchivedMathildeTransportResponseV1Payload,
        max_response_bytes: usize,
    ) -> Result<Vec<u8>>;

    pub fn metamorphose_arrow_ipc_archived(
        archived: &ArchivedMathildeTransportResponseV1Payload,
        max_response_bytes: usize,
    ) -> Result<Vec<u8>>;

    pub fn metamorphose_parquet_archived(
        archived: &ArchivedMathildeTransportResponseV1Payload,
        max_response_bytes: usize,
    ) -> Result<Vec<u8>>;
}
```

If this path is chosen, the codegen spec and implementation plan must also
bind whether these archived entrypoints are public production API or
bench-only/test-only API. They cannot be hand-written into generated files.

### Path B: Current trusted bytes boundary

Amend the spec to compare current trusted bytes APIs against a matching old
public or otherwise adjusted baseline, not the old archived baseline.

This path is simpler but does not prove exact old archived lane parity.

## Benchmark-Only Corrections After Blocker Resolution

Once the archived-boundary blocker is resolved, the following benchmark-only
edits are approved by this plan shape and must be re-audited before code.

## Files To Edit

Only these existing files may be edited for the benchmark correction:

```text
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
```

No dependency file edit is required for the benchmark-only correction.

No production crate source file may be edited under this blocked plan.

## Files To Create

Create the corrective result review only after corrective benchmarks run:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

No new code file is required.

## Dependency Changes

No dependency changes are approved.

`Cargo.lock` must not change for the benchmark-only correction.

## Corrective Edit Plan

### 1. Replace the row source

Edit:

```text
crates/benches/src/bars_regression.rs
```

Current wrong function:

```rust
pub fn bars_rows(row_count: usize) -> Vec<MathildeBarRowV1> {
    crate::projection::bars_rows(row_count)
}
```

Required replacement:

```text
bars_rows(row_count: usize) -> BenchResult<Vec<MathildeBarRowV1>>
```

The implementation must port the old fixture from:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/fixtures.rs
```

Required constants:

```text
DEFAULT_SEED = 0x4d415448494c4445
FIRST_CLOSE_MS = 1_546_300_860_000
BAR_MS = 60_000
```

Required RNG:

```text
state = state * 6364136223846793005 + 1442695040888963407, wrapping u64
```

Required generated-name mapping:

| Old fixture name | New generated name |
|---|---|
| `TF_1M` | `TIMEFRAME_1M` |
| `source` | `source_ordinal` |
| `process` | `process_ordinal` |
| `recomputed_reason` | `recomputed_reason_ordinal` |
| `P_VW` | `PRESENCE_VW` |
| `P_N` | `PRESENCE_N` |
| `P_PROCESS` | `PRESENCE_PROCESS_ORDINAL` |
| `P_INGESTED_AT_MS` | `PRESENCE_INGESTED_AT_MS` |
| `P_TARGET_INGESTED_AT_MS` | `PRESENCE_TARGET_INGESTED_AT_MS` |
| `P_BUILT_AT_MS` | `PRESENCE_BUILT_AT_MS` |
| `P_COMMITTED_AT_MS` | `PRESENCE_COMMITTED_AT_MS` |
| `P_HARMONIZED_AT_MS` | `PRESENCE_HARMONIZED_AT_MS` |
| `P_RECOMPUTED_AT_MS` | `PRESENCE_RECOMPUTED_AT_MS` |
| `P_RECOMPUTED_REASON` | `PRESENCE_RECOMPUTED_REASON_ORDINAL` |
| `P_COVERED_1M_COUNT` | `PRESENCE_COVERED_1M_COUNT` |
| `P_EXPECTED_1M_COUNT` | `PRESENCE_EXPECTED_1M_COUNT` |
| `P_COVERAGE_RATIO` | `PRESENCE_COVERAGE_RATIO` |
| `P_INPUTS_SOURCE_COUNTS_FRONTIER` | `PRESENCE_INPUTS_SOURCE_COUNTS_FRONTIER` |
| `P_INPUTS_SOURCE_COUNTS_API` | `PRESENCE_INPUTS_SOURCE_COUNTS_API` |
| `P_INPUTS_SOURCE_COUNTS_SYNTHETIC` | `PRESENCE_INPUTS_SOURCE_COUNTS_SYNTHETIC` |
| `P_INPUTS_SOURCE_COUNTS_FIX_DATA` | `PRESENCE_INPUTS_SOURCE_COUNTS_FIX_DATA` |
| `P_FRONTIER_5S_INPUTS_COVERAGE_RATIO` | `PRESENCE_FRONTIER_5S_INPUTS_COVERAGE_RATIO` |
| `P_FRONTIER_5S_EXPECTED` | `PRESENCE_FRONTIER_5S_EXPECTED` |
| `P_FRONTIER_5S_SYNTH_N` | `PRESENCE_FRONTIER_5S_SYNTH_N` |
| `P_FRONTIER_5S_SYNTH_RATIO` | `PRESENCE_FRONTIER_5S_SYNTH_RATIO` |
| `P_FRONTIER_5S_TRADE_N` | `PRESENCE_FRONTIER_5S_TRADE_N` |
| `P_FRONTIER_5S_TRADE_RATIO` | `PRESENCE_FRONTIER_5S_TRADE_RATIO` |
| `P_AGE_MS` | `PRESENCE_AGE_MS` |

Required validation:

```rust
metamorphic_binary_transport_schema_bars::bars_v1::validate_rows(&rows)?;
```

The fixture must not call `crate::projection::bars_rows`.

### 2. Add row-count configuration with iterations

Edit:

```text
crates/benches/src/bars_regression.rs
```

Add a small public config type:

```text
RowCountConfig { label: &'static str, rows: usize, iterations: usize }
```

Required configs:

| Label | Rows | Iterations |
|---|---:|---:|
| `one` | 1 | 50 |
| `small` | 100 | 50 |
| `page_500` | 500 | 50 |
| `page_1000` | 1,000 | 50 |
| `medium` | 10,000 | 10 |
| `large` | 100,000 | 3 |

Keep `ROW_COUNTS` only if existing tests/report metadata still need the row
array. It must be derived manually from the same values and tested against
`ROW_CONFIGS`.

### 3. Parse old semantic checksums

Edit:

```text
crates/benches/src/bars_regression.rs
```

Extend:

```text
OldBaselineEntry
parse_old_bars_baselines
verify_required_old_baselines
comparison_for
```

Required parsed columns from old full benchmark markdown rows:

| Field | Old table column |
|---|---|
| row label | column 0 |
| row count | column 1 |
| format label | column 2 |
| output bytes | column 3 |
| total ms | column 4 |
| rows/sec | column 5 |
| MB/sec | column 6 |
| semantic or response checksum | column 12 |

The parser must fail closed if a required label and row count is missing.

For `mathilde_binary_generated`, the checksum is the semantic checksum produced
by old `BarsV1::inspect`. The corrective benchmark must require the new fixture
semantic checksum to match this checksum before calculating old speed ratios.

### 4. Replace single-pass measurement with accumulated measurement

Edit:

```text
crates/benches/src/bin/mbt_bars_regression_bench.rs
```

Current wrong shape:

```text
for row_count in ROW_COUNTS:
    measure each label once
```

Required shape:

```text
for config in ROW_CONFIGS:
    generate fixture rows once outside measured timing
    build serde DTO rows once outside measured timing
    for each label:
        run config.iterations iterations
        accumulate output bytes
        track max single output bytes
        accumulate measured total time
        compute rows/sec from config.rows * config.iterations
        compute MB/sec from accumulated output bytes
```

Add a local accumulator in the binary or support module:

```text
LaneAccumulator
```

Required accumulator fields:

```text
output_bytes
max_output_bytes
total_milliseconds
response_checksum
response_checksum_stable
semantic_checksum
semantic_checksum_stable
minimal_projection_checksum
minimal_projection_checksum_stable
```

Do not average per-iteration rows/sec.

### 5. Preserve old setup boundaries

For full MBT:

```text
timed:
  BarsV1::encode(rows, MAX_RESPONSE_BYTES)
  BarsV1::inspect(&bytes)
  response_checksum(&bytes)
```

For current serde JSON baseline:

```text
outside timing:
  fixture row generation
  SerdeBarRow conversion
timed:
  serde_json::to_vec(&serde_rows)
  response_checksum(&bytes)
```

For checked JSON/protobuf:

```text
outside timing:
  BarsV1::encode(rows, MAX_RESPONSE_BYTES)
timed:
  BarsV1::metamorphose_json(&encoded, MAX_RESPONSE_BYTES)
  BarsV1::metamorphose_protobuf(&encoded, MAX_RESPONSE_BYTES)
  response_checksum(&bytes)
```

Important limitation:

The old JSON/protobuf lanes included old decode checksum work inside the timed
total. The current workspace does not provide old-equivalent JSON/protobuf
decode checksum helpers in this benchmark. The corrective result review must
state this limitation unless a later spec adds exact decode-oracle parity.

For trusted CSV, Arrow IPC, and Parquet:

Exact old archived timing is blocked until the spec resolves Path A or Path B
from this plan's blocking finding.

### 6. Update JSON report fields

Edit:

```text
crates/benches/src/bars_regression.rs
```

Extend `BarsRegressionRow` and JSON writer with:

```text
row_label
iterations
max_output_bytes
old_semantic_checksum
semantic_checksum_matches_old
response_checksum_stable
semantic_checksum_stable
minimal_projection_checksum_stable
```

Keep existing fields:

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

The report writer must reject:

- zero output bytes;
- non-finite timing or throughput;
- unstable response checksums for repeated deterministic iterations;
- unstable semantic checksum when semantic checksum exists;
- full MBT semantic checksum mismatch against old `mathilde_binary_generated`.

### 7. Update tests

Edit:

```text
crates/benches/src/tests/test_bars_regression_bench_output.rs
```

Required test changes:

1. `required_old_baselines_are_found`
   - verify old label, row count, rows/sec, MB/sec, and checksum are parsed.
2. New `old_fixture_matches_old_mbt_checksums`
   - for each `ROW_CONFIGS` row count:
     - generate rows through corrected `bars_rows`;
     - encode with `BarsV1::encode`;
     - inspect with `BarsV1::inspect`;
     - compare semantic checksum with old `mathilde_binary_generated`.
3. `required_labels_are_exact`
   - verify labels remain unchanged;
   - verify row configs and iteration counts.
4. `serde_rows_preserve_selected_fields`
   - use corrected old fixture rows.
5. `report_contains_required_fields`
   - include new JSON fields:
     - `row_label`;
     - `iterations`;
     - `max_output_bytes`;
     - `old_semantic_checksum`;
     - `semantic_checksum_matches_old`;
     - checksum stability flags.

All tests must stay panic-free and must not use `unwrap`, `expect`, or
`assert_eq!`.

### 8. Update result review status

Edit:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
```

Before new benchmark runs, amend status to:

```text
RESULT_REVIEW_SUPERSEDED_BY_CORRECTIVE_SPEC
```

Add a short correction note:

- the prior runs used the projection fixture;
- the prior runs used single-pass timing;
- the prior old-baseline ratios are not valid parity evidence.

After corrective benchmark runs, create:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

## Validation Commands After Blocker Resolution

Formatting:

```text
cargo fmt --all --check
```

Forbidden-pattern scan:

```text
rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!|unwrap_or\\(|unwrap_or_default\\(|assert!|assert_eq!|assert_ne!" crates/benches/src/bars_regression.rs crates/benches/src/bin/mbt_bars_regression_bench.rs crates/benches/src/tests/test_bars_regression_bench_output.rs
```

Codegen checks:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter transponding --out crates/schemas/bars_core/src/bars_v1_transponding.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter arrow-ipc --out crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter parquet --out crates/schemas/bars_core/src/bars_v1_parquet.rs
```

Tests:

```text
cargo test -p metamorphic_binary_transport_benches --lib
```

Compile checks:

```text
/usr/bin/time -v cargo check -p metamorphic_binary_transport_benches --all-targets
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
/usr/bin/time -v cargo check -p metamorphic_binary_transport_core
cargo tree -p metamorphic_binary_transport_core
```

Benchmark runs:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

Companion projection run:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer
```

## Expected Outputs After Blocker Resolution

The implementation is accepted only if:

- fixture semantic checksums match old `mathilde_binary_generated` for every
  required row count;
- report rows include the old iteration counts;
- repeated deterministic iterations have stable response checksums;
- old-baseline ratios are calculated only from accumulated timing;
- result review explicitly separates:
  - proved old-fixture parity;
  - speed ratios;
  - any remaining timing-boundary limitation.

## Rollback Boundary

Rollback is limited to:

```text
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

No generated files, production crates, schema crates, adapter crates, or
dependency files are in rollback scope for the benchmark-only correction.

## Known Risks

1. Exact archived adapter timing requires generated archived entrypoints or a
   spec change to the comparison boundary.
2. JSON/protobuf old timing included decode checksum work that is not currently
   reproduced by the new benchmark.
3. The old baseline source is markdown; parsing must fail closed.
4. The repository is currently dirty; result reviews must report dirty state.

## Required Next Step

Do not implement this blocked plan.

Choose and specify the archived adapter boundary decision:

```text
Approved: amend docs/specs/mbt_bars_regression_benchmark_SPEC.md to authorize generated archived adapter entrypoints for exact old archived timing parity, then write docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_peer_audit_v2.md
```

or explicitly choose the current trusted bytes boundary and accept that it is
not exact old archived parity.
