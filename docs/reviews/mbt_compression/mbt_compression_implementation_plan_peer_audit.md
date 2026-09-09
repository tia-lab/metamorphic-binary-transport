# MBT Compression Implementation Plan Peer Audit

Slug: `mbt_compression`

Audited plan:

```text
docs/reviews/mbt_compression/mbt_compression_implementation_plan.md
```

Audited spec:

```text
docs/specs/mbt_compression_SPEC.md
```

Prior spec audit:

```text
docs/reviews/mbt_compression/mbt_compression_peer_audit_v3.md
```

Status: `BLOCKED`

Correction after amendment read:

- The duplicate `out.clear()` finding below was caused by overlapping line reads,
  not by duplicated plan text. It is withdrawn.
- The audit remains blocked only by the static forbidden-call validation command
  mismatch.

This audit does not authorize implementation.

## Audit Scope

This audit checks whether the implementation plan is executable and exactly
bound to the approved compression spec.

Audit lenses:

- file edit/create bindings;
- dependency and lockfile bindings;
- compression crate API bindings;
- failure behavior bindings;
- benchmark source and report bindings;
- validation command executability;
- rollback boundary.

## Evidence Read

Protocol evidence:

- `docs/protocols/peer_audit_protocol.md` requires blocking when code bindings,
  benchmark proof, dependency behavior, or correctness oracle are incomplete.
- `docs/protocols/implementation_protocol.md` requires implementation to start
  only when exact files, generated artifacts, dependency changes, tests,
  benchmarks, failure contract, and correctness oracle are bound.

Spec evidence:

- `docs/specs/mbt_compression_SPEC.md` binds compression to a separate opt-in
  crate and forbids core, schema, codegen, metamorphose, transponding, adapter,
  and proto edits.
- `docs/specs/mbt_compression_SPEC.md` binds zstd exactly as `=0.13.3`.
- `docs/specs/mbt_compression_SPEC.md` binds the required benchmark lanes,
  source fixture, schema hashes, report fields, evidence paths, and result
  review.

Plan evidence:

- `docs/reviews/mbt_compression/mbt_compression_implementation_plan.md` binds
  the compression crate, benchmark files, evidence files, inventory updates,
  dependency changes, API signatures, capped writer shape, tests, benchmark
  command, and rollback boundary.

Code-read evidence:

- `crates/core/src/error.rs` defines `Result`, `TransportError::MalformedArchive`,
  and `TransportError::ResponseTooLarge`.
- `crates/benches/src/projection.rs` defines `bars_rows` and
  `MAX_RESPONSE_BYTES`.
- `crates/schemas/bars_core/src/bars_v1.rs` defines `BarsV1::encode`,
  `BarsV1::project_no_metadata`, `BarsV1::project_ohlcv_only`, and the schema
  hashes bound by the spec.

## Blocking Finding 1: Static Forbidden-Call Check Is Not Spec-Exact

The implementation plan uses this executable validation command:

```bash
! rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!" crates/compression/src crates/benches/src/compression.rs crates/benches/src/bin/mbt_compression_bench.rs
```

The spec still binds the positive command:

```bash
rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!" crates/compression/src crates/benches/src/compression.rs crates/benches/src/bin/mbt_compression_bench.rs
```

with expected output `no matches`.

That positive command exits non-zero when no matches are found, so it is not an
executable passing validation step under normal shell semantics. The plan is
correct in spirit, but it is not exact against the current spec text.

Required amendment:

- Amend `docs/specs/mbt_compression_SPEC.md` so the static forbidden-call check
  is explicitly the negated executable command.
- State that success means no forbidden calls are found.
- Keep the same file scope.

## Withdrawn Finding 2: Plan Contains A Duplicate `out.clear()` In The Decompression Recipe

Withdrawn.

Follow-up read evidence showed the plan contains one `out.clear();` for the
compression recipe and one `out.clear();` for the decompression recipe. That is
correct and requires no amendment.

Required amendment: none.

## Non-Blocking Observations

The plan's crate/file boundaries match the spec:

- `crates/compression` is the only new production crate.
- `zstd = "=0.13.3"` is the only new external dependency.
- `Cargo.lock` ownership is explicitly limited to Cargo resolution for zstd.
- Core, generated schemas, codegen, metamorphose, transponding, adapters, and
  proto files are forbidden edits.

The plan's benchmark source construction matches the spec:

- `mbt_full` comes from `BarsV1::encode(&bars_rows(row_count), MAX_RESPONSE_BYTES)`.
- `mbt_no_metadata` comes from full MBT bytes and `BarsV1::project_no_metadata`.
- `mbt_ohlcv_only` comes from full MBT bytes and `BarsV1::project_ohlcv_only`.
- Construction happens before the timed compression/decompression loop.

The plan's smoke-run row counts are stricter than the spec's optional smoke
command. This is acceptable because the plan narrows validation behavior without
changing the full-run measured object.

## Required Next Step

Amend:

```text
docs/specs/mbt_compression_SPEC.md
```

Then write a second implementation-plan peer audit.

## Classification

`BLOCKED`

Implementation remains blocked until the required amendments are complete and a
new implementation-plan peer audit passes.
