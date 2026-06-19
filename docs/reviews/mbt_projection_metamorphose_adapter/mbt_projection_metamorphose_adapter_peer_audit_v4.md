# MBT Projection Metamorphose Adapter Peer Audit v4

## Status

Status: `BLOCKED`

Audited spec:

```text
docs/specs/mbt_projection_metamorphose_adapter_SPEC.md
```

Research brief:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_research_brief.md
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

## Findings

### 1. BLOCKER: Generated artifact commands are not fully exact

The spec binds command patterns plus adapter tables in Section 20. That is
close, but `docs/protocols/spec_protocol.md` requires exact command surfaces
before peer audit. The spec should list the exact check/write command expansion
policy in a way that cannot be interpreted as implementation-plan work.

Required fix:

- state that every adapter-table row is an exact command after literal
  substitution;
- explicitly bind that the implementation plan must not add or remove command
  flags;
- add the exact `--check` command pattern alongside the `--write` command
  pattern, not only a prose sentence.

### 2. BLOCKER: Projected row-format metadata derivation is not precise enough

Section 9 says projection support completes `derived_utc_fields`,
`json_csv_output_fields`, and `protobuf_messages`, but it does not define the
exact mapping algorithm.

The risky case is protobuf nested output: a removed source field can make a
nested protobuf message empty, and the spec must define whether that message is
dropped or retained as an empty message. Empty generated messages would change
boundary semantics.

Required fix:

- define a source-index-to-projected-index map from `ProjectionModel` field
  mappings;
- define JSON/CSV projected output derivation from the source output list;
- define derived UTC retention only when the UTC source field is retained;
- define protobuf projected message recursion:
  - retained physical output maps to projected field index;
  - retained derived UTC output maps to projected derived UTC index;
  - nested messages with no retained child fields are dropped;
  - parent messages with no retained child fields are dropped.

### 3. BLOCKER: Arrow family import contract is under-specified

Current Arrow, Arrow IPC, and Parquet generated files import:

```text
use crate::{module}_transponding::*;
```

If transponding generation starts emitting source and projection marker
sections in one `{module}_transponding.rs` file, the columnar adapter files can
reuse that import. The spec should bind that single-file transponding artifact
owns both source and projected marker transponding support.

Required fix:

- add an explicit statement that `*_transponding.rs` contains source plus all
  projection marker column batch types and private `transpond_archived` methods;
- add an explicit statement that Arrow family adapter files must not duplicate
  transponding logic and must continue importing `{module}_transponding::*`.

### 4. BLOCKER: Test file bindings allow ambiguous file creation

Section 19 binds both existing `test_metamorphose.rs` files and possible new
`test_projection_metamorphose.rs` files. That is acceptable only if the spec
defines what each owns. Currently ownership is ambiguous.

Required fix:

- keep existing `test_metamorphose.rs` for source marker adapter tests;
- bind new `test_projection_metamorphose.rs` files for projection marker
  adapter tests, or explicitly state existing files will be extended instead.

## Non-Blocking Observations

- The corrected architecture now matches the core invariant that projection is
  MBT-to-MBT before boundary conversion.
- The spec correctly rejects serving-specific generated artifacts inside the
  MBT repository.
- The spec correctly keeps MBT core and adapter crates out of this scope unless
  a later plan proves a helper gap.
- The spec correctly requires a separate implementation-plan audit before
  code.

## Classification

`BLOCKED`

The spec is directionally correct but not implementation-ready. The blockers
are specific and can be fixed by amending the spec before another peer audit.
