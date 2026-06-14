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

# SPEC: MBT Workspace Architecture

## 1. Identification

Slug: `mbt_workspace_architecture`

Repository: `/home/tia/_DEV/MATHILDE/metamorphic-binary-transport`

Task class: spec authoring.

Research brief:

```text
docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_research_brief.md
```

Architecture extraction analysis:

```text
docs/reviews/mbt_workspace_architecture/mbt_experiment_architecture_extraction_analysis.md
```

## 2. Status

Status: `DRAFT_AWAITING_PEER_AUDIT_V4`

This spec does not authorize implementation.

The previous three-crate skeleton contract is superseded by this amendment.
The earlier verbose crate folders are rejected:

```text
crates/metamorphic_binary_transport_core
crates/metamorphic_binary_transport_codegen
crates/metamorphic_binary_transport_benches
```

Any existing spec, implementation plan, code, or generated artifact that still
targets those folders is stale and must be amended before implementation
continues.

Known stale downstream bindings at the time of this amendment:

```text
docs/specs/mbt_core_runtime_migration_SPEC.md
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_implementation_plan.md
docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_implementation_plan.md
```

Implementation may start only after:

1. this amended spec passes peer audit;
2. all dependent implementation plans are amended to this canonical structure;
3. the amended implementation plan is explicitly approved.

## 3. Purpose

Define the full production workspace skeleton up front so the repository cannot
drift into a partial split or recreate a monolithic compile surface.

The repository is already named `metamorphic-binary-transport`; therefore
workspace folder names are short and architectural:

```text
core
codegen
projection
metamorphose
transponding
adapters/*
benches
```

Cargo package names remain globally explicit Rust package names so dependency
trees are unambiguous:

```text
metamorphic_binary_transport_core
metamorphic_binary_transport_codegen
metamorphic_binary_transport_projection
metamorphic_binary_transport_metamorphose
metamorphic_binary_transport_transponding
metamorphic_binary_transport_adapter_*
metamorphic_binary_transport_benches
```

This is a repository-shape and crate-boundary spec. It preserves the existing
experiment MBT architecture while splitting compile surfaces. It does not
redesign envelope semantics, checked access, trusted access, projection,
metamorphose, or transponding.

## 4. Non-goals

This spec does not:

- implement runtime logic;
- implement codegen logic;
- implement full protobuf option definitions;
- generate schema bindings;
- implement JSON, protobuf, CSV, Arrow, Arrow IPC, or Parquet conversion logic;
- implement benchmark fixtures;
- implement schema-specific code;
- make runtime performance claims;
- make compile-time improvement claims.

Creating empty crate/file placeholders is allowed only after an approved
implementation plan. Placeholder crates must not contain transport logic,
adapter logic, generated code, or benchmark logic.

## 5. Measured Object

The measured object for this architecture phase is workspace separation:

```text
Cargo workspace
  -> core compiles without schemas, adapters, codegen, or benches
  -> codegen compiles without generated schemas or adapters
  -> projection compiles without boundary adapters
  -> metamorphose compiles without concrete format writers
  -> transponding compiles without Arrow, Arrow IPC, or Parquet
  -> adapter crates compile only their own format surface
  -> benchmark crate compiles outside production libraries
```

No speed, memory, or compile-time claim is made by this spec.

## 6. Schema Source Contract

Schemas are `.proto` files plus approved MBT custom options.

Allowed application schema source locations:

```text
external schema repository
application crate proto directory
shared schema crate source directory
```

The MBT repository owns the MBT option file path:

```text
proto/mathilde/options.proto
```

The skeleton creates that file only as an ownership placeholder. Full option
extension definitions are deferred to a later codegen/options spec.

The MBT repository owns:

- MBT option definitions;
- codegen interpretation of MBT options;
- generated runtime contract;
- generated adapter contract.

The MBT repository does not own:

- Aggregator schema semantics;
- Primitives schema semantics;
- Regime schema semantics;
- application finality or hole policy;
- application cache or database policy.

