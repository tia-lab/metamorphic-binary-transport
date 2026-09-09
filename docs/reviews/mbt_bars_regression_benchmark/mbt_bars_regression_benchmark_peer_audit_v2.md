# Peer Audit V2: MBT Bars Regression Benchmark

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15
Status: `PEER_AUDIT_PASSED`

## Required Reads

| Evidence type        | Source                                                                                   | Observation                                                                                                                               |
| -------------------- | ---------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Protocol evidence    | `AGENTS.md`                                                                              | Peer audit is a no-code phase. Code remains blocked until an approved implementation plan exists.                                         |
| Protocol evidence    | `docs/protocols/lifecycle_protocol.md`                                                   | A passed peer audit permits implementation planning, not code.                                                                            |
| Protocol evidence    | `docs/protocols/spec_protocol.md`                                                        | The spec must close command, artifact, dependency, code binding, generated artifact, and benchmark contracts.                             |
| Protocol evidence    | `docs/protocols/peer_audit_protocol.md`                                                  | The audit must try to falsify the amended spec and classify exactly `PEER_AUDIT_PASSED` or `BLOCKED`.                                     |
| Protocol evidence    | `docs/protocols/testing_benchmark_protocol.md`                                           | Benchmark artifacts must record environment, command, row counts, payload size, profile, and raw output path.                             |
| Invariant evidence   | `docs/invariants/core_invariants.md`                                                     | Adapter dependencies must remain outside core; benchmark surfaces must be isolated.                                                       |
| Prior audit evidence | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_peer_audit.md` | V1 blocked on Bars schema feature activation, exact label mapping, and conditional CSV comparison.                                        |
| Spec evidence        | `docs/specs/mbt_bars_regression_benchmark_SPEC.md`                                       | The amended spec binds dependency feature activation, benchmark labels, old labels, mandatory CSV comparison, and compile-surface checks. |

## Audit Result

`PEER_AUDIT_PASSED`

The amended spec resolves the blockers from the first peer audit. It is
implementation-plan ready. It still does not authorize code changes.

## Findings

No blocking findings remain.

## Blocker Resolution Check

### 1. Bars schema feature activation is now explicit

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:251-281
```

The spec now allows only `crates/benches/Cargo.toml` dependency edits and binds
the existing Bars schema dependency with:

```toml
features = ["json", "protobuf", "csv", "arrow_ipc", "parquet"]
```

Code-read evidence:

```text
crates/schemas/bars_core/Cargo.toml
```

The referenced features exist and enable the expected adapter surfaces. This
keeps adapter dependencies on the benches dependency edge and does not put them
into MBT core.

### 2. Generated function surface exists for the measured lanes

Code-read evidence:

```text
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
```

Observed functions include:

```text
BarsV1::encode
BarsV1::inspect
BarsV1::metamorphose_json
BarsV1::metamorphose_json_trusted_unchecked
BarsV1::metamorphose_protobuf
BarsV1::metamorphose_protobuf_trusted_unchecked
BarsV1::metamorphose_csv
BarsV1::metamorphose_csv_trusted_unchecked
BarsV1::metamorphose_arrow_ipc
BarsV1::metamorphose_arrow_ipc_trusted_unchecked
BarsV1::metamorphose_parquet
BarsV1::metamorphose_parquet_trusted_unchecked
```

The measured function names in the spec match the generated surface.

### 3. Exact label mapping is now bound in the spec

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:352-386
```

The spec now binds exact current labels, exact old baseline labels, and the
current-to-old comparison table.

Required old labels:

```text
mathilde_binary_generated
metamorphose_json
metamorphose_protobuf
metamorphose_csv_full_archived
metamorphose_arrow_ipc_full_archived
metamorphose_parquet_full_archived
```

Baseline evidence:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md
```

All required labels are present for the old tracked Bars benchmark table.

### 4. CSV comparison is now mandatory

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:384-386
```

The spec now states that CSV is mandatory because the old baseline contains
`metamorphose_csv_full_archived`. Missing CSV baseline evidence is a benchmark
failure under the failure contract.

### 5. Compile-surface checks now include dependency graph evidence

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:331-346
```

The spec requires:

```text
cargo check -p metamorphic_binary_transport_benches --all-targets
cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
cargo check -p metamorphic_binary_transport_core
cargo tree -p metamorphic_binary_transport_benches
```

This is sufficient for implementation planning. Actual build evidence belongs
to implementation validation and result review.

## Non-Blocking Notes

### JSON report field names remain an implementation-plan detail

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:466-474
docs/specs/mbt_bars_regression_benchmark_SPEC.md:542-550
```

The spec binds required report contents: command, environment, row count, label,
bytes, checksum, rows/sec, MB/sec, and optional old comparison. The exact JSON
field names are allowed to be bound by the implementation plan because the
report semantics are already fixed.

### Projection evidence remains a separate stability surface

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:418-428
```

The spec preserves the existing projection benchmark as the projection owner.
The result review must not merge a single projection run with the required
three-run Bars regression stability evidence.

## Passed Checks

- Mandatory section order is present.
- The old blockers are resolved.
- Benchmark code remains constrained to `crates/benches`.
- Production crates remain outside the approved edit surface.
- Adapter feature activation is isolated to the benches dependency edge.
- Serde dependencies are isolated to `crates/benches`.
- Generated schema artifacts remain inputs only and are owned by codegen check
  commands.
- Runtime benchmark labels and old baseline labels are exact.
- CSV old-baseline comparison is mandatory.
- Projection measurement remains owned by the existing projection benchmark.

## Final Decision

`PEER_AUDIT_PASSED`

The next required step is an implementation plan. No code changes are approved
by this audit.
