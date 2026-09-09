# Peer Audit: MBT Bars Regression Benchmark Corrective Amendment

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15
Audited spec:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md
```

Classification: `PEER_AUDIT_PASSED`

## Audit Scope

This audit reviews only the corrective amendment that requires:

- exact old Bars fixture parity;
- exact old iteration-count parity;
- exact timing-boundary parity;
- invalidation of prior single-pass projection-fixture evidence for
  old-baseline parity claims.

This audit does not approve code changes. A corrective implementation plan is
still required before editing benchmark code.

## Required Reads

| Evidence type      | Source                                                                                         | Observed contract                                                                                                                     |
| ------------------ | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Protocol evidence  | `AGENTS.md`                                                                                    | Specs require separate peer audit and an approved implementation plan before code changes.                                            |
| Protocol evidence  | `docs/invariants/core_invariants.md`                                                           | Baselines must use identical logical payloads; benchmark setup work must be outside measured loops unless specified.                  |
| Protocol evidence  | `docs/protocols/spec_protocol.md`                                                              | Specs must bind exact code paths, benchmark method, correctness oracle, and artifact ownership.                                       |
| Protocol evidence  | `docs/protocols/peer_audit_protocol.md`                                                        | The audit must try to falsify the spec and classify exactly `PEER_AUDIT_PASSED` or `BLOCKED`.                                         |
| Protocol evidence  | `docs/protocols/testing_benchmark_protocol.md`                                                 | Performance claims require identical logical payload and correctness before speed claims.                                             |
| Spec evidence      | `docs/specs/mbt_bars_regression_benchmark_SPEC.md`                                             | Corrective amendment binds old fixture, old iteration table, timing boundaries, and blocked prior artifacts.                          |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/fixtures.rs` | Old benchmark fixture uses `DEFAULT_SEED`, `FIRST_CLOSE_MS`, old `Lcg`, pair/time ordering, and selective presence branches.          |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/bench.rs`    | Old benchmark uses row-count-specific iteration counts and computes throughput from `row_count * iterations` over accumulated timing. |
| Code-read evidence | `crates/benches/src/projection.rs`                                                             | Current projection fixture is not the old fixture and must not be used for old-baseline parity comparison.                            |
| Result evidence    | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md`    | Existing run ratios were produced before the corrective amendment and cannot prove old-baseline parity.                               |

## Findings

No blocking findings remain for the corrective spec amendment.

## Falsification Checks

### 1. Old fixture parity is now bound

The amended spec now points to the old fixture source:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/fixtures.rs
```

It binds:

- old `generate_rows(row_count, DEFAULT_SEED)` semantics;
- seed `0x4d415448494c4445`;
- first close timestamp `1_546_300_860_000`;
- old `Lcg` multiplier and increment;
- old pair and time ordering;
- old optional presence branches;
- generated row validation before returning rows.

This resolves the prior benchmark payload mismatch risk.

### 2. Projection fixture is explicitly excluded from old-baseline comparisons

The amended spec states that:

```rust
metamorphic_binary_transport_benches::projection::bars_rows(row_count)
```

must not be used for old-baseline regression comparisons.

This directly addresses the observed implementation mistake where the new
regression benchmark delegated to the projection fixture.

### 3. Old iteration-count parity is now bound

The amended spec binds the old iteration table:

| Label       |    Rows | Iterations |
| ----------- | ------: | ---------: |
| `one`       |       1 |         50 |
| `small`     |     100 |         50 |
| `page_500`  |     500 |         50 |
| `page_1000` |   1,000 |         50 |
| `medium`    |  10,000 |         10 |
| `large`     | 100,000 |          3 |

It also defines throughput as:

```text
rows_per_second = (row_count * iterations) / accumulated_total_seconds
mb_per_second = accumulated_output_bytes / 1024 / 1024 / accumulated_total_seconds
```

This resolves the single-pass timing mismatch.

### 4. Timing-boundary parity is now explicit enough for implementation planning

The amended spec binds the old-baseline lanes and required measured boundary:

- full MBT includes encode, inspect, checksum extraction, and response checksum
  inside each timed iteration;
- archived/trusted CSV, Arrow IPC, and Parquet conversion occurs after encode
  and checked archive access setup;
- JSON and protobuf checked lanes must use the old fixture and old iteration
  count.

The corrective implementation plan must bind the exact code edits that enforce
these boundaries. The spec no longer leaves the measurement boundary to
implementation discretion.

### 5. Correctness oracle now proves fixture identity before speed comparison

The amended spec requires parsing old `mathilde_binary_generated` semantic
checksums and matching them for every required row count.

This is the correct blocker: if semantic checksum parity fails, performance
comparison against the old crate is blocked.

### 6. Prior run artifacts are correctly downgraded

The amended spec states that:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_1.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_2.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_3.json
```

are valid only as evidence of the first benchmark implementation behavior and
must not be used to claim old-baseline parity or regression.

This prevents the current result review from being interpreted as a proven
runtime regression.

### 7. Corrective implementation-plan path is bound

The amended spec requires the corrective plan at:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md
```

This satisfies the no-code-before-plan gate.

## Residual Risks

1. The old tracked benchmark file is markdown, not a structured machine
   artifact. The corrective implementation must parse required rows and fail
   closed if a required semantic checksum or baseline row is missing.
2. The old and new generated APIs have different crate/module structure. The
   implementation plan must bind exact function calls for every measured lane
   before code edits.
3. The current result review remains useful only as a failed benchmark-method
   artifact until a corrective result review replaces the old-baseline
   comparison.

These risks are implementation-plan requirements, not blockers for the amended
spec.

## Required Next Step

Write the corrective implementation plan:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md
```

The plan must bind:

- exact files to edit;
- exact fixture-port code location;
- exact old baseline parser additions;
- exact timing loop structure;
- exact JSON report field changes;
- exact tests;
- exact validation commands;
- exact benchmark rerun commands;
- result-review replacement or amendment path.

## Decision

`PEER_AUDIT_PASSED`

The amended spec is now sufficient for corrective implementation planning. It
does not authorize code changes.
