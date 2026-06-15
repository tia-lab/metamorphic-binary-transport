# MBT Projection Direct Writer Implementation Plan

Status: DRAFT_AWAITING_APPROVAL

Spec: `docs/specs/mbt_projection_direct_writer_SPEC.md`
Peer audit: `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit_v2.md`

This plan does not authorize implementation. Code changes may start only after
this plan is approved.

## Goal

Replace generated MBT-to-MBT projection with a schema-specific direct archive
writer.

The implementation must remove the current production projection shape:

1. allocate `Vec<ProjectedRow>`,
2. copy archived source fields into owned projected rows,
3. call projected `encode_owned(rows)`.

The replacement must write the projected archive payload directly from the
source archived rows, then prepend the standard MBT transport header. The only
allowed output allocation is the final response buffer and serializer scratch
required to write that response.

## Required Reads Before Code

Before editing, re-read:

1. `AGENTS.md`
2. `docs/invariants/core_invariants.md`
3. `docs/protocols/lifecycle_protocol.md`
4. `docs/protocols/implementation_protocol.md`
5. `docs/protocols/code_style_protocol.md`
6. `docs/protocols/codegen_protocol.md`
7. `docs/specs/mbt_projection_direct_writer_SPEC.md`
8. `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit_v2.md`
9. Current codegen projection implementation in `crates/codegen/src/rust_emit.rs`
10. Old benchmark evidence in `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md`

If any read changes the implementation assumptions, stop and amend the spec or
plan before code.

## Files To Edit

Edit only these existing files:

1. `Cargo.toml`
2. `crates/codegen/src/rust_emit.rs`
3. `crates/codegen/src/tests/mod.rs`
4. `crates/codegen/src/tests/test_rust_emit_core.rs`
5. `crates/codegen/src/tests/test_cli.rs`
6. `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`
7. `crates/schemas/test_compatibility_core/tests/test_projection.rs`
8. `crates/benches/Cargo.toml`
9. `crates/benches/src/lib.rs`
10. `crates/benches/src/tests/mod.rs`

Do not edit `crates/core` for this task.

Do not edit `crates/codegen/src/model.rs` or
`crates/codegen/src/descriptor.rs`; the existing projection model is sufficient.
If implementation proves otherwise, stop and amend this plan.

## Files To Create

Create these source and test files:

1. `crates/codegen/src/tests/test_rust_emit_projection_direct_writer.rs`
2. `crates/schemas/bars_core/Cargo.toml`
3. `crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto`
4. `crates/schemas/bars_core/src/lib.rs`
5. `crates/schemas/bars_core/src/bars_v1.rs`
6. `crates/schemas/bars_core/tests/test_bars_shape.rs`
7. `crates/schemas/bars_core/tests/test_bars_projection.rs`
8. `crates/schemas/bars_core/tests/expected_bars_projection_inspect.txt`
9. `crates/benches/src/projection.rs`
10. `crates/benches/src/bin/mbt_projection_bench.rs`
11. `crates/benches/src/tests/test_projection_bench_output.rs`

Create these evidence and review files during validation:

1. `docs/evidence/mbt_projection_direct_writer/compile_surface.md`
2. `docs/evidence/mbt_projection_direct_writer/test_compatibility_projection_inspect.txt`
3. `docs/evidence/mbt_projection_direct_writer/bars_projection_inspect.txt`
4. `docs/evidence/mbt_projection_direct_writer/current_owned_row_projection_baseline.json`
5. `docs/evidence/mbt_projection_direct_writer/projection_run_1.json`
6. `docs/evidence/mbt_projection_direct_writer/projection_run_2.json`
7. `docs/evidence/mbt_projection_direct_writer/projection_run_3.json`
8. `docs/evidence/mbt_projection_direct_writer/projection_summary.md`
9. `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_result_review.md`

## Dependency Changes

No new external dependencies are approved.

Allowed local dependency changes:

1. add `crates/schemas/bars_core` as a workspace member;
2. add `metamorphic_binary_transport_schema_bars` as a local dependency of
   `crates/benches`;
3. add `metamorphic_binary_transport_schema_test_compatibility` as a local
   dependency of `crates/benches`;
4. keep `rkyv = "=0.8.16"` in schema crates.

Benchmark JSON must be written deterministically with local code, not by adding
`serde` or `serde_json`.

## Implementation Sequence

