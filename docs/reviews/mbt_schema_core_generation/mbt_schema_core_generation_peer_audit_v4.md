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

# Peer Audit V4: MBT Schema Core Generation

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

Result review that exposed the blocker:

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
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_research_brief.md
docs/specs/mbt_schema_core_generation_SPEC.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_implementation_plan.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_result_review.md
crates/codegen/src/rust_emit.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

## Findings

No blocking findings remain.

## Amendment Under Audit

The previous implementation run passed codegen formatting and codegen crate
validation, then stopped at schema crate compilation.

Run evidence from the result review:

```text
cargo check -p metamorphic_binary_transport_schema_test_compatibility
```

Observed failure:

```text
error[E0599]: no method named `to_native` found for type `bool` in the current scope
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs:305:56
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs:306:56
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs:799:32
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs:805:32
```

Generated-code evidence:

```rust
checksum = update_bool(checksum, row.required_bool.to_native());
checksum = update_bool(checksum, row.optional_bool.to_native());
```

```rust
pub fn required_bool(&self) -> bool {
    self.row.required_bool.to_native()
}
```

Code-read evidence:

```text
crates/codegen/src/rust_emit.rs
```

Observed generator sites:

```text
emit_archived_checksum_line:
  FieldKind::Bool emits update_bool(checksum, {access}.to_native())

archived_value_access:
  FieldKind::Bool is grouped with numeric fields and emits {access}.to_native()
```

The amendment authorizes only this behavior change:

```text
Archived bool fields must be emitted as native bool access.
Archived numeric, dictionary, bitmask, and floating fields keep .to_native().
```

Exact implementation binding under audit:

```text
crates/codegen/src/rust_emit.rs

emit_archived_checksum_line:
  FieldKind::Bool emits update_bool(checksum, {access})
  not update_bool(checksum, {access}.to_native())

archived_value_access:
  FieldKind::Bool emits {access}
  not {access}.to_native()
```

The prior formatter amendment remains bound:

```text
crates/codegen/src/emit.rs
  rustfmt --edition 2021
  -> rustfmt --edition 2024
```

## Audit Lens Results

| Lens | Result |
| --- | --- |
| Measured object clarity | Passed |
| Schema source ownership | Passed |
| Wire/archive validation | Passed |
| Trusted-access safety | Passed |
| Codegen determinism | Passed |
| Generated-code compile surface | Passed |
| Crate boundary isolation | Passed |
| Dependency containment | Passed |
| Correctness oracle | Passed |
| Benchmark isolation | Passed |
| Runtime performance budget | Passed |
| Failure behavior | Passed |
| Code binding completeness | Passed |
| Generated artifact binding completeness | Passed |
| Client/operator interpretation safety | Passed |

## Specific Checks

### Narrowness

The amendment is narrow enough because it binds two generated archived bool
emission sites and no schema model, descriptor, runtime, envelope, validation,
trusted-access, dependency, or adapter behavior.

### Wire Compatibility

The amendment does not alter owned row layout, archived row layout, header
layout, schema identity, schema hash calculation, payload checksum algorithm,
or archive validation rules. It changes only the Rust source emitted for reading
archived bool values that are already native bools.

### Correctness Oracle

The existing validation sequence remains sufficient after the amendment because
it reruns:

```text
mbt_codegen --check
cargo fmt --all --check
cargo check -p metamorphic_binary_transport_codegen
cargo test -p metamorphic_binary_transport_codegen
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
cargo check -p metamorphic_binary_transport_schema_test_compatibility
cargo test -p metamorphic_binary_transport_schema_test_compatibility
cargo clippy -p metamorphic_binary_transport_schema_test_compatibility --all-targets -- -D warnings
cargo check --workspace
```

The schema crate test suite is expected to exercise owned encode, checked
access, trusted access after validation, deterministic output, and explicit
failure behavior for the all-fields fixture.

### Stop Gate

Implementation must stop if any additional codegen change is needed beyond:

```text
crates/codegen/src/emit.rs
crates/codegen/src/rust_emit.rs
```

with the exact branch-level limits stated in the spec and implementation plan.

## Classification

```text
PEER_AUDIT_PASSED
```
