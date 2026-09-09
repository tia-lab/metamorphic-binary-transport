# Implementation Plan Peer Audit: MBT Projection Migration

Status: `BLOCKED`

Slug: `mbt_projection_migration`

Target implementation plan:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan.md
```

Target spec:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Passed spec audit:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v6.md
```

## Required Reads

Completed reads:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/lifecycle_protocol.md
docs/protocols/spec_protocol.md
docs/protocols/implementation_protocol.md
docs/protocols/code_style_protocol.md
docs/protocols/codegen_protocol.md
docs/protocols/peer_audit_protocol.md
docs/specs/mbt_projection_migration_SPEC.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v6.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan.md
```

No run evidence was used. This is a no-code implementation-plan audit.

## Findings

### P0: Implementation plan does not bind the newly required plan-audit gate

The implementation plan still allows implementation after only this approval
text:

```text
Approved: implement docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan.md
```

That is incomplete under the now-required stricter workflow: the
implementation plan itself must state that implementation may start only after
this separate implementation-plan audit passes.

Required amendment:

- add this audit artifact to the plan inputs or required gates;
- update the approval gate so implementation is allowed only after a passed
  implementation-plan peer audit;
- keep the existing explicit user approval requirement after the audit passes.

This is a process blocker, not a design blocker.

## Non-Blocking Checks Passed

The plan otherwise binds the expected implementation surface:

- exact files to edit;
- exact file to create;
- exact generated file and codegen ownership;
- files explicitly out of scope;
- no dependency changes;
- codegen inspect/write/check commands;
- validation commands;
- compile-surface evidence commands;
- pre-test audit checklist;
- rollback boundary.

The plan also preserves the main constraints from the passed spec audit:

- `--surface core` remains projection-free;
- `--surface projection` owns the committed compatibility generated artifact;
- generated projection code is MBT-to-MBT only;
- no adapter, MDB, cache, lookup, or service code enters this phase;
- checked and trusted projection paths remain distinct;
- wrong-marker rejection tests are required;
- nullable-array null versus present-empty tests are required.

## Required Amendment Before Re-Audit

Amend:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan.md
```

The amendment must add:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan_peer_audit_v2.md
```

as the required passed audit artifact before implementation.

The approval gate should become equivalent to:

```text
Implementation may start only after:
1. implementation-plan peer audit v2 passes;
2. the user explicitly approves:

Approved: implement docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan.md
```

## Classification

```text
BLOCKED
```
