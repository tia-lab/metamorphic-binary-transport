# MBT Projection Metamorphose Adapter Peer Audit v6

## Status

Status: `PEER_AUDIT_PASSED`

Audited spec:

```text
docs/specs/mbt_projection_metamorphose_adapter_SPEC.md
```

Reason for v6 audit:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_implementation_plan_peer_audit_v3.md
```

The v3 implementation-plan audit found that Parquet runtime readback was
under-specified. The spec was amended to avoid adding a new public Parquet
readback helper in this scope.

No code change is authorized by this audit.

## Required Reads

Read:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/specs/mbt_projection_metamorphose_adapter_SPEC.md`
- `crates/adapters/arrow_ipc/src/lib.rs`
- `crates/adapters/parquet/src/lib.rs`

## Audit Findings

No blocking findings remain.

## Falsification Checks

### Parquet readback scope

Passed.

Code-read evidence:

- `crates/adapters/arrow_ipc/src/lib.rs` exposes
  `record_batch_from_ipc_stream`, so Arrow IPC schema readback can be tested.
- `crates/adapters/parquet/src/lib.rs` exposes write and checksum helpers but
  does not expose a public Parquet readback helper.

The amended spec no longer requires a new Parquet public readback helper. It
requires:

- deterministic non-empty Parquet bytes for checked/trusted projection marker
  paths;
- generated-source assertions proving the Parquet adapter builds its Arrow
  schema from the projected field list.

That is consistent with the no-adapter-crate-edits boundary.

### Architecture

Passed.

The amendment does not weaken the MBT-to-MBT projection before adapter
conversion rule.

### Correctness scope

Passed.

Arrow direct and Arrow IPC still provide runtime schema-field readback. Parquet
uses generated-source proof plus byte production because adding Parquet
readback would require adapter API work outside the spec boundary.

## Classification

`PEER_AUDIT_PASSED`

The amended spec remains ready for an implementation plan.
