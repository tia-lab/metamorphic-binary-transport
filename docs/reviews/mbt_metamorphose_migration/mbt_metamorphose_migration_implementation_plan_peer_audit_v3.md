# Implementation Plan Peer Audit V3: MBT Metamorphose Migration

Status: `PEER_AUDIT_PASSED`
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

Prior implementation plan audits:

```text
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan_peer_audit.md
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan_peer_audit_v2.md
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
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan_peer_audit.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan_peer_audit_v2.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md`

## Audit Scope

This audit checks only the implementation-plan amendment that moved shared
byte-output helpers from `crates/metamorphose` to neutral `crates/core`
ownership. It does not authorize code by itself. Code may start only after
owner approval of the amended audited plan.

Audit lenses:

- dependency direction;
- adapter plug-and-play isolation;
- core compile-surface containment;
- code binding completeness;
- validation command completeness;
- rollback boundary consistency.

## Classification

`PEER_AUDIT_PASSED`

The amended implementation plan is implementation-ready after owner approval.

## Findings

### 1. Adapter-to-metamorphose dependency direction is fixed

Result: passed.

The previous implementation attempt exposed a plan contradiction: adapter
writer crates needed `CheckedBytes`, but the plan placed it under
`crates/metamorphose`, which would have forced adapters to depend on
`metamorphose` or duplicate helper code.

The amended plan now says adapter crates must not depend on
`metamorphic_binary_transport_metamorphose`; they import checked output helpers
from `metamorphic_binary_transport_core::output`.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:216-221`

No blocker found.

### 2. Core helper ownership is narrow and dependency-free

Result: passed.

The amended plan authorizes only a dependency-free core output helper module:

```text
crates/core/src/output.rs
```

The plan binds `CheckedBytes`, length helpers, protobuf length helpers, UTC
helpers, and base64 writing helpers there.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:352-388`

The plan also states that no external dependency may be added to `crates/core`.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:400`

No blocker found.

### 3. Metamorphose remains traits and dispatch, not adapter utility storage

Result: passed.

The amended plan keeps the public trait and dispatch surface in
`crates/metamorphose/src/lib.rs`, while `crates/metamorphose/src/runtime.rs`
may remain small or empty. It must not contain duplicated checked byte writer
logic.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:270-349`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:402-404`

No blocker found.

### 4. Core edit boundary is explicitly limited

Result: passed.

The plan still forbids broad core edits, but now allows exactly:

- `crates/core/src/output.rs`;
- `crates/core/src/lib.rs`;
- `crates/core/src/tests/test_output.rs`.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:80-89`

The rollback boundary uses the same exception list.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:1060-1071`

No blocker found.

### 5. Validation now covers the new core helper surface

Result: passed.

The validation commands now run core tests before adapter and schema tests:

```bash
cargo test -p metamorphic_binary_transport_core --all-targets
```

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:961-972`

No blocker found.

### 6. Prior V2 blockers remain closed

Result: passed.

The amendment does not reopen the prior V2 closures:

- Arrow IPC and Parquet still do not depend on the Arrow adapter crate;
- adapter failures still map to existing `TransportError` variants;
- generated adapter files still have one codegen owner;
- transponding remains hidden.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:223-238`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:242-273`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:786-816`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:833-876`

No blocker found.

## Non-Blocking Observations

### A. Core output helper is public adapter support, not a user API

The plan states `CheckedBytes` is public only because adapter crates are
separate crates.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:391-392`

The implementation should keep documentation clear that this is support API.

### B. The plan still requires owner approval before code

The plan status is amended and audit-ready. This audit passes, but code still
requires owner approval of this amended implementation plan.

## Verdict

`PEER_AUDIT_PASSED`

The amended implementation plan now matches the corrected architecture:

```text
core::output
  -> adapters/*

metamorphose
  -> public traits and dispatch only

schema generated adapter modules
  -> selected adapter + metamorphose traits
```

No implementation blocker remains in the audited plan.
