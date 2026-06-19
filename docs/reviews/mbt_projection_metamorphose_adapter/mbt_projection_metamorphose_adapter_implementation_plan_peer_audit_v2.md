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

# MBT Projection Metamorphose Adapter Implementation Plan Peer Audit v2

## Classification

`PEER_AUDIT_PASSED`

## Scope

Audited implementation plan:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_implementation_plan.md
```

Prior implementation plan peer audit:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_implementation_plan_peer_audit.md
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
- amended implementation plan
- prior implementation plan peer audit
- approved spec
- passing spec peer audit v3
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/model.rs`
- `crates/codegen/src/descriptor.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`
- `crates/schemas/bars_core/tests/test_bars_metamorphose.rs`
- `crates/schemas/test_compatibility_core/tests/test_projection.rs`

## Prior Blocker Resolution Check

### Blocker 1: projection JSON optional-condition helpers were not bound to projection scope

Status: resolved.

Plan evidence:

```text
json_output_optional_condition(scope, output)
archived_optional_condition(scope, field)
derived_utc_optional_condition(scope, field)
```

The plan now requires optional physical fields and optional derived UTC fields
to use the same active `EmitScope` as their writer. It also requires projection
JSON output to avoid source-scoped presence constants for projected optional
fields.

The plan now requires a second codegen oracle with an inline proto fixture that
proves a projection-retained optional field emits projection-scoped presence
checks.

### Blocker 2: required implementation refresh omitted `spec_protocol.md`

Status: resolved.

Plan evidence:

```text
4. docs/protocols/spec_protocol.md
```

The implementation plan now satisfies the implementation protocol locked
refresh rule.

## Audit Lenses

### File and artifact bindings

Passed.

The plan allows edits only to:

```text
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
crates/schemas/bars_core/tests/test_bars_metamorphose.rs
```

The plan allows creation only of:

```text
crates/schemas/test_compatibility_core/tests/test_projection_metamorphose.rs
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_result_review.md
```

Generated artifacts remain codegen-owned:

```text
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs
```

### Forbidden edits and dependency containment

Passed.

The plan forbids edits to core, metamorphose runtime, adapters, compression,
serving, `Cargo.toml`, `Cargo.lock`, and proto files. It also forbids any new
dependency.

### Codegen behavior binding

Passed.

The plan binds:

- source and projection JSON sections;
- projection marker sections in proto projection order;
- source marker helper names staying unprefixed;
- projection marker helper names using existing projection scope;
- projected JSON output fields rebuilt from projection mappings;
- retained derived UTC fields rebased to projected fields;
- empty projection JSON output rejected with `CodegenError::InvalidSchema`;
- non-JSON adapters unchanged.

### Optional/presence scope proof

Passed.

The plan now requires scope-aware optional-condition helpers and a dedicated
codegen oracle for a retained optional projected field. This directly closes
the prior risk that projection JSON would emit source-scope presence constants.

Implementation should prefer making the inline fixture's derived UTC source
field optional as well, because that gives the strongest single proof for both
physical optional and derived-UTC optional conditions. This is a non-blocking
recommendation because the plan already binds the helper behavior directly.

### Correctness oracle

Passed.

The plan covers:

- generic source-plus-projection JSON generation from the non-serving fixture;
- retained optional projected-field presence scoping;
- Bars no-metadata projection JSON checked and trusted paths;
- test-compatibility projection JSON checked and trusted paths;
- generated artifact write/check reproducibility.

### Compile surface and validation

Passed.

The plan requires default-feature schema checks and a timed codegen
`cargo check -p mbt_codegen --all-targets`. It does not make a runtime speed
claim.

### Result review and rollback boundary

Passed.

The result review path is bound and must record generated commands, tests,
compile checks, forbidden-source output, and residual risk. Rollback is limited
to the approved files and generated artifacts.

## Residual Risks

The implementation must be careful when refactoring shared optional-condition
helpers because protobuf and CSV currently call those helpers too. The plan
keeps non-JSON adapters unchanged, so any helper signature change must preserve
source-scope behavior for non-JSON adapter paths.

This is not a plan blocker because the file binding, validation commands, and
non-JSON unchanged requirement are explicit.

## Required Next Action

Implementation may proceed only after the user explicitly approves:

```text
Approved: implement docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_implementation_plan.md
```