### Phase 1: Record Current State

Run and record:

```bash
git status --short
```

Record the output in
`docs/evidence/mbt_projection_direct_writer/compile_surface.md`.

Run compile-surface baseline commands:

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_core
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_codegen --all-targets
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_test_compatibility --all-targets
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Append command, exit status, and output to
`docs/evidence/mbt_projection_direct_writer/compile_surface.md`.

### Phase 2: Add Bars Schema Crate

Add workspace member:

```toml
"crates/schemas/bars_core"
```

Create `crates/schemas/bars_core/Cargo.toml` with:

1. package name `metamorphic_binary_transport_schema_bars`;
2. dependency on `metamorphic_binary_transport_core`;
3. `rkyv = "=0.8.16"`.

Create `crates/schemas/bars_core/src/lib.rs`:

```rust
pub mod bars_v1;
```

Create `crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto`
from the old Bars proto at:

```text
/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/binary_transport/v1/bars.proto
```

The copied proto must:

1. preserve MBT transport, dictionary, bitmask, key, presence, ignored, and
   projection annotations;
2. remove options that do not exist in this repository's
   `proto/mathilde/options.proto`;
3. not introduce cache, database, or lookup annotations.

### Phase 3: Add Projection Bench Harness Before Direct Writer Replacement

Add the projection bench crate support while the generator still emits the
current owned-row projection path.

Create `crates/benches/src/projection.rs` with deterministic fixtures for:

1. Bars rows;
2. test compatibility rows;
3. row counts `1, 100, 500, 1000, 10000, 100000`.

Create `crates/benches/src/bin/mbt_projection_bench.rs`.

The benchmark binary must:

1. accept `--report-dir <path>`;
2. create the report directory if missing;
3. write the next available `projection_run_N.json`;
4. refuse to overwrite existing run files;
5. write deterministic JSON without external JSON dependencies;
6. write benchmark result rows using the exact semantic field list below;
7. include old-crate baseline values parsed from
   `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md`;
8. fail if old-crate baseline rows required by the spec are missing.

Every benchmark result row must include exactly these semantic fields:

1. `label`;
2. `schema`;
3. `row_count`;
4. `output_bytes`;
5. `total_milliseconds`;
6. `rows_per_second`;
7. `mb_per_second`;
8. `source_access_milliseconds`;
9. `projection_milliseconds`;
10. `projected_inspect_milliseconds`;
11. `response_checksum`;
12. `semantic_checksum`, nullable when not inspected;
13. `minimal_projection_checksum`, nullable when not inspected;
14. `old_crate_comparison`, nullable for non-Bars lanes;
15. `current_owned_row_comparison`, required for direct-writer result rows and
    nullable only while producing the current owned-row baseline artifact.

The benchmark output test must reject a result row that omits any field above,
uses nondeterministic field ordering, or writes comparison rows under
implementation-specific names.

The benchmark rows are:

Bars:

1. `mathilde_binary_project_no_metadata_public`
2. `mathilde_binary_project_no_metadata_archived`
3. `mathilde_binary_project_no_metadata_inspect`
4. `mathilde_binary_project_ohlcv_only_public`
5. `mathilde_binary_project_ohlcv_only_archived`
6. `mathilde_binary_project_ohlcv_only_inspect`

Compatibility:

1. `mbt_project_no_optional_public`
2. `mbt_project_no_optional_archived`
3. `mbt_project_no_optional_inspect`
4. `mbt_project_numeric_only_public`
5. `mbt_project_numeric_only_archived`
6. `mbt_project_numeric_only_inspect`

Add tests in `crates/benches/src/tests/test_projection_bench_output.rs` for:

1. stable JSON field ordering;
2. no overwrite behavior;
3. required lane names;
4. required row counts;
5. every mandatory benchmark row field.

Update `crates/benches/src/tests/mod.rs` to include:

```rust
mod test_projection_bench_output;
```

### Phase 4: Generate Current Owned-Row Baseline

Generate compatibility schema with the current generator:

