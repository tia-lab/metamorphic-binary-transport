# MBT Projection Direct Writer Peer Audit

Status: complete
Classification: BLOCKED
Slug: `mbt_projection_direct_writer`
Audited spec: `docs/specs/mbt_projection_direct_writer_SPEC.md`
Research brief: `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_research_brief.md`

This audit does not authorize implementation.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_research_brief.md`
- `docs/specs/mbt_projection_direct_writer_SPEC.md`
- `crates/codegen/src/rust_emit.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/bench.rs`
- local `rkyv 0.8.16` source under Cargo registry for `ArchivedVec::serialize_from_iter`

## Findings

### 1. BLOCKER: spec does not follow mandatory MBT spec section order

Evidence:

- `docs/protocols/spec_protocol.md` requires sections 1 through 24 in this exact order, from `Identification` through `Open questions`.
- `docs/specs/mbt_projection_direct_writer_SPEC.md` currently has 17 custom sections, starting with `Goal` and ending with `Approval Status`.

Impact:

- This violates the pre-audit closure gate.
- Required contracts are present only partially or under non-standard headings, making implementation-plan review error-prone.

Required amendment:

- Rewrite the spec into the exact mandatory section order:
  `Identification`, `Status`, `Purpose`, `Non-goals`, `Measured object`, `Schema source contract`, `Wire and archive contract`, `Checked and trusted access contract`, `Codegen contract`, `Crate boundary contract`, `Dependency contract`, `Determinism contract`, `Failure contract`, `Compile-surface budget`, `Runtime performance budget`, `Correctness oracle`, `Benchmark methodology`, `Test plan`, `Code bindings`, `Generated artifact bindings`, `Review artifact bindings`, `Implementation plan requirement`, `Approval checklist`, `Open questions`.

### 2. BLOCKER: generated-code compile-surface budget is missing

Evidence:

- `docs/protocols/spec_protocol.md` requires compile-surface budget for generated-code work.
- The spec touches generated code and adds a Bars schema crate, but has no compile-surface budget section and no compile-surface evidence commands.
- `docs/invariants/core_invariants.md` requires generated APIs to keep compile surface bounded and measured for wide schemas.

Impact:

- The implementation could improve runtime projection while regressing compile time or wide-schema compile behavior.
- This repeats a known MBT risk: wide generated schemas can dominate compile surface.

Required amendment:

- Add exact build commands and acceptance budget for:
  - `metamorphic_binary_transport_core`
  - `metamorphic_binary_transport_codegen`
  - `test_compatibility_core`
  - new `bars_core`
  - new projection benchmark crate
- Include profile, command, output artifact path, and failure threshold.

### 3. BLOCKER: Bars schema equivalence is asserted but not provable

Evidence:

- The spec requires `crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto` to be derived from the old Bars proto and to remove only non-MBT cache/db annotations.
- The spec states that removing those annotations must not change the generated MBT physical field list.
- No command, fixture, generated inspection artifact, or field-list oracle is defined to prove that physical field list, field order, presence bits, dictionaries, and projection definitions match the old MBT shape.

Impact:

- The benchmark could compare against the old crate while silently using a different schema shape.
- That would invalidate the performance comparison and the correctness oracle.

Required amendment:

- Add a deterministic Bars schema equivalence oracle before benchmarks.
- Bind exact artifact paths for a generated field/projection inspection output from old and new schemas, or an equivalent codegen inspection command.
- The oracle must fail on field order, field kind, presence-bit, dictionary, key-order, projection-selection, schema-id, schema-version, transport-name, or schema-hash contract drift.

### 4. BLOCKER: direct-writer allocation contract is still ambiguous

Evidence:

- The spec requires `ArchivedVec::serialize_from_iter` for projected rows and forbids heap allocation whose only purpose is projected field values.
- Local `rkyv 0.8.16` exposes `ArchivedVec::serialize_from_iter`, but the spec does not classify rkyv internal resolver allocation or scratch storage as allowed, forbidden, or measured.
- The research brief still lists internal resolver cost as an unknown.

Impact:

- The implementation could satisfy text checks while still adding hidden per-row or per-field allocation through serializer mechanics.
- The hot-path zero-copy claim cannot be audited without an explicit allowed-allocation boundary.

Required amendment:

- State exactly which allocation points are allowed:
  - final `Vec<u8>` output buffer,
  - serializer scratch/resolver storage if unavoidable,
  - variable-width bytes copied only into final archive payload.
- State exactly which allocations are forbidden:
  - owned projected row vector,
  - owned string/bytes/array staging,
  - schema-independent dynamic field buffers.
- Add a measurement or inspection plan to detect unexpected hot-path allocations if the claim is zero projected-field staging.

### 5. BLOCKER: generated direct-writer API and type shape are under-specified

Evidence:

- The spec says the crate-private helper name may be implementation-specific.
- The generated borrowed reference types are named conceptually, but exact generated type names, trait impl boundaries, lifetime shape, and unsafe surface are not fully specified.
- The spec does not define the exact generated `Archive` and `Serialize` implementation structure needed for variable-width fields and nullable arrays.

Impact:

- The implementation plan could choose incompatible names or shapes while claiming compliance.
- Generated code may become harder to test by exact text checks and harder to compare across schema crates.

Required amendment:

- Define deterministic generated names for:
  - direct projection helper,
  - borrowed payload reference,
  - borrowed row collection reference,
  - borrowed row reference,
  - any resolver types if emitted.
- Define the exact generated trait implementation strategy for fixed-width fields, raw strings, bytes, arrays, optional fields, and nullable arrays.
- Define unsupported combinations explicitly if any MBT field kind cannot be emitted without owned staging.

### 6. BLOCKER: exact test and artifact bindings are incomplete

Evidence:

- The spec uses `crates/schemas/bars_core/tests/*` instead of exact test file paths.
- Temporary directories are not bound.
- Review artifacts are only partly bound; the peer audit and implementation plan artifact paths are not listed in a dedicated review artifact binding section.
- The spec does not bind exact generated-output negative-check test names or files.

Impact:

- The implementation plan could add tests in arbitrary locations or miss tests while still appearing to satisfy the wildcard.
- Reproducibility and auditability are weaker than required by protocol.

Required amendment:

- Replace wildcards with exact paths.
- Bind exact paths for:
  - each Bars schema test,
  - each compatibility schema projection test,
  - each generated-output negative-check test,
  - benchmark output directory,
  - temporary benchmark data directory if used,
  - peer audit,
  - implementation plan,
  - result review.

### 7. BLOCKER: benchmark stability rule is insufficiently defined

Evidence:

- The spec rejects direct writer if slower by more than 3.5 percent in a repeated stable run.
- It does not define what repeated means, how many runs are required, which statistic is compared, how instability is detected, or what machine/build state metadata is mandatory.
- `docs/invariants/core_invariants.md` requires failed and unstable runs to be recorded and benchmark setup work to be outside measured loops unless declared.

Impact:

- A later result could be accepted or rejected based on subjective interpretation of jitter.
- The comparison against old Bars rows may not isolate runtime projection if build/profile/setup differences are not fixed.

Required amendment:

- Define exact run count, comparison statistic, stability threshold, and metadata fields.
- Define whether the old-crate baseline is copied from the old result document or re-run in the same session.
- Define how to handle disagreement between old published rows and a fresh old-crate run.

### 8. BLOCKER: response-cap and envelope semantics need a direct-writer-specific oracle

Evidence:

- The spec says projected payload larger than `max_response_bytes` must fail.
- It does not define whether the cap applies to payload bytes, envelope plus payload bytes, or the same total length semantics as current `encode_owned`.
- The direct writer bypasses `ProjectedSchema::encode_owned`, so it must reproduce envelope sizing, checksum, schema header, and response-cap behavior exactly.

Impact:

- A direct writer can be byte-valid but semantically drift from owned encode behavior under response caps or checksum calculation.

Required amendment:

- Define response-cap semantics exactly.
- Add an oracle comparing direct-writer bytes with owned projected encode for small fixtures where owned projection is still allowed as a test-only reference.
- Add corrupt, partial, wrong-schema, and cap-boundary tests specific to direct-writer projection output.

## What Passed

- The measured object is correctly scoped as MBT-to-MBT projection, not boundary-format conversion.
- The spec correctly rejects generic runtime projection dispatch in the hot path.
- The spec correctly identifies the current generated owned-row projection path as unacceptable.
- The spec correctly binds old Bars projection lanes as the performance comparison shape.
- The spec correctly forbids handwritten generated artifacts.
- The spec correctly keeps adapter crates out of projection benchmarks.

## Required Next Step

Amend `docs/specs/mbt_projection_direct_writer_SPEC.md` before implementation planning.

The amendment must close all blockers above and then request a second peer audit:

```text
Approved: amend docs/specs/mbt_projection_direct_writer_SPEC.md per peer audit, then write docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit_v2.md
```
