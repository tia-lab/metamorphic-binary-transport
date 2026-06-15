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

# Result Review: MBT Bars Regression Benchmark

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15
Status: `RESULT_REVIEW_COMPLETE_WITH_RUN_EVIDENCE`

## Scope

Implemented the approved Bars regression benchmark in:

```text
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
```

The benchmark measures:

- full Bars MBT encode plus checked inspect;
- checked Bars metamorphose to JSON, protobuf, and CSV;
- trusted Bars metamorphose to JSON, protobuf, CSV, Arrow IPC, and Parquet;
- bench-only Rust DTO serialization through `serde_json::to_vec`;
- old tracked Bars benchmark comparisons where the spec binds an old label.

Projection stability remains a separate evidence surface.

## Validation Evidence

| Command | Result |
|---|---|
| `cargo fmt --all --check` | passed |
| `rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!|unwrap_or\\(|unwrap_or_default\\(|assert!|assert_eq!|assert_ne!" crates/benches/src/bars_regression.rs crates/benches/src/bin/mbt_bars_regression_bench.rs crates/benches/src/tests/test_bars_regression_bench_output.rs` | no matches |
| `cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check ... --surface projection --out crates/schemas/bars_core/src/bars_v1.rs` | passed |
| `cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check ... --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs` | passed |
| `cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check ... --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs` | passed |
| `cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check ... --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs` | passed |
| `cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check ... --adapter transponding --out crates/schemas/bars_core/src/bars_v1_transponding.rs` | passed |
| `cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check ... --adapter arrow-ipc --out crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs` | passed |
| `cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check ... --adapter parquet --out crates/schemas/bars_core/src/bars_v1_parquet.rs` | passed |
| `cargo test -p metamorphic_binary_transport_benches --lib` | 8 passed, 0 failed |
| `cargo tree -p metamorphic_binary_transport_core` | core tree contains only `thiserror` and macro transitives |

Compile-surface checks with `/usr/bin/time -v`:

| Command | Wall time | Max RSS |
|---|---:|---:|
| `cargo check -p metamorphic_binary_transport_benches --all-targets` | 0.13 s | 43,592 KB |
| `cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow_ipc,parquet` | 0.10 s | 43,392 KB |
| `cargo check -p metamorphic_binary_transport_core` | 0.09 s | 35,988 KB |

Dependency graph validation:

- `serde = 1.0.228` and `serde_json = 1.0.145` are present only through the
  benchmark crate dependency graph.
- `Cargo.lock` did not change during this implementation run because the
  required serde packages were already present in the workspace lockfile.
- MBT core did not acquire serde, serde_json, schema, adapter, benchmark,
  Arrow, Parquet, or prost dependencies.

## Run Artifacts

