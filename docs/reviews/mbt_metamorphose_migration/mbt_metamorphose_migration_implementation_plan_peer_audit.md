# Implementation Plan Peer Audit: MBT Metamorphose Migration

Status: `BLOCKED`
Date: 2026-06-15
Slug: `mbt_metamorphose_migration`

Audited plan:

```text
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md
```

Approved spec:

```text
docs/specs/mbt_metamorphose_migration_SPEC.md
```

Passed spec audit:

```text
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_peer_audit.md
```

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`
- `docs/specs/mbt_metamorphose_migration_SPEC.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_peer_audit.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md`
- `crates/core/src/error.rs`

## Audit Scope

This audit checks whether the implementation plan is ready to authorize code.
It does not amend the plan and does not authorize implementation.

Audit lenses:

- exact dependency binding;
- crate boundary isolation;
- selected adapter compile surface;
- generated artifact ownership;
- failure contract closure;
- rollback boundary;
- validation command binding.

## Findings

### 1. Blocker: Arrow IPC and Parquet depend on the Arrow adapter crate

The spec binds strict adapter ownership:

- `crates/adapters/arrow` owns Arrow RecordBatch construction helpers only;
- `crates/adapters/arrow_ipc` owns Arrow IPC stream helpers only;
- `crates/adapters/parquet` owns Parquet byte writer helpers only.

Code-read evidence:

- `docs/specs/mbt_metamorphose_migration_SPEC.md:454-458`

The dependency contract also states:

- `crates/adapters/arrow_ipc`: Arrow IPC dependencies only;
- `crates/adapters/parquet`: Parquet writer dependencies only;
- each selected feature must pull only its selected adapter family.

Code-read evidence:

- `docs/specs/mbt_metamorphose_migration_SPEC.md:487-489`
- `docs/specs/mbt_metamorphose_migration_SPEC.md:508`

The implementation plan violates that boundary by adding:

```text
crates/adapters/arrow_ipc -> metamorphic_binary_transport_adapter_arrow
crates/adapters/parquet -> metamorphic_binary_transport_adapter_arrow
```

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:199-205`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:211-213`

Why this blocks code:

- enabling `arrow_ipc` or `parquet` would pull another adapter family;
- the dependency-tree evidence required by the spec would not prove selected
  adapter isolation;
- the plan introduces an implementation decision that the spec did not allow.

Required fix:

Either:

1. amend the implementation plan so Arrow IPC and Parquet do not depend on
   `metamorphic_binary_transport_adapter_arrow`; or
2. amend the spec to explicitly authorize a shared Arrow bridge crate or
   cross-adapter dependency, then rerun spec peer audit before implementation
   planning proceeds.

The lower-risk fix is to keep the current spec boundary and remove the
cross-adapter dependency from the plan.

### 2. Blocker: adapter failure mapping is not closed

The spec requires public metamorphose functions to return `Result` and defines
failure cases including response caps, unsupported adapter field kinds, invalid
UTF-8, invalid finite numeric values, and compile-time missing features.

Code-read evidence:

- `docs/specs/mbt_metamorphose_migration_SPEC.md:526-543`

The implementation protocol requires the implementation plan to bind the
failure contract before code starts.

Code-read evidence:

- `docs/protocols/implementation_protocol.md`

The plan currently leaves adapter error behavior unresolved:

```text
Stop and amend the spec before code if adapter error behavior requires new
TransportError variants.
```

and:

```text
Columnar adapter error variants may require a core error-surface amendment.
```

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:98-101`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:958-961`

The approved plan also forbids edits to `crates/core/src/*`.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:88-105`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:948-956`

Current `TransportError` has no adapter-specific error variant.

Code-read evidence:

- `crates/core/src/error.rs:5-47`

Why this blocks code:

- implementation would either need an unapproved core error change, or
  map adapter failures to existing errors without a bound contract;
- source, runtime, tests, and result review would not share one failure model;
- the plan is not closed enough to authorize adapter implementation.

Required fix:

Amend the plan to bind exact failure mapping for every adapter without core
edits, or amend the spec and plan to authorize exact core error variants and
the corresponding files. If a format cannot meet the failure contract under
the current core boundary, that format must be marked blocked for this
implementation plan.

## Non-Blocking Observations

### A. Unsafe public trusted helpers are intentionally outside `forbid(unsafe_code)`

The plan replaces `#![forbid(unsafe_code)]` in `crates/metamorphose/src/lib.rs`
with `#![deny(unsafe_op_in_unsafe_fn)]`.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:342-348`

This is acceptable because the approved spec requires visibly unsafe trusted
helpers and the plan says no unsafe blocks are introduced in
`crates/metamorphose`.

### B. Generated command ownership is bound

The plan lists write commands for every Bars and test-compatibility adapter
module and defines matching `--check` commands by replacement.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:680-708`

This is acceptable for implementation planning.

## Decision

`BLOCKED`

The implementation plan must be amended before code starts. The two blockers
are dependency-boundary violation for Arrow IPC/Parquet and incomplete adapter
failure mapping.

## Required Next Artifact

```text
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md
```

Required amendment:

1. remove or explicitly respecify the Arrow IPC / Parquet dependency on the
   Arrow adapter crate;
2. bind exact adapter error mapping or authorize exact core error variants
   through an audited spec amendment;
3. keep generated command ownership and validation commands aligned with the
   amended dependency and failure contracts.
