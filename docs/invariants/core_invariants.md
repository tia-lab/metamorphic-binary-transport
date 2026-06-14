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

# MBT Core Invariants

Status: active
Scope: entire repository

These invariants are mandatory unless a later approved spec explicitly narrows
or supersedes one. Supersession must be local, justified, and audited.

## Transport Invariants

1. MBT is schema-driven. `.proto + MBT options` is the source of truth.
2. MBT core owns the envelope, schema identity, validation contract, and access
   contract.
3. Application schemas may live outside this repository.
4. Generated schema code must come only from approved codegen.
5. Generated files are not edited by hand.
6. MBT core does not own application finality, watermark, hole repair, or
   serving policy.
7. Row existence is never finality.
8. MBT payloads remain binary until an explicit boundary adapter is requested.
9. Projection, when present, is MBT-to-MBT before boundary conversion.
10. Compression is outside the MBT envelope unless a later spec proves and
    approves a different contract.

## Crate Boundary Invariants

1. MBT core must stay small.
2. Schema crates are separate from MBT core.
3. Adapter crates are separate from MBT core.
4. Codegen is a tool/build surface, not runtime.
5. Benchmarks and evidence harnesses are separate from production libraries.
6. Arrow, Parquet, prost, serde_json, and similar adapter dependencies do not
   enter MBT core unless an approved spec proves the need.
7. A service or SDK must compile only the schema and adapter crates it imports.
8. Features may be used for narrow optional behavior, but they are not the main
   isolation mechanism for schemas or adapters.

## Codegen Invariants

1. Codegen output must be deterministic.
2. Codegen input must include the exact proto file set and root message.
3. Codegen must emit stable schema IDs, schema versions, and schema hashes.
4. Codegen must reject ambiguous annotations.
5. Codegen must reject unsupported field combinations before runtime.
6. Codegen must not emit handwritten schema-specific branches.
7. Codegen must not emit all possible boundary surfaces for every schema by
   default.
8. Codegen must keep compile surface bounded and measured for wide schemas.
9. Generated APIs must make checked and trusted access distinct.
10. Unsafe generated APIs require a documented safety contract.

## Runtime Invariants

1. Checked access validates before archive access.
2. Trusted access is allowed only for immutable bytes that were validated before
   storage or transport handoff.
3. Trusted access must be visibly unsafe or otherwise impossible to call
   accidentally.
4. Hot paths must not allocate or copy unless the spec declares the copy point.
5. All row ordering must be deterministic.
6. Failure behavior must be explicit for corrupt bytes, wrong schema, old
   version, partial payloads, response caps, and missing payloads.
7. No `unwrap`, `expect`, or `panic!` is allowed in runtime or reusable support
   logic.

## Adapter Invariants

1. Adapters are boundary surfaces, not internal transport.
2. JSON, protobuf, CSV, Arrow, Arrow IPC, Parquet, and future adapters are
   opt-in crates or explicitly bound modules.
3. Adapter output must preserve semantic equivalence with the MBT schema.
4. Adapter correctness requires a checksum, field-for-field, or byte-for-byte
   oracle defined by spec.
5. Adapter performance claims require comparison against an identical logical
   payload.
6. Adapter code must not force unrelated adapters to compile.

## Benchmark Invariants

1. Performance claims require run evidence.
2. Compile-time claims require build evidence.
3. Correctness and determinism must pass before speed claims.
4. Baselines must use identical logical payloads.
5. Warm-cache and cold-cache results must be separated when storage is involved.
6. Failed and unstable runs are evidence and must not be hidden.
7. Tolerances must not be relaxed before proving correctness.
8. Benchmark setup work must be outside the measured loop unless the spec
   declares it part of the measured object.

## Documentation Invariants

1. Specs are the source of truth.
2. Peer audits are separate artifacts.
3. Implementation plans are separate artifacts.
4. Result reviews record evidence and limitations.
5. Documentation must separate proved facts from hypotheses.
6. Conversation trace is not documentation.
7. Public claims must not exceed recorded evidence.
8. A first spec draft must already close prior-spec conflicts, command
   surfaces, generated-artifact ownership, dispatch path bindings, exact code
   bindings, and test migration from old behavior to new behavior.
9. A peer audit must not be used as a substitute for author-side spec
   completeness. If basic protocol compliance is missing, the work restarts at
   spec authoring.
