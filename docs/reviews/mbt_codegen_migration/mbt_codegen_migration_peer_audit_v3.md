# Peer Audit V3: MBT Codegen Migration

Status: `PEER_AUDIT_PASSED`

Slug: `mbt_codegen_migration`

Audited spec:

```text
docs/specs/mbt_codegen_migration_SPEC.md
```

Prior audits:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_peer_audit.md
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_peer_audit_v2.md
```

## Audit Reads

Protocol and invariant evidence:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/peer_audit_protocol.md
docs/protocols/spec_protocol.md
```

Spec and review evidence:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_research_brief.md
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_peer_audit_v2.md
docs/specs/mbt_codegen_migration_SPEC.md
```

Code-read evidence:

```text
crates/core/src/runtime.rs
crates/core/src/envelope.rs
crates/core/Cargo.toml
crates/codegen/Cargo.toml
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/descriptor.rs
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/rust_emit.rs
```

Run evidence:

```text
rg -n "Status:|DRAFT_AWAITING_PEER_AUDIT_V3|Dictionary alias behavior|Projection option behavior|Required generated core type surface|Core schema hash normal form|project_|projection_|PEER_AUDIT_PASSED|BLOCKED" docs/specs/mbt_codegen_migration_SPEC.md docs/reviews/mbt_codegen_migration/mbt_codegen_migration_peer_audit_v2.md
rg -n "prost|serde|arrow|parquet|rkyv|thiserror|prost-reflect" Cargo.toml crates/*/Cargo.toml
```

Observed:

- the amended spec is marked `DRAFT_AWAITING_PEER_AUDIT_V3`;
- the amended spec contains explicit dictionary alias behavior;
- the amended spec contains explicit projection behavior for `--surface core`;
- the amended spec contains the generated view/row API contract;
- the amended spec contains exact schema hash normal-form rules;
- the amended spec forbids projection strings in core generated output;
- current `crates/codegen/Cargo.toml` has no production dependencies yet;
- current `crates/core/Cargo.toml` contains only `thiserror = "=2.0.17"`.

No build, codegen, or benchmark command was run for this audit.

## V2 Blocker Resolution

| V2 blocker                                            | V3 status                                                                                                                                                        |
| ----------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Generated view and archived-row API surface not exact | Resolved. Section 8 now binds marker, view, rows iterator, archived-row wrapper, getters, and presence accessors.                                                |
| Schema hash normal form under-specified               | Resolved. Section 9 now binds FNV-1a constants, normalized lines, ordering, included values, and excluded values.                                                |
| Dictionary alias behavior undefined                   | Resolved. Section 6 now states aliases are accepted but do not affect ordinals, archive fields, core hash, or core helper APIs.                                  |
| Projection option behavior ambiguous in core mode     | Resolved. Section 6 now states `--surface core` recognizes but does not construct, hash, or emit projection output; section 9 forbids projection output strings. |

## Findings

No blocking findings remain at the spec level.

The spec now binds the codegen source surface, output surface, dependency
surface, generated API surface, schema hash behavior, correctness oracle,
temporary generated artifacts, and validation commands tightly enough to allow
an implementation plan.

## Audit Lenses

| Lens                                    | Result                                                                                                                             |
| --------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Measured object clarity                 | Passed. The measured object is codegen migration and isolation, not runtime speed.                                                 |
| Schema source ownership                 | Passed. `.proto + proto/mathilde/options.proto` is the source of truth, and external schema roots are allowed.                     |
| Wire/archive validation                 | Passed. Core envelope use, checked access, trusted access, and no envelope change are specified.                                   |
| Trusted-access safety                   | Passed. Trusted access is unsafe and has a caller contract.                                                                        |
| Codegen determinism                     | Passed. Input order, root, module, rustfmt, deterministic compare, and schema hash normal form are specified.                      |
| Generated-code compile surface          | Passed. The smoke crate compile is mandatory.                                                                                      |
| Crate boundary isolation                | Passed. Core, adapters, projection, transponding, benches, and codegen boundaries are explicit.                                    |
| Dependency containment                  | Passed. `prost-build`, adapter deps, serde, and zstd are forbidden for codegen in this spec; generated consumer deps are separate. |
| Correctness oracle                      | Passed. Descriptor, model, invalid-schema, deterministic output, forbidden-surface, API, hash, and smoke compile checks are bound. |
| Benchmark isolation                     | Passed. No runtime benchmark claim is made.                                                                                        |
| Performance budget                      | Passed. Runtime parity is explicitly deferred.                                                                                     |
| Failure behavior                        | Passed. Typed error classes and non-zero CLI behavior are specified.                                                               |
| Code binding completeness               | Passed. Implementation files, test files, forbidden files, and artifact paths are bound.                                           |
| Generated artifact binding completeness | Passed. No committed generated artifacts; temporary paths and smoke crate paths are bound.                                         |
| Client/operator interpretation safety   | Passed. Generated API and deferred surfaces are explicit enough for planning.                                                      |

## Implementation-Plan Requirements

The next implementation plan must still bind:

- every old experiment codegen function migrated, changed, or rejected;
- exact dependency additions and versions;
- exact fixture proto contents or fixture writer helpers;
- exact temporary directory cleanup policy;
- exact validation commands and expected outputs;
- rollback boundary;
- known risks, especially wide-schema compile-surface risk.

These are planning requirements, not remaining spec blockers.

## Classification

```text
PEER_AUDIT_PASSED
```
