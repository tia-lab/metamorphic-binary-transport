# PROTOCOL: MBT Reviews and Documentation

Version: 1.0
Status: active
Scope: `docs/reviews/**`, architecture docs, README docs

## Purpose

Review documents are due-diligence artifacts. They preserve evidence,
limitations, and decisions so future work can continue without guessing.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- lifecycle protocol
- target spec
- target implementation plan
- test and benchmark artifacts when writing result reviews
- code paths when writing technical reviews

## Review Documents

Per slug:

- `docs/reviews/[slug]/[slug]_research_brief.md`
- `docs/reviews/[slug]/[slug]_peer_audit.md`
- `docs/reviews/[slug]/[slug]_implementation_plan.md`
- `docs/reviews/[slug]/[slug]_result_review.md`

Optional when useful:

- `[slug]_technical_review.md`
- `[slug]_benchmark_review.md`
- `[slug]_failure_review.md`
- `[slug]_compile_surface_review.md`

## Source-of-Truth Chain

1. Research brief
2. Spec
3. Peer audit
4. Implementation plan
5. Code
6. Tests and benchmarks
7. Result review

Later documents may not claim more than earlier evidence proves.

## Writing Rules

- Findings first when reviewing.
- Use impersonal due-diligence style.
- Separate proved facts from hypotheses.
- Include evidence limitations.
- Do not include conversation trace.
- Do not use promotional language.

## Result Review Minimum

The result review must include:

- slug,
- status,
- source artifacts,
- commands run,
- schema identity,
- dataset identity when relevant,
- correctness result,
- determinism result,
- compile-surface result when relevant,
- benchmark result when relevant,
- failures,
- interpretation limits,
- recommendation:
  - continue,
  - redesign,
  - abandon,
  - promote to product-spec discussion.