Future schema specs must bind:

- proto root;
- root message;
- imports;
- schema ID policy;
- schema version policy;
- schema hash policy;
- generated destination;
- selected output surfaces.

## 7. Wire And Archive Contract

The skeleton phase does not implement wire/archive bytes.

The workspace reserves ownership:

```text
crates/core
  -> envelope and archive access contracts
```

No crate outside `crates/core` may invent its own envelope or trusted-access
semantics. Future runtime specs must define:

- envelope layout;
- schema header;
- payload boundary;
- checked validation;
- trusted access;
- corrupt payload behavior;
- old-version behavior;
- response cap behavior;
- allowed copy points.

## 8. Checked And Trusted Access Contract

The workspace must preserve separate checked and trusted surfaces.

Required ownership:

```text
crates/core
  -> checked access trait/error surface
  -> trusted access safety contract

generated schema crate or generated schema module
  -> schema-specific checked access
  -> schema-specific trusted access wrapper
```

Trusted access may be used only for bytes that were validated before immutable
storage, cache insertion, or transport handoff.

This skeleton does not implement the unsafe trusted API. A later runtime spec
must bind the exact spelling and safety contract before code.

## 9. Codegen Contract

Codegen is a separate tool/build surface:

```text
crates/codegen
```

Codegen must not depend on generated schema crates.

Codegen must not compile the full runtime/generated schema graph just to:

- inspect options;
- read descriptors;
- generate code;
- check generated output.

Future codegen specs must define:

- proto source roots;
- imported options;
- root message;
- generated crate or module destination;
- deterministic formatting;
- generated API surface;
- unsupported schema combinations;
- schema hash behavior;
- compile-surface budget;
- codegen-check command.

## 10. Crate Boundary Contract

### Canonical Workspace Tree

The corrected skeleton implementation plan must create this folder/file
structure exactly, unless a later peer-audited amendment changes it.

```text
Cargo.toml
Cargo.lock
README.md
AGENTS.md
proto/
  mathilde/
    options.proto
docs/
  architecture/
  evidence/
  invariants/
  protocols/
  reviews/
  specs/
crates/
  core/
    Cargo.toml
    src/
      lib.rs
      codec.rs
      envelope.rs
      error.rs
      runtime.rs
      tests/
        mod.rs
  codegen/
    Cargo.toml
    src/
      lib.rs
      main.rs
      descriptor.rs
      model.rs
      options.rs
      rust_emit.rs
      tests/
        mod.rs
  projection/
    Cargo.toml
    src/
      lib.rs
      runtime.rs
      tests/
        mod.rs
  metamorphose/
    Cargo.toml
    src/
      lib.rs
      runtime.rs
      tests/
        mod.rs
  transponding/
    Cargo.toml
    src/
      lib.rs
      runtime.rs
      tests/
        mod.rs
  adapters/
    json/
      Cargo.toml
      src/
        lib.rs
        tests/
          mod.rs
    protobuf/
      Cargo.toml
      src/
        lib.rs
        tests/
          mod.rs
    csv/
      Cargo.toml
      src/
        lib.rs
        tests/
          mod.rs
    arrow/
      Cargo.toml
      src/
        lib.rs
        tests/
          mod.rs
    arrow_ipc/
      Cargo.toml
      src/
        lib.rs
        tests/
          mod.rs
    parquet/
      Cargo.toml
      src/
        lib.rs
        tests/
          mod.rs
  benches/
    Cargo.toml
    src/
      lib.rs
      tests/
        mod.rs
```

No generated schema crate is created in this architecture skeleton. Generated
schemas may later live in application crates, external schema crates, or
approved workspace schema crates.

Reserved future workspace schema path pattern:

```text
crates/schemas/[schema_name]/
```

No `crates/schemas/*` directory may be created in this phase.

### Workspace Members

The corrected root workspace manifest must use these members:

