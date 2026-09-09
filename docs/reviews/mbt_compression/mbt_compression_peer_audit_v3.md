# MBT Compression Peer Audit V3

Slug: `mbt_compression`

Audited spec:

```text
docs/specs/mbt_compression_SPEC.md
```

Prior audits:

```text
docs/reviews/mbt_compression/mbt_compression_peer_audit.md
docs/reviews/mbt_compression/mbt_compression_peer_audit_v2.md
```

Status: `PEER_AUDIT_PASSED`

This audit approves the spec for implementation planning only. It does not
authorize code implementation.

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
- `docs/reviews/mbt_compression/mbt_compression_peer_audit_v2.md`
- `docs/specs/mbt_core_runtime_migration_SPEC.md`

## Prior Blocker Closure

| Prior blocker                                                 | Result                                                                                                                               |
| ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Dependency no-match checks were not executable success checks | Closed. The spec now uses explicit negated dependency checks.                                                                        |
| Benchmark fixture identity was under-specified                | Closed. The spec now binds Bars fixture source, no-RNG seed policy, source byte construction, max response bytes, and schema hashes. |
| Optional compatibility-schema lane deferred scope             | Closed. The spec now excludes that lane from phase one.                                                                              |
| `Cargo.lock` ownership was missing                            | Closed. The spec now binds `Cargo.lock` as a Cargo-generated dependency artifact for `zstd = "=0.13.3"` only.                        |

## Audit Lens Results

| Lens                                    | Result                                              |
| --------------------------------------- | --------------------------------------------------- |
| pre-audit closure gate completeness     | passed                                              |
| measured object clarity                 | passed                                              |
| schema source ownership                 | passed; compression is schema-agnostic              |
| wire/archive validation                 | passed; compressed bytes are not an MBT envelope    |
| trusted-access safety                   | passed; decompression does not imply trusted access |
| codegen determinism                     | passed; no codegen changes authorized               |
| generated-code compile surface          | passed; no generated schema artifacts owned         |
| crate boundary isolation                | passed                                              |
| dependency containment                  | passed with lockfile policy                         |
| correctness oracle                      | passed                                              |
| benchmark isolation                     | passed                                              |
| performance budget                      | passed; no speed claim without evidence             |
| failure behavior                        | passed                                              |
| code binding completeness               | passed                                              |
| generated artifact binding completeness | passed, including `Cargo.lock` policy               |
| client/operator interpretation safety   | passed                                              |

## Residual Risks

These are not blockers for implementation planning:

- zstd may allocate internally. The spec explicitly avoids claiming zero
  allocations inside zstd and requires measurement of the complete path.
- The first benchmark is Bars-only. The spec explicitly excludes the all-fields
  compatibility schema from this phase.
- The lockfile validation command proves `zstd 0.13.3` exists in the lockfile,
  but the implementation plan should use review of the `Cargo.lock` diff to
  reject unrelated dependency drift.

## Implementation Plan Requirements To Preserve

The implementation plan must preserve:

- no edits to core, codegen, schemas, metamorphose, transponding, adapters, or
  proto files;
- no generated schema file edits;
- `crates/compression` as the only production crate addition;
- `Cargo.lock` update only through Cargo dependency resolution for
  `zstd = "=0.13.3"`;
- production `*_into` APIs as caller-buffer paths;
- convenience APIs as explicit allocation points;
- benchmark evidence under `docs/evidence/mbt_compression`;
- result review after implementation before performance claims.

## Classification

`PEER_AUDIT_PASSED`
