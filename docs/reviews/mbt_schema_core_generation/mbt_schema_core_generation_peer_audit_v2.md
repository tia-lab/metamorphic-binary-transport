# Peer Audit V2: MBT Schema Core Generation

Status: `PEER_AUDIT_PASSED`

Slug: `mbt_schema_core_generation`

Target spec:

```text
docs/specs/mbt_schema_core_generation_SPEC.md
```

Previous audit:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_peer_audit.md
```

Research brief:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_research_brief.md
```

## Required Reads

Completed reads:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/lifecycle_protocol.md
docs/protocols/spec_protocol.md
docs/protocols/peer_audit_protocol.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_research_brief.md
docs/specs/mbt_schema_core_generation_SPEC.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_peer_audit.md
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_result_review.md
proto/mathilde/options.proto
crates/core/src/runtime.rs
crates/core/src/envelope.rs
crates/core/src/error.rs
crates/codegen/src/config.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/model.rs
crates/codegen/src/rust_emit.rs
```

## Prior Blocker Resolution

Prior blocker:

```text
Wrong schema ID and schema hash tests were optional.
```

Amended spec section:

```text
13. Failure contract
```

Observed amended requirement:

```text
- wrong schema ID through header mutation, asserting
  `TransportError::UnknownSchemaId`;
- wrong schema hash through header mutation, asserting
  `TransportError::SchemaHashMismatch`.
```

Assessment:

The blocker is resolved. The failure contract now requires concrete header
mutation tests and concrete `TransportError` variants.

## Audit Lens Results

| Lens                                    | Result |
| --------------------------------------- | ------ |
| Measured object clarity                 | Passed |
| Schema source ownership                 | Passed |
| Wire/archive validation                 | Passed |
| Trusted-access safety                   | Passed |
| Codegen determinism                     | Passed |
| Generated-code compile surface          | Passed |
| Crate boundary isolation                | Passed |
| Dependency containment                  | Passed |
| Correctness oracle                      | Passed |
| Benchmark isolation                     | Passed |
| Runtime performance budget              | Passed |
| Failure behavior                        | Passed |
| Code binding completeness               | Passed |
| Generated artifact binding completeness | Passed |
| Client/operator interpretation safety   | Passed |

## Findings

No blocking findings remain.

## Implementation Planning Requirements

The implementation plan must preserve these audit conditions:

- derive an MBT-only proto from the experiment compatibility source;
- exclude DB/cache/lookup options from the MBT fixture;
- create only the approved schema crate;
- commit generated Rust only through `mbt_codegen --surface core`;
- run `mbt_codegen --check`;
- prove deterministic encode/access/inspect behavior;
- prove the required failure variants, including schema ID and schema hash
  mutation;
- record compile-surface evidence and dependency-tree evidence;
- stop and amend the spec if codegen needs production source changes.

## Classification

```text
PEER_AUDIT_PASSED
```
