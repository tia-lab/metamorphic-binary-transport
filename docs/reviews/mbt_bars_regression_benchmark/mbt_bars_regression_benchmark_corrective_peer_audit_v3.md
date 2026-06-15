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

# Peer Audit V3: MBT Bars Regression Benchmark Corrective Feature Amendment

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15

Audited spec:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md
```

Classification: `PEER_AUDIT_PASSED`

## Audit Scope

This audit reviews only the feature-gated old parity amendment added after the
implementation-plan audit v2 blocked the plan. The reviewed amendment must
prove that the spec now authorizes:

- the old-crate `bars-regression-parity-only` feature;
- old `Cargo.toml` and old `src/lib.rs` edits;
- feature-gated old generated module exposure;
- feature-gated old validation and benchmark commands;
- no generated-file writes;
- no dependency package or lockfile changes.

This audit does not approve code changes. Code still requires an amended
implementation plan, an implementation-plan peer audit, and explicit
implementation approval.

## Required Reads

| Evidence type | Source | Observed contract |
|---|---|---|
| Protocol evidence | `/home/tia/_DEV/MATHILDE/experiments/AGENTS.md` | Code changes require approved spec, approved implementation plan, and evidence-bound validation. |
| Protocol evidence | `/home/tia/_DEV/MATHILDE/experiments/docs/protocols/experiment_lifecycle_protocol.md` | Spec, peer audit, implementation plan, implementation, validation, and result review are separate phases. |
| Protocol evidence | `/home/tia/_DEV/MATHILDE/experiments/docs/protocols/peer_audit_protocol.md` | Audit must try to falsify the target artifact and classify exactly `PEER_AUDIT_PASSED` or `BLOCKED`. |
| Spec evidence | `docs/specs/mbt_bars_regression_benchmark_SPEC.md` | The spec now binds the old parity-only feature, old `Cargo.toml`, old `src/lib.rs`, feature-gated old commands, and generated-file prohibitions. |
| Prior audit evidence | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan_peer_audit_v2.md` | The prior blocker required this spec amendment before implementation could proceed. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs` | Old crate currently exposes `pub mod generated;` unconditionally, so the old parity-only feature requires a bound `src/lib.rs` edit. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs` | Old binary currently imports non-parity benchmark modules, so the old parity-only feature requires a bound `src/main.rs` command/import gate. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs` | Old benchmark module graph currently exports non-parity benchmark modules. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/mod.rs` | Old test module graph currently exports non-parity tests, including wide/primitives surfaces. |

## Findings

No blocking findings remain.

## Falsification Checks

### 1. The old feature is now explicitly authorized by the spec

The spec now authorizes exactly one old-crate feature:

```toml
bars-regression-parity-only = []
```

It states that the feature is benchmark-only, is not a dependency feature, and
must not change non-featured old-crate behavior.

This resolves the prior plan-audit blocker where the plan authorized a feature
that the spec had not bound.

### 2. Old `Cargo.toml` and old `src/lib.rs` are now bound edit targets

The spec now lists these old files in the approved old edit set:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/Cargo.toml
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs
```

This resolves the prior spec-plan mismatch. The implementation plan may now
bind these files, but only for the feature-gated parity path.

### 3. The feature-gated generated module contract is explicit

The spec now states:

```text
feature enabled:
  old generated module surface exposes only bars_v1 and bars_v1_proto

feature disabled:
  old generated module graph remains unchanged
```

It also forbids editing:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/*.rs
```

This is sufficient for the old parity-only compile reduction without
hand-editing generated artifacts.

### 4. The feature-gated old command surfaces are now bound

The spec now binds the old parity benchmark command as:

```text
cd /home/tia/_DEV/MATHILDE/experiments
cargo run --release -p mathilde_binary_transport --features bars-regression-parity-only -- bench-bars-regression-parity --report-dir crates/mathilde-binary-transport/docs/evidences/bars_regression_parity
```

It also binds the old parity compile and test commands with:

```text
--features bars-regression-parity-only
```

This resolves the prior mismatch where the spec named unfeatured old commands
while the implementation plan needed feature-gated commands.

### 5. Dependency and lockfile boundaries are explicit

The spec states:

- no dependency package may be added or changed;
- no lockfile may change;
- the old `Cargo.toml` edit is limited to the empty
  `bars-regression-parity-only` feature;
- root workspace manifests and lockfiles remain forbidden to edit.

This is strict enough to prevent the feature amendment from becoming an
unbounded dependency change.

### 6. Benchmark interpretation remains bounded

The spec still requires:

- old and new benchmarks to use the same current Bars fixture semantics;
- old and new labels to match;
- at least three old parity runs and three new split runs before stability
  claims;
- full MBT semantic checksum comparison before speed ratios;
- historical `bench_results.md` to remain context only, not a speed baseline.

The feature amendment does not weaken the benchmark correctness oracle.

## Residual Risks

1. The old parity-only feature may still compile an unintended non-Bars module
   if the implementation gates are incomplete. The implementation plan and its
   audit must require code-read verification of the feature-gated module graph.
2. The old benchmark remains single-pass if the new benchmark remains
   single-pass. Three old and three new release runs are still required before
   any stability statement.
3. Arrow IPC and Parquet byte equality across old and new dependency surfaces
   remains unproved. The result review must record byte checksums as evidence
   and avoid byte-equality claims unless determinism is proved.

## Required Next Step

Amend the corrective implementation plan if needed so it matches this spec
exactly, then rerun the implementation-plan audit:

```text
Approved: amend docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md per peer audit v3
```

## Decision

`PEER_AUDIT_PASSED`

The amended spec now authorizes the old parity-only feature and resolves the
spec-plan blockers identified by the implementation-plan audit v2. It does not
authorize code changes by itself.
