# MBT Projection Metamorphose Adapter Implementation Plan Peer Audit v3

## Status

Status: `BLOCKED`

Audited implementation plan:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_implementation_plan.md
```

Audited spec:

```text
docs/specs/mbt_projection_metamorphose_adapter_SPEC.md
```

Passing spec peer audit:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit_v5.md
```

No code change is authorized by this audit.

## Findings

### 1. BLOCKER: `test_metamorphose.rs` edits are not justified

The spec says existing `test_metamorphose.rs` files remain source marker
adapter test owners and new `test_projection_metamorphose.rs` files own
projection marker adapter tests.

The plan lists existing `test_metamorphose.rs` files under files to edit, but
does not bind a specific source-marker test change. That creates avoidable
scope ambiguity.

Required fix:

- remove existing `test_metamorphose.rs` files from `Files To Edit` unless the
  plan binds exact source-marker assertions to change;
- keep projection tests only in the new `test_projection_metamorphose.rs`
  files.

### 2. BLOCKER: Runtime test readback policy is under-specified for IPC and Parquet

The plan says to assert columnar schema fields and mentions adapter readback
helpers, but it does not bind the exact helper functions.

Code-read evidence:

- `crates/adapters/arrow_ipc/src/lib.rs` exposes
  `record_batch_from_ipc_stream`.
- `crates/adapters/parquet/src/lib.rs` should be checked and exact readback
  helper names must be bound if Parquet schema assertions are required.

Required fix:

- bind exact Arrow direct schema inspection through `ArrowRecordBatch`;
- bind exact Arrow IPC readback helper if used;
- bind exact Parquet readback helper if used, or narrow Parquet runtime oracle
  to non-empty deterministic bytes plus generated-source schema assertions if
  no helper exists.

### 3. BLOCKER: Generated check commands are still delegated to “patterns”

The spec passed with literal substitution, but the implementation plan should
be operationally explicit. It must state that the implementation executes all
14 write commands and all 14 check commands by iterating the two exact adapter
tables, and that each command must be recorded in the result review.

Required fix:

- add an explicit command execution rule for all adapter table rows;
- add result review requirement to record all 28 concrete commands or the
  exact generated command list.

## Non-Blocking Observations

- The plan now correctly targets every adapter selector.
- The plan correctly forbids core, adapter crate, proto, dependency, and serving
  edits.
- The helper scoping strategy matches the spec.
- The projected metadata derivation step now matches the corrected spec.

## Classification

`BLOCKED`

The implementation plan is close but not ready. The blockers are scope and
validation precision issues, not architecture issues.
