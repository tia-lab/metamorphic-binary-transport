# PROTOCOL: MBT Lifecycle

Version: 1.0
Status: active
Scope: entire repository

## Purpose

Define the mandatory lifecycle for MBT research, design, implementation,
validation, and review.

MBT is intended to become production infrastructure. No prototype status
permits unclear contracts, unbound dependencies, unverifiable benchmarks, or
generated-code shortcuts.

## Required Reads

Before any non-trivial task:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- this protocol
- task-specific protocol under `docs/protocols/`
- existing specs under `docs/specs/`
- existing reviews under `docs/reviews/`
- target code paths when implementation is requested

## Lifecycle

### Phase 0: Intake

Output:

- goal restatement,
- task class,
- required reads,
- required inputs,
- unknowns,
- stop/go risks.

No code is allowed.

### Phase 1: Context Read

Output:

- protocols read,
- docs read,
- source files read,
- external docs read when dependency behavior matters,
- observed constraints,
- hypotheses still open.

No code is allowed.

### Phase 2: Research Brief

Output path:

- `docs/reviews/[slug]/[slug]_research_brief.md`

No code is allowed.

### Phase 3: Spec

Output path:

- `docs/specs/[slug]_SPEC.md`

The spec must pass `spec_protocol.md`.

No code is allowed.

### Phase 4: Peer Audit

Output path:

- `docs/reviews/[slug]/[slug]_peer_audit.md`

The audit must classify exactly:

- `PEER_AUDIT_PASSED`
- `BLOCKED`

No code is allowed.

### Phase 5: Implementation Plan and Approval

Output path:

- `docs/reviews/[slug]/[slug]_implementation_plan.md`

The plan binds every code file, generated file, dependency change, test,
benchmark, artifact, and validation command.

No code is allowed until the plan is approved.

### Phase 6: Implementation

Code changes may start only after Phase 5 approval.

Implementation must stay within the approved spec and plan.

### Phase 7: Testing and Benchmarking

Testing follows `testing_benchmark_protocol.md`.

Correctness and determinism evidence are required before performance claims.

### Phase 8: Result Review

Output path:

- `docs/reviews/[slug]/[slug]_result_review.md`

The result review states:

- what was built,
- what was measured,
- what passed,
- what failed,
- what is proved,
- what remains unproved,
- whether the work should continue, be redesigned, or be promoted.

## Stop Gates

Stop if:

- required reads are incomplete,
- the measured object is unclear,
- schema or wire contract is unclear,
- generated-code ownership is unclear,
- spec is missing or unapproved,
- peer audit is missing or blocked,
- implementation plan is missing or unapproved,
- benchmark method cannot prove the intended claim,
- compile-surface impact is relevant but unmeasured.
