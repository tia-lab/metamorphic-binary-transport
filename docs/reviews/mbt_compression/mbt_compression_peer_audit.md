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

# MBT Compression Peer Audit

Slug: `mbt_compression`

Audited spec:

```text
docs/specs/mbt_compression_SPEC.md
```

Research brief:

```text
docs/reviews/mbt_compression/mbt_compression_research_brief.md
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
- `crates/core/src/envelope.rs`
- `crates/core/src/error.rs`
- `crates/core/src/output.rs`
- `crates/metamorphose/src/runtime.rs`
- `crates/schemas/bars_core/src/bars_v1.rs`
- old MBT compression evidence under
  `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport`
- zstd `0.13.3` docs.rs and crates.io pages cited by the spec

## Findings

### 1. Dependency-containment commands are not executable as success checks

Severity: blocking.

Evidence:

- `docs/specs/mbt_compression_SPEC.md:350-356`
- `docs/specs/mbt_compression_SPEC.md:361-363`

The spec requires:

```bash
cargo tree -p metamorphic_binary_transport_core | rg -n "zstd"
cargo tree -p metamorphic_binary_transport_schema_bars | rg -n "zstd"
cargo tree -p metamorphic_binary_transport_schema_test_compatibility | rg -n "zstd"
```

and then says the expected output is no matches.

That is not an executable success command because `rg` exits nonzero on no
matches. The spec therefore marks command surfaces exact while defining commands
whose success condition conflicts with shell exit behavior.

Required amendment:

- Replace the no-match commands with explicit negative checks, for example:

```bash
! cargo tree -p metamorphic_binary_transport_core | rg -n "zstd"
! cargo tree -p metamorphic_binary_transport_schema_bars | rg -n "zstd"
! cargo tree -p metamorphic_binary_transport_schema_test_compatibility | rg -n "zstd"
```

or bind an equivalent checked script. The expected exit behavior must be stated
directly.

### 2. Benchmark fixture identity is under-specified

Severity: blocking.

Evidence:

- `docs/specs/mbt_compression_SPEC.md:451-466`
- `docs/specs/mbt_compression_SPEC.md:468-484`

The benchmark section defines row counts and lanes, but it does not bind:

- deterministic row source;
- seed;
- schema hash fields;
- MBT byte construction path for full and projected lanes;
- max response bytes used to build source MBT bytes;
- report fields for `row_seed` and schema identity.

The old compression evidence recorded seed, schema hash, and max response bytes.
The new spec keeps `max_response_bytes` in the report but drops seed and schema
identity from required report fields.

This weakens reproducibility. A compression benchmark can only be interpreted if
the input byte source is fully identified.

Required amendment:

- Bind the deterministic Bars fixture source used by `crates/benches`.
- Bind the seed value or the fixture constant name.
- Bind source MBT construction for each lane:
  - full MBT encode;
  - full MBT -> no-metadata projection;
  - full MBT -> OHLCV-only projection.
- Add report fields for at least:
  - `row_seed`;
  - full schema hash;
  - projected schema hashes or projected schema labels with stable identity;
  - source MBT byte checksum before compression.

### 3. Optional compatibility-schema lane defers scope to the implementation plan

Severity: blocking.

Evidence:

- `docs/specs/mbt_compression_SPEC.md:89-96`
- `docs/specs/mbt_compression_SPEC.md:730-744`

The spec says `mbt_test_compatibility_all_fields` is optional if the
implementation plan binds a fixture.

That conflicts with the pre-audit closure rule that no design decision is
deferred to the implementation plan. If this lane is necessary, the spec must
bind it. If it is not necessary, the spec must remove it.

Required amendment:

- Either remove the optional compatibility lane entirely from this phase, or
  bind it fully in the spec with fixture source, row counts, byte construction,
  report fields, and correctness oracle.

## Audit Lens Results

| Lens | Result |
| --- | --- |
| pre-audit closure gate completeness | blocked by findings 1 and 3 |
| measured object clarity | passed for MBT bytes <-> zstd frame bytes |
| schema source ownership | passed; compression is schema-agnostic |
| wire/archive validation | passed; compressed output is not an MBT envelope |
| trusted-access safety | passed; decompression does not imply trusted access |
| codegen determinism | passed; no codegen changes authorized |
| generated-code compile surface | passed, subject to command fix |
| crate boundary isolation | passed |
| dependency containment | blocked by executable command shape |
| correctness oracle | passed for byte equality and caps |
| benchmark isolation | blocked by fixture identity gap |
| performance budget | passed; no numeric claim without evidence |
| failure behavior | passed for first-phase scope |
| code binding completeness | passed for file paths |
| generated artifact binding completeness | passed; no generated artifacts owned |
| client/operator interpretation safety | passed after fixing benchmark identity |

## Required Spec Amendment

Amend:

```text
docs/specs/mbt_compression_SPEC.md
```

Minimum required changes:

1. Make no-match dependency checks executable with explicit negative checks.
2. Bind deterministic benchmark source bytes, seed, schema identity, source MBT
   byte construction, and report fields.
3. Remove or fully bind the optional compatibility-schema lane.

After amendment, write:

```text
docs/reviews/mbt_compression/mbt_compression_peer_audit_v2.md
```

## Classification

`BLOCKED`
