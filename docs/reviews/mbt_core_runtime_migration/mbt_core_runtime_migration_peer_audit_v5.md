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

# Peer Audit V5: MBT Core Runtime Migration

Audit classification: `PEER_AUDIT_PASSED`

Spec audited:

```text
docs/specs/mbt_core_runtime_migration_SPEC.md
```

Supersedes:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit_v4.md
```

## Required Reads

Completed reads:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/research_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/review_documentation_protocol.md`
- `docs/specs/mbt_workspace_architecture_SPEC.md`
- `docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_peer_audit_v4.md`
- `docs/specs/mbt_core_runtime_migration_SPEC.md`
- `docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_research_brief.md`
- `docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit_v4.md`
- `docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_implementation_plan.md`
- current workspace source tree under `crates/`

## Blocking Findings

None for the amended spec.

The spec is ready for an amended implementation plan. It does not authorize
code implementation.

## Audit Findings

### F1. Rejected Verbose Filesystem Path Removed From Spec

Severity: resolved.

The previous spec was blocked because it targeted the rejected folder:

```text
crates/metamorphic_binary_transport_core
```

The amended spec now binds filesystem work to:

```text
crates/core
```

Code-read evidence:

- Section 2 now states `DRAFT_AWAITING_PEER_AUDIT_V5`.
- Section 3 migrates runtime core into `crates/core`.
- Section 5 maps experiment runtime modules to
  `crates/core/src/{codec,envelope,error,runtime}.rs`.
- Section 10 allows edits only under `crates/core`, plus explicitly bound root
  manifest and lockfile cases.
- Section 18 creates tests under `crates/core/src/tests`.
- Section 19 binds allowed implementation files under `crates/core`.

Run evidence:

```text
rg -n "crates/metamorphic_binary_transport_core|crates/metamorphic_binary_transport_codegen|crates/metamorphic_binary_transport_benches|crates/metamorphic_binary_transport_json|crates/metamorphic_binary_transport_protobuf|crates/metamorphic_binary_transport_csv|crates/metamorphic_binary_transport_arrow|crates/metamorphic_binary_transport_parquet|BLOCKED_BY_MBT_WORKSPACE_ARCHITECTURE_AMENDMENT" docs/specs/mbt_core_runtime_migration_SPEC.md
```

returned no matches.

### F2. Cargo Package Name Remains Correct

Severity: passed.

The spec keeps Cargo package commands using:

```text
metamorphic_binary_transport_core
```

This is correct because the approved workspace architecture binds:

```text
crates/core -> metamorphic_binary_transport_core
```

The audit distinguishes package names from filesystem paths. The remaining
`metamorphic_binary_transport_core` references in the spec are Cargo package
references for `cargo check -p`, `cargo test -p`, `cargo clippy -p`, and
`cargo tree -p`.

### F3. Dependency Contract Remains Contained

Severity: passed.

The amended spec keeps the exact dependency pin:

```toml
thiserror = "=2.0.17"
```

It also keeps the dependency-tree proof commands against the package name:

```bash
cargo tree -p metamorphic_binary_transport_core
cargo tree -p metamorphic_binary_transport_core | rg -n "thiserror v2\\.0\\.17"
cargo tree -p metamorphic_binary_transport_core | rg -n "thiserror-impl v2\\.0\\.17"
```

No adapter, schema, generated-code, benchmark, Arrow, Parquet, prost, serde,
serde_json, zstd, or rkyv dependency is authorized for core.

### F4. Workspace Boundary Alignment

Severity: passed.

The amended spec aligns with the approved workspace architecture:

- core runtime target path is `crates/core`;
- codegen is forbidden through `crates/codegen/*`;
- projection is forbidden through `crates/projection/*`;
- metamorphose is forbidden through `crates/metamorphose/*`;
- transponding is forbidden through `crates/transponding/*`;
- adapters are forbidden through `crates/adapters/*`;
- benches are forbidden through `crates/benches/*`;
- future schema crates are forbidden through `crates/schemas/*`;
- `proto/*` is forbidden for this runtime migration.

This preserves the crate boundary invariant that MBT core remains small and
does not absorb adapters, schemas, codegen, or benches.

### F5. Existing Implementation Plan Is Still Stale

Severity: not a spec blocker.

The existing implementation plan still targets rejected verbose paths. The
amended spec explicitly requires the implementation plan to be amended before
code may start.

This audit does not approve implementation. The next phase must amend:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_implementation_plan.md
```

to use `crates/core` for filesystem bindings while retaining the package name
`metamorphic_binary_transport_core` for Cargo commands.

### F6. Historical Research Brief Contains Old Path Names

Severity: not a spec blocker.

The research brief was written before the workspace architecture correction and
contains old path names. The amended spec and the passed workspace architecture
spec supersede those path bindings.

The research brief remains valid for the measured object and extraction
rationale. The implementation plan must use the amended spec as the binding
source for paths.

## Audit Lenses

Measured object clarity: passed.

Schema source ownership: passed.

Wire/archive validation: passed.

Trusted-access safety: passed.

Codegen determinism: passed for this slice because no codegen is implemented
or touched.

Generated-code compile surface: passed because generated artifacts remain
forbidden.

Crate boundary isolation: passed.

Dependency containment: passed.

Correctness oracle: passed.

Benchmark isolation: passed because no runtime benchmark claim is made for this
core-only slice.

Performance budget: passed because full runtime performance parity remains
deferred to later generated-schema and benchmark migration specs.

Failure behavior: passed.

Code binding completeness: passed.

Generated artifact binding completeness: passed.

Client/operator interpretation safety: passed because implementation remains
blocked until an amended plan is approved.

## Decision

`PEER_AUDIT_PASSED`

The amended spec is ready for an amended implementation plan. No code changes,
generated artifacts, adapter migration, codegen migration, schema migration, or
benchmark migration are authorized by this audit.
