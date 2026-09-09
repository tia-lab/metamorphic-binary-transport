# MBT Workspace Architecture Peer Audit V2

Classification: `PEER_AUDIT_PASSED`

Slug: `mbt_workspace_architecture`

Audited spec:

```text
docs/specs/mbt_workspace_architecture_SPEC.md
```

Supersedes:

```text
docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_peer_audit.md
```

The first audit accepted architecture direction but left skeleton decisions to
the implementation plan. This v2 audit applies the stricter requirement that
the spec itself must resolve exact workspace file structure and skeleton crate
decisions before implementation planning.

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
- `docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_peer_audit.md`

## Blocking Findings

None.

The amended spec is implementation-plan ready for the architecture skeleton
phase only.

## V2 Audit Checks

### Exact Root Strategy

Result: passed.

The spec now requires the root package to become a virtual workspace. It also
requires the implementation plan to delete the current root `src/main.rs` and
leave no root crate, root binary, or root library.

### Exact Initial Crates

Result: passed.

The spec now allows exactly three initial crate directories:

```text
crates/metamorphic_binary_transport_core
crates/metamorphic_binary_transport_codegen
crates/metamorphic_binary_transport_benches
```

No other crate directory may be created in this phase.

### Adapter Placeholder Decision

Result: passed.

The spec reserves adapter crate names but explicitly forbids creating adapter
placeholder crates in this phase.

Forbidden now:

```text
crates/metamorphic_binary_transport_json
crates/metamorphic_binary_transport_protobuf
crates/metamorphic_binary_transport_csv
crates/metamorphic_binary_transport_arrow
crates/metamorphic_binary_transport_parquet
```

### Schema and Options Crate Decision

Result: passed.

The spec forbids schema crates and options crates in this phase. It defers
schema generation and protobuf option ownership to later specs.

### Exact File Structure

Result: passed.

The spec binds the exact initial structure:

```text
Cargo.toml
README.md
AGENTS.md
docs/
crates/
  metamorphic_binary_transport_core/
    Cargo.toml
    src/
      lib.rs
      codec.rs
      envelope.rs
      error.rs
      runtime.rs
  metamorphic_binary_transport_codegen/
    Cargo.toml
    src/
      lib.rs
      main.rs
  metamorphic_binary_transport_benches/
    Cargo.toml
    src/
      lib.rs
```

The code bindings also include root `src/main.rs` only so the implementation
plan can delete it.

### Exact Skeleton Code Scope

Result: passed.

The spec now states that the skeleton is structural only and includes exact
source text for:

- root virtual-workspace `Cargo.toml`;
- all three crate `Cargo.toml` files;
- all created Rust source files;
- the empty codegen placeholder binary.

The exact source text enforces:

- core `lib.rs` declares `codec`, `envelope`, `error`, and `runtime` modules;
- core module files contain only module-level documentation or empty marker
  declarations needed to compile;
- core does not implement envelope, runtime trait, trusted access, rkyv archive
  access, checksums, or transport errors;
- codegen files do not parse descriptors, load protobuf options, generate
  code, run rustfmt, or depend on prost/prost-reflect;
- codegen `main.rs` is a minimal placeholder binary that performs no codegen;
- benches crate contains no benchmark fixtures, generated schemas, adapter
  baselines, or benchmark dependencies.

This is now precise enough for an implementation plan to copy the exact source
text without adding unapproved runtime/codegen behavior.

### Dependency Boundary

Result: passed.

The spec rejects core dependencies on Arrow, Arrow IPC, Parquet, prost,
generated prost DTOs, serde_json, zstd, benchmark code, schema-specific
generated code, and adapters.

For the skeleton phase, the benches crate also must compile without benchmark
dependencies.

### Validation Commands

Result: passed.

The spec requires:

- workspace compile checks;
- per-crate compile checks;
- dependency tree audits;
- checks that root `src/main.rs` is absent;
- checks that adapter placeholders are absent;
- metadata or equivalent checks proving the root manifest is a virtual
  workspace and members are exactly the three initial crates.

### Generated Artifacts

Result: passed.

The spec still forbids generated MBT schema artifacts, generated protobuf DTOs,
and generated adapter bindings in this phase.

### Open Questions

Result: passed.

The spec now says no open questions remain for the architecture skeleton phase.
Deferred items are explicitly listed as later specs, not current implementation
choices.

## Remaining Non-Blocking Requirements For Implementation Plan

The implementation plan must now provide:

- exact copy of the source text already bound in the spec;
- exact deletion of root `src/main.rs`;
- validation commands and expected output shape;
- rollback boundary.

These are implementation-plan execution details, not remaining spec gaps,
because the spec now fully constrains the allowed decisions and source text.

## Final Classification

`PEER_AUDIT_PASSED`

The spec is ready for an implementation plan for the exact workspace skeleton
only. It does not authorize runtime migration, codegen implementation, schema
generation, adapter implementation, benchmark fixtures, benchmark execution,
or generated artifacts.
