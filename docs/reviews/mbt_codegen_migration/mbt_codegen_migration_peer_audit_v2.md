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

# Peer Audit V2: MBT Codegen Migration

Status: `BLOCKED`

Slug: `mbt_codegen_migration`

Audited spec:

```text
docs/specs/mbt_codegen_migration_SPEC.md
```

Prior audit:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_peer_audit.md
```

## Audit Reads

Protocol and spec evidence:

```text
docs/protocols/peer_audit_protocol.md
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_research_brief.md
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_peer_audit.md
docs/specs/mbt_codegen_migration_SPEC.md
```

Code-read evidence:

```text
crates/core/Cargo.toml
crates/codegen/Cargo.toml
crates/core/src/error.rs
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/emit.rs
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/descriptor.rs
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/rust_emit.rs
/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/options.proto
```

Run evidence:

```text
protoc --help | sed -n '1,120p'
```

Observed output states `--proto_path` may be specified multiple times and
directories are searched in order. This supports the amended repeatable
`--proto-root` ordering contract.

## Previous Blocker Resolution

| V1 blocker | V2 status |
| --- | --- |
| Exact MBT option surface missing | Resolved: spec now defines MBT-only `proto/mathilde/options.proto`. |
| Exact CLI/check command missing | Resolved: spec now binds exact `--inspect`, `--write`, and `--check` command shapes. |
| Generated compile oracle optional | Resolved: smoke crate compile is now mandatory. |
| `prost-build` allowed without core-only need | Resolved: `prost-build` is now forbidden for `crates/codegen`. |
| Generated consumer dependencies incomplete | Resolved: smoke/generated consumer deps are defined separately. |
| Nullable array semantics unclear | Resolved: required versus nullable array semantics are now stated. |

## Findings

### 1. Generated view and archived-row API surface is not exact

Severity: blocker

Evidence:

- Spec section 8 defines `encode`, `encode_owned`, `access`, `inspect`, and
  the `MbtSchema` implementation.
- Spec section 7 says checked access exposes an archived view without
  materializing owned rows.
- Spec section 22 leaves the "exact generated core output API subset" to the
  implementation plan.
- Code-read evidence from the experiment emitter shows concrete generated
  surface names and behavior: `<Marker>View<'a>`, `<Marker>Rows<'a>`,
  `Archived<Marker>Row<'a>`, `len`, `is_empty`, `rows`, `Iterator`, field
  getters, and presence accessors.

Why this blocks:

Generated API surface is a generated-code spec requirement. The implementation
plan must not decide whether generated schema consumers can iterate archived
rows or call typed field accessors. A generated module could compile and still
be unusable for core MBT consumers if this remains unspecified.

Required amendment:

Bind the exact core generated API surface in the spec:

```rust
pub struct <Marker>;
pub struct <Marker>View<'a> { ... }
pub struct <Marker>Rows<'a> { ... }
pub struct Archived<Marker>Row<'a> { ... }
```

Required methods must include:

```rust
<Marker>View::len
<Marker>View::is_empty
<Marker>View::rows
Iterator for <Marker>Rows
Archived<Marker>Row::<field accessors>
Archived<Marker>Row::<presence accessors when presence exists>
```

The spec must state that accessors borrow archived data where possible and do
not materialize owned rows.

### 2. Schema hash normal form is still under-specified

Severity: blocker

Evidence:

- Spec section 9 says schema hash is derived from normalized schema model
  content, not raw source file bytes.
- Spec section 16 requires tests to check expected schema hash.
- The spec does not define the exact normalized fields, ordering, separators,
  dictionary treatment, ignored-field treatment, projection treatment, or
  alias treatment.
- Code-read evidence from the experiment `descriptor.rs::normalized_hash`
  shows the old implementation used explicit normalized text over schema
  identity, dictionaries, fields, and selected metadata before hashing.

Why this blocks:

Schema hash is part of the MBT envelope identity. If the implementation
chooses the normal form during coding, the wire identity contract is not
auditable and later generated schema parity cannot be proved.

