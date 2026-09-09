# Peer Audit V4: MBT Projection Migration

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
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v3.md
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
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit_v3.md
docs/specs/mbt_projection_migration_SPEC.md
docs/specs/mbt_workspace_architecture_SPEC.md
docs/specs/mbt_core_runtime_migration_SPEC.md
docs/specs/mbt_codegen_migration_SPEC.md
docs/specs/mbt_schema_core_generation_SPEC.md
proto/mathilde/options.proto
crates/codegen/src/config.rs
crates/codegen/src/emit.rs
crates/codegen/src/main.rs
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/test_rust_emit_core.rs
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

No run evidence was used. This is a no-code spec audit.

## Findings

### 1. Pre-Audit Closure Gate Is Still False For Command Surfaces

Severity: blocking

The amended spec now includes a pre-audit closure checklist and resolves the
major V3 ownership conflict. It explicitly states that:

```text
--surface core remains core-only
--surface projection owns the committed compatibility generated file
```

That resolves the prior generated-artifact collision.

However, `docs/protocols/spec_protocol.md` now requires every command surface
to be exact before peer audit, including:

```text
CLI flags
output paths
ownership of check/write/inspect behavior
```

The projection spec defines the exact `--check` command, but still defers exact
`--write`, `--inspect`, and core-surface regression command/test shape to the
implementation plan.

Spec evidence:

```text
docs/specs/mbt_projection_migration_SPEC.md
Section 9:
  The implementation plan must also bind the matching --inspect and --write
  commands, but the check command above is the spec-level generated-code
  reproducibility contract.

Section 9:
  The implementation plan must also bind a core-surface regression check that
  proves --surface core still emits no projection code. That check may write to
  a temporary path...

Section 22:
  exact generated file update command
  exact core-surface temporary-output or unit-test regression check
```

Code-read evidence:

```text
crates/codegen/src/config.rs
```

The current CLI has three distinct action surfaces:

```rust
Action::Inspect
Action::Write
Action::Check
```

`--inspect` forbids `--out`, while `--write` and `--check` require `--out`.
Therefore the exact inspect/write/check command shapes are not interchangeable
implementation details.

Code-read evidence:

```text
crates/codegen/src/emit.rs
```

Current inspect/write/check all load the model and call the same emitter:

```rust
let model = load_schema_model(&request)?;
let source = generated_schema(&model)?;
```

The spec correctly binds `emit.rs` for future dispatch, but it still needs to
define the exact projection inspect and write commands and the exact
core-regression proof surface before implementation planning.

Why this blocks:

- the checklist says command surfaces are closed;
- the spec still delegates exact non-check command surfaces to the
  implementation plan;
- the pre-audit closure gate says those surfaces must be exact before audit;
- implementation planning must not choose whether old core behavior is proved
  by a temporary output command or by a unit test.

Required amendment:

1. Add the exact projection `--inspect` command. It must omit `--out`, matching
   current CLI semantics.
2. Add the exact projection `--write` command. It must use the same proto roots,
   schema, root, module, surface, and committed output path as the `--check`
   command.
3. Define inspect/write/check ownership:
   - `--inspect --surface projection` reports the projection-surface model;
   - `--write --surface projection` writes the committed compatibility
     generated file;
   - `--check --surface projection` checks that same file.
4. Replace the open choice:

```text
temporary-output or unit-test regression check
```

with one exact required core-regression proof. If it uses a temporary output
file, bind the exact path. If it uses a unit test, bind the exact test file and
test behavior. 5. Update the pre-audit closure checklist so it no longer claims exact
command-surface closure while deferring command surfaces to the
implementation plan.

## Resolved Previous Audit Blockers

| Previous blocker                                        | V4 result                          |
| ------------------------------------------------------- | ---------------------------------- |
| Mandatory spec section order                            | Resolved                           |
| Original exact codegen-check command missing            | Resolved for `--check`             |
| Projected schema-hash inputs under-specified            | Resolved                           |
| Compile-surface budget not measurable                   | Resolved                           |
| Correctness oracle and test plan combined               | Resolved                           |
| `--surface core` conceptual conflict                    | Resolved by `--surface projection` |
| Existing projection-ignored assertion migration missing | Resolved directionally             |
| Generated artifact ownership conflict                   | Resolved                           |
| `crates/codegen/src/emit.rs` missing from bindings      | Resolved                           |

## Audit Lens Results

| Lens                                    | Result                                          |
| --------------------------------------- | ----------------------------------------------- |
| Pre-audit closure gate completeness     | Blocked by command-surface deferral             |
| Measured object clarity                 | Passed                                          |
| Schema source ownership                 | Passed                                          |
| Wire/archive validation                 | Passed                                          |
| Trusted-access safety                   | Passed                                          |
| Codegen determinism                     | Passed directionally                            |
| Generated-code compile surface          | Passed                                          |
| Crate boundary isolation                | Passed                                          |
| Dependency containment                  | Passed                                          |
| Correctness oracle                      | Passed                                          |
| Benchmark isolation                     | Passed                                          |
| Performance budget                      | Passed                                          |
| Failure behavior                        | Passed                                          |
| Code binding completeness               | Passed for files                                |
| Generated artifact binding completeness | Passed                                          |
| Client/operator interpretation safety   | Blocked until exact command surfaces are closed |

## Required Amendment Summary

Amend:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Required changes:

1. Add exact projection `--inspect` command.
2. Add exact projection `--write` command.
3. Bind projection inspect/write/check ownership in the codegen contract.
4. Replace the `temporary-output or unit-test` core-regression choice with one
   exact proof surface.
5. Update the pre-audit closure checklist to match those exact command
   bindings.

No implementation is authorized by this audit.

## Classification

```text
BLOCKED
```
