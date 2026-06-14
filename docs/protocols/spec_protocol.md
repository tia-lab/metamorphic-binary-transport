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

# PROTOCOL: MBT Spec Authoring

Version: 1.0
Status: active
Scope: `docs/specs/*_SPEC.md`

## Purpose

The spec is the source of truth. Implementation may not contain a behavior,
dependency, file binding, generated artifact, benchmark, or interpretation not
present in the spec.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/research_protocol.md`
- this protocol
- target research brief
- relevant source code
- relevant dependency docs

## Entry Conditions

Spec authoring may start only when:

- slug is known,
- measured object is stated,
- candidate approach is known,
- major unknowns are listed,
- no code change is required to finish the spec.

## Mandatory Spec Location

- `docs/specs/[slug]_SPEC.md`

## Mandatory Sections

Use this section order:

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

## Pre-Audit Closure Gate

Before requesting or writing the first peer audit, the spec author must prove
the spec is internally closed. This gate is inviolable.

The first spec draft must explicitly verify:

- mandatory section order matches this protocol exactly;
- every prior approved spec that may conflict has been searched and either
  preserved, superseded locally, or cited as out of scope;
- every command surface is exact, including CLI flags, output paths, and
  ownership of check/write/inspect behavior;
- every generated artifact has one owner and one reproducibility command;
- no generated artifact is bound to two incompatible command surfaces;
- every runtime or codegen dispatch path that must change is listed in code
  bindings;
- every test that encodes old behavior is either preserved or explicitly
  migrated to the new behavior;
- every exact code path needed for the change is bound before the audit;
- no design decision is deferred to the implementation plan when it belongs in
  the spec;
- if the spec touches generated code, compile-surface evidence commands are
  defined before the audit.

The spec must include a short "pre-audit closure checklist" under the approval
checklist or a task-specific equivalent section. If this checklist is missing
or false, the spec must not proceed to peer audit.

## Required Code Bindings

The spec must bind exact paths for:

- implementation files,
- generated source files,
- codegen input files,
- test files,
- benchmark files,
- review artifacts,
- temporary data directories.

If a path is intentionally deferred, the spec must say why and name the
blocker.

## Generated-Code Requirements

A generated-code spec must define:

- proto source roots,
- imported options,
- root message,
- generated crate or module destination,
- deterministic formatting,
- generated API surface,
- unsupported schema combinations,
- schema hash behavior,
- compile-surface budget,
- codegen-check command.

## Zero-Copy Requirements

A zero-copy spec must define:

- archive type boundary,
- schema version,
- validation-before-access policy,
- allowed copy points,
- forbidden copy points,
- key byte layout if storage is involved,
- ordering properties,
- corrupt payload behavior,
- partial write behavior,
- old-version behavior,
- benchmark baseline.

## Approval Checklist

A spec is not implementation-ready until all are true:

- required reads complete,
- pre-audit closure gate satisfied before the first peer audit,
- measured object precise,
- correctness oracle defined,
- benchmark method defined,
- compile-surface budget defined when generated code is touched,
- code bindings exact,
- generated artifact bindings exact,
- test and review bindings exact,
- peer audit passed,
- implementation plan still required before code.

## Stop Gates

Stop if:

- pre-audit closure gate is incomplete,
- the spec depends on unverified library behavior,
- schema or archive semantics are unclear,
- correctness oracle is missing,
- benchmark does not isolate the intended measurement,
- compile-surface impact is ignored,
- code bindings are missing,
- peer audit has not passed.