Required amendment:

Add an exact schema hash normal-form section that binds:

- FNV-1a 64-bit hash;
- schema identity fields and line format;
- dictionary order and value treatment;
- physical field order and line format;
- included field metadata, at minimum `proto_path`, generated Rust name, field
  kind, presence bit, and key order for core schemas;
- ignored fields excluded from the core physical hash unless a later adapter
  spec binds otherwise;
- projection definitions and projection groups excluded from the core surface
  hash for this migration unless the spec explicitly binds a different rule;
- dictionary aliases excluded from the core transport hash unless the spec
  explicitly binds alias semantics into the hash.

### 3. Dictionary alias behavior is present in options but not specified

Severity: blocker

Evidence:

- Spec section 6 defines `DictionaryAlias` and `Dictionary.alias`.
- Spec sections 9 and 16 do not state whether aliases are parsed, rejected,
  ignored, included in schema hash, emitted as helper functions, or used by
  generated validation.
- Code-read evidence from the experiment option file shows aliases exist in
  the shared option surface that this migration is narrowing.

Why this blocks:

An option field in the source-of-truth proto cannot have undefined semantics.
For core binary transport, aliases likely should not affect wire bytes, but
that must be stated in the spec before implementation.

Required amendment:

Define alias behavior for this migration:

- aliases are accepted in `Dictionary`;
- aliases do not affect dictionary ordinals;
- aliases do not affect generated binary archive fields;
- aliases do not affect core schema hash;
- aliases are not emitted as core-surface helper APIs in this migration unless
  the spec explicitly adds such helpers.

### 4. Projection option behavior in core-only mode remains ambiguous

Severity: blocker

Evidence:

- Spec section 6 keeps `projection` and `projection_group` in the MBT option
  file.
- Spec says the model "may preserve projection metadata" for future surfaces.
- Spec section 9 forbids adapter/transponding/metamorphose APIs, but does not
  explicitly forbid projection output strings such as `project_`, projected
  marker types, projection schema constants, or projection-specific helpers.
- Code-read evidence from the experiment emitter shows projection APIs and
  projection hashes were emitted by the old monolithic generator.

Why this blocks:

"May preserve" leaves codegen behavior to implementation. It is acceptable for
the core-only emitter to parse and ignore projection metadata, but the spec
must make that deterministic and test-bound.

Required amendment:

Define core-surface projection behavior:

- codegen parses projection options only enough to recognize the extensions;
- no projection model construction is required for `--surface core`;
- projection definitions do not affect core generated output or core schema
  hash;
- generated core output must not contain `project_`, projected marker names,
  projection-specific schema hash constants, projection helpers, or projection
  APIs;
- projection emission is deferred to a separate projection codegen spec.

## Audit Lenses

| Lens | Result |
| --- | --- |
| Measured object clarity | Mostly clear. |
| Schema source ownership | Improved. Still blocked by alias and projection option behavior. |
| Wire/archive validation | Clear enough except schema hash normal form. |
| Trusted-access safety | Clear. |
| Codegen determinism | Blocked by schema hash normal form. |
| Generated-code compile surface | Improved by mandatory smoke compile. |
| Crate boundary isolation | Clear. |
| Dependency containment | Clear after `prost-build` removal. |
| Correctness oracle | Improved, but missing hash and view/accessor exactness. |
| Benchmark isolation | Acceptable; no benchmark claim. |
| Performance budget | Acceptable; no runtime speed claim. |
| Failure behavior | Clear enough for this phase. |
| Code binding completeness | Mostly clear. |
| Generated artifact binding completeness | Blocked by generated API ambiguity. |
| Client/operator interpretation safety | Blocked by undefined generated consumer API. |

## Required Amendments Before Next Audit

1. Add exact schema hash normal form.
2. Bind exact generated view, rows iterator, archived row wrapper, and field
   accessor API.
3. Define dictionary alias behavior.
4. Define projection-option behavior for `--surface core` and add projection
   forbidden strings/tests.

## Classification

```text
BLOCKED
```