```toml
[workspace]
resolver = "3"
members = [
    "crates/core",
    "crates/codegen",
    "crates/projection",
    "crates/metamorphose",
    "crates/transponding",
    "crates/adapters/json",
    "crates/adapters/protobuf",
    "crates/adapters/csv",
    "crates/adapters/arrow",
    "crates/adapters/arrow_ipc",
    "crates/adapters/parquet",
    "crates/benches",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["MATHILDE"]
```

The root package must be a virtual workspace. There must be no root binary, no
root library, and no root `src/` after the corrected skeleton implementation.

### Ownership Model

`crates/core`

- package: `metamorphic_binary_transport_core`;
- owns envelope, error, codec, and runtime contract surfaces;
- must not own generated schema code, adapters, transponding builders, or
  benchmark fixtures.

`crates/codegen`

- package: `metamorphic_binary_transport_codegen`;
- owns descriptor parsing, option validation, schema model construction,
  deterministic Rust emission, and output-surface selection;
- must not depend on generated schema crates.

`crates/projection`

- package: `metamorphic_binary_transport_projection`;
- owns shared MBT-to-MBT projection contracts;
- must not own boundary format conversion.

`crates/metamorphose`

- package: `metamorphic_binary_transport_metamorphose`;
- owns public boundary conversion traits/enums and shared checked-output
  helpers only;
- must not own concrete JSON, protobuf, CSV, Arrow, Arrow IPC, or Parquet
  writers.

`crates/transponding`

- package: `metamorphic_binary_transport_transponding`;
- owns shared row-to-columnar transformation contracts;
- must not force Arrow, Arrow IPC, or Parquet dependencies into core.

`crates/adapters/*`

- own concrete boundary format writers and their format-specific dependencies;
- must not import unrelated adapters.

`crates/benches`

- package: `metamorphic_binary_transport_benches`;
- owns benchmark and evidence harnesses only;
- must not be a dependency of production crates.

### Exact Skeleton Manifests

Root `Cargo.toml`:

```toml
[workspace]
resolver = "3"
members = [
    "crates/core",
    "crates/codegen",
    "crates/projection",
    "crates/metamorphose",
    "crates/transponding",
    "crates/adapters/json",
    "crates/adapters/protobuf",
    "crates/adapters/csv",
    "crates/adapters/arrow",
    "crates/adapters/arrow_ipc",
    "crates/adapters/parquet",
    "crates/benches",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["MATHILDE"]
```

All package manifests must use this shape, with the package name from the
table below:

```toml
[package]
name = "[package_name]"
version.workspace = true
edition.workspace = true
authors.workspace = true
publish = false

[dependencies]
```

Package name table:

```text
crates/core                       -> metamorphic_binary_transport_core
crates/codegen                    -> metamorphic_binary_transport_codegen
crates/projection                 -> metamorphic_binary_transport_projection
crates/metamorphose               -> metamorphic_binary_transport_metamorphose
crates/transponding               -> metamorphic_binary_transport_transponding
crates/adapters/json              -> metamorphic_binary_transport_adapter_json
crates/adapters/protobuf          -> metamorphic_binary_transport_adapter_protobuf
crates/adapters/csv               -> metamorphic_binary_transport_adapter_csv
crates/adapters/arrow             -> metamorphic_binary_transport_adapter_arrow
crates/adapters/arrow_ipc         -> metamorphic_binary_transport_adapter_arrow_ipc
crates/adapters/parquet           -> metamorphic_binary_transport_adapter_parquet
crates/benches                    -> metamorphic_binary_transport_benches
```

The skeleton has no dependencies in any crate.

### Exact Skeleton Source Text

`proto/mathilde/options.proto`:

```proto
syntax = "proto3";

package mathilde;

// MBT option extensions are defined by a later approved codegen/options spec.
```

`crates/core/src/lib.rs`:

