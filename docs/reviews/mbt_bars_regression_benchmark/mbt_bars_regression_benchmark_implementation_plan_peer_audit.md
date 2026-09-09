# Implementation Plan Peer Audit: MBT Bars Regression Benchmark

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15
Status: `BLOCKED`

## Required Reads

| Evidence type        | Source                                                                                            | Observation                                                                                                                              |
| -------------------- | ------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Protocol evidence    | `AGENTS.md`                                                                                       | Implementation may start only after approved spec, passed peer audit, approved implementation plan.                                      |
| Protocol evidence    | `docs/protocols/lifecycle_protocol.md`                                                            | The implementation plan must bind every code file, generated file, dependency change, test, benchmark, artifact, and validation command. |
| Protocol evidence    | `docs/protocols/implementation_protocol.md`                                                       | Implementation must stay within approved spec and plan.                                                                                  |
| Protocol evidence    | `docs/protocols/peer_audit_protocol.md`                                                           | Block if dependency behavior or artifact bindings are under-specified.                                                                   |
| Spec evidence        | `docs/specs/mbt_bars_regression_benchmark_SPEC.md`                                                | Dependency edits are limited to `crates/benches/Cargo.toml`; code bindings do not list `Cargo.lock`.                                     |
| Prior audit evidence | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_peer_audit_v2.md`       | Spec passed after binding Bars features and exact benchmark labels.                                                                      |
| Plan evidence        | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md` | The plan introduces `Cargo.lock` as a Cargo-generated dependency artifact.                                                               |

## Audit Result

`BLOCKED`

The implementation plan is mostly aligned with the approved benchmark scope,
but it adds `Cargo.lock` to the implementation surface while the spec does not
bind `Cargo.lock` as an artifact. The plan cannot widen the approved spec.

## Findings

### 1. Blocker: `Cargo.lock` is introduced by the plan but not bound by the spec

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:249-281
```

The spec limits dependency edits to:

```text
crates/benches/Cargo.toml
```

Spec evidence:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md:482-502
```

The spec code bindings list:

```text
crates/benches/Cargo.toml
crates/benches/src/lib.rs
crates/benches/src/tests/mod.rs
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
docs/evidence/mbt_bars_regression_benchmark/.gitkeep
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
```

Plan evidence:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md:74-89
```

The plan adds:

```text
Cargo.lock may be changed only by Cargo as a dependency-resolution artifact
```

Plan evidence:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md:590-602
```

The rollback boundary also includes `Cargo.lock`.

This is the correct kind of artifact to bind because the workspace lockfile
currently does not contain `serde` or `serde_json`, and adding those pinned
bench-only dependencies can require a lockfile update. However, because the spec
does not bind `Cargo.lock`, implementation against the current plan would exceed
the approved spec.

Required amendment:

- amend `docs/specs/mbt_bars_regression_benchmark_SPEC.md` to bind
  `Cargo.lock` as a Cargo-generated dependency artifact;
- state that manual lockfile edits are forbidden;
- state that lockfile changes are allowed only as the Cargo resolution effect
  of the approved `crates/benches/Cargo.toml` dependency edits;
- require validation of the lockfile/dependency graph in the result review;
- then amend the implementation plan to reference the updated spec.

### 2. Non-blocking: Test wording should avoid panic-style implementation

Plan evidence:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md:378-414
```

The test descriptions use the word "assert", but the plan also says tests
should avoid `unwrap`, `expect`, and `panic!`. This is acceptable only if
implementation uses explicit `Result` returns instead of panic-style macros in
new test files.

Recommended plan amendment after the blocker is fixed:

- replace "assert" wording with "verify by returning an error";
- extend the pre-test grep to include `assert!`, `assert_eq!`, and
  `assert_ne!` for the new benchmark test file if the intended standard is
  panic-free tests.

This is non-blocking because the plan already states the no-panic intent for
new tests. The implementation must honor that intent.

## Passed Checks

- The measured benchmark scope matches the passed spec.
- Benchmark code remains under `crates/benches`.
- Production crate source files remain outside the edit list.
- The Bars schema feature activation matches the spec.
- The exact current benchmark labels and old baseline labels match the spec.
- The plan keeps projection measurement under the existing projection benchmark.
- The plan binds report fields, test file, validation commands, three-run
  benchmark sequence, and result review output.
- The plan keeps `serde_json` for the measured serde JSON baseline and forbids
  using it for benchmark report serialization.

## Final Decision

`BLOCKED`

No implementation is approved. The next step is to amend the spec and then
amend this implementation plan so `Cargo.lock` is bound at the spec level before
implementation approval.
