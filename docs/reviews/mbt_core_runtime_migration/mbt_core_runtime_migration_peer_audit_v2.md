# Peer Audit V2: MBT Core Runtime Migration

Audit classification: `PEER_AUDIT_PASSED`

Spec audited:

```text
docs/specs/mbt_core_runtime_migration_SPEC.md
```

Prior audit:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit.md
```

## Required Reads

Completed reads:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_research_brief.md`
- `docs/specs/mbt_core_runtime_migration_SPEC.md`
- `docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit.md`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/envelope.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/runtime.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codec.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/error.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`

## Findings

No blocking findings remain.

### F1 From Prior Audit Is Resolved

Prior blocker:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit.md
```

The v1 audit blocked the spec because the validation command rejected the
literal `rkyv`, while the spec also required the wire encoding constant:

```rust
pub const ENCODING_MBT_RKYV: u16 = 1;
```

The amended spec now keeps the encoding constant and changes validation to
reject forbidden dependency, import, and module surfaces:

```bash
! cargo tree -p metamorphic_binary_transport_core | rg -n "rkyv|serde|serde_json|prost|prost-build|prost-reflect|arrow-array|arrow-buffer|arrow-ipc|arrow-schema|parquet|zstd|itoa"
! rg -n "rkyv::|use .*rkyv|extern crate rkyv|serde::|serde_json::|prost::|arrow::|arrow_|parquet::|zstd::" crates/metamorphic_binary_transport_core/src crates/metamorphic_binary_transport_core/Cargo.toml
! rg -n "pub mod (generated|metamorphose|transponding|benches)|mod (generated|metamorphose|transponding|benches)" crates/metamorphic_binary_transport_core/src
```

This resolves the contradiction without weakening the core boundary.

## Evidence

Code-read evidence:

- Experiment `src/envelope.rs` defines the fixed wire constants, 128-byte
  header layout, FNV-1a checksum, checked header validation, and trusted
  payload validation.
- Experiment `src/envelope.rs` also contains schema-specific helpers
  `SCHEMA_ID_MATHILDE_BAR_ROW`, `SCHEMA_VERSION_V1`, `PROTO_BYTES`,
  `schema_hash()`, `TransportHeader::new(...)`, and `validate_header(...)`.
  The spec forbids these in core.
- Experiment `src/runtime.rs` defines `BinaryInspection`, `MbtSchema`, and the
  generic dispatch helpers without adapter dependencies.
- Experiment `src/codec.rs` only delegates `response_checksum` to `fnv1a64`.
- Experiment `src/error.rs` mixes transport errors with schema-specific and
  adapter errors. The spec lists the allowed core error surface and forbids
  adapter errors.
- Generated `bars_v1.rs` uses `SchemaHeaderSpec`,
  `TransportHeader::new_with_schema`, `encode_header`,
  `validate_header_for_schema`, `trusted_payload_for_schema`,
  `BinaryInspection`, and `MbtSchema`; it owns rkyv archive access outside
  core.

Run evidence:

- The amended negated validation commands were executed on the current
  skeleton and returned success.
- `git diff --check` passed after the amendment.

## Audit Lenses

Passed:

- measured object clarity;
- schema source ownership;
- wire/archive validation;
- trusted-access safety;
- codegen exclusion from this migration;
- generated-code compile-surface containment;
- crate boundary isolation;
- dependency containment;
- correctness oracle;
- benchmark isolation;
- performance claim discipline;
- failure behavior;
- code binding completeness for this migration slice;
- generated artifact exclusion;
- client/operator interpretation safety.

## Non-Blocking Implementation-Plan Requirements

The implementation plan must still bind:

- exact `thiserror = "2.0.17"` placement;
- exact `TransportError` source;
- exact `TransportHeader` field source;
- exact `codec`, `envelope`, and `runtime` source;
- exact tests;
- exact validation commands;
- expected command outputs.

These are already required by the spec, so they do not block the spec.

## Decision

The amended spec passes peer audit.

Implementation planning may start after explicit user approval.
