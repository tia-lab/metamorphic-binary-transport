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

# Peer Audit: MBT Codegen Migration

Status: `BLOCKED`

Slug: `mbt_codegen_migration`

Audited spec:

```text
docs/specs/mbt_codegen_migration_SPEC.md
```

Research brief:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_research_brief.md
```

## Audit Reads

Code-read and protocol evidence:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/peer_audit_protocol.md
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_research_brief.md
docs/specs/mbt_codegen_migration_SPEC.md
crates/codegen/src/{lib,main,descriptor,model,options,rust_emit}.rs
proto/mathilde/options.proto
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/emit.rs
/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/options.proto
```

## Findings

### 1. Exact MBT option surface is not specified

Severity: blocker

Evidence:

- Spec section 20 says `proto/mathilde/options.proto` is the codegen-owned
  source option file.
- Spec section 23 says the MBT option surface must be exact.
- The current spec does not list the exact messages, enums, extension targets,
  extension numbers, allowed values, or invalid combinations that must be
  written to `proto/mathilde/options.proto`.
- Code-read evidence from the experiment
  `/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/options.proto`
  shows a mixed option file containing MBT transport options plus DB, lookup,
  MLDB, and cache options.

Why this blocks:

The implementation would have to decide option file contents during coding.
That violates the spec-source-of-truth rule and can accidentally migrate
non-MBT DB/cache/lookup annotations into this repository.

Required amendment:

Add an exact MBT-only option definition section that binds:

- `syntax = "proto2"`;
- `package mathilde`;
- import of `google/protobuf/descriptor.proto`;
- exact MBT messages, currently including `DictionaryAlias`, `Dictionary`,
  and, if retained in this options file, `ProjectionDefinition`;
- exact `FileOptions`, `MessageOptions`, and `FieldOptions` extension numbers;
- exact field names and types for `dictionary_values`, `schema_id`,
  `schema_version`, `transport_name`, `payload_root`, `dictionary`,
  `bitmask_dictionary`, `presence_bit`, `key_part`, `key_order`, `const_u16`,
  `repeated_payload`, `raw_string`, `ignored`, `projection_group`, and
  `derived_utc_from`;
- whether `projection` remains in the option file but is ignored by the
  first core-only emitter, or is deferred entirely.

### 2. Codegen CLI and check command are deferred to the implementation plan

Severity: blocker

Evidence:

- Spec section 20 defines only conceptual actions and says `--proto-root` is
  repeatable or single path "as implementation plan binds".
- Spec section 20 says the implementation plan must bind exact CLI spelling.
- `docs/protocols/spec_protocol.md` requires generated-code specs to define
  the codegen-check command.

Why this blocks:

The generator command is part of the generated-artifact contract. Deferring
the exact command to the implementation plan leaves codegen behavior
under-specified at the spec level.

Required amendment:

Define the exact CLI and the exact check command in the spec, including:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --check \
  --proto-root <path> \
  --schema <relative.proto> \
  --root <fully.qualified.Message> \
  --module <snake_case_module> \
  --surface core \
  --out <path>