Bars regression runs:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_1.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_2.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_3.json
```

Companion projection run:

```text
docs/evidence/mbt_projection_direct_writer/projection_run_5.json
```

Artifact checks:

| Artifact set | Result |
|---|---|
| Bars run row count | 60 rows in each of the three run files |
| Zero output bytes | 0 rows in each Bars run file |
| Old crate comparisons | 36 rows in each Bars run file |
| Serde JSON comparisons | 6 rows in each Bars run file |
| Projection companion row count | 72 rows |

Run metadata from `bars_regression_run_3.json`:

| Field | Value |
|---|---|
| command | `target/release/mbt_bars_regression_bench --report-dir docs/evidence/mbt_bars_regression_benchmark` |
| build_profile | `release` |
| dirty_state | `dirty` |
| rustc | `rustc 1.90.0 (1159e78c4 2025-09-14)` |
| os_kernel | `Linux 5.15.0-156-generic` |
| cpu | `: Intel(R) Xeon(R) W-2295 CPU @ 3.00GHz` |
| ram | `MemTotal:       527843880 kB` |
| cache_mode | `not_applicable` |

## Large-Row Benchmark Table

The full six-row-count evidence is in the JSON run files. This table records
the 100,000-row lane for the three required runs.

| Label | Run 1 rows/s | Run 1 MB/s | Run 2 rows/s | Run 2 MB/s | Run 3 rows/s | Run 3 MB/s |
|---|---:|---:|---:|---:|---:|---:|
| `bars_mbt_full_encode_inspect_checked` | 391,850 | 125.563 | 395,634 | 126.775 | 395,574 | 126.756 |
| `bars_metamorphose_json_checked` | 320,450 | 391.138 | 318,669 | 388.965 | 321,073 | 391.899 |
| `bars_metamorphose_protobuf_checked` | 1,161,312 | 358.287 | 1,211,335 | 373.720 | 1,225,400 | 378.059 |
| `bars_metamorphose_csv_checked` | 491,577 | 156.061 | 488,296 | 155.019 | 484,463 | 153.802 |
| `bars_metamorphose_json_trusted` | 360,486 | 440.006 | 353,840 | 431.893 | 357,763 | 436.683 |
| `bars_metamorphose_protobuf_trusted` | 2,036,024 | 628.152 | 2,071,573 | 639.119 | 2,078,337 | 641.206 |
| `bars_metamorphose_csv_trusted` | 584,459 | 185.548 | 588,203 | 186.736 | 589,991 | 187.304 |
| `bars_metamorphose_arrow_ipc_trusted` | 1,830,920 | 561.536 | 1,835,852 | 563.049 | 1,853,164 | 568.358 |
| `bars_metamorphose_parquet_trusted` | 650,309 | 107.959 | 642,078 | 106.592 | 645,679 | 107.190 |
| `bars_serde_json_baseline` | 483,994 | 501.259 | 494,818 | 512.470 | 492,501 | 510.070 |

## Old Baseline Comparison

Ratios are `new rows/s / old rows/s` for the labels bound by the spec.
This comparison is benchmark-contract evidence, not a broader production claim.

| Label | Row count | Run 1 ratio | Run 2 ratio | Run 3 ratio |
|---|---:|---:|---:|---:|
| `bars_mbt_full_encode_inspect_checked` vs `mathilde_binary_generated` | 100,000 | 0.718946 | 0.725889 | 0.725779 |
| `bars_metamorphose_json_checked` vs `metamorphose_json` | 100,000 | 3.146758 | 3.129272 | 3.152879 |
| `bars_metamorphose_protobuf_checked` vs `metamorphose_protobuf` | 100,000 | 6.947227 | 7.246473 | 7.330616 |
| `bars_metamorphose_csv_trusted` vs `metamorphose_csv_full_archived` | 100,000 | 1.475244 | 1.484693 | 1.489208 |
| `bars_metamorphose_arrow_ipc_trusted` vs `metamorphose_arrow_ipc_full_archived` | 100,000 | 0.838360 | 0.840618 | 0.848545 |
| `bars_metamorphose_parquet_trusted` vs `metamorphose_parquet_full_archived` | 100,000 | 0.616831 | 0.609024 | 0.612440 |

## Serde JSON Comparison

`serde_json_comparison` is recorded only for
`bars_metamorphose_json_checked`, as required by the spec. Ratios are
`checked generated JSON rows/s / serde_json rows/s`.

| Row count | Run 1 ratio | Run 2 ratio | Run 3 ratio |
|---:|---:|---:|---:|
| 1 | 1.950518 | 1.945758 | 2.107617 |
| 100 | 0.735322 | 0.763669 | 0.668985 |
| 500 | 0.662429 | 0.657810 | 0.652590 |
| 1,000 | 0.600797 | 0.617026 | 0.602085 |
| 10,000 | 0.846365 | 0.846236 | 0.824059 |
| 100,000 | 0.662095 | 0.644013 | 0.651924 |

## Correctness Results

Proved by tests and artifact checks in this run:

- all required old baseline labels were parsed for all required row counts;
- the run path helper skips existing report files;
- direct report overwrite is rejected;
- the current Bars regression label set is exact;
- serde DTO rows preserve the selected source fields;
- reports contain all required metadata and row fields;
- each generated report has exactly 60 rows;
- no generated report row has zero output bytes;
- old baseline comparisons and serde JSON comparisons are present in the
  required counts.

## Limitations

- The old-baseline comparison uses the old tracked benchmark file exactly as
  bound by the spec. It does not prove broader production parity beyond this
  benchmark contract.
- `dirty_state` is `dirty`, so these results bind this working tree state and
  not a clean release commit.
- The companion projection run is separate evidence. It is not merged into the
  Bars regression stability claim.
