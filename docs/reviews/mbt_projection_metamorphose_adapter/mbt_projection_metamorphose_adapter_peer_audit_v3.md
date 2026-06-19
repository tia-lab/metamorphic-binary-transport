# MBT Projection Metamorphose Adapter Peer Audit v3

## Classification

`PEER_AUDIT_PASSED`

## Scope

Audited spec:

```text
docs/specs/mbt_projection_metamorphose_adapter_SPEC.md
```

Prior audits:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit.md
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit_v2.md
```

No code was changed by this audit.

## Required Reads

Completed:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- target research brief
- amended target spec
- prior peer audits
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/emit.rs`
- `crates/codegen/src/config.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`

## Prior Blocker Resolution Check

### Peer audit blocker 1: code bindings conflicted with schema test edits

Status: resolved.

Evidence:

- Section 19 now separates:
  - codegen implementation source files;
  - codegen unit test files;
  - schema runtime test files.
- The required schema runtime test files are explicitly bound:

```text
crates/schemas/bars_core/tests/test_bars_metamorphose.rs
crates/schemas/test_compatibility_core/tests/test_projection_metamorphose.rs
```

### Peer audit blocker 2: derived UTC projection JSON behavior was unproved

Status: resolved.

Evidence:

- The generic codegen oracle now requires retained derived UTC generated source
  evidence:

```text
SMALL_JSON_FIELD_IGNORED_UTC
small_write_json_response
writer.utc_value(row.close_ms.to_native())?;
```

- The Bars runtime oracle now requires retained top-level derived UTC fields and
  excluded metadata-derived UTC fields:

```text
open_utc
close_utc
metadata.ingested_at_utc
```

### Peer audit v2 blocker 1: forbidden-source command scanned tests

Status: resolved.

Evidence:

- The forbidden-source command now excludes codegen tests:

```bash
rg -n 'crates/serving|BarsV1|BarsV1NoMetadata' crates/codegen/src --glob '!tests/**'
```

- The generated-source negative oracle remains present, so tests can still
  assert that generated output is not Bars-specific.

### Peer audit v2 blocker 2: review artifact binding was stale

Status: resolved for the v2 amendment.

Evidence:

- Section 21 binds the v2 audit artifact required by the v2 blocker:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit_v2.md
```

This v3 artifact records the current passing audit after that amendment.

## Audit Lenses

### Pre-audit closure gate

Passed.

The spec has the mandatory section order and includes an explicit pre-audit
closure checklist. Prior relevant specs are cited and preserved:

- projection remains MBT-to-MBT before JSON conversion;
- JSON remains an opt-in adapter surface;
- serving remains downstream and outside this repository.

### Measured object clarity

Passed.

The measured object is restricted to generated JSON metamorphose adapter support
for proto-declared projection markers. It does not include HTTP serving,
protobuf, CSV, Arrow, Arrow IPC, Parquet, compression, or cache behavior.

### Schema source ownership

Passed.

The schema source remains `.proto + proto/mathilde/options.proto`. The generic
proof uses the existing non-serving projection fixture and the spec forbids
Rust-name, serving-path, benchmark-label, crate-name, or Bars-specific
inference.

### Wire/archive validation and trusted access

Passed.

The spec does not change MBT bytes. Projection marker JSON checked access must
validate projected marker bytes through projected marker accessors. Trusted
access remains unsafe and uses the existing trusted projection marker archive
access contract.

### Codegen determinism and dispatch

Passed.

The dispatch path is explicit:

```text
Surface::Metamorphose -> generated_metamorphose_adapter_schema(model, adapter)
```

JSON emission order is deterministic:

```text
source marker, then projection markers in proto projection order
```

Projection helper names must use existing projection scope prefixes to avoid
collisions.

### Generated-code compile surface

Passed.

The compile-surface budget is explicit:

- default schema builds remain adapter-free;
- JSON feature builds may grow only by projection count and selected fields;
- codegen `cargo check --all-targets` timing must be recorded in the result
  review.

### Crate boundary and dependency containment

Passed.

Allowed production code change is limited to `mbt_codegen`; generated artifacts
are schema-owned. The spec forbids new dependencies and forbids edits to core,
metamorphose runtime, adapters, compression, cache, and serving.

### Correctness oracle

Passed.

The oracle covers:

- generic generated source proof;
- Bars runtime projected JSON proof;
- test-compatibility runtime projected JSON proof;
- generated artifact `--check` proof;
- retained and excluded derived UTC behavior.

### Benchmark isolation

Passed.

No benchmark is required because this is a correctness prerequisite. The spec
does not make a runtime speed claim.

### Failure behavior

Passed.

The spec uses existing MBT errors for wrong schema, wrong hash/version,
corrupt payload, invalid archive shape, and response cap errors.

### Code and generated artifact bindings

Passed.

Implementation files, test files, generated files, codegen write/check
commands, compile commands, and review artifacts are bound.

## Remaining Risks For Implementation Plan

These are not audit blockers, but the implementation plan must handle them
explicitly:

1. The JSON emitter helper refactor must preserve source marker output shape.
2. Projection `SchemaModel` construction must rebuild JSON/CSV output fields
   without changing projection archive semantics.
3. Generated source tests must avoid hardcoding Bars behavior while still
   proving no generated Bars strings in the generic fixture.
4. Generated files must be updated only through the bound `mbt_codegen`
   commands.

## Result

The amended spec is implementation-plan ready.

Code remains blocked until an implementation plan is written, audited if
required, and explicitly approved.

