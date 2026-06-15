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

# Peer Audit: MBT Bars Regression Benchmark Corrective Amendment V2

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15

Audited spec:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md
```

Classification: `PEER_AUDIT_PASSED`

## Audit Scope

This audit reviews the corrective V2 amendment only:

- remove generated archived adapter entrypoint work from scope;
- keep current new split codegen and runtime APIs unchanged;
- add an old-MBT benchmark parity port under the experiments crate;
- compare old and new implementations under the same current benchmark
  semantics;
- treat historical `bench_results.md` as context only, not as the corrective
  speed baseline.

This audit does not approve code changes. The corrective implementation plan
must still be amended, audited, and approved before implementation.

## Required Reads

| Evidence type | Source | Observed contract |
|---|---|---|
| Protocol evidence | `AGENTS.md` | Specs require separate peer audit and approved implementation plan before code changes. |
| Protocol evidence | `docs/invariants/core_invariants.md` | Baselines must use identical logical payloads; benchmark setup work must be outside measured loops unless specified. |
| Protocol evidence | `docs/protocols/lifecycle_protocol.md` | Spec, peer audit, implementation plan, implementation, validation, and result review are separate phases. |
| Protocol evidence | `docs/protocols/spec_protocol.md` | Spec must bind code paths, generated artifacts, benchmark method, correctness oracle, and pre-audit closure. |
| Protocol evidence | `docs/protocols/peer_audit_protocol.md` | Audit must try to falsify the spec and classify exactly `PEER_AUDIT_PASSED` or `BLOCKED`. |
| Spec evidence | `docs/specs/mbt_bars_regression_benchmark_SPEC.md` | Corrective V2 binds old-MBT parity-port benchmark and removes generated-entrypoint work. |
| Code-read evidence | `crates/benches/src/bars_regression.rs` | Current new benchmark fixture delegates to `projection::bars_rows(row_count)`. |
| Code-read evidence | `crates/benches/src/projection.rs` | Current Bars fixture uses one BTCUSDT entity, deterministic timestamps starting at `1_700_000_000_000`, all allowed presence bits, and deterministic arithmetic values. |
| Code-read evidence | `crates/benches/src/bin/mbt_bars_regression_bench.rs` | Current new benchmark labels include checked and trusted metamorphose lanes and currently measures one execution per row count and label. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs` | Old generated Bars code exposes checked metamorphose functions, JSON/protobuf trusted functions, trusted archived access, and crate-internal archived helpers for CSV, Arrow IPC, and Parquet. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/compatibility.rs` | Old crate benchmark modules can use trusted archived access plus crate-internal archived helpers in measured lanes. |

## Findings

No blocking findings remain for the corrective V2 spec amendment.

## Falsification Checks

### 1. The prior generated-entrypoint path is removed

The amended spec states that the prior path requiring generated archived
adapter entrypoints is no longer approved.

This resolves the previous blocker where exact historical archived timing in
the new split benchmark could not be reproduced without changing generated API
surface.

### 2. The speed baseline is now a same-semantics old parity port

The amended spec defines the only valid corrective comparison as:

```text
old MBT implementation + new benchmark semantics
vs
new split MBT implementation + same benchmark semantics
```

This satisfies the benchmark invariant requiring identical logical payloads
and avoids mixing old historical fixture/timing boundaries with current split
workspace measurements.

### 3. Historical `bench_results.md` is no longer a speed baseline

The amended spec keeps:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md
```

only as context. It explicitly forbids using that markdown table as the speed
baseline for the corrective V2 comparison.

This prevents the old archived timing boundary from re-entering through result
review interpretation.

### 4. Fixture identity is bound to the current new benchmark fixture

The amended spec binds the fixture source to:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport/crates/benches/src/projection.rs
```

and requires the old parity port to mirror `projection::bars_rows(row_count)`.

This is the correct binding for the accepted comparison because the goal is to
port the new benchmark shape into the old crate, not to force the new benchmark
back to the old `generate_rows` fixture.

### 5. Trusted access policy is explicit

The amended spec requires checked and trusted lanes to remain separate and
states that trusted lanes are valid only because benchmark bytes are produced
by the schema encoder and remain immutable.

The old parity port is allowed to use old-crate internal archived helpers only
to mirror current trusted byte-boundary semantics. It is forbidden from using
the historical old archived setup timing boundary as the comparison baseline.

This is precise enough for implementation planning.

### 6. Code bindings are exact for both repositories

The amended spec binds new split files:

```text
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
```

It also binds old experiments files:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/bars_regression_parity.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_bars_regression_parity.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/mod.rs
```

No generated file is approved for editing.

### 7. Command surfaces are exact

The amended spec binds the new benchmark command:

```text
cd /home/tia/_DEV/MATHILDE/metamorphic-binary-transport
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

and the old parity-port command:

```text
cd /home/tia/_DEV/MATHILDE/experiments
cargo run --release -p mathilde_binary_transport -- bench-bars-regression-parity --report-dir crates/mathilde-binary-transport/docs/evidences/bars_regression_parity
```

This satisfies the command-surface requirement for the next implementation
plan.

### 8. Correctness oracle is sufficient for this corrective pass

The amended spec requires full MBT semantic checksums to match between old and
new before speed comparison.

For byte outputs it requires response checksum and output length, while noting
that byte equality is required only if the implementation plan proves writer
determinism across old and new dependency surfaces.

This is acceptable because the measured regression claim is implementation
speed under identical input fixture and timing boundary, not a new cross-format
semantic decoder proof.

## Residual Risks

1. The current new benchmark is single-pass per row count and label. The spec
   handles this by requiring the old parity port to match that boundary and by
   requiring at least three runs before stability claims. This is a residual
   measurement-stability risk, not a spec blocker.
2. The old parity port spans another repository under
   `/home/tia/_DEV/MATHILDE/experiments`. The implementation plan must bind
   validation commands from both repository roots.
3. Arrow IPC and Parquet bytes may include dependency-specific metadata. The
   spec correctly requires response checksums to be recorded and avoids
   requiring byte equality unless determinism is proved in the implementation
   plan.

## Required Next Step

Amend the corrective implementation plan:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md
```

The amended plan must remove the `BLOCKED_BEFORE_CODE` generated-entrypoint
path and bind the old-MBT parity-port implementation exactly.

## Decision

`PEER_AUDIT_PASSED`

The amended spec is sufficient for corrective implementation-plan amendment.
It does not authorize code changes.