```bash
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Generate Bars schema with the current generator:

```bash
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs
```

Run the benchmark once with the current owned-row projection path:

```bash
rm -rf docs/evidence/mbt_projection_direct_writer/current_owned_row_baseline
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer/current_owned_row_baseline
```

Verify the baseline run shape and copy the generated run JSON with exact
commands:

```bash
test -f docs/evidence/mbt_projection_direct_writer/current_owned_row_baseline/projection_run_1.json
test ! -e docs/evidence/mbt_projection_direct_writer/current_owned_row_baseline/projection_run_2.json
cp docs/evidence/mbt_projection_direct_writer/current_owned_row_baseline/projection_run_1.json docs/evidence/mbt_projection_direct_writer/current_owned_row_projection_baseline.json
```

This file proves the current in-repo projection baseline before replacing the
generator.

### Phase 4.5: Record Current-Owned-Row Pre-Direct-Writer Compile Baseline

Before editing `crates/codegen/src/rust_emit.rs`, append a checkpoint labeled
`current-owned-row pre-direct-writer compile baseline` to:

```text
docs/evidence/mbt_projection_direct_writer/compile_surface.md
```

Run exactly:

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_bars --all-targets
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_benches --all-targets
wc -l crates/schemas/bars_core/src/bars_v1.rs
```

This checkpoint is the same-session compile-surface baseline for the generated
Bars schema and benchmark harness while projection still uses the current
owned-row path. The result review must compare final direct-writer compile
surface against this checkpoint.

### Phase 5: Replace Projection Codegen With Direct Writer

Edit `crates/codegen/src/rust_emit.rs`.

The generated projection API must:

1. keep public checked APIs unchanged;
2. keep trusted archived APIs unchanged;
3. stop building `Vec<ProjectedRow>`;
4. stop calling projected `encode_owned(rows)` from source projection;
5. emit schema-specific borrowed projection payload wrappers;
6. emit schema-specific borrowed projection row wrappers;
7. serialize the borrowed projected payload directly with rkyv;
8. write the standard MBT transport header after payload serialization;
9. enforce `max_response_bytes` before returning the output;
10. preserve response-too-large, count-overflow, invalid-schema, and corrupt
    payload failures;
11. preserve semantic checksum equality with the owned-row baseline;
12. preserve byte-level validity under checked access of the projected schema.

Remove or stop emitting any helper that exists only for owned-row projection
copies. In particular, generated projection code must not contain:

1. `Vec::with_capacity(archived.`
2. `.to_string()`
3. `.to_vec()`
4. `.collect()`
5. `rows.push(`
6. `encode_owned(rows`

If a helper becomes unused in `rust_emit.rs`, remove it in the same change.

### Phase 6: Add Codegen Tests

Create `crates/codegen/src/tests/test_rust_emit_projection_direct_writer.rs`.

Tests must assert generated source contains:

1. direct writer helper for each source-to-projection path;
2. borrowed payload wrapper;
3. borrowed row wrapper;
4. standard header creation;
5. max-response-size guard;
6. checked projected access path.

Tests must assert generated source does not contain:

1. `Vec::with_capacity(archived.`
2. `.to_string()`
3. `.to_vec()`
4. `.collect()`
5. `rows.push(`
6. `encode_owned(rows`

These negative checks must be scoped to generated direct projection helper
bodies. The test helper must:

1. find only functions named `project_<projection>_archived_direct`;
2. locate each function body by balanced braces;
3. scan only those body slices for rejected patterns;
4. fail if no direct helper body is found.

Full-file scans are forbidden for this oracle because owned encode and test
fixtures may legitimately contain the rejected patterns outside direct
projection helpers.

Update `crates/codegen/src/tests/mod.rs` to include the new test module.

Update existing CLI/core emit tests only where direct writer output changes
expected source snippets.

### Phase 7: Regenerate Schema Artifacts

Run compatibility write/check/inspect:

```bash
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --inspect --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection > docs/evidence/mbt_projection_direct_writer/test_compatibility_projection_inspect.txt
```

Run Bars write/check/inspect:

```bash
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --inspect --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection > docs/evidence/mbt_projection_direct_writer/bars_projection_inspect.txt
```

Copy the reviewed Bars inspect output into:

```text
crates/schemas/bars_core/tests/expected_bars_projection_inspect.txt
```

### Phase 8: Schema Correctness Tests

Update `crates/schemas/test_compatibility_core/tests/test_projection.rs` to
assert:

1. projected checked access succeeds;
2. projected trusted access succeeds after checked validation;
3. semantic checksum matches current owned-row baseline;
4. projected payload size is within the spec cap;
5. generated direct projection helper bodies have no owned-row projection copy
   patterns.

