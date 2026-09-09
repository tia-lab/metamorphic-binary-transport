# Peer Audit: MBT Projection Migration

Status: `BLOCKED`

Slug: `mbt_projection_migration`

Target research brief:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_research_brief.md
```

Target spec:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

## Required Reads

Completed reads:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/lifecycle_protocol.md
docs/protocols/spec_protocol.md
docs/protocols/peer_audit_protocol.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_research_brief.md
docs/specs/mbt_projection_migration_SPEC.md
proto/mathilde/options.proto
crates/codegen/src/options.rs
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

## Findings

### 1. Mandatory Spec Section Contract Is Not Followed

Severity: blocking

`docs/protocols/spec_protocol.md` requires this exact section order:

```text
1. Identification
2. Status
3. Purpose
4. Non-goals
5. Measured object
6. Schema source contract
7. Wire and archive contract
8. Checked and trusted access contract
9. Codegen contract
10. Crate boundary contract
11. Dependency contract
12. Determinism contract
13. Failure contract
14. Compile-surface budget
15. Runtime performance budget
16. Correctness oracle
17. Benchmark methodology
18. Test plan
19. Code bindings
20. Generated artifact bindings
21. Review artifact bindings
22. Implementation plan requirement
23. Approval checklist
24. Open questions
```

The current spec has useful content, but it uses different headings and combines
multiple required contracts:

```text
Key And Time-Grid Contract
Projection Selection Contract
Schema And Archive Contract
Presence Contract
Generated API Contract
Storage And Dependency Contract
Performance Budget
Correctness Proof Plan
Exact Code Bindings
Exact Artifact Bindings
Validation Commands
Approval Status
```

This is not only formatting. For generated-code work, the required sections
force separate treatment of wire/archive, checked/trusted access, codegen,
crate boundaries, dependency policy, compile surface, runtime performance,
correctness oracle, test plan, code bindings, generated artifacts, review
artifacts, implementation-plan requirements, and open questions.

Required amendment:

- rewrite the spec into the mandatory section order;
- preserve the useful existing projection content under the correct sections;
- add explicit sections for any mandatory heading currently represented only
  indirectly.

### 2. Codegen-Check Command Is Deferred Instead Of Defined By The Spec

Severity: blocking

`docs/protocols/spec_protocol.md` requires generated-code specs to define the
codegen-check command.

The current spec states:

```text
The implementation plan must also include the exact codegen reproducibility
commands for `test_compatibility_core`.
```

That defers a generated-code requirement from the spec to the implementation
plan. The implementation plan may bind exact execution order and expected
outputs, but the spec must already define the command shape and inputs.

Required amendment:

- add the exact `mbt_codegen --check` command for the compatibility schema;
- include the exact `--proto-root`, `--schema`, `--root`, `--module`,
  `--surface core`, and output path arguments used by the schema crate;
- keep the implementation plan responsible only for execution and result
  recording.

### 3. Projected Schema Hash Behavior Is Too Loose For Codegen Determinism

Severity: blocking

The spec currently says:

```text
schema_hash = hash(source normalized schema + projection name + selected physical fields)
```

This is directionally correct but not deterministic enough for implementation.
`docs/invariants/core_invariants.md` requires stable schema hashes, and
`docs/protocols/peer_audit_protocol.md` blocks codegen behavior that is
under-specified.

The current rule does not define:

- whether source projection declarations are excluded from the source hash and
  included only in projected hashes;
- the exact projected hash input order;
- whether `rust_marker`, `transport_name`, selected field logical paths,
  proto paths, field kinds, dictionary names, key order, and projected presence
  remapping are included;
- whether unselected fields affect projected hash;
- whether dictionary value lists for selected dictionary fields are included;
- how nullable array presence participates in projected hash;
- whether schema ID/version are included directly or through the source
  normalized schema.

Required amendment:

- define a stable projected schema-hash input list and ordering;
- state which source-level fields are excluded from the source hash so adding
  projection declarations cannot alter canonical source bytes;
- state which projection-level fields are included in each projected hash;
- state that changing selected field set, selected field kind, dictionary value
  list, projected presence layout, projection name, or projected marker changes
  the projected hash.

### 4. Compile-Surface Budget Is Present But Not Measurable Enough

Severity: blocking

The spec says generated code must avoid unnecessary derives and emit only
declared projections, but it does not define a measurable compile-surface
budget.

For generated-code work, `docs/invariants/core_invariants.md` requires
compile-time claims to have build evidence and codegen to keep compile surface
bounded and measured for wide schemas.

This spec does not claim compile-time improvement, but it does add generated
schema code for each declared projection. The implementation plan needs a
measurable bound that can be checked.

Required amendment:

- add a compile-surface budget section with required evidence:
  - generated line count before and after projection generation;
  - schema crate `cargo check` command under `/usr/bin/time -v`;
  - dependency tree check proving no adapter crates enter the schema crate;
  - statement that no compile-time improvement is claimed unless measured.

### 5. Test Plan And Correctness Oracle Need Separation

Severity: blocking

The current spec combines correctness proof and tests under:

```text
Correctness Proof Plan
```

The mandatory spec protocol requires separate:

```text
Correctness oracle
Test plan
```

The existing content is mostly adequate, but the oracle must identify the
comparison mechanism, not only list test cases. For this phase the oracle
should state:

- source rows are the semantic reference for selected projected fields;
- projected bytes must validate through the projected marker;
- wrong-marker rejection proves schema-hash isolation;
- checked and trusted projection equality is byte-for-byte;
- field comparisons are explicit and do not require generated `PartialEq`.

Required amendment:

- split the current proof plan into a correctness oracle and test plan;
- keep explicit field-by-field comparison requirements;
- bind null versus present-empty nullable array comparisons as part of the
  oracle.

## Audit Lens Results

| Lens                                    | Result                                                                |
| --------------------------------------- | --------------------------------------------------------------------- |
| Measured object clarity                 | Passed                                                                |
| Schema source ownership                 | Passed                                                                |
| Wire/archive validation                 | Blocked by missing mandatory section split                            |
| Trusted-access safety                   | Blocked by missing mandatory section split                            |
| Codegen determinism                     | Blocked by projected schema-hash under-specification                  |
| Generated-code compile surface          | Blocked by insufficient measurable compile-surface budget             |
| Crate boundary isolation                | Passed directionally; must be preserved under mandatory section order |
| Dependency containment                  | Passed directionally; must be preserved under mandatory section order |
| Correctness oracle                      | Blocked by oracle/test-plan combination                               |
| Benchmark isolation                     | Passed because no runtime benchmark is required for this spec         |
| Performance budget                      | Passed directionally; must be split into compile and runtime sections |
| Failure behavior                        | Passed directionally                                                  |
| Code binding completeness               | Passed directionally                                                  |
| Generated artifact binding completeness | Passed directionally                                                  |
| Client/operator interpretation safety   | Passed                                                                |

## Non-Blocking Observations

1. The decision to keep `proto/mathilde/options.proto` unchanged is correct
   based on code-read evidence: the option file already defines
   `ProjectionDefinition`, `mathilde.projection`, and
   `mathilde.projection_group`.
2. The decision to avoid `crates/projection` runtime dependency is acceptable
   for this phase. Code-read evidence shows generated schema modules already
   own checked and trusted archived accessors needed for projection.
3. The compatibility fixture is a good first fixture because it already has two
   projection declarations and covers current scalar, bytes, raw string,
   dictionary, bitmask, numeric array, and nullable array kinds.

## Required Amendment Summary

Amend:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Required changes:

1. Reorder and rename sections to match `docs/protocols/spec_protocol.md`.
2. Define the exact codegen-check command in the spec.
3. Define exact projected schema-hash inputs and ordering.
4. Add measurable compile-surface budget evidence.
5. Split correctness oracle from test plan.

No code implementation is authorized by this audit.

## Classification

```text
BLOCKED
```
