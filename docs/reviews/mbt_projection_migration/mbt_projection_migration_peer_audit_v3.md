# Peer Audit V3: MBT Projection Migration

Status: `BLOCKED`

Slug: `mbt_projection_migration`

Target research brief:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_research_brief.md
```

Target spec:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Previous audits:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v2.md
```

## Required Reads

Completed reads:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/lifecycle_protocol.md
docs/protocols/spec_protocol.md
docs/protocols/peer_audit_protocol.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_research_brief.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v2.md
docs/specs/mbt_projection_migration_SPEC.md
docs/specs/mbt_codegen_migration_SPEC.md
docs/specs/mbt_schema_core_generation_SPEC.md
proto/mathilde/options.proto
crates/codegen/src/config.rs
crates/codegen/src/emit.rs
crates/codegen/src/main.rs
crates/codegen/src/lib.rs
crates/codegen/src/options.rs
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_cli.rs
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

## Findings

### 1. Generated Artifact Ownership Still Conflicts With Core-Surface Reproducibility

Severity: blocking

The spec now correctly introduces a new opt-in surface:

```text
--surface projection
```

It also states that the older `--surface core` contract remains active and does
not emit projection code. That resolves the conceptual conflict from peer audit
v2.

However, the spec still binds the projection check command to the same generated
file used by the previous core-generation phase:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

This file cannot be simultaneously reproducible by both:

```text
--surface core
--surface projection
```

if the projection surface emits additional projected marker schemas and
projection methods.

Earlier schema-core artifacts bind the same file to core generation:

```text
docs/specs/mbt_schema_core_generation_SPEC.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_result_review.md
```

The projection spec must therefore choose one exact artifact ownership model:

1. **Replace ownership:** `test_compatibility_v1.rs` becomes projection-surface
   output after this migration, and the old core-surface reproducibility check
   for that file is explicitly superseded for this schema crate.
2. **Separate artifact:** keep `test_compatibility_v1.rs` as core-surface
   output and generate projection output to a separate file such as
   `test_compatibility_v1_projection.rs`, with exact module wiring and tests
   specified.
3. **Other explicit model:** define another deterministic generated-artifact
   layout and its check commands.

Required amendment:

- define the generated artifact ownership model;
- if replacing ownership, explicitly supersede the schema-core generated-file
  reproducibility contract for this schema crate;
- if using a separate artifact, bind exact generated file path, module wiring,
  check command, and tests;
- update Section 20 generated artifact bindings and Section 18 validation
  commands accordingly.

### 2. `crates/codegen/src/emit.rs` Is Missing From Code Bindings

Severity: blocking

Code-read evidence:

```text
crates/codegen/src/emit.rs
```

currently calls:

```rust
let model = load_schema_model(&request)?;
let source = generated_schema(&model)?;
```

for inspect/write/check. It does not branch on `config.surface`.

The spec introduces a second surface. Implementing distinct `--surface core`
and `--surface projection` output requires the emission dispatch layer to use
the parsed surface, either by:

- calling a different emitter;
- passing the surface into the emitter;
- or otherwise selecting source-only versus projection output.

The spec binds `crates/codegen/src/config.rs` and CLI tests, but not
`crates/codegen/src/emit.rs`. That makes the implementation bindings
incomplete.

Required amendment:

- add `crates/codegen/src/emit.rs` to Section 19 allowed code files;
- state that `emit.rs` must dispatch `--surface core` and
  `--surface projection` without changing action semantics for inspect, write,
  or check;
- if `crates/codegen/src/lib.rs` documentation is updated to remove
  "core-only" wording, bind it as an allowed documentation-only code file.

## Resolved Previous Audit Blockers

| Previous blocker                                        | V3 result                                      |
| ------------------------------------------------------- | ---------------------------------------------- |
| Mandatory spec section order                            | Resolved                                       |
| Exact codegen-check command missing                     | Resolved for projection surface                |
| Projected schema-hash inputs under-specified            | Resolved directionally                         |
| Compile-surface budget not measurable                   | Resolved                                       |
| Correctness oracle and test plan combined               | Resolved                                       |
| `--surface core` conceptual conflict                    | Resolved by introducing `--surface projection` |
| Existing projection-ignored assertion migration missing | Resolved directionally                         |

## Audit Lens Results

| Lens                                    | Result                                                 |
| --------------------------------------- | ------------------------------------------------------ |
| Measured object clarity                 | Passed                                                 |
| Schema source ownership                 | Passed                                                 |
| Wire/archive validation                 | Passed                                                 |
| Trusted-access safety                   | Passed                                                 |
| Codegen determinism                     | Blocked by generated artifact ownership conflict       |
| Generated-code compile surface          | Passed directionally                                   |
| Crate boundary isolation                | Passed directionally                                   |
| Dependency containment                  | Passed                                                 |
| Correctness oracle                      | Passed                                                 |
| Benchmark isolation                     | Passed                                                 |
| Runtime performance budget              | Passed                                                 |
| Failure behavior                        | Passed                                                 |
| Code binding completeness               | Blocked because `emit.rs` is missing                   |
| Generated artifact binding completeness | Blocked by core/projection output collision            |
| Client/operator interpretation safety   | Blocked until generated artifact ownership is explicit |

## Required Amendment Summary

Amend:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Required changes:

1. Define the generated artifact ownership model for core versus projection
   output.
2. Update generated artifact bindings and codegen-check commands to match that
   model.
3. Add `crates/codegen/src/emit.rs` to code bindings and bind surface dispatch
   behavior.
4. Optionally add `crates/codegen/src/lib.rs` as documentation-only code
   binding if its "core-only" crate docs are updated.

No code implementation is authorized by this audit.

## Classification

```text
BLOCKED
```
