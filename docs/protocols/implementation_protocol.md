# PROTOCOL: MBT Implementation

Version: 1.0
Status: active
Scope: code, generated artifacts, and build files

## Purpose

Control implementation after a spec and implementation plan are approved.

## Locked Refresh Rule

Before implementation, reread:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/codegen_protocol.md` when generated code is touched
- `docs/protocols/testing_benchmark_protocol.md`
- approved spec
- approved implementation plan

Memory of prior work is not evidence.

## Entry Conditions

Implementation may start only when:

- spec exists and is approved,
- peer audit passed,
- implementation plan exists and is approved,
- exact files are bound,
- generated artifacts are bound,
- dependency changes are bound,
- tests and benchmarks are bound,
- failure contract is bound,
- correctness oracle is bound.

## Implementation Rules

- Implement only what the spec and plan bind.
- Prefer small, explicit modules.
- Keep MBT core small.
- Keep schemas, adapters, codegen, and benches separated by crate or explicit
  module boundary.
- Validate before compute, archive, or storage access.
- Keep hot paths allocation-conscious.
- Do not use `unwrap`, `expect`, or `panic!` in runtime, codegen, measurement,
  or reusable support logic.
- Do not add hidden defaulting.
- Do not add unbound configuration.
- Do not add unbound dependencies.
- Do not widen the measured object.
- Do not create benchmark shortcuts that are not part of the measured system.

## Pre-Test Audit

Before testing, audit final code against:

- approved spec,
- approved implementation plan,
- `code_style_protocol.md`,
- `codegen_protocol.md` when relevant,
- expected test and benchmark bindings.

The audit must confirm:

- code path matches binding,
- generated files match codegen output,
- dependency changes match binding,
- failure behavior matches spec,
- correctness oracle can be tested,
- benchmark code measures the intended object,
- no unapproved behavior was added.

## Stop Gates

Stop implementation if:

- code requires behavior not in spec,
- dependency API contradicts spec assumptions,
- correctness oracle cannot be implemented,
- benchmark would measure a different object,
- hidden allocation or copy behavior cannot be bounded,
- code cannot stay within approved file bindings,
- generated output requires manual editing.
