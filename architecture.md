# MBT Architecture

This document explains the repository architecture. It is not benchmark
evidence. Runtime, compile-time, projection, and adapter claims remain bound to
the result reviews and evidence artifacts that measured them.

## Core Architecture

```text
+---------------------- CORE ARCHITECTURE ----------------------+
|                                                               |
|  .proto + proto/mathilde/options.proto                        |
|              |                                                |
|              v                                                |
|  crates/codegen                                               |
|              |                                                |
|              v                                                |
|  generated schema crate                                       |
|              |                                                |
|              v                                                |
|  crates/core: envelope + checksum + response caps             |
|              |                                                |
|              v                                                |
|  rkyv archive payload inside MBT bytes                         |
|              |                                                |
|              +--> checked access                              |
|              |       validates before archive access           |
|              |                                                |
|              +--> trusted access                              |
|                      caller proves prior validation boundary   |
|                                                               |
+---------------------------------------------------------------+
```

Core owns the envelope, schema identity, checked validation boundary, trusted
payload boundary, response caps, and runtime traits.

Core does not own application finality, hole repair, cache policy, database
policy, or adapter output semantics.

## Projection Architecture

```text
+------------------- PROJECTION ARCHITECTURE -------------------+
|                                                               |
|  full MBT bytes                                               |
|       |                                                       |
|       v                                                       |
|  checked or trusted source access                             |
|       |                                                       |
|       v                                                       |
|  generated direct projection writer                           |
|       |                                                       |
|       v                                                       |
|  projected MBT bytes                                          |
|       |                                                       |
|       v                                                       |
|  projected schema marker                                      |
|       |                                                       |
|       v                                                       |
|  optional later metamorphose boundary                         |
|                                                               |
+---------------------------------------------------------------+
```

Projection is MBT-to-MBT. It happens before JSON, protobuf, CSV, Arrow, Arrow
IPC, or Parquet conversion.

Projected bytes use distinct schema identity. The wrong reader must reject the
wrong payload shape.

The current hot projection behavior is generated schema code. The
`crates/projection` crate is not the production projection hot path unless a
later approved spec changes that.

## Metamorphose Architecture

```text
+------------------ METAMORPHOSE ARCHITECTURE ------------------+
|                                                               |
|  MBT bytes                                                    |
|       |                                                       |
|       v                                                       |
|  generated schema checked or trusted access                   |
|       |                                                       |
|       v                                                       |
|  generated schema-local writer                                |
|       |                                                       |
|       v                                                       |
|  selected adapter crate                                       |
|       |                                                       |
|       v                                                       |
|  JSON / protobuf / CSV / Arrow / Arrow IPC / Parquet output   |
|                                                               |
+---------------------------------------------------------------+
```

Metamorphose is boundary conversion. MBT remains the canonical internal
payload, and adapters write requested boundary formats from validated MBT bytes
or trusted archived views.

Adapters are selected surfaces. Importing core does not compile every adapter.

## Transponding Architecture

```text
+------------------- TRANSPONDING ARCHITECTURE -----------------+
|                                                               |
|  validated archived rows                                      |
|       |                                                       |
|       v                                                       |
|  generated row-to-column kernel                               |
|       |                                                       |
|       v                                                       |
|  physical column buffers                                      |
|       |                                                       |
|       v                                                       |
|  columnar adapter                                             |
|       |                                                       |
|       v                                                       |
|  Arrow / Arrow IPC / Parquet output                           |
|                                                               |
+---------------------------------------------------------------+
```

Transponding is the hidden row-to-column mechanism for columnar metamorphose
paths. It is not the stable public user API in the current repository status.

The public caller asks for a boundary format through metamorphose. Generated
schema code selects transponding internally when the selected format needs
columnar data.

## Crate Boundary Summary

Current workspace boundary:

```text
crates/core
  envelope, response caps, error surface, runtime traits

crates/codegen
  descriptor reading, MBT option validation, Rust generation

crates/schemas/*
  generated schema crates and schema-specific marker APIs

crates/metamorphose
  boundary dispatch traits and shared adapter-facing contracts

crates/transponding
  row-to-column contracts for columnar paths

crates/adapters/*
  selected boundary writers for JSON, protobuf, CSV, Arrow, Arrow IPC, Parquet

crates/benches
  measurement harnesses and evidence production only
```

Compile-surface rule:

```text
core != schemas
schemas != adapters
adapters != benches
codegen != runtime
```

No service or SDK should compile unrelated schemas, unrelated adapters, or
benchmark support.

## Evidence Boundaries

Architecture diagrams explain intended data movement. They are not benchmark
results.

Evidence remains in result reviews and evidence artifacts:

- workspace split and dependency containment: workspace architecture result;
- core runtime correctness: core runtime result;
- schema generation correctness: schema core generation result;
- projection direct writer correctness and measured lanes: projection result;
- Bars runtime/archive parity and adapter caveats: Bars regression corrective
  result.

Do not infer source-data finality, adapter parity, compile-time improvement, or
production serving behavior from this architecture document alone.