Create `crates/schemas/bars_core/tests/test_bars_shape.rs` to assert:

1. dictionaries and projection groups are generated;
2. ignored UTC string fields are not emitted in MBT payload;
3. Bars key fields are preserved.

Create `crates/schemas/bars_core/tests/test_bars_projection.rs` to assert:

1. no-metadata projection checked and trusted access;
2. ohlcv-only projection checked and trusted access;
3. semantic checksum equality with owned-row baseline evidence;
4. projected row count equality;
5. generated direct projection helper bodies have no owned-row projection copy
   patterns.

## Validation Commands

Run correctness checks:

```bash
cargo test -p metamorphic_binary_transport_codegen --all-targets
cargo test -p metamorphic_binary_transport_schema_test_compatibility --all-targets
cargo test -p metamorphic_binary_transport_schema_bars --all-targets
cargo test -p metamorphic_binary_transport_benches --all-targets
```

Run compile checks:

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_core
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_codegen --all-targets
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_test_compatibility --all-targets
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_bars --all-targets
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_benches --all-targets
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
wc -l crates/schemas/bars_core/src/bars_v1.rs
```

Append compile command output to:

```text
docs/evidence/mbt_projection_direct_writer/compile_surface.md
```

Run dependency containment checks:

```bash
cargo tree -p metamorphic_binary_transport_schema_bars
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_benches
```

Append dependency command output to:

```text
docs/evidence/mbt_projection_direct_writer/compile_surface.md
```

The result review must state whether these dependency trees contain any
codegen, JSON, protobuf, CSV, Arrow, Arrow IPC, Parquet, or metamorphose adapter
dependency. Any such dependency in generated schema crates or projection
benchmarks fails the dependency gate.

Run final projection benchmarks three sequential times:

```bash
rm -f docs/evidence/mbt_projection_direct_writer/projection_run_1.json docs/evidence/mbt_projection_direct_writer/projection_run_2.json docs/evidence/mbt_projection_direct_writer/projection_run_3.json docs/evidence/mbt_projection_direct_writer/projection_summary.md
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer
```

The benchmark binary must write `projection_summary.md` after the third run.

## Expected Outputs

Correctness checks must pass with no warnings introduced by this task.

Generated direct projection helper bodies must not contain owned-row projection
copy patterns. This is validated by the scoped helper-body tests in:

1. `crates/codegen/src/tests/test_rust_emit_projection_direct_writer.rs`;
2. `crates/schemas/test_compatibility_core/tests/test_projection.rs`;
3. `crates/schemas/bars_core/tests/test_bars_projection.rs`.

Expected result: each scoped helper-body scanner finds at least one direct
projection helper body and finds no rejected pattern inside those bodies.

Benchmark acceptance:

1. direct writer projected bytes must pass checked access;
2. direct writer semantic checksums must match owned-row baseline;
3. direct writer public and archived projection lanes must be within 3.5% of
   old-crate baseline rows/sec or faster;
4. each lane is unstable if max/min rows/sec differs by more than 7% across the
   three runs;
5. instability must be reported, not averaged away.

## Result Review

Create:

```text
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_result_review.md
```

The review must include:

1. exact commands run;
2. exit status for each command;
3. generated source pattern check result;
4. compile-surface result;
5. benchmark table for current owned-row baseline, direct writer, and old crate;
6. pass/fail against the 3.5% parity rule;
7. unstable lanes, if any;
8. dependency tree result for schema and benchmark crates;
9. generated line counts for compatibility and Bars schema outputs;
10. remaining hypotheses.

## Rollback Boundary

If implementation fails and cannot be corrected inside this plan:

1. stop;
2. do not use destructive git commands;
3. remove newly created files listed in this plan;
4. restore edited files from the pre-implementation diff;
5. leave evidence files that explain the failure;
6. write the failure in the result review.

## Known Risks

1. Direct rkyv serialization of borrowed projected wrappers may require
   generated wrapper implementations that are more explicit than the current
   owned-row derive path.
2. Bytes, raw strings, and arrays must not be copied into projected row storage.
   They may only be serialized into the final projected output.
3. Old Bars proto contains annotations not present in this repository's
   `proto/mathilde/options.proto`; the Bars benchmark proto must strip those
   without changing MBT semantics.
4. Benchmark instability must be treated as evidence, not hidden by averages.

## Approval Gate

Implementation may start only after explicit approval of this plan.
