# MBT Projection Metamorphose Adapter Implementation Plan Peer Audit v4

## Status

Status: `PEER_AUDIT_PASSED`

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
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit_v6.md
```

Prior blocking implementation-plan audit:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_implementation_plan_peer_audit_v3.md
```

No code change is authorized by this audit.

## Required Reads

Read:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`
- `docs/specs/mbt_projection_metamorphose_adapter_SPEC.md`
- `docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_implementation_plan.md`
- `crates/codegen/src/rust_emit.rs`
- `crates/adapters/arrow_ipc/src/lib.rs`
- `crates/adapters/parquet/src/lib.rs`

## Audit Findings

No blocking findings remain.

## Falsification Checks

### Spec and audit linkage

Passed.

The plan points to the v6 spec peer audit after the Parquet readback amendment.

### File scope

Passed.

Production implementation is limited to:

```text
crates/codegen/src/rust_emit.rs
```

Codegen tests are limited to:

```text
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
```

Projection runtime tests are isolated to new projection test files. Existing
source marker `test_metamorphose.rs` files are read but not edited.

### Forbidden edits

Passed.

The plan forbids MBT core, metamorphose runtime, transponding crate, adapter
crates, compression, proto, Cargo manifest, and lockfile edits.

### Projection metadata derivation

Passed.

The plan binds:

- source-to-projected field mapping;
- derived UTC remapping;
- JSON/CSV output remapping;
- protobuf recursive message remapping;
- dropping empty protobuf messages.

### Adapter matrix

Passed.

The plan covers all seven adapter selectors and all fourteen generated adapter
artifacts across Bars and test-compatibility schemas.

### Command surface

Passed.

The plan requires execution of:

- 14 write commands;
- 14 check commands;
- formatting;
- codegen tests;
- schema tests with all adapter features;
- default no-feature schema checks;
- timed `mbt_codegen` compile check;
- forbidden-source `rg` check.

The result review must record all concrete write/check commands or the exact
generated command list.

### IPC and Parquet validation

Passed.

The plan binds:

- direct Arrow schema inspection through `ArrowRecordBatch`;
- Arrow IPC readback through
  `mbt_adapter_arrow_ipc::record_batch_from_ipc_stream`;
- Parquet deterministic non-empty bytes plus generated-source projection schema
  assertions, without adding a new public Parquet readback helper.

### Dependency boundary

Passed.

No dependency changes are allowed. The required runtime test imports are
already through feature-gated schema crate adapter dependencies.

## Residual Risks

These are implementation risks, not plan blockers:

- projection protobuf message remapping is the highest-risk generator edit;
- source/projection helper scoping must be applied consistently across every
  adapter;
- generated adapter files may grow substantially for schemas with many
  projections, but compile-surface checks are required.

## Classification

`PEER_AUDIT_PASSED`

The implementation plan is ready for explicit implementation approval.
