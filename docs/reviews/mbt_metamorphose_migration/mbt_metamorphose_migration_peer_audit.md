# Peer Audit: MBT Metamorphose Migration

Status: `PEER_AUDIT_PASSED`
Date: 2026-06-15
Slug: `mbt_metamorphose_migration`

## Scope

Audited spec:

```text
docs/specs/mbt_metamorphose_migration_SPEC.md
```

Audited research brief:

```text
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_research_brief.md
```

This is a spec audit only. No implementation, benchmark, or performance claim is
accepted by this audit.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_research_brief.md`
- `docs/specs/mbt_metamorphose_migration_SPEC.md`

Additional source-oracle reads were inherited from the research brief and are
accepted as code-read evidence for the old MBT behavior:

- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/metamorphose/mod.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md`

## Classification

`PEER_AUDIT_PASSED`

The spec is implementation-plan ready. It does not authorize code changes by
itself.

## Audit Findings

### 1. Public API and Hidden Transponding

Result: passed.

The spec preserves the old public shape under `metamorphose::` and explicitly
forbids public transponding. Columnar formats are bound to call one hidden
generated transponding helper automatically. This matches the old architecture
described in the research brief.

### 2. Trusted Access Safety

Result: passed.

The spec now binds explicit unsafe trusted public helpers for JSON, protobuf,
CSV, Arrow, Arrow IPC, and Parquet. It states the safety contract and requires
generated trusted paths to call existing generated trusted schema accessors
instead of revalidating through checked public methods.

### 3. Hot Path Allocation and Copy Discipline

Result: passed.

The spec forbids row DTO materialization, serde DTO paths, prost DTO
materialization, public transponding dispatch, generic reflection in hot paths,
and per-row field-name formatting or escaping. It also requires static field
fragments and declared output pre-sizing for row formats.

The spec correctly treats columnar output buffers as declared allocation points
and requires exactly one hidden transponding pass.

### 4. Wide Schema Compile and Function Shape

Result: passed.

The spec binds automatic private 32-field chunking for generated row-format
adapter writers and transponding writers. The chunking is internal to generated
modules and cannot become a user API. Source-level guard tests are required to
prove the generated hot-path helpers do not exceed the threshold.

This is sufficient for implementation planning. Compile-time improvement is not
claimed until the compile evidence commands run.

### 5. Codegen Ownership

Result: passed.

Generated artifacts have one owner: `mbt_codegen --surface metamorphose
--adapter <adapter>`. The spec lists exact generated files for Bars and test
compatibility schemas and forbids hand editing.

The spec does not authorize edits to `crates/core/src/*`,
`crates/projection/src/*`, or `proto/mathilde/options.proto`, which keeps this
migration scoped to metamorphose, transponding, adapters, schema feature gates,
codegen surface expansion, tests, and benches.

### 6. Crate Boundary and Compile Surface

Result: passed.

The spec keeps `core` and `projection` adapter-free. Schema crates remain
adapter-free by default and may pull one selected adapter family only through an
explicit feature. Dependency-tree commands are required for each feature.

### 7. Dependency Containment

Result: passed with implementation-plan gate.

The spec does not pin exact adapter dependency versions. That is acceptable for
this audit because it explicitly states that no dependency is authorized until
the implementation plan binds the exact version and owner crate. If that plan
does not bind versions, the plan must fail audit.

### 8. Correctness Oracle

Result: passed.

The spec defines correctness oracles for MBT, JSON, protobuf, CSV, Arrow, Arrow
IPC, Parquet, and projections. Bars is bound to old MBT output/semantic parity,
and test compatibility is bound to non-Bars field coverage.

### 9. Benchmark Methodology

Result: passed.

The benchmark command, output directory, old baseline source, required old
labels, three-run requirement, spread rule, rows/sec, MB/sec, bytes, checksum,
and feature-set recording are specified. The spec correctly separates checked
and trusted paths.

### 10. Failure Contract

Result: passed.

The spec binds wrong schema, corrupt header/archive, old version,
response-too-large, unsupported field kind, invalid UTF-8, finite numeric
policy, and missing selected feature behavior. Unsupported formats must fail at
codegen or compile time, not through runtime fallback.

## Residual Risks

- Performance parity is unproved until the benchmark suite runs after
  implementation.
- Compile-surface budget is unproved until the required `cargo check` and
  `cargo tree` commands run after implementation.
- Arrow, Arrow IPC, and Parquet determinism depends on writer settings that
  must be bound in the implementation plan.

These risks do not block the spec because the spec defines the evidence that
must be produced before any readiness or performance claim.

## Required Next Step

Write the implementation plan:

```text
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md
```

The implementation plan must be separately audited or explicitly approved
before any code change.