```rust
#![forbid(unsafe_code)]
//! Core MBT ownership boundary.
//!
//! Runtime semantics are implemented only by later approved specs.

pub mod codec;
pub mod envelope;
pub mod error;
pub mod runtime;

#[cfg(test)]
mod tests;
```

`crates/core/src/codec.rs`:

```rust
//! Codec ownership boundary.
//!
//! Response checksum helpers are implemented only by later approved specs.
```

`crates/core/src/envelope.rs`:

```rust
//! Envelope ownership boundary.
//!
//! Header layout and validation are implemented only by later approved specs.
```

`crates/core/src/error.rs`:

```rust
//! Error ownership boundary.
//!
//! Transport errors are implemented only by later approved specs.
```

`crates/core/src/runtime.rs`:

```rust
//! Runtime ownership boundary.
//!
//! Runtime traits and access helpers are implemented only by later approved specs.
```

`crates/codegen/src/lib.rs`:

```rust
#![forbid(unsafe_code)]
//! Codegen ownership boundary.
//!
//! Descriptor parsing and generation are implemented only by later approved specs.

pub mod descriptor;
pub mod model;
pub mod options;
pub mod rust_emit;

#[cfg(test)]
mod tests;
```

`crates/codegen/src/main.rs`:

```rust
fn main() {}
```

`crates/codegen/src/descriptor.rs`:

```rust
//! Descriptor parsing boundary.
//!
//! Descriptor loading is implemented only by later approved specs.
```

`crates/codegen/src/model.rs`:

```rust
//! Codegen model boundary.
//!
//! Schema modeling is implemented only by later approved specs.
```

`crates/codegen/src/options.rs`:

```rust
//! MBT options boundary.
//!
//! Option validation is implemented only by later approved specs.
```

`crates/codegen/src/rust_emit.rs`:

```rust
//! Rust emission boundary.
//!
//! Deterministic code emission is implemented only by later approved specs.
```

`crates/projection/src/lib.rs`:

```rust
#![forbid(unsafe_code)]
//! Projection ownership boundary.
//!
//! MBT-to-MBT projection contracts are implemented only by later approved specs.

pub mod runtime;

#[cfg(test)]
mod tests;
```

`crates/projection/src/runtime.rs`:

```rust
//! Projection runtime boundary.
//!
//! Projection helpers are implemented only by later approved specs.
```

`crates/metamorphose/src/lib.rs`:

```rust
#![forbid(unsafe_code)]
//! Metamorphose ownership boundary.
//!
//! Boundary conversion traits are implemented only by later approved specs.

pub mod runtime;

#[cfg(test)]
mod tests;
```

`crates/metamorphose/src/runtime.rs`:

```rust
//! Metamorphose runtime boundary.
//!
//! Shared output helpers are implemented only by later approved specs.
```

`crates/transponding/src/lib.rs`:

```rust
#![forbid(unsafe_code)]
//! Transponding ownership boundary.
//!
//! Row-to-columnar contracts are implemented only by later approved specs.

pub mod runtime;

#[cfg(test)]
mod tests;
```

`crates/transponding/src/runtime.rs`:

```rust
//! Transponding runtime boundary.
//!
//! Columnar transformation helpers are implemented only by later approved specs.
```

Every adapter `src/lib.rs` must use this exact template with the adapter name
substituted as listed below:

```rust
#![forbid(unsafe_code)]
//! [adapter_name] adapter ownership boundary.
//!
//! Concrete format conversion is implemented only by later approved specs.

#[cfg(test)]
mod tests;
```

Adapter name substitutions:

```text
crates/adapters/json/src/lib.rs       -> JSON
crates/adapters/protobuf/src/lib.rs   -> Protobuf
crates/adapters/csv/src/lib.rs        -> CSV
crates/adapters/arrow/src/lib.rs      -> Arrow
crates/adapters/arrow_ipc/src/lib.rs  -> Arrow IPC
crates/adapters/parquet/src/lib.rs    -> Parquet
```

