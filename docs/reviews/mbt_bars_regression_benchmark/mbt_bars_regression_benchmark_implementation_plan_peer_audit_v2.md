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

# Implementation Plan Peer Audit V2: MBT Bars Regression Benchmark

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15
Status: `PEER_AUDIT_PASSED`

## Required Reads

| Evidence type | Source | Observation |
|---|---|---|
| Protocol evidence | `AGENTS.md` | Implementation requires a passed peer audit, approved implementation plan, and exact bindings before code changes. |
| Invariant evidence | `docs/invariants/core_invariants.md` | Benchmarks stay outside production libraries; dependency and compile surfaces must be bounded and measured. |
| Protocol evidence | `docs/protocols/lifecycle_protocol.md` | The implementation plan must bind every code file, generated file, dependency change, test, benchmark, artifact, and validation command. |
| Protocol evidence | `docs/protocols/peer_audit_protocol.md` | The audit must classify exactly `PEER_AUDIT_PASSED` or `BLOCKED`. |
| Protocol evidence | `docs/protocols/implementation_protocol.md` | Implementation must stay inside approved spec and plan bindings. |
| Spec evidence | `docs/specs/mbt_bars_regression_benchmark_SPEC.md` | The amended spec now binds `Cargo.lock` as a Cargo-generated dependency artifact and forbids manual lockfile edits. |
| Prior audit evidence | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan_peer_audit.md` | V1 blocked because the plan introduced `Cargo.lock` while the spec did not bind it. |
| Plan evidence | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md` | The amended plan now binds `Cargo.lock`, lockfile validation, and panic-free test implementation. |

## Audit Result

`PEER_AUDIT_PASSED`

The amended implementation plan resolves the blocker from the first
implementation-plan audit. It is aligned with the amended spec and is ready for
implementation approval. This audit does not authorize code by itself.

## Findings

No blocking findings remain.

## Blocker Resolution Check

### 1. `Cargo.lock` is now bound by both spec and plan

Prior audit blocker:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan_peer_audit.md
```

The first implementation-plan audit blocked because the plan allowed
`Cargo.lock` to change but the spec did not bind `Cargo.lock`.

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md
```

Observed amended contract:

- `crates/benches/Cargo.toml` is the only hand-edited dependency file.
- `Cargo.lock` is an allowed Cargo-generated dependency artifact.
- manual `Cargo.lock` edits are forbidden.
- `Cargo.lock` may change only as Cargo resolution from the approved
  `crates/benches/Cargo.toml` dependency edits.
- the result review must record lockfile/dependency graph validation.

Plan evidence:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md
```

Observed amended contract:

- `Cargo.lock` appears in the Cargo-generated artifact section.
- lockfile changes are limited to the approved `serde` and `serde_json`
  benchmark dependency edits and their required transitives.
- implementation must stop if Cargo resolves unexplained dependencies.
- `Cargo.lock` is included in the diff and rollback boundary.

This resolves the prior spec/plan mismatch.

### 2. Test implementation is now explicitly panic-free

Prior audit note:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan_peer_audit.md
```

The first audit requested that test wording avoid panic-style implementation if
the intended standard is panic-free tests.

Plan evidence:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md
```

Observed amended contract:

- tests must return `Result<(), Box<dyn std::error::Error>>`;
- tests must avoid `unwrap`, `expect`, `panic!`, `assert!`, `assert_eq!`, and
  `assert_ne!`;
- the pre-test grep includes those forbidden constructs for the new benchmark
  files and expects no matches.

This resolves the prior non-blocking note and makes the implementation rule
auditable.

## Passed Checks

- The implementation plan remains scoped to `crates/benches`.
- No production crate source file is in the approved edit list.
- The hand-edited dependency file remains only `crates/benches/Cargo.toml`.
- The Cargo-generated lockfile behavior now matches the spec.
- Bars schema feature activation matches the spec:
  `json`, `protobuf`, `csv`, `arrow_ipc`, and `parquet`.
- Bench-only `serde` and `serde_json` dependencies match the spec.
- Current benchmark labels match the spec.
- Old baseline labels and label mapping match the spec.
- Projection remains delegated to the existing projection benchmark.
- Report fields, evidence directory, result review path, validation commands,
  and three-run release benchmark sequence are bound.
- The plan keeps performance claims unproved until validation and benchmark
  evidence are recorded.

## Non-Blocking Notes

### Spec status line is stale but the artifact chain records the passed audit

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md
```

The spec status line still says `DRAFT_AMENDED_AWAITING_PEER_AUDIT_V2`.

Peer-audit evidence:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_peer_audit_v2.md
```

The separate spec peer audit v2 records `PEER_AUDIT_PASSED`. The stale status
line is documentation hygiene, not an implementation-plan blocker, because the
required separate audit artifact exists and passed.

### `/usr/bin/time` availability remains a validation-time dependency

The plan binds `/usr/bin/time -v` commands for build-surface evidence and
allows the same `cargo check` commands without `/usr/bin/time` if the tool is
unavailable. In that case, the result review must explicitly record that max
RSS was not collected.

## Final Decision

`PEER_AUDIT_PASSED`

The next required step is explicit user approval to implement:

```text
Approved: implement docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md
```
