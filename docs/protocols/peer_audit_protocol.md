# PROTOCOL: MBT Peer Audit

Version: 1.0
Status: active
Scope: spec and result audits

## Purpose

The peer audit tries to falsify the work before code starts or before claims
are accepted.

Default mode is findings-first and no-edit.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- target research brief
- target spec
- relevant source code
- relevant dependency docs when dependency behavior is material

## Spec Audit Lenses

The audit must challenge:

- pre-audit closure gate completeness,
- measured object clarity,
- schema source ownership,
- wire/archive validation,
- trusted-access safety,
- codegen determinism,
- generated-code compile surface,
- crate boundary isolation,
- dependency containment,
- correctness oracle,
- benchmark isolation,
- performance budget,
- failure behavior,
- code binding completeness,
- generated artifact binding completeness,
- client or operator interpretation safety.

## Output

Write:

- `docs/reviews/[slug]/[slug]_peer_audit.md`

Classify exactly one:

- `PEER_AUDIT_PASSED`
- `BLOCKED`

## Stop Gates

Block the spec if:

- the pre-audit closure gate is missing or false,
- a core assumption is unproved,
- benchmark cannot prove the intended claim,
- correctness oracle is weak,
- schema semantics are ambiguous,
- codegen behavior is under-specified,
- compile-surface impact is ignored,
- code bindings are incomplete,
- dependency behavior is assumed but not verified.
