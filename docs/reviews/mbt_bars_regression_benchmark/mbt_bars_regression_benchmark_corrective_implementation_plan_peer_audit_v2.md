# Implementation Plan Peer Audit V2: MBT Bars Regression Benchmark Corrective Plan

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15

Audited implementation plan:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md
```

Classification: `BLOCKED`

## Audit Scope

This audit reviews the amended corrective implementation plan after the
addition of the old-crate `bars-regression-parity-only` feature. The specific
question is whether the amended plan is still implementable under the approved
spec and protocol gates.

This audit does not approve code changes.

## Required Reads

| Evidence type      | Source                                                                                                       | Observed contract                                                                                                                                                                    |
| ------------------ | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Protocol evidence  | `AGENTS.md`                                                                                                  | Code changes require an approved spec and approved implementation plan; a plan cannot introduce behavior not bound by the spec.                                                      |
| Protocol evidence  | `/home/tia/_DEV/MATHILDE/experiments/docs/protocols/experiment_lifecycle_protocol.md`                        | Implementation plan must bind exact code files and must follow the approved spec.                                                                                                    |
| Protocol evidence  | `/home/tia/_DEV/MATHILDE/experiments/docs/protocols/implementation_protocol.md`                              | Implementation must stop if code requires behavior not in the spec or files outside approved bindings.                                                                               |
| Protocol evidence  | `/home/tia/_DEV/MATHILDE/experiments/docs/protocols/peer_audit_protocol.md`                                  | Audit must try to falsify the artifact and classify exactly `PEER_AUDIT_PASSED` or `BLOCKED`.                                                                                        |
| Spec evidence      | `docs/specs/mbt_bars_regression_benchmark_SPEC.md`                                                           | The current spec binds old parity edits only under old `src/benches`, old `src/tests`, and old `src/main.rs`; it also states no old `Cargo.toml` or lockfile edit is allowed.        |
| Plan evidence      | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md` | The amended plan now authorizes old `Cargo.toml`, old `src/lib.rs`, and feature-gated old module graph changes.                                                                      |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs`                            | Old crate currently exposes `pub mod generated;` unconditionally and would need `src/lib.rs` changes to compile only Bars generated modules under the proposed feature.              |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs`                           | Old binary currently imports `bars_regression_parity`, `bench`, `compatibility`, and `compression`; a parity-only feature must gate this import surface to avoid non-parity modules. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs`                    | Old benchmark module graph currently exports non-parity benchmark modules.                                                                                                           |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/mod.rs`                      | Old test module graph currently exports many non-parity tests, including feature-gated codegen tests and wide/primitives tests.                                                      |

## Findings

### 1. BLOCKER: The plan authorizes files that the spec does not bind

The amended implementation plan adds these old-crate files to the edit set:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/Cargo.toml
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs
```

The current spec code bindings list old edits only for:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/mod.rs
```

and old created files only under:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences
```

This is a spec-plan mismatch. The feature-gated compile reduction may be the
right implementation, but it cannot be implemented until the spec explicitly
authorizes the old `Cargo.toml` and old `src/lib.rs` edits.

### 2. BLOCKER: The spec currently forbids the old-crate manifest edit

The amended plan approves this old-crate manifest change:

```toml
bars-regression-parity-only = []
```

The current spec dependency contract says:

```text
No `Cargo.toml` or `Cargo.lock` edit is allowed for the old parity port unless a
later implementation plan proves it is required and is separately audited.
```

That sentence is not enough to authorize the feature now because the same spec
still does not bind the old `Cargo.toml` file or define the feature contract.
The spec must be amended to state that the old-crate empty feature is approved,
that it has no dependencies, and that old `Cargo.lock` must remain unchanged.

### 3. BLOCKER: The benchmark command in the spec does not match the amended plan

The current spec binds the old parity command as:

```text
cargo run --release -p mathilde_binary_transport -- bench-bars-regression-parity --report-dir crates/mathilde-binary-transport/docs/evidences/bars_regression_parity
```

The amended plan changes the old parity command to:

```text
cargo run --release -p mathilde_binary_transport --features bars-regression-parity-only -- bench-bars-regression-parity --report-dir crates/mathilde-binary-transport/docs/evidences/bars_regression_parity
```

This is the command needed to avoid compiling old non-Bars generated modules,
but it is not yet the command bound in the spec. The spec must be amended so
the benchmark methodology, compile-surface budget, validation commands, and
evidence metadata all name the feature-gated old command.

### 4. BLOCKER: The spec does not define the feature-gated generated module contract

The amended plan requires old `src/lib.rs` to expose only:

```rust
pub mod generated {
    pub mod bars_v1;
    pub mod bars_v1_proto;
}
```

under `bars-regression-parity-only`.

That is a material crate-boundary behavior. It is acceptable only if the spec
defines:

- the feature is benchmark-only;
- the feature must not edit `src/generated/mod.rs`;
- the feature must not edit generated files;
- non-featured old `pub mod generated;` behavior remains unchanged;
- the feature-gated old crate may compile only Bars generated modules, the
  parity benchmark module, the parity test module, and the parity CLI dispatch.

Without this spec binding, the implementation plan is too broad for code.

### 5. Non-blocking formatting issue in the plan

The plan section "The plan intentionally does not" has one bullet ending with
a period before the next bullet. This is a documentation consistency issue,
not a blocker. It can be cleaned during the spec/plan amendment pass.

## Required Amendments

Before implementation can proceed, amend:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md
```

Required spec additions:

1. Add old `Cargo.toml` and old `src/lib.rs` to the approved old file edit
   bindings.
2. Authorize only this old-crate feature:

```toml
bars-regression-parity-only = []
```

3. State that the feature is benchmark-only and has no dependencies.
4. State that old `Cargo.lock` must remain unchanged.
5. Bind old `src/lib.rs` feature behavior:

```text
feature enabled: generated exposes only bars_v1 and bars_v1_proto
feature disabled: existing generated module graph remains unchanged
```

6. Bind old `src/benches/mod.rs`, old `src/tests/mod.rs`, and old `src/main.rs`
   feature behavior:

```text
feature enabled: compile only parity benchmark, parity test, and parity command
feature disabled: existing old modules and commands remain unchanged
```

7. Replace old validation and benchmark commands with the feature-gated
   commands.
8. State that old `src/generated/mod.rs` and all generated files remain
   forbidden to edit.

After the spec is amended, amend the implementation plan if needed and rerun
the implementation-plan audit.

## Decision

`BLOCKED`

The amended implementation plan is technically plausible, but it is not yet
compliant with the approved spec. The next step is a spec amendment that binds
the old parity-only feature, old `Cargo.toml`, old `src/lib.rs`, and
feature-gated old validation and benchmark commands.