```

If multiple proto roots are required for external schemas, the spec must say
whether `--proto-root` is repeatable and how root precedence works.

### 3. Generated Rust compile oracle is optional

Severity: blocker

Evidence:

- Spec section 16 says generated output can be compiled against core in a
  temporary smoke crate "if the implementation plan binds that smoke crate".
- Spec section 17 does not require a compile check for generated output.
- Codegen protocol requires generated output to reject unsupported shapes and
  keep compile surface bounded and measured.

Why this blocks:

Forbidden-string scans and deterministic source comparison prove only that
text is stable and some old surfaces are absent. They do not prove the
generated core schema module compiles against `metamorphic_binary_transport_core`
and `rkyv`.

Required amendment:

Make a temporary generated smoke crate mandatory for this migration. The spec
must bind:

- temporary crate path under `target/mbt-codegen-check/`;
- generated fixture module path;
- smoke crate dependencies, including `metamorphic_binary_transport_core` and
  the exact `rkyv` dependency needed by generated code;
- exact `cargo check` command for the smoke crate;
- line-count and forbidden-string checks on the same generated file.

### 4. Dependency contract still includes `prost-build` without proven need

Severity: blocker

Evidence:

- Spec section 11 allows `prost-build = "=0.13.5"`.
- Spec non-goals explicitly exclude prost DTO sidecar modules.
- Code-read evidence from experiment `src/codegen/emit.rs` shows
  `prost_build::Config` is used in `compile_proto` only for writing
  `*_proto.rs` sidecar output.
- Descriptor loading in experiment `src/codegen/descriptor.rs` uses
  `Command::new("protoc")` and `prost_reflect::DescriptorPool`, not
  `prost-build`.

Why this blocks:

The dependency rationale is inaccurate for the narrowed core-only migration.
This violates dependency hygiene and risks importing an unnecessary codegen
surface.

Required amendment:

Either remove `prost-build` from allowed production dependencies for this
spec, or add a proved reason that survives the non-goal of no prost DTO
sidecar output. If the smoke crate needs normal Cargo dependency resolution,
that does not require `prost-build` in the codegen crate.

### 5. Generated schema dependency contract is incomplete

Severity: blocker

Evidence:

- Spec section 7 requires generated schema output to use rkyv archive types.
- Spec section 11 says `rkyv` must not be a production dependency of
  `crates/codegen`, but does not bind the generated schema crate dependency
  surface.
- Spec section 16 makes the generated smoke crate optional.

Why this blocks:

Generated output cannot be compiled or validated without an exact dependency
contract for the consumer side. The spec currently protects `crates/codegen`
but not the generated module's compile contract.

Required amendment:

Define generated-core output dependencies separately from codegen
dependencies:

```text
metamorphic_binary_transport_core
rkyv with exact version/features
```

The spec must state that these dependencies belong to generated schema
consumers or the temporary smoke crate, not to `crates/codegen`.

### 6. Nullable array semantics need exact wording

Severity: blocker

Evidence:

- Spec section 9 says nullable arrays missing presence bits are rejected.
- Test plan says array fixtures cover repeated numeric arrays and nullable
  array presence.
- The spec does not define how a repeated numeric field is declared nullable
  versus required.

Why this blocks:

Repeated protobuf fields do not have standard scalar presence. The accepted
MBT design needs exact semantics so implementation does not infer behavior
from helper code.

Required amendment:

Define:

- repeated numeric field without `presence_bit` is a required non-null MBT
  array where empty means empty;
- repeated numeric field with `presence_bit` is a nullable MBT array where
  absent requires the backing vector to be empty;
- absent nullable array is omitted by future boundary adapters;
- present nullable array may be empty.

## Audit Lenses

| Lens | Result |
| --- | --- |
| Measured object clarity | Mostly clear. Blocked by optional generated compile oracle. |
| Schema source ownership | Clear at high level. Blocked by missing exact options surface. |
| Wire/archive validation | Clear enough for core-only migration. |
| Trusted-access safety | Clear enough for generated API shape. |
| Codegen determinism | Direction clear. Exact CLI/check command missing. |
| Generated-code compile surface | Blocked by optional smoke compile and incomplete generated dependency contract. |
| Crate boundary isolation | Direction clear. Dependency issue with `prost-build` must be corrected or justified. |
| Dependency containment | Blocked by `prost-build` rationale and missing generated-consumer deps. |
| Correctness oracle | Too weak until generated compile check is mandatory. |
| Benchmark isolation | Acceptable: no benchmark claim for this migration. |
| Performance budget | Acceptable: no runtime speed claim. |
| Failure behavior | Error classes are clear enough. |
| Code bindings | Mostly clear for codegen files. |
| Generated artifact bindings | Blocked by conceptual CLI and optional smoke output. |
| Client/operator safety | Needs exact generated command and option surface. |

## Required Amendments Before Next Audit

1. Add exact `proto/mathilde/options.proto` MBT-only contents.
2. Bind exact CLI spelling and exact codegen-check command.
3. Make generated smoke compile mandatory.
4. Remove `prost-build` or justify it with a proved core-only need.
5. Define generated schema consumer dependencies.
6. Define nullable array semantics exactly.

## Classification

```text
BLOCKED
```

The spec is directionally correct, but it is not implementation-ready. The
current blockers would leave too many decisions to the implementation plan or
the code itself.
