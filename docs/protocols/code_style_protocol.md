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

# PROTOCOL: MBT Code Style

Version: 1.0
Status: active
Scope: Rust and support code

## Purpose

Define the minimum code discipline for MBT production and evidence code.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- approved spec
- approved implementation plan
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`

## Rules

1. Validation first.
2. Deterministic ordering.
3. Panic-free runtime and measurement logic.
4. Explicit error handling.
5. Explicit copy and allocation boundaries.
6. No hidden defaults.
7. No unbound configuration.
8. No inline benchmark claims.
9. No implementation behavior absent from spec.
10. Small modules and explicit types.
11. No public API by accident.
12. No benchmark code in production libraries.

## Rust Shape

Preferred order inside non-trivial files:

1. imports,
2. constants,
3. config types,
4. error/result types,
5. public entrypoints,
6. validation helpers,
7. runtime or codegen core,
8. artifact/report helpers,
9. tests only if the approved plan permits colocated unit tests.

## Error Handling

Runtime, codegen, and measurement logic must not use:

- `unwrap`,
- `expect`,
- `panic!`,
- `todo!`,
- `unreachable!`,
- `unwrap_or` to hide failure,
- `unwrap_or_default` to hide failure.

Use typed errors or explicit result propagation.

## Allocation Discipline

When work claims zero-copy or low-copy behavior:

- identify all owned buffers,
- identify all borrowed views,
- identify all archive validation steps,
- keep copy points explicit,
- test or measure allocation behavior when the spec requires it.

## Compile-Surface Discipline

Generated and adapter code must:

- avoid compiling unrelated schemas;
- avoid compiling unrelated adapters;
- avoid compiling benchmark support into production libraries;
- avoid large generated functions unless the spec proves they are required;
- measure compile impact when wide schemas are involved.
