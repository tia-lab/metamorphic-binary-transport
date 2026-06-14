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

# Result Review: MBT Schema Core Generation

Status: `IMPLEMENTATION_VALIDATED`

Slug: `mbt_schema_core_generation`

Spec:

```text
docs/specs/mbt_schema_core_generation_SPEC.md
```

Implementation plan:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_implementation_plan.md
```

Latest peer audit:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_peer_audit_v5.md
```

## Built

Created the approved schema crate:

```text
crates/schemas/test_compatibility_core/Cargo.toml
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
crates/schemas/test_compatibility_core/src/lib.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
crates/schemas/test_compatibility_core/tests/test_roundtrip.rs
crates/schemas/test_compatibility_core/tests/test_failures.rs
crates/schemas/test_compatibility_core/tests/test_determinism.rs
```

Edited approved source files:

```text
Cargo.toml
crates/codegen/src/emit.rs
crates/codegen/src/rust_emit.rs
```

The approved codegen source edits were:

```text
crates/codegen/src/emit.rs
  rustfmt --edition 2021
  -> rustfmt --edition 2024

crates/codegen/src/rust_emit.rs
  archived bool checksum/getter access removes `.to_native()`
  generated VALID_*_MASK removes leading `0 |`
  generated key-order validation uses a clippy-clean single condition
  generated payload construction uses `rows` shorthand when valid
```

No `crates/core`, adapter, projection, transponding, benchmark, or
`proto/mathilde/options.proto` source file was edited.

## Codegen Evidence

Command:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --inspect --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface core
```

Observed:

```text
module=test_compatibility_v1
schema_id=40001
schema_version=1
transport_name=mathilde.test_compatibility.v1
payload_root=true
schema_hash=3233278346470496550
payload_type=TestCompatibilityResponseV1Payload
row_type=TestCompatibilityRowV1
generated_lines=634
```

The inspect output listed all expected MBT physical kinds from the all-fields
schema.

Command:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface core --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Observed:

```text
passed
```

Command:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface core --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Observed:

```text
passed
```

Generated source line count:

```text
958 crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Generated source evidence:

```rust
pub const VALID_VENUE_MASK: u64 =
    (1 << VENUE_BINANCE_BIT) | (1 << VENUE_BYBIT_BIT) | (1 << VENUE_OKX_BIT);
```

```rust
if previous.is_some_and(|prev| {
    (row.tenant_ordinal, row.entity_ordinal, row.close_ms)
        < (prev.tenant_ordinal, prev.entity_ordinal, prev.close_ms)
}) {
    return Err(TransportError::InvalidTimeGrid(
        "key order regression".to_string(),
    ));
}
```

```rust
let payload = TestCompatibilityResponseV1Payload {
    schema_version: SCHEMA_VERSION_VALUE,
    rows,
};
```

```rust
checksum = update_bool(checksum, row.required_bool);
checksum = update_bool(checksum, row.optional_bool);
```

Numeric archived fields still use `.to_native()`.

## Formatter Evidence

Command:

```text
cargo fmt --all --check
```

Observed:

```text
passed
```

## Codegen Crate Validation

Command:

```text
cargo check -p metamorphic_binary_transport_codegen
```

Observed:

```text
passed
```

Command:

```text
cargo test -p metamorphic_binary_transport_codegen
```

Observed:

```text
14 passed; 0 failed
```

Command:

```text
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
```

Observed:

```text
passed
```

## Schema Crate Validation

Command:

```text
cargo check -p metamorphic_binary_transport_schema_test_compatibility
```

Observed:

```text
passed
```

Command:

```text
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_test_compatibility
```

Observed:

```text
Exit status: 0
Elapsed wall clock time: 0:00.10
User time: 0.09s
System time: 0.01s
Maximum resident set size: 30500 KB
```

This was a warm build-surface check; it is evidence for this run only and does
not prove cold compile time.

Command:

```text
cargo test -p metamorphic_binary_transport_schema_test_compatibility
```

Observed:

```text
test_determinism.rs: 1 passed; 0 failed
test_failures.rs: 12 passed; 0 failed
test_roundtrip.rs: 1 passed; 0 failed
doc tests: 0 passed; 0 failed
```

Command:

```text
cargo clippy -p metamorphic_binary_transport_schema_test_compatibility --all-targets -- -D warnings
```

Observed:

```text
passed
```

## Dependency Boundary Evidence

Command:

```text
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
```

Observed dependency roots:

```text
metamorphic_binary_transport_schema_test_compatibility
├── metamorphic_binary_transport_core
└── rkyv =0.8.16
```

No adapter, serde, prost, Arrow, Parquet, DB, cache, lookup, or serving
dependency appeared in the schema crate tree.

Command:

```text
cargo tree -p metamorphic_binary_transport_core
```

Observed dependency root:

```text
metamorphic_binary_transport_core
└── thiserror =2.0.17
```

## Workspace Evidence

Command:

```text
cargo check --workspace
```

Observed:

```text
passed
```

## Proved

Proved by run evidence:

- the MBT-only compatibility proto is accepted by current codegen;
- generated core schema output is reproducible by `mbt_codegen --check`;
- generated Rust formatting is owned by codegen and accepted by workspace
  `cargo fmt --all --check`;
- generated output is 958 source lines in the checked generated file;
- generated archived bool access no longer calls `.to_native()`;
- generated clippy-cleanliness fixes remove the previous schema crate clippy
  blockers;
- the codegen crate compiles, tests, and passes clippy with warnings denied;
- the generated schema crate compiles, tests, and passes clippy with warnings
  denied;
- schema crate dependency boundary is limited to core and rkyv;
- MBT core dependency boundary remains thiserror only;
- the workspace check passes.

## Not Proved

Not proved:

- cold compile time;
- runtime throughput;
- adapter generation;
- projection generation;
- transponding generation;
- production Primitives or Bars schema migration.

## Result

The schema core generation phase is validated for the all-fields core schema
crate under the evidence above.
