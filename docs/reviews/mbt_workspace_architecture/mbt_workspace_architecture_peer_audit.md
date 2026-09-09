# MBT Workspace Architecture Peer Audit

Classification: `PEER_AUDIT_PASSED`

Slug: `mbt_workspace_architecture`

Audited spec:

```text
docs/specs/mbt_workspace_architecture_SPEC.md
```

Supporting evidence:

```text
docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_research_brief.md
docs/reviews/mbt_workspace_architecture/mbt_experiment_architecture_extraction_analysis.md
```

## Required Reads

Completed for this audit:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/specs/mbt_workspace_architecture_SPEC.md`
- `docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_research_brief.md`
- `docs/reviews/mbt_workspace_architecture/mbt_experiment_architecture_extraction_analysis.md`

Relevant experiment code evidence was already captured in the extraction
analysis and was rechecked through that artifact for this audit.

## Blocking Findings

None.

The spec is implementation-plan ready for the architecture skeleton phase.

## Audit Lenses

### Measured Object Clarity

Result: passed.

The measured object is workspace separation, not runtime speed or wire-format
behavior. The spec explicitly states that no speed or compile-time claim is
made by this phase.

### Architecture Preservation

Result: passed.

The spec now states that this is an extraction and crate-boundary split, not a
runtime redesign. It preserves:

- `.proto + MBT options`;
- envelope/header ownership;
- checked and trusted access split;
- generated marker/view/accessor shape;
- MBT-to-MBT projection;
- metamorphose boundary conversion;
- transponding for columnar adapters.

This addresses the core risk that a workspace split could silently become a new
transport design.

### Schema Source Ownership

Result: passed.

The spec binds schema source to `.proto + approved MBT options`, allows
external schema ownership, and limits MBT repository ownership to transport
options and generated runtime contracts.

The spec also states that downstream DB/cache/lookup/MLDB annotations are not
MBT core ownership. If those options are hosted here later, a separate
options-ownership spec is required.

### Wire and Archive Validation

Result: passed for this phase.

The spec does not re-specify every byte of the final wire/archive contract, but
that is acceptable because the architecture skeleton does not implement runtime
bytes. It binds ownership and preservation of the experiment architecture, and
requires later runtime specs to define exact envelope layout, payload boundary,
checked validation, trusted access, corrupt behavior, old-version behavior,
response caps, and copy points.

### Trusted-Access Safety

Result: passed.

The spec requires checked and trusted surfaces to remain separate, requires
trusted access to apply only to immutable bytes validated before storage or
handoff, and requires the unsafe trusted API to remain impossible to call
accidentally.

### Codegen Determinism and Generated Surface

Result: passed.

The spec binds codegen as a separate tool/build surface and requires generated
artifacts to be deterministic, reproducible, not hand-edited, and scoped to
requested surfaces.

The spec also corrects the experiment problem where transport runtime,
projections, JSON, protobuf, CSV, Arrow, Arrow IPC, Parquet, transponding, and
prost DTO modules were emitted into one generated module.

### Crate Boundary Isolation

Result: passed.

The spec defines:

- core crate;
- codegen crate;
- benches crate;
- reserved adapter crate names;
- reserved schema crate naming pattern;
- narrow schema crates by default.

It also requires the implementation plan to decide root package strategy and
prove that the root package does not reintroduce monolithic compile coupling.

### Dependency Containment

Result: passed.

The core dependency contract rejects Arrow, Arrow IPC, Parquet, generated prost
DTOs, serde_json, zstd, benchmarks, schema-specific generated modules, and
adapter dependencies.

The implementation plan must include dependency tree audit commands.

### Correctness Oracle

Result: passed for this phase.

The correctness oracle is structural:

- workspace members exist;
- dependency boundaries match the spec;
- root package is not monolithic;
- production crates do not depend on benchmark crates;
- core does not depend on adapters;
- codegen does not compile generated schema crates.

Runtime byte correctness is correctly out of scope for this architecture
phase.

### Benchmark Isolation

Result: passed.

No runtime benchmark is bound by this phase. Build-surface validation is bound
through `cargo check` and `cargo tree` checks. Runtime benchmarks are deferred
to later runtime/codegen/adapter specs.

### Performance Budget

Result: passed for this phase.

The spec does not set numeric runtime or compile thresholds because no crate
code exists yet. This is acceptable for a skeleton architecture phase. Later
runtime/codegen specs must add numeric budgets.

### Failure Contract

Result: passed.

The failure contract is structural and fits the phase. It requires future
implementation plans to check for core importing adapter crates, core importing
benchmark modules, codegen importing generated schema crates, generated files
edited by hand, adapter crates importing unrelated adapters, and schema crates
compiling unrelated schemas.

### Code Bindings

Result: passed.

The spec binds exact paths allowed for the implementation plan. The listed core
files may be created as skeleton ownership files in this phase; runtime logic
still requires later approved specs before implementation.

### Generated Artifact Bindings

Result: passed.

The spec forbids generated MBT schema artifacts, generated protobuf DTOs, and
generated adapter bindings in this phase. If implementation discovers a need
for generated output, the spec requires stop, amend, and re-audit.

## Non-Blocking Cautions

1. The implementation plan must resolve the root manifest strategy before code:
   virtual workspace or thin CLI placeholder.
2. The implementation plan must decide whether adapter placeholder crates are
   created now or deferred.
3. The implementation plan must not create schema crates or generated artifacts
   in this phase. The spec currently forbids generated artifacts, so a schema
   crate would require a later spec unless it is an empty reserved placeholder
   explicitly justified.
4. The options crate/package split is intentionally deferred. It must not be
   implemented under this spec.
5. The benchmark/evidence crate may be created as a workspace member, but it
   must not become a production dependency.

## Required Implementation Plan Constraints

The implementation plan must bind:

- root manifest conversion strategy;
- exact crate directories to create;
- exact file list;
- dependency list per crate;
- dependency tree audit commands;
- `cargo check` commands;
- whether adapter placeholder crates are created or deferred;
- proof that no generated artifact is created;
- proof that core does not depend on adapters, schemas, codegen outputs,
  benchmark code, Arrow, Arrow IPC, Parquet, generated prost DTOs, serde_json,
  or zstd.

## Final Classification

`PEER_AUDIT_PASSED`

The spec is ready for an implementation plan for the workspace architecture
skeleton only. It does not authorize runtime migration, codegen implementation,
schema generation, adapter implementation, or benchmark logic.
