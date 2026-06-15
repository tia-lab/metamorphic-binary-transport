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

# Implementation Plan Peer Audit: MBT Bars Regression Benchmark Corrective Plan

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15

Audited implementation plan:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md
```

Classification: `PEER_AUDIT_PASSED`

## Audit Scope

This audit reviews whether the amended corrective implementation plan is
sufficient to implement the approved corrective V2 spec without drifting back
to the prior generated archived-entrypoint path or to the historical markdown
baseline comparison.

The audit does not approve code by itself. Code may start only after the
implementation plan is explicitly approved.

## Required Reads

| Evidence type | Source | Observed contract |
|---|---|---|
| Protocol evidence | `AGENTS.md` | Code requires an approved spec, passed peer audit, approved implementation plan, and bounded file edits. |
| Protocol evidence | `docs/invariants/core_invariants.md` | Baselines must use identical logical payloads; generated files are not edited by hand; performance claims require run evidence. |
| Protocol evidence | `docs/protocols/lifecycle_protocol.md` | Implementation plans are separate artifacts and must bind files, commands, artifacts, and validation. |
| Protocol evidence | `docs/protocols/implementation_protocol.md` | Implementation may start only after spec, peer audit, and plan approval; no unbound dependencies or generated manual edits. |
| Protocol evidence | `docs/protocols/peer_audit_protocol.md` | Audit must try to falsify the artifact and classify exactly `PEER_AUDIT_PASSED` or `BLOCKED`. |
| Protocol evidence | `docs/protocols/testing_benchmark_protocol.md` | Benchmark artifacts must record command, profile, environment, dataset identity, row count, and raw output path. |
| Protocol evidence | `docs/protocols/codegen_protocol.md` | Generated artifacts must not be edited and generated output must not require manual edits. |
| Spec evidence | `docs/specs/mbt_bars_regression_benchmark_SPEC.md` | Corrective V2 requires old-MBT parity-port benchmark and forbids generated archived entrypoint work. |
| Audit evidence | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_peer_audit_v2.md` | Corrective V2 spec passed and requires implementation-plan amendment before code. |
| Plan evidence | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md` | Plan binds old parity-port files, new benchmark cleanup files, commands, outputs, validation, rollback, and risks. |
| Code-read evidence | `crates/benches/src/bars_regression.rs` | Current new benchmark still contains historical `OLD_BENCH_RESULTS` parsing and comparison fields. |
| Code-read evidence | `crates/benches/src/bin/mbt_bars_regression_bench.rs` | Current binary parses old markdown baselines before running benchmark lanes. |
| Code-read evidence | `crates/benches/src/projection.rs` | Current fixture `bars_rows` is deterministic BTCUSDT-only with timestamps from `1_700_000_000_000` and all allowed presence bits set. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs` | Old generated code exposes `encode`, `inspect`, checked metamorphose functions, trusted JSON/protobuf functions, trusted archived access, and crate-internal archived CSV/Arrow IPC/Parquet helpers. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs` | Old binary dispatches benchmark subcommands from `main.rs`. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs` | Old benchmark modules are exported through `src/benches/mod.rs`. |

## Findings

No blocking findings remain.

## Falsification Checks

### 1. The plan no longer depends on generated archived entrypoints

The plan explicitly excludes new codegen modification and generated archived
adapter entrypoints. It also lists generated files as consumed inputs only.

This satisfies the corrective V2 requirement to compare old and new
implementations by porting the current benchmark semantics into the old crate,
not by changing the new generated API.

### 2. Historical markdown results are removed from the corrective baseline

The plan requires removing mandatory `OLD_BENCH_RESULTS` parsing from the new
benchmark run path and forbids populating speed comparison fields from
historical `bench_results.md`.

This is necessary because the accepted comparison is now:

```text
old MBT implementation + new benchmark semantics
vs
new split MBT implementation + same benchmark semantics
```

The plan preserves historical files only as superseded context in the result
review, not as speed evidence.

### 3. Old parity fixture mapping is exact enough to implement

The plan binds the old fixture to the current new `projection::bars_rows`
semantics and lists the old field-name differences:

```text
source_ordinal -> source
process_ordinal -> process
recomputed_reason_ordinal -> recomputed_reason
```

It also binds constants, row counts, timestamp formula, numeric formulas, and
old generated row validation.

This is sufficient to prevent accidental reuse of the old `generate_rows`
fixture.

### 4. Timing boundaries match the corrective spec

The plan binds each old parity label to a timing boundary matching the current
new benchmark semantics.

For trusted CSV, Arrow IPC, and Parquet it explicitly measures trusted archived
access inside the closure before calling archived helpers. That matches the
current new public trusted API boundary, where trusted access occurs inside the
called function.

This resolves the prior mismatch between historical old archived setup timing
and the new split trusted public path.

### 5. File boundaries are sufficiently constrained

The plan binds exact files to edit and create in both repositories and states
that no other existing file is approved.

The bound old `main.rs` edit includes the benchmark dispatch arm. The existing
old `main.rs` import style means implementation may also need to add the new
benchmark module to that same import statement or use a fully qualified path.
That is covered by the approved `main.rs` edit and is not a separate file
binding issue.

No production crate source file in the new split repository is approved for
editing.

### 6. Dependency policy is explicit

The plan forbids edits to both workspace dependency files and the old
`mathilde-binary-transport` crate manifest.

It also states that if a dependency change is required, work must stop and
return to spec and plan amendment.

This satisfies the dependency hygiene requirement.

### 7. Validation commands cover both repositories

The plan binds:

- formatting checks;
- forbidden-pattern scans for new and old edited benchmark files;
- new codegen `--check` commands for consumed generated artifacts;
- tests in the new benchmark crate and old experiment crate;
- compile checks for new benches, new Bars schema, core, and old MBT;
- dependency graph inspection for the new benches crate.

The commands are sufficient for implementation entry. Benchmark claims still
require the later run artifacts and corrective result review.

### 8. Result-review method prevents premature performance claims

The plan requires three old parity reports and three new split reports before
stability claims. It requires matching rows by label and row count and checking
full MBT semantic checksum equality before computing speed ratios.

This satisfies the benchmark invariant that correctness and determinism must
precede speed claims.

### 9. Rollback boundary is explicit

The rollback list matches the approved file edits and created files. It
excludes generated files and dependency files.

This is adequate for a bounded corrective implementation.

## Residual Risks

1. The current benchmark remains single-pass. The plan correctly requires the
   old parity port to mirror that boundary and forbids stability claims before
   three old and three new runs.
2. Arrow IPC and Parquet output bytes may differ across old and new dependency
   surfaces. The plan correctly records checksums and treats byte mismatch as
   evidence unless writer determinism is proved.
3. The implementation spans two repository roots. The plan mitigates this by
   binding commands from both roots.
4. The old parity report writer is newly created. Its overwrite refusal and
   report shape must be tested before benchmark runs are accepted.

## Required Next Step

The implementation plan can now be explicitly approved for code work:

```text
Approved: implement docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md
```

## Decision

`PEER_AUDIT_PASSED`

The amended corrective implementation plan is sufficient to proceed once it is
explicitly approved for implementation.
