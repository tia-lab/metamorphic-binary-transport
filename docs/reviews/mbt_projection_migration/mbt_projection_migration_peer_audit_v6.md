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

# Peer Audit V6: MBT Projection Migration

Status: `PEER_AUDIT_PASSED`

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
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v5.md
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
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v5.md
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

No blocking findings.

## V5 Blocker Resolution

Peer audit v5 blocked on this stale Section 20 ambiguity:

```text
temporary output or codegen unit test
```

The current spec no longer contains that open proof choice. Section 20 now
binds the exact core-only regression proof:

```text
core_surface_ignores_projection_declarations_and_emits_no_projection_symbols
```

in:

```text
crates/codegen/src/tests/test_rust_emit_core.rs
```

The same named proof is now consistent across:

```text
Section 9: Codegen contract
Section 18: Test plan
Section 20: Generated artifact bindings
Section 22: Implementation plan requirement
Section 23: Pre-audit closure checklist
```

The exact projection command surfaces are also bound:

```text
--inspect --surface projection
--write --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
--check --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

## Audit Lens Results

| Lens | Result |
| --- | --- |
| Pre-audit closure gate completeness | Passed |
| Measured object clarity | Passed |
| Schema source ownership | Passed |
| Wire/archive validation | Passed |
| Trusted-access safety | Passed |
| Codegen determinism | Passed |
| Generated-code compile surface | Passed |
| Crate boundary isolation | Passed |
| Dependency containment | Passed |
| Correctness oracle | Passed |
| Benchmark isolation | Passed |
| Performance budget | Passed |
| Failure behavior | Passed |
| Code binding completeness | Passed |
| Generated artifact binding completeness | Passed |
| Client/operator interpretation safety | Passed |

## Conditions Before Code

This audit does not authorize implementation.

Implementation may start only after:

1. `docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan.md`
   is written;
2. the plan binds exact edits, generated output, test files, validation
   commands, expected outputs, and rollback boundary;
3. the plan is explicitly approved.

## Classification

```text
PEER_AUDIT_PASSED
```
