# MBT Compression Peer Audit V2

Slug: `mbt_compression`

Audited spec:

```text
docs/specs/mbt_compression_SPEC.md
```

Prior audit:

```text
docs/reviews/mbt_compression/mbt_compression_peer_audit.md
```

Status: `BLOCKED`

This audit does not authorize implementation.

## Required Reads

Completed reads:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/reviews/mbt_compression/mbt_compression_research_brief.md`
- `docs/specs/mbt_compression_SPEC.md`
- `docs/reviews/mbt_compression/mbt_compression_peer_audit.md`
- `docs/specs/mbt_core_runtime_migration_SPEC.md`
- `docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit_v4.md`
- local `Cargo.lock` presence check

## Prior Blocker Closure

| Prior blocker                                                            | Result                                                                                                                                           |
| ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Dependency no-match commands were not executable success checks          | Closed. The spec now uses explicit negative checks with `! cargo tree ... \| rg`.                                                                |
| Benchmark fixture identity was under-specified                           | Closed. The spec now binds the deterministic Bars fixture, no-RNG seed policy, max response bytes, source construction paths, and schema hashes. |
| Optional compatibility-schema lane deferred scope to implementation plan | Closed. The spec now excludes all-fields compatibility schema benchmarking from this phase.                                                      |

## Findings

### 1. Cargo.lock ownership is missing from code and generated-artifact bindings

Severity: blocking.

Evidence:

- `docs/specs/mbt_compression_SPEC.md:219-255`
- `docs/specs/mbt_compression_SPEC.md:594-664`
- `docs/specs/mbt_compression_SPEC.md:665-678`
- `Cargo.lock` exists in the repository root.
- `docs/specs/mbt_core_runtime_migration_SPEC.md` binds `Cargo.lock` as an
  allowed Cargo-generated dependency artifact for dependency changes.

The compression spec adds a new exact dependency:

```toml
zstd = "=0.13.3"
```

but code bindings do not include:

```text
Cargo.lock
```

and generated artifact bindings state that no generated artifact is owned by
the spec.

This is incomplete. The lockfile is a Cargo-generated dependency artifact, not
source code, but it is still a repository artifact affected by the dependency
change. The spec must define whether `Cargo.lock` may change, how it may
change, and what lockfile contents are accepted.

Required amendment:

- Add `Cargo.lock` to the exact bindings as a Cargo-generated dependency
  artifact.
- State that `Cargo.lock` must not be edited manually.
- State that it may change only through Cargo dependency resolution caused by
  adding `zstd = "=0.13.3"` to `crates/compression/Cargo.toml`.
- State that unrelated package upgrades or dependency drift in `Cargo.lock`
  are rejected.
- Add a validation command that checks the locked zstd version, for example:

```bash
rg -n 'name = "zstd"|version = "0.13.3"' Cargo.lock
```

or a stricter structured lockfile check if the implementation plan chooses one.

## Audit Lens Results

| Lens                                    | Result                                                           |
| --------------------------------------- | ---------------------------------------------------------------- |
| pre-audit closure gate completeness     | blocked by missing lockfile ownership                            |
| measured object clarity                 | passed                                                           |
| schema source ownership                 | passed                                                           |
| wire/archive validation                 | passed                                                           |
| trusted-access safety                   | passed                                                           |
| codegen determinism                     | passed; no codegen changes authorized                            |
| generated-code compile surface          | passed for generated schema files; blocked for lockfile artifact |
| crate boundary isolation                | passed                                                           |
| dependency containment                  | blocked only by missing lockfile binding                         |
| correctness oracle                      | passed                                                           |
| benchmark isolation                     | passed                                                           |
| performance budget                      | passed                                                           |
| failure behavior                        | passed                                                           |
| code binding completeness               | blocked by missing `Cargo.lock`                                  |
| generated artifact binding completeness | blocked by missing Cargo-generated artifact policy               |
| client/operator interpretation safety   | passed                                                           |

## Required Spec Amendment

Amend:

```text
docs/specs/mbt_compression_SPEC.md
```

Minimum required change:

1. Bind `Cargo.lock` as an allowed Cargo-generated dependency artifact for this
   dependency addition, with exact accepted update policy and validation
   command.

After amendment, write:

```text
docs/reviews/mbt_compression/mbt_compression_peer_audit_v3.md
```

## Classification

`BLOCKED`