`crates/benches/src/lib.rs`:

```rust
#![forbid(unsafe_code)]
//! Benchmark ownership boundary.
//!
//! Benchmark fixtures are implemented only by later approved specs.

#[cfg(test)]
mod tests;
```

Every `src/tests/mod.rs` listed in this spec must use this exact content:

```rust
#[test]
fn skeleton_compiles() {}
```

## 11. Dependency Contract

Core dependency policy:

- no Arrow dependency;
- no Arrow IPC dependency;
- no Parquet dependency;
- no prost dependency unless a later spec proves core wire-format need;
- no generated prost DTO dependency;
- no `serde_json` dependency;
- no zstd dependency;
- no benchmark dependency;
- no schema-specific generated dependency;
- no adapter dependency.

Codegen dependency policy:

- descriptor/protobuf build dependencies are allowed only in codegen after a
  later spec binds them;
- codegen dependencies must not leak into core runtime;
- codegen must not depend on adapter crates unless a later spec proves it is
  required.

Adapter dependency policy:

- JSON dependencies live in the JSON adapter crate;
- protobuf/prost dependencies live in protobuf adapter or codegen as bound by
  a later spec;
- CSV dependencies live in CSV adapter only if a later spec proves a need;
- Arrow dependencies live in Arrow adapter;
- Arrow IPC dependencies live in Arrow IPC adapter;
- Parquet dependencies live in Parquet adapter;
- compression dependencies live in benchmark/downstream transport surfaces and
  are not part of the MBT envelope unless a later spec explicitly changes the
  compression contract.

Benchmark dependency policy:

- benchmark-only dependencies live in `crates/benches`;
- benchmark dependencies must not enter production crates.

Skeleton dependency policy:

- no crate has dependencies in this phase.

## 12. Determinism Contract

Workspace and generated artifact behavior must be deterministic:

- workspace member list is explicit;
- generated code order is stable in later generated-code specs;
- generated schema hashes are stable for identical proto input in later
  generated-code specs;
- checks use explicit commands;
- benchmark evidence records dirty state and environment.

## 13. Failure Contract

The workspace must fail fast when boundaries are violated.

Future implementation plans must include checks for:

- rejected verbose crate paths still present;
- root `src/` still present;
- core importing adapter crates;
- core importing benchmark modules;
- codegen importing generated schema crates;
- generated files edited by hand;
- adapter crates importing unrelated adapters;
- schema crates compiling unrelated schemas.

No runtime failure behavior is defined by this architecture spec.

## 14. Compile-Surface Budget

This spec defines architecture-level compile budgets, not measured results.

Corrected skeleton implementation must prove:

```text
cargo check -p metamorphic_binary_transport_core
  compiles core only

cargo check -p metamorphic_binary_transport_codegen
  compiles codegen skeleton only

cargo check -p metamorphic_binary_transport_projection
  compiles without boundary adapters

cargo check -p metamorphic_binary_transport_metamorphose
  compiles without concrete format writers

cargo check -p metamorphic_binary_transport_transponding
  compiles without Arrow, Arrow IPC, or Parquet

cargo check -p metamorphic_binary_transport_benches
  compiles without production crates depending on it
```

Adapter skeleton crates must compile without concrete format logic until their
own specs authorize dependencies and implementation.

No numeric threshold is set by this spec because runtime logic is not
implemented here.

## 15. Runtime Performance Budget

No runtime benchmark is required for the workspace skeleton because it does not
implement runtime MBT logic.

Future runtime and adapter specs must define:

- encode throughput budget;
- checked access budget;
- trusted access budget;
- projection budget;
- metamorphose budget;
- transponding budget;
- adapter throughput budget;
- logical payload baseline;
- correctness oracle.

## 16. Correctness Oracle

For this workspace architecture phase, correctness is structural:

