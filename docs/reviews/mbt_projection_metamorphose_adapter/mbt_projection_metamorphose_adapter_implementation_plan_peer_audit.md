# MBT Projection Metamorphose Adapter Implementation Plan Peer Audit

## Classification

`BLOCKED`

## Scope

Audited implementation plan:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_implementation_plan.md
```

Approved spec:

```text
docs/specs/mbt_projection_metamorphose_adapter_SPEC.md
```

Passing spec peer audit:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit_v3.md
```

No code was changed by this audit.

## Required Reads

Completed:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`
- audited implementation plan
- approved spec
- passing spec peer audit v3
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/model.rs`
- `crates/codegen/src/descriptor.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`
- `crates/schemas/bars_core/tests/test_bars_metamorphose.rs`
- `crates/schemas/test_compatibility_core/tests/test_projection.rs`

## Findings

### BLOCKER 1: projection JSON optional-condition helpers are not bound to projection scope

The plan scopes JSON field constants and writer helper function names, but it
does not bind the optional-condition helpers that currently generate presence
checks.

Plan evidence:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_implementation_plan.md
  Required helper changes:
  emit_json_field_constants(out, scope)
  emit_json_inherent_api(out, scope)
  emit_json_writer_helpers(out, scope)
  emit_json_output_write(out, scope, output, indent)
  emit_json_value_write(out, scope, field, indent)
  emit_json_bitmask_helpers(out, scope)
  emit_json_field_prefix_helper(out, scope)
```

Code-read evidence:

```text
crates/codegen/src/rust_emit.rs
  archived_optional_condition(...) calls
  archived_presence_has_expr(&EmitScope::source(model), ...)

crates/codegen/src/rust_emit.rs
  derived_utc_optional_condition(...) calls
  archived_presence_has_expr(&EmitScope::source(model), ...)

crates/codegen/src/rust_emit.rs
  json_output_optional_condition(...) calls those helpers without a scope.
```

Why this blocks implementation:

- projection generated presence constants are emitted through
  `EmitScope::projection`;
- optional projected physical fields and optional derived UTC fields must use
  projection-scoped presence constants;
- the current implementation plan does not require
  `json_output_optional_condition`, `archived_optional_condition`, or
  `derived_utc_optional_condition` to accept the active `EmitScope`;
- the current planned tests do not prove a retained optional projected field,
  so the bug could pass the approved test plan.

Required amendment:

1. Add the optional-condition helpers to the scoped JSON refactor:

```text
json_output_optional_condition(scope, output)
archived_optional_condition(scope, field)
derived_utc_optional_condition(scope, field)
```

2. Require JSON projection writer generation to use the same active
   `EmitScope` for:

```text
optional physical field conditions
optional derived UTC field conditions
presence constant names
```

3. Add a codegen oracle in
   `crates/codegen/src/tests/test_rust_emit_metamorphose.rs` proving a
   projection-retained optional field emits projection-scoped presence checks.
   This can be a local inline fixture in that test file; it must not add a new
   shared fixture to `crates/codegen/src/tests/mod.rs`.

4. Keep the existing `valid_alias_and_projection_ignored_proto()` oracle,
   because the approved spec requires that fixture for the first generic
   projection JSON proof.

### BLOCKER 2: required implementation refresh omits `spec_protocol.md`

The implementation plan's "Required Reads Before Code" omits:

```text
docs/protocols/spec_protocol.md
```

Code-read evidence:

```text
docs/protocols/implementation_protocol.md
  Locked Refresh Rule requires rereading docs/protocols/spec_protocol.md
  before implementation.
```

Why this blocks implementation:

- generated-code work is governed by the spec as source of truth;
- the implementation plan must not weaken the repository's locked refresh
  rule before code starts.

Required amendment:

Add `docs/protocols/spec_protocol.md` to the implementation plan's required
reads before code.

## Non-Blocking Checks

The following parts are acceptable after the blockers are fixed:

- File bindings are bounded to `crates/codegen/src/rust_emit.rs`, one codegen
  test file, schema runtime tests, and generated JSON artifacts.
- Dependency policy states no dependency changes.
- Generated artifacts are bound to explicit `--write` and `--check` commands.
- Serving, core, runtime, adapters, compression, `Cargo.toml`, `Cargo.lock`,
  and proto edits are forbidden.
- Result review path and required evidence are bound.
- No benchmark is required because the approved scope is correctness for a
  downstream serving prerequisite, not a speed claim.

## Required Next Action

Amend the implementation plan before implementation.

The next audit must verify:

- optional-condition helpers are scope-aware in the plan;
- a retained optional projected field is covered by a codegen oracle;
- `docs/protocols/spec_protocol.md` is listed in required reads before code;
- no new file, dependency, or serving-surface expansion was introduced.
