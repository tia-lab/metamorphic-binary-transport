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

# Peer Audit: MBT Core Runtime Migration

Audit classification: `BLOCKED`

Spec audited:

```text
docs/specs/mbt_core_runtime_migration_SPEC.md
```

Research brief audited:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_research_brief.md
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
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/envelope.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/runtime.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/error.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs`

## Findings

### F1. Validation forbids a required core constant

Severity: blocking.

The spec requires this core constant:

```rust
pub const ENCODING_MBT_RKYV: u16 = 1;
```

The same spec requires this validation command:

```bash
rg -n "serde|serde_json|prost|arrow|parquet|zstd|rkyv|generated|metamorphose|transponding|benches" crates/metamorphic_binary_transport_core/src crates/metamorphic_binary_transport_core/Cargo.toml
```

This is contradictory. A correct implementation containing
`ENCODING_MBT_RKYV` will match the forbidden literal `rkyv`, causing the
validation command to fail. Removing or renaming the constant only to satisfy
the grep would weaken the spec because the experiment source explicitly uses
the encoding kind as the rkyv archive encoding marker:

```text
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/envelope.rs
```

Observed source evidence:

- `ENCODING_MATHILDE_RKYV: u16 = 1`
- `TransportHeader::new_with_schema` writes that encoding kind.
- checked and trusted validation reject any other encoding kind.

Required amendment:

- Keep the encoding kind explicit.
- Replace the literal forbidden-source scan with dependency/import/module
  checks that forbid compiling or importing rkyv, not mentioning the wire
  encoding name.

Acceptable validation shape:

```bash
cargo tree -p metamorphic_binary_transport_core
rg -n "rkyv::|use .*rkyv|extern crate rkyv|serde::|serde_json::|prost::|arrow_|parquet::|zstd::" crates/metamorphic_binary_transport_core/src crates/metamorphic_binary_transport_core/Cargo.toml
rg -n "pub mod (generated|metamorphose|transponding|benches)|mod (generated|metamorphose|transponding|benches)" crates/metamorphic_binary_transport_core/src
```

The exact amended commands should be bound in the spec and then repeated in
the implementation plan.

## Non-Blocking Notes

### N1. Dependency placement must be pinned in the implementation plan

The spec allows `thiserror = "2.0.17"` either directly in the core crate or in
workspace dependencies. This is acceptable only because the implementation
plan is required to bind the exact dependency declaration before code changes.

### N2. Archive error variants are acceptable in core if owned by generated schemas

The allowed error set includes archive and row-shape variants even though this
slice does not add rkyv validation to core. That is acceptable because future
generated schema crates need a stable core error surface. The implementation
must not add rkyv dependency or archive access to core under this spec.

## Passed Audit Lenses

The following audit lenses passed after code-read comparison against the
experiment crate:

- measured object is clear and limited to `codec`, `envelope`, `error`, and
  `runtime`;
- schema-specific experiment helpers are explicitly forbidden from core;
- generated artifacts are explicitly forbidden;
- adapter surfaces are explicitly out of scope;
- checked and trusted header validation are separated;
- full performance claims are deferred until generated schema migration;
- benchmark methodology is correctly excluded from this core-only slice.

## Decision

The spec is blocked until finding F1 is amended and re-audited.

No implementation plan should be written from the current spec.
