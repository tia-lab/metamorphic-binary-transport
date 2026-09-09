# MBT Repository Structure

Status: initial architecture lock

## Purpose

This document records the intended structure for the production MBT repository.
It is an architecture guardrail, not an implementation plan.

## Target Workspace Shape

```text
crates/core
  envelope, errors, schema metadata, checked access, trusted access

crates/codegen
  proto descriptor reader, option validation, Rust generation

crates/projection
  MBT-to-MBT projection helpers and generated projection contracts

crates/metamorphose
  boundary conversion traits and shared checked-output helpers only

crates/transponding
  row-to-columnar transformation contracts

crates/adapters/json
  JSON boundary adapter

crates/adapters/protobuf
  protobuf boundary adapter and prost DTO output when requested

crates/adapters/csv
  CSV boundary adapter

crates/adapters/arrow
  Arrow boundary adapter

crates/adapters/arrow_ipc
  Arrow IPC boundary adapter

crates/adapters/parquet
  Parquet boundary adapter

crates/benches
  benchmark and evidence harnesses only
```

The repository is already named `metamorphic-binary-transport`, so folder
names are short. Cargo package names remain globally explicit, for example
`mbt_core` for `crates/core`.

Generated schema crates are not part of the initial skeleton. When a shared
schema crate is approved, the reserved workspace pattern is:

```text
crates/schemas/[schema_name]
```

## Schema Ownership

Schemas are `.proto` files and may live outside this repository.

Allowed ownership shapes:

```text
external schema repo
  proto/mbt/options.proto
  proto/aggregator/*.proto
  proto/primitives/*.proto

application crate
  proto/*.proto
  generated schema module

shared schema crate
  generated Rust schema binding from an approved proto
```

MBT owns the transport options and generator contract. Applications own their
domain schemas.

MBT option ownership path:

```text
proto/mbt/options.proto
```

The options file path is repository-owned. Full extension definitions are added
only by an approved codegen/options spec.

Concrete boundary writers live in adapter crates, not in
`crates/metamorphose`.

## Compile-Surface Rule

No crate should compile unrelated surfaces.

Examples:

- MDB-style database code should depend on core and selected schema crates, not
  JSON, Arrow, Parquet, or benchmark crates.
- MBT-only serving should not compile boundary adapters.
- JSON serving should not compile Arrow or Parquet.
- A telemetry-only binary should not compile unrelated application schemas.
- Codegen should not compile the full runtime/generated schema graph just to
  inspect or generate schemas.

## Runtime Rule

Runtime movement is MBT-first:

```text
source row
  -> checked MBT encode
  -> MBT storage/transport/cache
  -> trusted access only after validation boundary
  -> optional adapter at consumer boundary
```

Adapters do not define the internal payload. They write a requested boundary
format from validated MBT bytes or archived views.

## Non-goals

- No monolithic crate that compiles every schema and every adapter.
- No benchmark support inside production library crates.
- No generated code edited by hand.
- No schema-specific handwritten branches in generic infrastructure.
- No performance or compile-time claim without recorded evidence.
