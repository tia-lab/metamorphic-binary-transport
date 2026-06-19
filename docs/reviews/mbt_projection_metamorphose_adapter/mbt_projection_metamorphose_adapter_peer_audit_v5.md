# MBT Projection Metamorphose Adapter Peer Audit v5

## Status

Status: `PEER_AUDIT_PASSED`

Audited spec:

```text
docs/specs/mbt_projection_metamorphose_adapter_SPEC.md
```

Prior blocking audit:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit_v4.md
```

No code change is authorized by this audit.

## Required Reads

Read:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/specs/mbt_projection_direct_writer_SPEC.md`
- `docs/specs/mbt_metamorphose_migration_SPEC.md`
- `docs/specs/mbt_projection_metamorphose_adapter_SPEC.md`
- `docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_research_brief.md`
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`

## Audit Findings

No blocking findings remain.

## Falsification Checks

### Pre-audit closure gate

Passed.

The spec includes the mandatory sections in the required order and now records
the pre-audit closure checklist. It cites the prior projection and metamorphose
specs and states how this spec preserves or completes them.

### Architecture

Passed.

The spec now makes the corrected architecture explicit:

```text
source MBT -> projected MBT -> selected adapter
```

Adapters do not own projection logic, and transponding remains hidden.

### Adapter matrix

Passed.

The spec binds projection marker support for:

```text
json
protobuf
csv
transponding
arrow
arrow-ipc
parquet
```

It also binds the source marker to remain supported in the same generated
adapter file.

### Projected metadata derivation

Passed.

The v4 blocker is resolved. The spec now defines:

- source-to-projected physical field mapping;
- derived UTC retention and remapping;
- JSON/CSV output remapping;
- protobuf recursive remapping;
- dropping empty nested and parent protobuf messages.

### Transponding and columnar behavior

Passed.

The spec now binds `{module}_transponding.rs` as the owner of source plus
projection marker transponding sections. Arrow family adapters must import that
module and must not duplicate transponding.

### Generated artifact ownership

Passed.

The spec binds every Bars and test-compatibility adapter artifact and provides
exact command patterns with exact adapter/out tables. It also states that
literal substitution is the exact command and the implementation plan must not
alter flags.

### Code boundaries

Passed.

The implementation scope is limited to:

```text
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
schema projection metamorphose tests
generated adapter artifacts
```

Core, adapter crates, compression, transponding crate source, and proto files
are explicitly forbidden unless a future spec amendment binds them.

### Failure and trusted-access contracts

Passed.

The spec preserves existing checked and trusted access behavior and requires
projection marker validation against projected schema identity.

### Compile surface

Passed.

The spec defines default schema build checks, adapter feature checks, and the
`mbt_codegen` timed compile-surface check.

### Correctness oracle

Passed.

The spec defines generic codegen, Bars runtime, test-compatibility runtime,
generated-artifact, and source-stability oracles.

## Residual Risks

These are implementation risks, not spec blockers:

- projected protobuf metadata remapping is likely the most error-prone codegen
  edit;
- helper scoping must be applied consistently across all adapters;
- generated files may grow materially for schemas with many projections, but
  the compile-surface checks are bound.

## Classification

`PEER_AUDIT_PASSED`

The corrected spec is ready for an implementation plan. Code remains forbidden
until the implementation plan is written, audited, and explicitly approved.
