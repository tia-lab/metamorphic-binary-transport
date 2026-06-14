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

# Peer Audit V5: MBT Projection Migration

Status: `BLOCKED`

Slug: `mbt_projection_migration`

Target research brief:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_research_brief.md
```

Target spec:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Previous audits:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v2.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v3.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v4.md
```

## Required Reads

Completed reads:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/lifecycle_protocol.md
docs/protocols/spec_protocol.md
docs/protocols/peer_audit_protocol.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_research_brief.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v2.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v3.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v4.md
docs/specs/mbt_projection_migration_SPEC.md
docs/specs/mbt_workspace_architecture_SPEC.md
docs/specs/mbt_core_runtime_migration_SPEC.md
docs/specs/mbt_codegen_migration_SPEC.md
docs/specs/mbt_schema_core_generation_SPEC.md
proto/mathilde/options.proto
crates/codegen/src/config.rs
crates/codegen/src/emit.rs
crates/codegen/src/main.rs
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/test_rust_emit_core.rs
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

No run evidence was used. This is a no-code spec audit.

## Findings

### 1. Section 20 Still Contains The Old Core-Regression Choice

Severity: blocking

Peer audit v4 required the spec to replace the open choice:

```text
temporary-output or unit-test regression check
```

with one exact proof surface.

The amended Section 9 now correctly chooses:

```text
cargo test -p metamorphic_binary_transport_codegen core_surface_ignores_projection_declarations_and_emits_no_projection_symbols
```

and binds:

```text
crates/codegen/src/tests/test_rust_emit_core.rs
```

The amended Section 23 also correctly repeats that named test in the
pre-audit closure checklist.

However, Section 20 still says:

```text
--surface core remains core-only and must be tested through a temporary
output or codegen unit test;
```

This keeps the exact ambiguity that v4 blocked. Because Section 20 is the
generated artifact binding section, the conflict matters: one part of the spec
chooses the named test, while another part still leaves the proof shape open.

Why this blocks:

- `docs/protocols/spec_protocol.md` requires one exact command or test surface
  before audit;
- the pre-audit closure checklist says the command surface is closed;
- Section 20 still gives implementation planning a choice;
- the spec is the source of truth, so conflicting sections cannot be treated as
  harmless wording.

Required amendment:

Replace the Section 20 bullet with the exact named test. The replacement should
state:

```text
--surface core remains core-only and must be tested by
core_surface_ignores_projection_declarations_and_emits_no_projection_symbols
in crates/codegen/src/tests/test_rust_emit_core.rs;
```

After that amendment, search the spec for all of:

```text
temporary output
temporary-output
unit-test regression
or codegen unit test
```

The spec should contain none of those ambiguous phrases for this proof.

## Resolved Previous Audit Blockers

| Previous blocker | V5 result |
| --- | --- |
| Mandatory spec section order | Resolved |
| Original exact codegen-check command missing | Resolved |
| Projected schema-hash inputs under-specified | Resolved |
| Compile-surface budget not measurable | Resolved |
| Correctness oracle and test plan combined | Resolved |
| `--surface core` conceptual conflict | Resolved by `--surface projection` |
| Existing projection-ignored assertion migration missing | Resolved directionally |
| Generated artifact ownership conflict | Resolved |
| `crates/codegen/src/emit.rs` missing from bindings | Resolved |
| Exact projection `--inspect` command missing | Resolved |
| Exact projection `--write` command missing | Resolved |
| Projection inspect/write/check ownership missing | Resolved |
| Core-regression proof choice in Section 9/22/23 | Resolved |

## Audit Lens Results

| Lens | Result |
| --- | --- |
| Pre-audit closure gate completeness | Blocked by stale Section 20 ambiguity |
| Measured object clarity | Passed |
| Schema source ownership | Passed |
| Wire/archive validation | Passed |
| Trusted-access safety | Passed |
| Codegen determinism | Passed directionally |
| Generated-code compile surface | Passed |
| Crate boundary isolation | Passed |
| Dependency containment | Passed |
| Correctness oracle | Passed |
| Benchmark isolation | Passed |
| Performance budget | Passed |
| Failure behavior | Passed |
| Code binding completeness | Passed |
| Generated artifact binding completeness | Blocked by stale proof ambiguity |
| Client/operator interpretation safety | Blocked until Section 20 matches Section 9 |

## Required Amendment Summary

Amend:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Required change:

1. Replace the Section 20 `temporary output or codegen unit test` wording with
   the exact named core-regression test already defined in Sections 9, 18, 22,
   and 23.

No implementation is authorized by this audit.

## Classification

```text
BLOCKED
```
