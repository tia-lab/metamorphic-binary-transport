# MBT Projection Metamorphose Adapter Peer Audit

## Classification

`BLOCKED`

## Scope

Audited spec:

```text
docs/specs/mbt_projection_metamorphose_adapter_SPEC.md
```

Research brief:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_research_brief.md
```

No code was changed by this audit.

## Required Reads

Completed:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- target research brief
- target spec
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/emit.rs`
- `crates/codegen/src/config.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`
- relevant schema generated/proto paths under `crates/schemas/bars_core`
  and `crates/schemas/test_compatibility_core`

## Findings

### 1. Code bindings conflict with required schema test edits

Severity: blocking

Evidence:

- Spec section 19 lists only these allowed implementation code files:

```text
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
```

- The same section says:

```text
No other code file may be edited unless the implementation plan amends this
binding before approval.
```

- Spec section 20 requires schema test files:

```text
crates/schemas/bars_core/tests/test_bars_metamorphose.rs
crates/schemas/test_compatibility_core/tests/test_projection_metamorphose.rs
```

Problem:

The runtime test files are code files. The spec simultaneously requires them
and forbids editing any file outside section 19 unless the implementation plan
amends the binding. That violates the pre-audit closure gate because exact code
and test paths must be bound before audit, not repaired later by the
implementation plan.

Required amendment:

Section 19 must explicitly separate:

- implementation source files;
- test source files;
- generated source files;
- forbidden production/runtime crates.

The schema test files required by section 20 must be listed as allowed test
code files before the spec can pass.

### 2. Derived UTC projection JSON behavior is specified but not proved

Severity: blocking

Evidence:

- Spec section 9 requires:

```text
Projection JSON output includes only fields present in the projected schema,
plus derived UTC outputs whose source physical field is still present in the
projected schema.
```

- `crates/codegen/src/rust_emit.rs` currently clears projected
  `derived_utc_fields` and `json_csv_output_fields` inside
  `projection_schema_model`.
- The generic codegen oracle only checks that `SmallProjection` receives JSON
  APIs and trait impls.
- The Bars runtime oracle checks fields such as `pair`, `close_ms`, and `c`,
  and absence of `metadata.`, but it does not require retained top-level
  derived UTC fields such as `open_utc` or `close_utc`.
- The test-compatibility runtime oracle does not bind a derived UTC fixture.

Problem:

The spec introduces a concrete derived UTC rule for projection JSON, but the
correctness oracle can pass even if projection JSON emits no derived UTC output.
This is exactly the current known risk from the research brief: projected
models currently clear JSON/CSV output fields.

Required amendment:

The correctness oracle must explicitly prove derived UTC behavior. Acceptable
proof shape:

- generic codegen oracle asserts the non-serving projection fixture emits a
  retained derived UTC JSON constant/write path when the source physical field
  remains projected; and
- Bars runtime oracle asserts `open_utc` and/or `close_utc` are present for
  `no_metadata`, while metadata-derived UTC fields such as
  `metadata.ingested_at_utc` remain absent.

## Non-Blocking Observations

- The scope is correctly limited to JSON projection metamorphose for the
  serving blocker.
- The spec correctly forbids Bars-serving-specific codegen branches.
- The CLI/write/check command surfaces are exact.
- The compile-surface checks are sufficient for this first correctness scope.
- Keeping protobuf, CSV, and columnar projection adapters out of scope is
  appropriate for this pass.

## Required Amendment Summary

Before a second peer audit:

1. Amend code bindings so required schema runtime test files are explicitly
   allowed test edits, without weakening the production crate boundary.
2. Amend the correctness oracle to prove retained and excluded derived UTC
   behavior for projection JSON.

