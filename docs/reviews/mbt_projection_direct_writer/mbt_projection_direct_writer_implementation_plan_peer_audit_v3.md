# MBT Projection Direct Writer Implementation Plan Peer Audit v3

Status: complete
Classification: PEER_AUDIT_PASSED
Slug: `mbt_projection_direct_writer`
Audited plan:
`docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan.md`
Prior plan audits:

```text
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan_peer_audit.md
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan_peer_audit_v2.md
```

Audited spec: `docs/specs/mbt_projection_direct_writer_SPEC.md`
Spec peer audit:
`docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit_v2.md`

This audit does not itself implement code. It confirms the implementation plan
is ready for explicit implementation approval.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/specs/mbt_projection_direct_writer_SPEC.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit_v2.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan_peer_audit.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan_peer_audit_v2.md`
- `Cargo.toml`
- `crates/benches/Cargo.toml`
- `crates/benches/src/lib.rs`
- `crates/benches/src/tests/mod.rs`
- `crates/codegen/src/rust_emit.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`

## Audit Result

The implementation plan passes peer audit.

The plan now binds:

- exact file edits and created files;
- generated artifacts and owning codegen commands;
- no new external dependencies;
- local dependency additions only;
- current owned-row projection baseline capture before generator replacement;
- current-owned-row pre-direct-writer compile baseline for Bars and benches;
- direct writer codegen replacement boundaries;
- helper-body scoped forbidden-pattern checks;
- benchmark row schema and deterministic output tests;
- dependency tree checks;
- compile-surface evidence commands;
- three-run benchmark procedure;
- result review contents;
- rollback boundary.

## Prior Blocker Closure

### Closed: helper-body scoped generated-output checks

Evidence:

- The plan requires scoped scanners for functions named
  `project_<projection>_archived_direct`.
- The plan forbids full-file generated-source scans for this oracle.

### Closed: compile-surface and dependency commands

Evidence:

- The plan uses the spec timing format:

```text
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M'
```

- The plan includes both generated line-count commands:

```text
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
wc -l crates/schemas/bars_core/src/bars_v1.rs
```

- The plan includes dependency containment checks:

```text
cargo tree -p metamorphic_binary_transport_schema_bars
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_benches
```

### Closed: complete benchmark row schema

Evidence:

- The plan requires source access time, projection time, inspect time, response
  checksum, semantic checksum, minimal projection checksum, old-crate
  comparison, and current-owned-row comparison.
- The plan requires the bench output test to reject missing fields and
  nondeterministic ordering.

### Closed: bench test wiring

Evidence:

- The plan adds `crates/benches/src/tests/mod.rs` to the edit list.
- The plan requires:

```rust
mod test_projection_bench_output;
```

### Closed: exact current-owned-row baseline copy

Evidence:

- The plan binds exact existence checks and copy command for
  `current_owned_row_projection_baseline.json`.

### Closed: Bars pre-direct-writer compile baseline

Evidence:

- The plan adds `Phase 4.5: Record Current-Owned-Row Pre-Direct-Writer Compile Baseline`.
- The phase runs:

```text
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_bars --all-targets
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_benches --all-targets
wc -l crates/schemas/bars_core/src/bars_v1.rs
```

- The checkpoint is explicitly labeled
  `current-owned-row pre-direct-writer compile baseline`.

## No Blocking Findings

No implementation-plan blockers remain.

## Conditions For Implementation

Implementation must still obey the plan exactly:

- no source-code work outside the plan bindings;
- no hand-edited generated schema source;
- no production owned-row projection fallback;
- no new external dependencies;
- no full-file generated-source rejection oracle for projection copy patterns;
- no speed or compile-time claim before recorded evidence.

If implementation requires a file, command, dependency, benchmark shape, or
fallback path not named in the plan, implementation must stop and the plan must
be amended before continuing.

## Next Step

The next valid step is explicit implementation approval:

```text
Approved: implement docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan.md
```
