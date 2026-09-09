# PROTOCOL: MBT Testing and Benchmarking

Version: 1.0
Status: active
Scope: tests, benches, and evidence artifacts

## Purpose

Compilation is not proof. A benchmark number is not proof unless correctness,
determinism, and measurement boundaries are already established.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- approved spec
- approved implementation plan
- implemented code paths

## Entry Conditions

Testing may start only when:

- implementation is complete,
- pre-test audit is complete,
- test paths are bound,
- benchmark paths are bound,
- correctness oracle is implemented or ready to implement,
- artifact paths are bound.

## Test Categories

Every code change must cover relevant categories:

- Category A: contract and failure behavior.
- Category B: deterministic replay.
- Category C: correctness oracle.
- Category D: edge and corrupt input.
- Category E: generated-code reproducibility.
- Category F: storage/query behavior when storage is involved.
- Category G: copy/allocation behavior when claimed.
- Category H: compile-surface behavior when generated code is touched.
- Category I: runtime benchmark execution.

## Benchmark Requirements

Benchmark artifacts must record:

- UTC timestamp,
- operator,
- git commit or dirty state,
- command,
- build profile,
- CPU,
- RAM,
- OS/kernel,
- Rust toolchain,
- input dataset identity,
- row count and payload size,
- warm/cold cache mode when relevant,
- result summary,
- raw output path.

## Compile Benchmark Requirements

Compile-surface artifacts must record:

- command,
- profile,
- clean/dirty state,
- target crate,
- enabled features,
- dependency graph condition,
- wall time,
- user/system CPU time when available,
- max RSS when available,
- generated source size,
- macro-expanded size when relevant.

## Performance Claim Rules

- Report median, tail, and throughput when the spec asks for latency.
- Separate encode, access, validation, projection, adapter conversion, and
  storage phases when relevant.
- Do not compare two systems unless logical payload and correctness oracle are
  identical.
- Do not hide failed or unstable runs.

## Failure Rule

When a test or benchmark fails, assume first that the implementation or
measurement design is wrong.

Do not change expected values, tolerances, or benchmark scope until independent
evidence proves the original expectation was wrong.

## Exit Conditions

Testing is complete only when:

- mandatory categories pass or blocked categories are explicitly justified,
- benchmark artifacts exist when performance is claimed,
- correctness oracle passes,
- determinism is demonstrated,
- result review is ready to write.
