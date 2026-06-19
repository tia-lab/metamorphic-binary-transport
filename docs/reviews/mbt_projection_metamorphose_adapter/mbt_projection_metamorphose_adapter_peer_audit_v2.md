# MBT Projection Metamorphose Adapter Peer Audit v2

## Classification

`BLOCKED`

## Scope

Audited spec:

```text
docs/specs/mbt_projection_metamorphose_adapter_SPEC.md
```

Prior audit:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit.md
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
- prior peer audit
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/emit.rs`
- `crates/codegen/src/config.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`

## Prior Blocker Resolution Check

### Prior blocker 1: code bindings conflict with schema test edits

Status: resolved.

Evidence:

- The amended spec now separates allowed codegen implementation source files,
  codegen unit test files, and schema runtime test files.
- The schema runtime test files are now explicitly bound:

```text
crates/schemas/bars_core/tests/test_bars_metamorphose.rs
crates/schemas/test_compatibility_core/tests/test_projection_metamorphose.rs
```

### Prior blocker 2: derived UTC projection JSON behavior was unproved

Status: resolved.

Evidence:

- The generic codegen oracle now requires retained derived UTC projection JSON
  symbols and write path:

```text
SMALL_JSON_FIELD_IGNORED_UTC
small_write_json_response
writer.utc_value(row.close_ms.to_native())?;
```

- The Bars runtime oracle now requires retained top-level derived UTC fields
  and excluded metadata-derived UTC fields:

```text
open_utc
close_utc
metadata.ingested_at_utc
```

## Findings

### 1. Forbidden-source command conflicts with required codegen test oracle

Severity: blocking

Evidence:

- The correctness oracle requires the generated source not to contain these
  schema-specific strings:

```text
BarsV1
BarsV1NoMetadata
```

- The test plan also requires this repository command to return no matches:

```bash
rg -n 'crates/serving|BarsV1|BarsV1NoMetadata' crates/codegen/src
```

- The allowed codegen unit test file is:

```text
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
```

Problem:

The implementation will naturally need to assert that generated source does not
contain `BarsV1` and `BarsV1NoMetadata`. If those exact negative assertions are
written in the allowed codegen test file, the required forbidden-source command
will match the test source itself and fail.

The intended hardcoding proof is valid, but the command surface is too broad.
It scans tests as well as production generator code.

Required amendment:

Keep the generated-source negative oracle, but narrow the repository hardcoding
search to production codegen sources. Acceptable command shape:

```bash
rg -n 'crates/serving|BarsV1|BarsV1NoMetadata' crates/codegen/src --glob '!tests/**'
```

Expected result remains:

```text
no matches
```

### 2. Review artifact binding still points to the first audit path

Severity: blocking

Evidence:

- The amended spec status says it is ready for peer audit v2.
- Section 21 still binds the required next peer audit as:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit.md
```

Problem:

The review artifact binding is stale after the first blocked audit. The spec
must bind the current next audit artifact exactly before audit passes.

Required amendment:

Change the required next peer audit binding to:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit_v2.md
```

If a future amendment is needed, it must then bind v3 explicitly.

## Non-Blocking Observations

- The JSON-only scope remains appropriate.
- The first audit blockers were addressed without broadening production code
  scope.
- The spec still correctly forbids MBT core, adapters, compression, cache, and
  serving changes.
- Generated artifact write/check commands remain exact.
- The derived UTC proof is now concrete enough for implementation planning once
  the command and review artifact bindings are corrected.

## Required Amendment Summary

Before peer audit v3:

1. Narrow the forbidden-source `rg` command so it excludes
   `crates/codegen/src/tests/**` while keeping the generated-source negative
   oracle.
2. Update the review artifact binding for the current required peer audit path
   from `peer_audit.md` to `peer_audit_v2.md`.

