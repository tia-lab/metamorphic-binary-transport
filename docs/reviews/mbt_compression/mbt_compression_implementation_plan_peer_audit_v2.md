# MBT Compression Implementation Plan Peer Audit v2

Slug: `mbt_compression`

Audited spec:

```text
docs/specs/mbt_compression_SPEC.md
```

Audited implementation plan:

```text
docs/reviews/mbt_compression/mbt_compression_implementation_plan.md
```

Prior implementation-plan audit:

```text
docs/reviews/mbt_compression/mbt_compression_implementation_plan_peer_audit.md
```

Status: `PEER_AUDIT_PASSED`

This audit does not implement code. Implementation still requires explicit user
approval.

## Audit Scope

This v2 audit checks whether the amendments resolved the prior implementation
plan blocker and whether the plan remains exact against the approved
compression spec.

Checked surfaces:

- compression crate boundary;
- dependency and `Cargo.lock` ownership;
- production API signatures;
- capped writer implementation shape;
- failure contract;
- benchmark lanes and source byte construction;
- evidence and result-review bindings;
- validation commands;
- rollback boundary.

## Evidence Read

Protocol evidence:

- `AGENTS.md` requires no code change before approved spec and approved
  implementation plan.
- `docs/invariants/core_invariants.md` requires compression outside the MBT
  envelope unless a later spec changes that contract.
- `docs/protocols/peer_audit_protocol.md` requires the audit to falsify the
  artifact and classify exactly `PEER_AUDIT_PASSED` or `BLOCKED`.
- `docs/protocols/implementation_protocol.md` requires exact file, dependency,
  test, benchmark, failure, and correctness bindings before implementation.

Spec evidence:

- `docs/specs/mbt_compression_SPEC.md` now binds the static forbidden-call check
  as the executable negated command:

```bash
! rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!" crates/compression/src crates/benches/src/compression.rs crates/benches/src/bin/mbt_compression_bench.rs
```

- `docs/specs/mbt_compression_SPEC.md` keeps compression as an opt-in crate
  outside the MBT envelope.
- `docs/specs/mbt_compression_SPEC.md` binds `zstd = "=0.13.3"` and constrains
  `Cargo.lock` to Cargo-generated dependency resolution for that dependency.
- `docs/specs/mbt_compression_SPEC.md` binds the measured lanes:
  `mbt_full`, `mbt_no_metadata`, and `mbt_ohlcv_only`.

Plan evidence:

- `docs/reviews/mbt_compression/mbt_compression_implementation_plan.md` uses the
  same executable negated static check as the amended spec.
- The plan binds only the approved production crate, benchmark files, evidence
  files, inventory files, and dependency changes.
- The plan forbids edits to core, codegen, generated schemas, metamorphose,
  transponding, adapters, and proto files.
- The plan states that zstd internal allocation behavior is dependency-owned
  and not proved zero.

Code-read evidence:

- `crates/core/src/error.rs` defines the required `Result`,
  `TransportError::MalformedArchive`, and `TransportError::ResponseTooLarge`
  surfaces.
- `crates/benches/src/projection.rs` defines the required deterministic
  `bars_rows` fixture and `MAX_RESPONSE_BYTES`.
- `crates/schemas/bars_core/src/bars_v1.rs` defines the required Bars encode,
  projection, and schema-hash surfaces used by the benchmark plan.

## Prior Blocker Resolution

Prior blocker:

- The spec used a positive `rg` static check while the implementation plan used
  a negated executable check.

Resolution:

- The spec now uses the same negated executable `! rg ...` command as the plan.
- The expected result is stated as command success from no matches.

Result:

- Resolved.

Prior withdrawn finding:

- The first audit incorrectly reported a duplicated decompression `out.clear()`
  after reading overlapping line ranges.

Resolution:

- The implementation plan contains one `out.clear()` in the compression recipe
  and one `out.clear()` in the decompression recipe.

Result:

- No implementation-plan change required.

## Falsification Checks

### Crate Boundary

The plan adds only:

```text
crates/compression
```

and a benchmark dependency from:

```text
crates/benches
```

This matches the spec. Core, schema crates, codegen, metamorphose,
transponding, and adapters do not gain compression dependencies.

Finding: no blocker.

### Dependency Boundary

The plan adds only:

```toml
zstd = "=0.13.3"
```

inside `crates/compression/Cargo.toml`, plus a local benchmark dependency on
the compression crate. `Cargo.lock` is explicitly limited to Cargo dependency
resolution for this exact dependency.

Finding: no blocker.

### Production API

The plan binds:

- `CompressionConfig`;
- `DEFAULT_ZSTD_LEVEL = 3`;
- allocating convenience APIs;
- `compress_into`;
- `decompress_into`;
- `metamorphic_binary_transport_core::error::Result`.

This is sufficient for the approved compression surface. The plan does not
introduce trusted MBT access, schema inspection, or an envelope wrapper.

Finding: no blocker.

### Failure Contract

The plan maps:

- output cap overflow to `TransportError::ResponseTooLarge`;
- invalid/truncated zstd and encoder/decoder errors to
  `TransportError::MalformedArchive`.

This matches the spec and available core error variants.

Finding: no blocker.

### Benchmark Object

The plan builds MBT source bytes outside the timed loop and measures only
compression/decompression for:

- full Bars MBT;
- no-metadata projected MBT;
- OHLCV-only projected MBT.

This matches the spec's measured object and avoids mixing MBT encode/projection
work into the timing lane.

Finding: no blocker.

### Validation Commands

The plan binds:

- formatting;
- compression crate tests;
- bench crate tests;
- core and compression `cargo check`;
- dependency tree isolation checks;
- lockfile check;
- forbidden-call static check;
- smoke and full benchmark commands;
- inventory regeneration.

The prior non-executable static check mismatch is resolved.

Finding: no blocker.

### Rollback Boundary

The plan binds rollback to all edited/created files, evidence artifacts, result
review, and zstd-caused lockfile resolution changes only.

Finding: no blocker.

## Remaining Limitations

These are not blockers because they are explicitly bound:

- zstd internal allocation behavior is not proved zero.
- The first benchmark covers Bars full and projected MBT only.
- The benchmark uses the bench crate's existing `serde` and `serde_json` only for
  evidence artifact writing.
- No speed or ratio claim is approved until the benchmark commands are run and
  result review records the evidence.

## Classification

`PEER_AUDIT_PASSED`

Implementation may proceed only after explicit user approval for:

```text
docs/reviews/mbt_compression/mbt_compression_implementation_plan.md
```
