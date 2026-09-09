# Peer Audit V3: MBT Schema Core Generation

Status: `PEER_AUDIT_PASSED`

Slug: `mbt_schema_core_generation`

Target spec:

```text
docs/specs/mbt_schema_core_generation_SPEC.md
```

Target implementation plan:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_implementation_plan.md
```

Previous result review:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_result_review.md
```

## Required Reads

Completed reads:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/lifecycle_protocol.md
docs/protocols/spec_protocol.md
docs/protocols/implementation_protocol.md
docs/protocols/code_style_protocol.md
docs/protocols/codegen_protocol.md
docs/protocols/testing_benchmark_protocol.md
docs/specs/mbt_schema_core_generation_SPEC.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_implementation_plan.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_result_review.md
crates/codegen/src/emit.rs
```

## Amendment Under Audit

The partial implementation stopped because generated Rust was reproducible by
`mbt_codegen --check`, but failed workspace `cargo fmt --all --check`.

Code-read evidence:

```text
crates/codegen/src/emit.rs
```

Observed source behavior before amendment:

```text
rustfmt --edition 2021
```

The amended spec and plan now allow exactly one codegen source edit:

```text
crates/codegen/src/emit.rs
  rustfmt --edition 2021
  -> rustfmt --edition 2024
```

No other codegen behavior, dependency, CLI argument, output surface, descriptor
logic, schema modeling, emitted API, or runtime semantic change is authorized.

## Findings

No blocking findings remain.

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
| Formatter ownership                     | Passed |
| Client/operator interpretation safety   | Passed |

## Specific Checks

### Formatter Ownership

The amendment is valid because generated files must be owned by codegen, not by
manual formatting. Aligning the generator's rustfmt edition with workspace
edition keeps both invariants true:

- generated files remain reproducible by `mbt_codegen --check`;
- workspace `cargo fmt --all --check` can pass without hand-editing generated
  output.

### Codegen Boundary

The implementation plan forbids every codegen source edit except the single
edition argument in `crates/codegen/src/emit.rs`. This prevents the formatting
fix from becoming a broader generator rewrite.

### Validation Boundary

The amended spec and plan require validation of both:

```text
metamorphic_binary_transport_codegen
metamorphic_binary_transport_schema_test_compatibility
```

Required codegen validation:

```text
cargo check -p metamorphic_binary_transport_codegen
cargo test -p metamorphic_binary_transport_codegen
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
```

Required generated schema validation remains bound by the prior spec.

## Implementation Conditions

Implementation may proceed only under these conditions:

- edit only `crates/codegen/src/emit.rs` for rustfmt edition alignment;
- regenerate `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`;
- rerun `mbt_codegen --check`;
- rerun `cargo fmt --all --check`;
- validate the codegen crate;
- validate the schema crate;
- update the existing result review with final pass/fail evidence.

If any behavior change beyond formatter edition is needed, implementation must
stop and request another amendment.

## Classification

```text
PEER_AUDIT_PASSED
```
