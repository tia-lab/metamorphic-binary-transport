# MBT Projection Direct Writer Peer Audit v2

Status: complete
Classification: PEER_AUDIT_PASSED
Slug: `mbt_projection_direct_writer`
Audited spec: `docs/specs/mbt_projection_direct_writer_SPEC.md`
Prior audit: `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit.md`

This audit does not authorize implementation. It authorizes writing an
implementation plan.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_research_brief.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit.md`
- `docs/specs/mbt_projection_direct_writer_SPEC.md`
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/config.rs`
- `crates/codegen/src/emit.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`
- `/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/binary_transport/v1/bars.proto`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/bench.rs`
- local `rkyv 0.8.16` source for `ArchivedVec::serialize_from_iter`

## Audit Result

The amended spec passes peer audit.

The prior blockers are closed:

- mandatory MBT spec section order is now used;
- generated-code compile-surface budget is defined;
- Bars schema equivalence is bound through an exact inspect fixture;
- rkyv resolver scratch is explicitly classified as serializer scratch and must
  be measured;
- generated helper/type naming is deterministic;
- exact test, artifact, review, benchmark, and temporary paths are bound;
- benchmark run stability and 3.5 percent jitter policy are defined;
- response-cap and envelope semantics are tied to existing `encode_owned`
  behavior;
- current owned-row projection baseline capture is bound before replacement.

## Findings

No blocking findings.

### Non-blocking risk: direct writer may not beat old path

Evidence:

- The spec correctly records that `ArchivedVec::serialize_from_iter` uses
  rkyv resolver scratch.
- The spec requires benchmark rejection if archived direct projection is slower
  than both old-crate projection and current owned-row projection by more than
  3.5 percent.

Risk:

- The direct writer design is implementation-ready, but performance superiority
  is not proved until the required benchmark evidence exists.

Required handling:

- The implementation plan must preserve the runtime-performance rejection rule
  and must not weaken it after tests pass.

### Non-blocking risk: Bars fixture must be reviewed carefully

Evidence:

- The spec binds
  `crates/schemas/bars_core/tests/expected_bars_projection_inspect.txt` as the
  Bars physical-shape oracle.

Risk:

- A wrong fixture would make the test deterministic but not meaningful.

Required handling:

- The implementation plan must describe how the fixture is derived from the old
  Bars proto and old generated field shape.

## Implementation Plan Requirements

The next implementation plan must bind:

- exact ordered edits;
- current owned-row projection baseline capture before replacing the generator;
- creation of `crates/schemas/bars_core`;
- generation and check commands for both schema crates;
- direct-writer codegen edits;
- generated-output negative checks scoped to direct helper bodies;
- compile-surface evidence commands;
- three-run projection benchmark command;
- rollback boundary that removes Bars schema crate and bench additions if the
  direct writer fails correctness or performance gates.

## Next Step

Write the implementation plan:

```text
Approved: write docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan.md
```
