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

# Research Brief: MBT Runtime Archive Parity Corrective

Slug: `mbt_runtime_archive_parity_corrective`

Status: `COMPLETE_IMPLEMENTATION_AWAITING_APPROVAL`

## Goal

Identify the concrete runtime/archive causes behind the new workspace Bars
regression benchmark being slower than the old MBT implementation before
making any code change.

This brief does not authorize implementation.

## Required Reads Completed

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`
- `docs/specs/mbt_bars_regression_benchmark_SPEC.md`
- `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md`

## Measured Object

The measured object is the generated MBT archive layout and generated trusted
access path used by:

- Bars full MBT encode and inspect;
- trusted metamorphose adapters that call generated trusted access;
- Bars regression benchmark comparison against the old MBT implementation.

## Code-Read Evidence

| Evidence | Observation |
| --- | --- |
| `/home/tia/_DEV/MATHILDE/experiments/Cargo.toml:24` | Old experiments workspace uses `rkyv = { version = "0.8.16", features = ["unaligned"] }`. |
| `crates/schemas/bars_core/Cargo.toml:18` | New Bars schema crate uses `rkyv = "=0.8.16"` without `unaligned`. |
| `crates/schemas/test_compatibility_core/Cargo.toml:18` | New compatibility schema crate uses `rkyv = "=0.8.16"` without `unaligned`. |
| `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/rust_emit.rs:1409-1416` | Old generated trusted access takes `trusted_payload_for_schema` then calls `rkyv::access_unchecked` without archived payload validation. |
| `crates/codegen/src/rust_emit.rs:2527-2542` | New generated trusted access decodes the header and calls `validate_archived_payload` before returning the unchecked archive view. |
| `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md` | Current benchmark run completed, but old-vs-new parity was not proved. |

## Run Evidence

The existing result review recorded:

- new run file:
  `docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_7.json`;
- old run file used for manual comparison:
  `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_3.json`.

Observed 100k full MBT checked lane:

| Implementation | Output bytes | Elapsed |
| --- | ---: | ---: |
| old MBT parity run | `32,400,138` | `155.915458 ms` |
| new split MBT run | `33,600,140` | `254.36049 ms` |

The new output is about `1,200,002` bytes larger at `100,000` rows, matching
about `12` extra bytes per row plus envelope difference. This is consistent
with aligned archive layout replacing the old unaligned archive layout.

## Candidate Approach

1. Restore workspace-wide schema crate `rkyv` feature parity with the old MBT:
   `rkyv = { version = "=0.8.16", features = ["unaligned"] }`.
2. Restore generated trusted access parity with the old MBT:
   trusted access checks the MBT header/schema boundary and then returns the
   unchecked archived payload view without archived payload validation.
3. Regenerate affected generated core schema files through codegen.
4. Rerun narrow schema checks and Bars regression benchmark evidence.

## Non-Candidate Approaches

- Do not add new benchmark-only shortcuts.
- Do not hand-edit generated schema files.
- Do not change MBT wire format.
- Do not change checked access validation.
- Do not change metamorphose adapter logic unless regeneration proves it is
  affected.

## Risks

- `rkyv` feature changes affect archive layout and must be validated for every
  schema crate in the workspace.
- Restoring old trusted access semantics is correct only for bytes that were
  previously checked and then stored or transported without mutation.
- Existing old-vs-new semantic checksum rows are not a valid parity oracle
  until the result review states exactly which checksum fields are comparable.

## Evidence Required Before Claims

- `cargo tree` evidence that schema crates resolve `rkyv` with `unaligned`.
- Generated-file check evidence after regeneration.
- Code-read or grep evidence that generated trusted access no longer calls
  `validate_archived_payload`.
- Bars benchmark rerun evidence after the fixes.
- Result review stating whether performance parity is proved, failed, or still
  unproved.
