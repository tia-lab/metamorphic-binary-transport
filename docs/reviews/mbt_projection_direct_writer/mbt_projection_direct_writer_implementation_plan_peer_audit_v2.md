# MBT Projection Direct Writer Implementation Plan Peer Audit v2

Status: complete
Classification: BLOCKED
Slug: `mbt_projection_direct_writer`
Audited plan:
`docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan.md`
Prior plan audit:
`docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan_peer_audit.md`
Audited spec: `docs/specs/mbt_projection_direct_writer_SPEC.md`
Spec peer audit:
`docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit_v2.md`

This audit does not authorize implementation.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/specs/mbt_projection_direct_writer_SPEC.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit_v2.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan_peer_audit.md`
- `Cargo.toml`
- `crates/benches/Cargo.toml`
- `crates/benches/src/lib.rs`
- `crates/benches/src/tests/mod.rs`
- `crates/codegen/src/rust_emit.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`

## Audit Result

The amended plan closes the blockers from the first implementation-plan audit,
but one compile-surface blocker remains.

## Prior Blocker Closure

### Closed: generated-output rejection check scope

Evidence:

- The amended plan replaces the full-file `rg` acceptance check with
  helper-body scoped tests.
- The plan requires scanning only functions named
  `project_<projection>_archived_direct`, using balanced braces, and failing if
  no direct helper body is found.

### Closed: compile command format and dependency tree checks

Evidence:

- The amended plan uses the spec's time format:

```text
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M'
```

- The amended plan adds:

```text
cargo tree -p metamorphic_binary_transport_schema_bars
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_benches
```

### Closed: benchmark output schema

Evidence:

- The amended plan binds mandatory benchmark row fields for source access time,
  projection time, inspect time, response checksum, semantic checksum, minimal
  projection checksum, old-crate comparison, and current-owned-row comparison.

### Closed: bench test wiring

Evidence:

- The amended plan adds `crates/benches/src/tests/mod.rs` to files to edit.
- The plan requires `mod test_projection_bench_output;`.

### Closed: current-owned-row baseline copy command

Evidence:

- The amended plan binds exact commands:

```text
test -f docs/evidence/mbt_projection_direct_writer/current_owned_row_baseline/projection_run_1.json
test ! -e docs/evidence/mbt_projection_direct_writer/current_owned_row_baseline/projection_run_2.json
cp docs/evidence/mbt_projection_direct_writer/current_owned_row_baseline/projection_run_1.json docs/evidence/mbt_projection_direct_writer/current_owned_row_projection_baseline.json
```

## Blocking Finding

### 1. Bars schema compile baseline is not captured before direct-writer replacement

Evidence:

- Spec Section 14 says generated schema crate build time regression must be
  evaluated against a same-session pre-implementation baseline recorded by the
  implementation plan.
- Spec Section 15 requires the current owned-row generated projection baseline
  before replacing `emit_source_projection_api`.
- The implementation plan Phase 1 records compile baseline for:

```text
metamorphic_binary_transport_core
metamorphic_binary_transport_codegen
metamorphic_binary_transport_schema_test_compatibility
```

- At Phase 1, `metamorphic_binary_transport_schema_bars` does not yet exist.
- The plan creates the Bars schema crate and generates Bars output with the
  current owned-row projection in Phases 2 through 4, but it does not record a
  compile-surface baseline for:

```text
metamorphic_binary_transport_schema_bars
metamorphic_binary_transport_benches
wc -l crates/schemas/bars_core/src/bars_v1.rs
```

before Phase 5 replaces projection codegen.

Risk:

- The result review cannot evaluate whether direct-writer generation regressed
  the new Bars schema crate compile surface against the same-session owned-row
  baseline.
- This weakens the compile-surface budget for the primary Bars benchmark
  schema.

Required amendment:

- Add a mandatory checkpoint after Phase 4 and before Phase 5.
- The checkpoint must append to
  `docs/evidence/mbt_projection_direct_writer/compile_surface.md`.
- The checkpoint must run exactly:

```text
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_bars --all-targets
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_benches --all-targets
wc -l crates/schemas/bars_core/src/bars_v1.rs
```

- The checkpoint must be explicitly labeled as the
  `current-owned-row pre-direct-writer compile baseline`.

## Required Next Step

Amend the implementation plan to add the missing pre-direct-writer
compile-surface checkpoint, then run a third implementation-plan peer audit.

Suggested next command:

```text
Approved: amend docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan.md per implementation plan peer audit v2
```
