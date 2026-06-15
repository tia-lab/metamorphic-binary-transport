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

# Implementation Plan Peer Audit V3: MBT Bars Regression Benchmark Corrective Plan

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15

Audited implementation plan:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md
```

Classification: `PEER_AUDIT_PASSED`

## Audit Scope

This audit reviews whether the amended corrective implementation plan now
matches the passed corrective spec peer audit v3. The specific focus is the
old-crate `bars-regression-parity-only` feature added to avoid compiling old
non-Bars generated modules and old non-parity benchmark/test modules for the
parity benchmark command.

This audit does not approve code by itself. Code may start only after this
plan is explicitly approved for implementation.

## Required Reads

| Evidence type | Source | Observed contract |
|---|---|---|
| Protocol evidence | `/home/tia/_DEV/MATHILDE/experiments/AGENTS.md` | Code changes require approved spec, passed peer audit, approved implementation plan, and bounded validation. |
| Protocol evidence | `/home/tia/_DEV/MATHILDE/experiments/docs/protocols/experiment_lifecycle_protocol.md` | Implementation plan must bind files, commands, artifacts, validation, and rollback before code. |
| Protocol evidence | `/home/tia/_DEV/MATHILDE/experiments/docs/protocols/implementation_protocol.md` | Implementation must stop if code requires behavior outside the spec or plan. |
| Protocol evidence | `/home/tia/_DEV/MATHILDE/experiments/docs/protocols/peer_audit_protocol.md` | Audit must try to falsify the artifact and classify exactly `PEER_AUDIT_PASSED` or `BLOCKED`. |
| Spec evidence | `docs/specs/mbt_bars_regression_benchmark_SPEC.md` | The spec now authorizes the old parity-only feature, old `Cargo.toml`, old `src/lib.rs`, feature-gated old commands, and no generated-file edits. |
| Spec audit evidence | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_peer_audit_v3.md` | The spec amendment passed and resolved the prior spec-plan mismatch. |
| Prior plan audit evidence | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan_peer_audit_v2.md` | Prior blocker was old `Cargo.toml`, old `src/lib.rs`, and feature-gated command behavior not yet being spec-bound. |
| Plan evidence | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md` | The plan now binds old `Cargo.toml`, old `src/lib.rs`, feature-gated module surfaces, feature-gated validation, and feature-gated benchmark commands. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs` | Old crate currently exposes `pub mod generated;` unconditionally; implementing the feature requires a bound `src/lib.rs` module-gate edit. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs` | Old binary currently imports `bars_regression_parity`, `bench`, `compatibility`, and `compression`; implementing the feature requires gating non-parity imports and dispatch under the feature. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs` | Old benchmark module graph currently exports non-parity benchmark modules. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/mod.rs` | Old test module graph currently exports many non-parity tests. |

## Findings

No blocking findings remain.

## Falsification Checks

### 1. The plan now matches the spec file bindings

The plan binds old edits to:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/Cargo.toml
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/mod.rs
```

Those files are now approved by the amended spec. The prior plan-audit blocker
is resolved.

### 2. The old feature contract is bounded

The plan approves exactly one old-crate feature:

```toml
bars-regression-parity-only = []
```

It forbids dependency package changes, workspace manifest changes, and
lockfile changes. It also states that the feature must not change old
non-featured behavior.

This matches the spec dependency contract.

### 3. Generated files remain protected

The plan forbids edits to:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/mod.rs
generated files generally
```

The feature-gated old Bars-only generated surface is implemented at the old
`src/lib.rs` module boundary, which the spec now allows.

### 4. Feature-gated validation and benchmark commands are exact

The plan binds old validation to:

```text
cargo test -p mathilde_binary_transport --features bars-regression-parity-only test_bars_regression_parity
/usr/bin/time -v cargo check -p mathilde_binary_transport --all-targets --features bars-regression-parity-only
```

It binds old release runs to:

```text
cargo run --release -p mathilde_binary_transport --features bars-regression-parity-only -- bench-bars-regression-parity --report-dir crates/mathilde-binary-transport/docs/evidences/bars_regression_parity
```

This matches the amended spec and directly targets the blocked old compile
surface.

### 5. Benchmark correctness and result-review gates remain intact

The plan still requires:

- old parity fixture semantics to mirror current new `projection::bars_rows`;
- exact label and row-count parity;
- full MBT semantic checksum equality before speed ratios;
- three old parity runs and three new split runs;
- historical markdown ratios not to be used as corrective speed evidence;
- result review separation between raw evidence and old-vs-new ratios.

The feature amendment does not weaken the benchmark proof path.

### 6. Rollback and expected outputs are bounded

The rollback boundary includes exactly the amended file set plus created
parity artifacts. The expected outputs include no generated-file changes, no
lockfile changes, and only the empty feature edit in old `Cargo.toml`.

This is sufficient for implementation containment.

## Residual Risks

1. The feature-gated old binary command surface is intentionally narrow. The
   implementation must ensure that this narrow surface exists only under
   `bars-regression-parity-only` and that non-featured old dispatch remains
   unchanged.
2. The feature gate may still compile an unintended old non-Bars module if any
   old module import is missed. The plan mitigates this by requiring code-read
   verification that the feature-gated build compiles only Bars generated
   modules and parity benchmark/test modules.
3. The old and new benchmarks remain single-pass unless a later plan changes
   both. Three old and three new release runs remain mandatory before any
   stability claim.
4. Arrow IPC and Parquet byte determinism across old and new dependency
   surfaces remains unproved. The result review must not require byte equality
   unless determinism is separately proved.

## Required Next Step

The implementation plan can now be explicitly approved for code work:

```text
Approved: implement docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md
```

## Decision

`PEER_AUDIT_PASSED`

The amended corrective implementation plan is now compliant with the amended
spec and is sufficient to proceed once explicitly approved for implementation.
