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

# PROTOCOL: MBT Codegen

Version: 1.0
Status: active
Scope: schema options, generator code, generated artifacts

## Purpose

Define how `.proto + MBT options` become Rust schema/runtime/adaptor bindings.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/implementation_protocol.md`
- approved spec
- approved implementation plan

## Source Contract

The codegen source of truth is:

```text
.proto files + approved MBT custom options
```

Schemas may live:

- inside this repository;
- in a separate schema repository;
- inside an application crate.

The generator must receive explicit proto roots and root messages. It must not
discover schema intent from Rust code.

## Generated Artifact Rules

1. Generated files are not edited by hand.
2. Generated files must include a deterministic header.
3. Generated files must be reproducible by a checked command.
4. Generated files must not include surfaces absent from the spec.
5. Generated code must reject unsupported schema shapes before runtime.
6. Generated code must keep checked and trusted access distinct.
7. Generated code must not emit adapter logic unless the adapter is requested.
8. Generated code must not force unrelated schemas to compile.

## Compile-Surface Rules

Generated-code specs must measure or bound:

- generated source lines,
- macro-expanded lines when relevant,
- `cargo check` time for affected crates,
- release build time when runtime performance code is added,
- dependency graph changes.

Wide schemas require explicit compile-surface evidence.

## Option Rules

MBT options must be centralized and stable.

New options require a spec that defines:

- option name,
- protobuf extension target,
- allowed values,
- invalid combinations,
- codegen behavior,
- generated runtime behavior,
- test cases,
- migration or compatibility risk.

## Stop Gates

Stop if:

- codegen output cannot be regenerated;
- generated output requires manual edits;
- an option has ambiguous behavior;
- a schema shape compiles but is unsupported at runtime;
- codegen introduces schema-specific hardcoding not derived from proto;
- compile-surface impact is unknown for wide schemas.
