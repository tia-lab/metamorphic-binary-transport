# Peer Audit: MBT Bars Regression Benchmark

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15
Status: `BLOCKED`

## Required Reads

| Evidence type      | Source                                                                                       | Observation                                                                                                                      |
| ------------------ | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Protocol evidence  | `AGENTS.md`                                                                                  | Peer audit is a no-code phase and must classify exactly `PEER_AUDIT_PASSED` or `BLOCKED`.                                        |
| Protocol evidence  | `docs/protocols/lifecycle_protocol.md`                                                       | Code cannot start while peer audit is blocked.                                                                                   |
| Protocol evidence  | `docs/protocols/spec_protocol.md`                                                            | The spec must close command, artifact, dependency, code binding, and generated artifact contracts before audit.                  |
| Protocol evidence  | `docs/protocols/peer_audit_protocol.md`                                                      | Block if dependency behavior, code bindings, or benchmark method are under-specified.                                            |
| Invariant evidence | `docs/invariants/core_invariants.md`                                                         | Benchmarks are isolated from production crates; adapter dependencies must not enter core.                                        |
| Spec evidence      | `docs/specs/mbt_bars_regression_benchmark_SPEC.md`                                           | The benchmark scope is Bars full MBT plus metamorphose JSON/protobuf/CSV/Arrow IPC/Parquet and a bench-only serde JSON baseline. |
| Research evidence  | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_research_brief.md` | The intended scope is one new benchmark binary and existing projection benchmark reuse.                                          |

## Audit Result

`BLOCKED`

The measured object is valid, but the spec is not internally closed. The main
issue is that the spec requires feature-gated Bars metamorphose surfaces while
the dependency contract only allows adding `serde` and `serde_json`.

## Findings

### 1. Blocker: Dependency contract conflicts with required schema features

Code-read evidence:

```text
crates/benches/Cargo.toml:10
metamorphic_binary_transport_schema_bars = { path = "../schemas/bars_core" }
```

Code-read evidence:

```text
crates/schemas/bars_core/Cargo.toml
json, protobuf, csv, arrow_ipc, parquet are feature-gated adapter surfaces.
```

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:146-150
The benchmark must compile the Bars schema crate with json,protobuf,csv,arrow_ipc,parquet.
```

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:251-266
Allowed dependency change is only serde and serde_json; no other dependency change is allowed.
```

The benchmark cannot call generated Bars metamorphose functions unless the
benches crate enables the required Bars schema features. This is not a new
external dependency, but it is still a dependency surface change and must be
bound explicitly in the dependency contract and compile-surface budget.

Required amendment:

```toml
# crates/benches/Cargo.toml
metamorphic_binary_transport_schema_bars = {
    path = "../schemas/bars_core",
    features = ["json", "protobuf", "csv", "arrow_ipc", "parquet"]
}
```

or an equivalent explicitly bound feature activation.

### 2. Blocker: Exact benchmark label mapping is deferred too late

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:506-514
The implementation plan must bind exact benchmark labels and old baseline labels.
```

The spec already knows the benchmark surfaces and the old baseline labels. The
label mapping is part of the benchmark contract, not an implementation detail.
Deferring it to the implementation plan violates the pre-audit closure gate.

Old baseline evidence:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md
mathilde_binary_generated
metamorphose_json
metamorphose_protobuf
metamorphose_csv_full_archived
metamorphose_arrow_ipc_full_archived
metamorphose_parquet_full_archived
```

Required amendment:

- bind exact current output labels;
- bind exact old baseline labels;
- bind the current-label to old-label comparison table;
- require failure when any required label is absent for required row counts.

### 3. Blocker: CSV comparison is conditional despite proved baseline presence

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:341-342
current metamorphose CSV 100k rows versus old metamorphose_csv_full_archived if that label exists.
```

Baseline evidence:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md:767-812
metamorphose_csv_full_archived exists for all standard row counts, including 100,000.
```

The comparison must be mandatory. Keeping it conditional weakens the benchmark
contract and could silently hide a missing lane.

Required amendment:

- make `metamorphose_csv_full_archived` a required old baseline label;
- make missing CSV baseline a benchmark failure.

### 4. Warning: Projection evidence must remain clearly separated from stability claims

Run evidence exists for one projection smoke/regression run:

```text
docs/evidence/mbt_projection_direct_writer/projection_run_4.json
```

The spec correctly states that a single run cannot prove stability. The result
review must keep this separation explicit and must not merge one projection run
with the required three-run Bars regression evidence as if they had the same
stability level.

## Passed Checks

- The spec uses the mandatory section order.
- The benchmark code is constrained to `crates/benches`.
- Production crates are not approved for edits.
- The spec preserves the existing projection benchmark as the projection owner.
- The bench-only serde JSON baseline is isolated to the benchmark crate.
- Compression, wide schema, storage, service, and network benchmarks are out of
  scope for this pass.

## Required Amendments Before Re-Audit

1. Amend the dependency contract to allow the Bars schema feature activation in
   `crates/benches/Cargo.toml`.
2. Bind exact current benchmark labels and exact old baseline labels in the
   spec, not only in the implementation plan.
3. Make CSV comparison mandatory because the old baseline label exists.
4. Add feature-surface validation commands, including:

```text
cargo check -p metamorphic_binary_transport_benches --all-targets
cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
cargo tree -p metamorphic_binary_transport_benches
```

5. Keep the existing projection benchmark as the only projection measurement
   owner for this pass.

## Final Decision

`BLOCKED`

No implementation plan or code changes are allowed until the spec is amended
and a follow-up peer audit passes.
