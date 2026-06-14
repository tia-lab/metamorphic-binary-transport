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

# Peer Audit V4: MBT Core Runtime Migration

Audit classification: `PEER_AUDIT_PASSED`

Spec audited:

```text
docs/specs/mbt_core_runtime_migration_SPEC.md
```

Implementation plan audited:

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
- `docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_implementation_plan.md`

## Findings

No blocking findings remain in the spec or implementation plan.

### F1. `thiserror` Is Now Pinned Exactly

Severity: resolved.

The paused implementation proved that:

```toml
thiserror = "2.0.17"
```

allows Cargo to resolve `thiserror v2.0.18`. That violates the plan's intended
dependency tree. The amended spec and plan now require the exact Cargo pin:

```toml
thiserror = "=2.0.17"
```

The validation commands now also prove the resolved versions:

```bash
cargo tree -p metamorphic_binary_transport_core | rg -n "thiserror v2\\.0\\.17"
cargo tree -p metamorphic_binary_transport_core | rg -n "thiserror-impl v2\\.0\\.17"
```

This closes the semver drift gap.

## Evidence

Code-read evidence:

- The spec status is `DRAFT_AWAITING_PEER_AUDIT_V4`.
- The spec dependency contract now lists `thiserror = "=2.0.17"`.
- The spec lockfile policy now refers to `thiserror = "=2.0.17"`.
- The spec validation commands now check for `thiserror v2.0.17` and
  `thiserror-impl v2.0.17`.
- The implementation plan dependency binding now lists
  `thiserror = "=2.0.17"`.
- The implementation plan source binding for
  `crates/metamorphic_binary_transport_core/Cargo.toml` now emits the exact
  pin.
- The implementation plan validation commands now check for
  `thiserror v2.0.17` and `thiserror-impl v2.0.17`.

Run evidence:

- Documentation unresolved-marker scan returned no matches.
- `git diff --check` passed after the amendment.

## Current Implementation State

The paused implementation code and lockfile still need the follow-up code
change when implementation resumes:

- current `crates/metamorphic_binary_transport_core/Cargo.toml` still contains
  the loose dependency string from the previous approved plan;
- current `Cargo.lock` was generated with `thiserror v2.0.18`.

This does not block the amended spec and plan. It means the next implementation
step must update the manifest to `=2.0.17`, let Cargo regenerate the lockfile,
and rerun the full validation command list.

## Audit Lenses

Passed:

- dependency containment;
- dependency-version determinism;
- lockfile policy;
- validation command completeness;
- implementation-plan alignment with the amended spec.

## Decision

The amended spec and implementation plan pass peer audit.

Implementation may resume after explicit user approval.
