# MBT Crate Short Names Research Brief

## Status

Status: research complete for spec authoring.

Task class: package/import naming design for the current MBT workspace.

No code changes are authorized by this brief.

## Source Materials

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/research_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`
- `Cargo.toml`
- every workspace crate `Cargo.toml`
- `crates/codegen/src/rust_emit.rs`
- generated schema modules under `crates/schemas/*/src`
- current README and architecture docs

## Measured Object

The measured object is the Rust crate/package/import naming surface. The goal
is to replace long crate names such as
`metamorphic_binary_transport_core` with short names such as `mbt_core`.

This is not a runtime benchmark, wire-format change, schema change, or adapter
behavior change.

## Candidate Approach

Rename each workspace package and dependency key from
`metamorphic_binary_transport_*` to `mbt_*`, while preserving crate paths and
workspace boundaries.

The rename must include:

- package names in `Cargo.toml`;
- dependency keys and optional feature `dep:` entries;
- runtime source imports;
- codegen emitted import strings;
- generated schema artifacts regenerated from codegen;
- tests and benches imports;
- README, architecture, current inventory, and current command examples.

Historical review and evidence artifacts should remain historical records
unless the approved spec explicitly binds a current-doc rewrite.

## Evidence Table

| Evidence type         | Source                                          | Observation                                                                                                                                                                                            |
| --------------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Code-read evidence    | `Cargo.toml`                                    | Current workspace members are path-based and include core, codegen, metamorphose, transponding, compression, adapters, benches, and two schema crates. There is no current `crates/projection` member. |
| Code-read evidence    | workspace crate `Cargo.toml` files              | Package names and dependency keys currently use `metamorphic_binary_transport_*`.                                                                                                                      |
| Code-read evidence    | `crates/codegen/src/rust_emit.rs`               | Codegen emits long crate names into generated core, projection, metamorphose, transponding, Arrow, Arrow IPC, and Parquet modules.                                                                     |
| Code-read evidence    | `crates/schemas/*/src/*.rs`                     | Generated schema modules currently import long crate names and therefore must be regenerated or reproduced from codegen.                                                                               |
| Code-read evidence    | `README.md`                                     | Public examples currently expose long dependency and import names.                                                                                                                                     |
| Code-read evidence    | `docs/specs/mbt_workspace_architecture_SPEC.md` | Earlier workspace package table binds long package names. A new spec must locally supersede only that naming table while preserving crate boundaries.                                                  |
| Run evidence          | None                                            | No validation command was run for this research brief.                                                                                                                                                 |
| Build evidence        | None                                            | Build impact is expected to be naming-only but must be proved after implementation.                                                                                                                    |
| Benchmark evidence    | None                                            | No runtime speed claim is made.                                                                                                                                                                        |
| Schema evidence       | Existing generated files                        | Generated files are codegen-owned and must not be hand-edited to apply the rename.                                                                                                                     |
| External-doc evidence | None                                            | Cargo package/import rename behavior is standard Rust/Cargo behavior and no date-sensitive upstream claim is needed for the spec.                                                                      |
| Hypothesis            | Rename impact                                   | Runtime performance should be unchanged because symbols and dependency names change, not encoding/access algorithms. This remains a hypothesis until tests pass and no benchmark claim is made.        |

## MBT Binding Surface

The rename binds the public Rust import surface:

```text
metamorphic_binary_transport_core              -> mbt_core
metamorphic_binary_transport_codegen           -> mbt_codegen
metamorphic_binary_transport_metamorphose      -> mbt_metamorphose
metamorphic_binary_transport_transponding      -> mbt_transponding
metamorphic_binary_transport_compression       -> mbt_compression
metamorphic_binary_transport_adapter_json      -> mbt_adapter_json
metamorphic_binary_transport_adapter_protobuf  -> mbt_adapter_protobuf
metamorphic_binary_transport_adapter_csv       -> mbt_adapter_csv
metamorphic_binary_transport_adapter_arrow     -> mbt_adapter_arrow
metamorphic_binary_transport_adapter_arrow_ipc -> mbt_adapter_arrow_ipc
metamorphic_binary_transport_adapter_parquet   -> mbt_adapter_parquet
metamorphic_binary_transport_benches           -> mbt_benches
metamorphic_binary_transport_schema_bars       -> mbt_schema_bars
metamorphic_binary_transport_schema_test_compatibility
                                               -> mbt_schema_test_compatibility
```

The `mbt_codegen` binary name is already short and should remain
`mbt_codegen`.

## Unknowns

No behavioral unknown blocks the spec.

Build evidence is still required after implementation because package names
appear in generated artifacts, tests, benches, documentation, and command
surfaces.

## Risks

- A partial rename can compile one crate but break generated schema crates.
- Editing generated schema files by hand would violate codegen ownership.
- Leaving codegen emit strings unchanged would cause future generated files to
  drift back to long names.
- Rewriting historical evidence artifacts would obscure prior run records.
- Updating docs/specs indiscriminately can corrupt historical review context.

## Required Decisions Before Spec

The spec must decide:

1. whether all current crates are renamed in one bounded change;
2. whether schema crates use `mbt_schema_*`;
3. whether historical reviews/evidence are preserved;
4. whether generated files are regenerated through codegen;
5. the exact validation commands and grep checks that prove no long names
   remain in current source surfaces.

## Recommended Next Phase

Proceed to `docs/specs/mbt_crate_short_names_SPEC.md`.

The spec should be audited before an implementation plan is written.
