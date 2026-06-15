# MBT Projection Direct Writer Implementation Plan Peer Audit

Status: complete
Classification: BLOCKED
Slug: `mbt_projection_direct_writer`
Audited plan:
`docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan.md`
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
- `Cargo.toml`
- `crates/benches/Cargo.toml`
- `crates/benches/src/lib.rs`
- `crates/benches/src/tests/mod.rs`
- `crates/codegen/src/rust_emit.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`

## Audit Result

The implementation plan is blocked. It is close in scope, but it does not yet
carry all exact commands, artifacts, and validation semantics required by the
audited spec.

## Blocking Findings

### 1. Generated-output rejection check is incorrectly scoped

Evidence:

- Spec Section 16 says the generated-output oracle must reject owned-row copy
  patterns only inside direct projection helper bodies so normal owned encode
  and test fixtures do not create false failures.
- The implementation plan `Expected Outputs` section runs:

```text
rg 'Vec::with_capacity\(archived\.|\.to_string\(\)|\.to_vec\(\)|\.collect\(\)|rows\.push\(|encode_owned\(rows' crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs crates/schemas/bars_core/src/bars_v1.rs
```

- Current generated compatibility source already contains legitimate
  non-projection `encode_owned` and `rows.to_vec()` paths, and the plan would
  reject the whole file instead of direct helper bodies.

Risk:

- Implementation could either fail validation for valid generated owned encode
  code, or remove valid schema APIs to satisfy an overbroad check.

Required amendment:

- Replace the full-file `rg` acceptance check with a helper-body scoped check.
- Bind the exact helper-body extraction strategy, such as generated helper
  region markers or a deterministic test helper that scans only
  `project_<projection>_archived_direct` bodies.

### 2. Compile-surface and dependency validation commands do not match the spec

Evidence:

- Spec Section 11 requires dependency validation commands:

```text
cargo tree -p metamorphic_binary_transport_schema_bars
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_benches
```

- The implementation plan does not include those commands.
- Spec Section 14 requires timing format:

```text
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M'
```

- The implementation plan uses:

```text
/usr/bin/time -f 'elapsed=%E user=%U sys=%S maxrss_kb=%M'
```

- Spec Section 14 requires line counts for both generated schema outputs:

```text
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
wc -l crates/schemas/bars_core/src/bars_v1.rs
```

- The plan records only the compatibility line count during the baseline phase
  and does not bind the Bars line count in final compile-surface evidence.

Risk:

- Compile-surface and dependency evidence would not be reproducible against the
  approved spec.

Required amendment:

- Use the exact timing commands and output labels from the spec.
- Add both generated line-count commands to final compile-surface evidence.
- Add the three `cargo tree` dependency validation commands and require the
  result review to show no adapter or codegen dependency in schema/bench trees.

### 3. Benchmark output schema is missing required fields

Evidence:

- Spec Section 17 requires every benchmark result row to include:
  source access milliseconds, projection milliseconds, projected inspect
  milliseconds, response checksum, semantic checksum when inspected, minimal
  projection checksum when inspected, old-crate comparison row for Bars lanes,
  and current-owned-row comparison row for same-repository lanes.
- The implementation plan Phase 3 requires only bytes, rows, elapsed nanos,
  rows/sec, MB/sec, checksum, lane name, schema name, and row count.

Risk:

- The benchmark would not prove the intended measured object. It would not
  isolate source access, projection, and inspect time, and it would not carry
  both comparison rows required by the spec.

Required amendment:

- Bind the exact benchmark row schema from spec Section 17.
- Require tests for all mandatory fields and reject incomplete benchmark JSON.

### 4. Bench test file is created but not wired into the current test module

Evidence:

- Current `crates/benches/src/lib.rs` uses `#[cfg(test)] mod tests;`.
- Current `crates/benches/src/tests/mod.rs` contains only `skeleton_compiles`.
- The plan creates `crates/benches/src/tests/test_projection_bench_output.rs`
  but does not list or require editing `crates/benches/src/tests/mod.rs`.

Risk:

- `cargo test -p metamorphic_binary_transport_benches --all-targets` may not
  run the new bench-output tests, so the deterministic benchmark output schema
  would be unproved.

Required amendment:

- Either add `crates/benches/src/tests/mod.rs` to the edit list and wire the new
  module there, or move the test to an integration-test path and bind that path
  in the spec and plan.

### 5. Current-owned-row baseline copy step is not exact

Evidence:

- Spec Section 22 requires exact commands and exact benchmark order.
- The plan says to copy the generated run JSON to
  `current_owned_row_projection_baseline.json`, but does not bind the exact
  source path or command.

Risk:

- The baseline artifact can be produced inconsistently, especially because the
  benchmark binary writes the next available `projection_run_N.json`.

Required amendment:

- Bind the exact copy command, expected source run file, and failure behavior if
  the source run file is missing or if more than one baseline run file exists.

## Non-blocking Notes

### Plan correctly preserves the no-code gate

Evidence:

- The plan states `DRAFT_AWAITING_APPROVAL`.
- It states that implementation may start only after explicit approval.

### Plan correctly separates direct writer from production fallback

Evidence:

- The plan requires removing owned-row projection fallback from production
  generated projection paths.

## Required Next Step

Amend the implementation plan to close the blocking findings above, then run a
second implementation-plan peer audit before code.

Suggested next command:

```text
Approved: amend docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan.md per implementation plan peer audit
```
