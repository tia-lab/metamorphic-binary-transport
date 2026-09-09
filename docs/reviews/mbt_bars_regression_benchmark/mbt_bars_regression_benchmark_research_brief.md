# MBT Bars Regression Benchmark Research Brief

Slug: `mbt_bars_regression_benchmark`
Status: `RESEARCH_BRIEF_COMPLETE_AWAITING_SPEC_AUDIT`
Date: 2026-06-15
Repository: `/home/tia/_DEV/MATHILDE/metamorphic-binary-transport`

## Measured Object

The measured object is the new split MBT workspace runtime performance against
the old tracked `mathilde-binary-transport` Bars benchmark evidence.

The benchmark must cover only the surfaces needed for a final first regression
view:

- full Bars MBT encode plus checked access/inspect;
- full Bars metamorphose JSON, protobuf, CSV, Arrow IPC, and Parquet;
- current bench-only serde JSON baseline for the same generated Bars rows;
- existing MBT-to-MBT projection benchmark artifacts.

Compression and wide-schema performance are out of scope for this pass.

## Source Materials

| Evidence type               | Source                                                                                       | Observation                                                                                                                             |
| --------------------------- | -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| Code-read evidence          | `crates/benches/src/bin/mbt_projection_bench.rs`                                             | Existing dedicated projection benchmark writes JSON artifacts and compares Bars projection labels against old tracked baselines.        |
| Code-read evidence          | `crates/benches/src/projection.rs`                                                           | Existing fixture generation covers Bars and test compatibility rows and has old baseline parsing for projection labels.                 |
| Code-read evidence          | `crates/schemas/bars_core/Cargo.toml`                                                        | Adapter dependencies are feature-gated in the schema crate. Core schema builds without adapter dependencies.                            |
| Code-read evidence          | `crates/schemas/bars_core/src/lib.rs`                                                        | Adapter modules are included only when selected features are enabled.                                                                   |
| Code-read evidence          | `crates/schemas/bars_core/src/bars_v1*.rs`                                                   | Generated schema exposes core encode/access/inspect, projection functions, and feature-gated metamorphose functions.                    |
| Benchmark evidence          | `docs/evidence/mbt_projection_direct_writer/projection_run_4.json`                           | One projection smoke/regression run exists for the split workspace. It is not enough for final stability claims.                        |
| Benchmark baseline evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md` | Old tracked table includes Bars full MBT, protobuf, JSON, metamorphose JSON/protobuf, CSV, Arrow IPC, Parquet, and compression results. |
| Protocol evidence           | `docs/protocols/testing_benchmark_protocol.md`                                               | Benchmark artifacts must record environment, command, dataset identity, row counts, profile, and results.                               |

## Candidate Approach

Add exactly one new benchmark binary:

```text
crates/benches/src/bin/mbt_bars_regression_bench.rs
```

Move reusable Bars regression helpers into:

```text
crates/benches/src/bars_regression.rs
```

Keep the existing projection benchmark as the projection evidence source:

```text
crates/benches/src/bin/mbt_projection_bench.rs
docs/evidence/mbt_projection_direct_writer/*.json
```

The new benchmark must not duplicate projection measurement. It may parse and
reference existing projection output in the result review, but projection
measurement remains owned by the existing projection benchmark.

The benchmark command is:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

The benchmark crate may add bench-only `serde` and `serde_json` dependencies to
measure the current normal Rust DTO to JSON baseline. These dependencies must
not enter core, schema, codegen, metamorphose, transponding, or adapter crates.

## MBT Binding Surface

The benchmark binds:

- schema: `metamorphic_binary_transport_schema_bars::bars_v1`;
- row fixture: existing deterministic `projection::bars_rows`;
- max response bytes: `1_073_741_824`;
- row counts: `[1, 100, 500, 1_000, 10_000, 100_000]`;
- old baseline file:
  `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md`;
- current projection benchmark output:
  `docs/evidence/mbt_projection_direct_writer/projection_run_*.json`.

## Unknowns

| Unknown                                                                                     | Required evidence                                                                         |
| ------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Whether full core MBT split workspace stays within old tracked large-row performance        | Run `mbt_bars_regression_bench` under release profile and compare 100k rows to old table. |
| Whether metamorphose JSON/protobuf/CSV is at parity with old tracked evidence               | Run current generated adapter paths and compare labels by row count.                      |
| Whether current serde JSON baseline is slower or faster than current metamorphose JSON      | Add bench-only serde DTO path and measure same logical rows.                              |
| Whether Arrow IPC and Parquet adapter paths are already comparable enough for a final claim | Measure current full archived paths and compare to old tracked full archived labels.      |
| Whether benchmark output is stable enough                                                   | Require at least three runs before final result review claims stability.                  |

## Risks

| Risk                             | Mitigation                                                                                            |
| -------------------------------- | ----------------------------------------------------------------------------------------------------- |
| Over-bloating benchmark scope    | One new binary only; projection remains existing bench; no compression or wide-schema lane.           |
| Pulling serde into production    | Add serde dependencies only to `crates/benches`, which is `publish = false`.                          |
| Comparing non-identical payloads | Use the same deterministic `bars_rows` fixture for MBT, metamorphose, and serde JSON baseline.        |
| Old table parsing drift          | Fail if required old labels and row counts cannot be parsed.                                          |
| Hiding instability               | Write raw JSON per run and require a later result review to report spread across at least three runs. |

## Required Decisions Before Implementation

1. Approve that the current projection benchmark remains the sole projection
   measurement owner for this pass.
2. Approve bench-only `serde` and `serde_json` dependencies in `crates/benches`
   for the current normal Rust DTO to JSON baseline.
3. Approve that protobuf direct DTO baseline is not reimplemented in the new
   workspace for this pass. Old tracked protobuf baseline remains the external
   comparison table; current metamorphose protobuf is measured directly.
4. Approve that compression and wide-schema benchmarks are out of scope.

## Recommended Next Phase

Proceed to the spec and peer audit for `mbt_bars_regression_benchmark`.
