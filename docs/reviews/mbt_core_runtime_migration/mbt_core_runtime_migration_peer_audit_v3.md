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

# Peer Audit V3: MBT Core Runtime Migration

Audit classification: `PEER_AUDIT_PASSED`

Spec audited:

```text
docs/specs/mbt_core_runtime_migration_SPEC.md
```

Prior audit:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit_v2.md
```

Implementation plan reviewed for the blocker:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_implementation_plan.md
```

## Required Reads

Completed reads:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/specs/mbt_core_runtime_migration_SPEC.md`
- `docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit_v2.md`
- `docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_implementation_plan.md`

## Findings

No blocking findings remain in the spec.

### F1. `Cargo.lock` Is Now Bound

Severity: resolved.

The implementation plan identified a valid blocker: adding `thiserror` will
update `Cargo.lock`, but the previous spec did not bind that artifact.

The amended spec now binds `Cargo.lock` in three places:

- crate boundary contract;
- dependency contract;
- code bindings.

The amended contract is narrow:

- `Cargo.lock` must not be manually edited;
- the lockfile may change only because Cargo resolves
  `thiserror = "2.0.17"` and its required transitive dependencies;
- any other lockfile package addition is rejected unless
  `cargo tree -p metamorphic_binary_transport_core` proves it belongs to that
  dependency chain.

This resolves the implementation-plan blocker at the spec level.

## Evidence

Code-read evidence:

- The spec status is `DRAFT_AWAITING_PEER_AUDIT_V3`.
- Section 10 now lists `Cargo.lock` as an allowed Cargo-generated dependency
  artifact and forbids manual editing.
- Section 11 now states that any lockfile update must be recorded with the
  resulting dependency tree.
- Section 19 now includes `Cargo.lock` in allowed implementation files and
  restates the non-manual lockfile policy.
- Section 22 now requires the implementation plan to bind the exact
  `Cargo.lock` update policy.

Run evidence:

- Documentation unresolved-marker scan returned no matches.
- `git diff --check` passed after the amendment.

## Audit Lenses

Passed:

- dependency containment;
- code binding completeness for dependency-lock artifacts;
- generated artifact exclusion;
- crate boundary isolation;
- implementation-plan readiness at the spec level.

## Non-Blocking Follow-Up

The existing implementation plan still has status
`BLOCKED_PENDING_SPEC_AMENDMENT` and still contains prose stating that the spec
does not bind `Cargo.lock`. That is now stale because this spec amendment
resolved the blocker.

Before implementation approval, amend:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_implementation_plan.md
```

to reflect the v3 spec and this audit.

## Decision

The amended spec passes peer audit.

Implementation planning may continue. Code implementation is still blocked
until the implementation plan is amended and explicitly approved.