- workspace members exist exactly as specified;
- rejected verbose crate folders do not exist;
- dependency boundaries match this spec;
- root package is not monolithic;
- production crates do not depend on benchmark crates;
- core does not depend on adapters;
- codegen does not compile generated schema crates;
- metamorphose does not contain concrete format writer modules;
- adapters do not import unrelated adapters;
- options placeholder exists at the bound path.

Runtime byte correctness is out of scope for this spec.

## 17. Benchmark Methodology

No runtime benchmark is bound by this spec.

Build-surface validation for the future implementation plan must include:

```text
cargo check --workspace
cargo tree -p metamorphic_binary_transport_core
cargo tree -p metamorphic_binary_transport_codegen
cargo tree -p metamorphic_binary_transport_projection
cargo tree -p metamorphic_binary_transport_metamorphose
cargo tree -p metamorphic_binary_transport_transponding
cargo tree -p metamorphic_binary_transport_benches
test ! -e src/main.rs
test ! -d crates/metamorphic_binary_transport_core
test ! -d crates/metamorphic_binary_transport_codegen
test ! -d crates/metamorphic_binary_transport_benches
test ! -e crates/metamorphose/src/json.rs
test ! -e crates/metamorphose/src/protobuf.rs
test ! -e crates/metamorphose/src/csv.rs
test -e proto/mathilde/options.proto
```

The implementation plan must add `cargo metadata` or equivalent checks proving
that the root manifest is a virtual workspace and that the workspace members
match this spec exactly.

## 18. Test Plan

The future implementation plan must bind tests or checks for:

1. workspace manifest parses;
2. all workspace members compile;
3. core has no adapter dependencies;
4. core has no benchmark dependencies;
5. codegen has no schema crate dependencies;
6. benchmark crate is not a dependency of production crates;
7. root package does not exist;
8. metamorphose has no concrete JSON/protobuf/CSV writer modules;
9. core dependency tree contains no Arrow, Arrow IPC, Parquet, generated prost
   DTO, serde_json, zstd, or benchmark dependency;
10. rejected verbose crate directories do not exist;
11. schema crate directories do not exist;
12. generated artifact directories or generated schema files do not exist;
13. options placeholder exists at `proto/mathilde/options.proto`.

Generated-output import checks are deferred to later codegen specs because
generated output is forbidden in this skeleton phase.

## 19. Code Bindings

Files allowed for the corrected workspace skeleton implementation plan:

```text
Cargo.toml
Cargo.lock
README.md
src/main.rs
proto/mathilde/options.proto
crates/core/Cargo.toml
crates/core/src/lib.rs
crates/core/src/codec.rs
crates/core/src/envelope.rs
crates/core/src/error.rs
crates/core/src/runtime.rs
crates/core/src/tests/mod.rs
crates/codegen/Cargo.toml
crates/codegen/src/lib.rs
crates/codegen/src/main.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/model.rs
crates/codegen/src/options.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/mod.rs
crates/projection/Cargo.toml
crates/projection/src/lib.rs
crates/projection/src/runtime.rs
crates/projection/src/tests/mod.rs
crates/metamorphose/Cargo.toml
crates/metamorphose/src/lib.rs
crates/metamorphose/src/runtime.rs
crates/metamorphose/src/tests/mod.rs
crates/transponding/Cargo.toml
crates/transponding/src/lib.rs
crates/transponding/src/runtime.rs
crates/transponding/src/tests/mod.rs
crates/adapters/json/Cargo.toml
crates/adapters/json/src/lib.rs
crates/adapters/json/src/tests/mod.rs
crates/adapters/protobuf/Cargo.toml
crates/adapters/protobuf/src/lib.rs
crates/adapters/protobuf/src/tests/mod.rs
crates/adapters/csv/Cargo.toml
crates/adapters/csv/src/lib.rs
crates/adapters/csv/src/tests/mod.rs
crates/adapters/arrow/Cargo.toml
crates/adapters/arrow/src/lib.rs
crates/adapters/arrow/src/tests/mod.rs
crates/adapters/arrow_ipc/Cargo.toml
crates/adapters/arrow_ipc/src/lib.rs
crates/adapters/arrow_ipc/src/tests/mod.rs
crates/adapters/parquet/Cargo.toml
crates/adapters/parquet/src/lib.rs
crates/adapters/parquet/src/tests/mod.rs
crates/benches/Cargo.toml
crates/benches/src/lib.rs
crates/benches/src/tests/mod.rs
```

