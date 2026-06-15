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

# Implementation Plan Peer Audit V2: MBT Derived UTC Metamorphose Codegen

Slug: `mbt_derived_utc_metamorphose_codegen`

Target plan:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_implementation_plan.md
```

Prior audit:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_implementation_plan_peer_audit.md
```

Classification: `PEER_AUDIT_PASSED`

## Required Reads

Completed:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/specs/mbt_derived_utc_metamorphose_codegen_SPEC.md`
- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_peer_audit_v2.md`
- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_implementation_plan.md`
- `docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_implementation_plan_peer_audit.md`
- `crates/codegen/src/model.rs`
- `crates/codegen/src/descriptor.rs`
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`
- `crates/benches/src/bin/mbt_bars_regression_bench.rs`
- `crates/schemas/bars_core/Cargo.toml`

## Findings

No blocking findings.

## Audit Evidence

### 1. Prior blocker on exact evidence and result-review artifacts is resolved

Evidence:

- The amended plan binds the derived-UTC result review and the Bars regression
  corrective result review:

```text
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_result_review.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

- The amended plan binds exact compile-surface evidence files:

```text
docs/evidence/mbt_derived_utc_metamorphose_codegen/codegen_source_wc.txt
docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_row_format_wc.txt
docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_json_protobuf_csv_check_time.txt
docs/evidence/mbt_derived_utc_metamorphose_codegen/bars_arrow_ipc_parquet_check_time.txt
```

- The amended validation commands pipe the required `wc -l` and
  `/usr/bin/time -v cargo check` outputs to those evidence files with `tee`.
- The amended expected outputs require the compile-surface evidence directory
  to exist.
- The amended rollback boundary includes:

```text
docs/evidence/mbt_derived_utc_metamorphose_codegen/
docs/evidence/mbt_bars_regression_benchmark/
docs/reviews/mbt_derived_utc_metamorphose_codegen/mbt_derived_utc_metamorphose_codegen_result_review.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

Assessment:

The implementation plan now satisfies the spec requirement to bind exact
evidence files and review artifacts before implementation.

### 2. Prior blocker on conditional duplicate protobuf failure coverage is resolved

Evidence:

- The amended fixture list now requires:

```text
invalid duplicate protobuf output tag inside one message model
```

- It also requires duplicate protobuf helper-stem coverage, unless descriptor
  construction proves helper-stem duplication is structurally impossible and
  binds that validation point in `test_descriptor.rs`.

Assessment:

The duplicate protobuf output-tag failure test is now unconditional. The
helper-stem path is either directly tested or explicitly proved impossible at
the descriptor validation point. That matches the prior audit's required
amendment.

### 3. Code and generated artifact boundaries remain acceptable

Evidence:

- Code edits remain bound to:

```text
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
crates/schemas/bars_core/tests/test_bars_shape.rs
```

- Conditional test files remain limited to:

```text
crates/schemas/bars_core/tests/test_bars_metamorphose.rs
crates/schemas/bars_core/tests/mod.rs
```

- Generated writes remain limited to:

```text
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
```

- The plan continues to forbid changes to `Cargo.toml`, `Cargo.lock`, MBT core,
  adapter crates, metamorphose crate, transponding crate, benchmark code, and
  generated core/columnar files.

Assessment:

The plan remains bounded to codegen, tests, and generated Bars row-format
adapter artifacts. It does not authorize unrelated architecture or benchmark
code work.

### 4. Protobuf message-tree behavior remains bound

Evidence:

- The plan requires protobuf emission to replace `model.fields` iteration with
  `model.protobuf_messages`.
- It requires child message helpers:

```text
encoded_len_<child_stem>
write_protobuf_<child_stem>
```

- It requires generated Bars protobuf to contain:

```text
writer.message_prefix(22
write_protobuf_metadata
writer.utc(6, row.ingested_at_ms.to_native())
```

Assessment:

The plan stays aligned with the passed spec audit: JSON and CSV are flattened,
protobuf follows the schema message tree.

## Non-Blocking Notes

### 1. Result reviews are now implementation outputs

The plan now binds result reviews as outputs. During implementation, those
reviews must record failures if validation or benchmark parity fails. They must
not convert failed evidence into a speed claim.

### 2. Compile-surface evidence commands write to repository files

The plan uses `tee` for compile-surface evidence. That is acceptable because
the evidence files are explicitly bound and rollback-scoped.

## Final Classification

`PEER_AUDIT_PASSED`

The implementation plan is ready for explicit implementation approval. No code
is authorized by this audit alone.