`src/main.rs` is listed only so the implementation plan may delete it while
converting the root package into a virtual workspace.

The implementation plan must explicitly remove these rejected paths if they
exist:

```text
crates/metamorphic_binary_transport_core
crates/metamorphic_binary_transport_codegen
crates/metamorphic_binary_transport_benches
```

The implementation plan must explicitly remove these rejected duplicate
metamorphose writer paths if they exist:

```text
crates/metamorphose/src/json.rs
crates/metamorphose/src/protobuf.rs
crates/metamorphose/src/csv.rs
```

The future implementation plan must not edit runtime logic outside these paths
for this architecture phase.

## 20. Generated Artifact Bindings

No generated MBT schema artifacts are allowed in this phase.

No generated protobuf DTOs are allowed in this phase.

No generated adapter bindings are allowed in this phase.

The generated output surface is reserved for later specs only:

```text
core schema output
projection output
json adapter output
protobuf adapter output
csv adapter output
columnar/transponding output
arrow / arrow_ipc / parquet adapter output
benchmark/check output
```

Later generated-code specs must bind:

- generated file paths;
- generated API surface;
- deterministic check command;
- import/dependency containment checks.

If implementation discovers a need for generated output in this phase, work
must stop and the spec must be amended and re-audited.

## 21. Review Artifact Bindings

Required existing artifacts:

```text
docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_research_brief.md
docs/reviews/mbt_workspace_architecture/mbt_experiment_architecture_extraction_analysis.md
docs/specs/mbt_workspace_architecture_SPEC.md
docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_peer_audit.md
docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_peer_audit_v2.md
docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_peer_audit_v3.md
```

Required next artifact:

```text
docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_peer_audit_v4.md
```

The previous implementation plan is stale after this amendment and cannot be
used for code changes.

## 22. Implementation Plan Requirement

Before code changes, the corrected implementation plan must bind:

- deletion of rejected verbose crate folders;
- deletion of rejected duplicate metamorphose writer files if present;
- root manifest conversion to a virtual workspace;
- exact crate directories to create;
- exact files to edit/create;
- exact files to delete;
- exact skeleton code for every created source file;
- dependency list for each crate;
- validation commands;
- dependency tree audit commands;
- expected outputs;
- rollback boundary.

The implementation plan must explicitly state that generated artifacts,
runtime logic, codegen logic, adapter logic, and benchmark logic are not
implemented in this phase.

`Cargo.lock` is a Cargo-generated artifact. The implementation may delete the
stale lockfile before validation, but the final lockfile must be generated by
`cargo check --workspace`. Manual lockfile editing is forbidden.

`target/` is build output and not part of source cleanup. The implementation
plan may leave it untouched or remove it only if needed for clean validation
timing; it must not treat `target/` contents as source evidence.

## 23. Approval Checklist

Implementation is not approved until all are true:

- required reads complete;
- research brief exists;
- this amended spec exists;
- peer audit v4 passes;
- implementation plan is amended to this spec;
- implementation plan is explicitly approved;
- exact files are bound;
- dependency changes are bound;
- validation commands are bound;
- generated artifacts remain forbidden for this phase.

## 24. Open Questions

No open questions remain for the corrected workspace skeleton.

Deferred decisions for later specs:

1. Exact runtime envelope implementation.
2. Exact codegen implementation.
3. Full protobuf option extension definitions.
4. First generated schema crate or application-local generated module.
5. Adapter logic and dependencies.
6. Benchmark fixtures and baseline implementations.
