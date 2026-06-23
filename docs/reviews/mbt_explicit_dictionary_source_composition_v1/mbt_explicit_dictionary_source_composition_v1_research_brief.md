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

# Research Brief: MBT Explicit Dictionary Source Composition V1

Slug: `mbt_explicit_dictionary_source_composition_v1`

Status: `RESEARCH_BRIEF_COMPLETE`

## Source Materials

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/research_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `crates/codegen/src/config.rs`
- `crates/codegen/src/descriptor.rs`
- `crates/codegen/src/options.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/codegen/src/tests/test_cli.rs`
- `crates/codegen/src/tests/test_descriptor.rs`
- `/home/tia/_DEV/MATHILDE/mathilde-mbt-schemas/docs/specs/schema_registry_shared_instrument_dictionary_v1_SPEC.md`

## Measured Object

The measured object is MBT codegen dictionary resolution.

Current behavior:

```text
target source proto file options -> dictionary set -> field dictionary lookup
```

Required behavior:

```text
target source proto file options
  + explicitly listed dictionary source proto file options
  -> one deterministic dictionary set
  -> field dictionary lookup
```

The immediate consumer is `mathilde-mbt-schemas`, which needs one shared
`proto/mathilde/shared/instruments_v1.proto` dictionary and collector ingestion
schemas that import and reference it without duplicating dictionary values.

## Candidate Approach

Add an explicit dictionary source list to the MBT codegen request/config path:

```text
--dictionary-source mathilde/shared/instruments_v1.proto
```

The flag is repeatable. It is accepted by `--inspect`, `--write`, and
`--check` for every surface because dictionary resolution happens before
surface emission.

No new protobuf option is required. The dictionary source list is a codegen
input contract, not schema syntax.

## MBT Binding Surface

The change is owned by `crates/codegen` only:

- `crates/codegen/src/config.rs`
  - carry `dictionary_sources: Vec<PathBuf>` in `CodegenConfig` and
    `SchemaRequest`;
  - parse repeated `--dictionary-source <path>` flags;
  - reject empty dictionary source values.
- `crates/codegen/src/descriptor.rs`
  - include dictionary source inputs in the `protoc` descriptor request;
  - load dictionaries from the root source file plus each explicit dictionary
    source file in request order;
  - reuse existing dictionary option parsing;
  - reuse existing duplicate-name and duplicate-value validation.
- `crates/codegen/src/tests/mod.rs`
  - update test config helpers for the new config field;
  - add helper fixture support if needed.
- `crates/codegen/src/tests/test_cli.rs`
  - test CLI parsing for repeated dictionary source flags.
- `crates/codegen/src/tests/test_descriptor.rs`
  - test imported dictionary source success, missing dictionary failure,
    duplicate dictionary failure, existing local dictionary behavior, and hash
    sensitivity to composed dictionary values.

`crates/codegen/src/options.rs` is read evidence only. It already parses
`mathilde.dictionary_values` and does not need a new option.

## Evidence Table

| Evidence type | Surface | Observation |
| --- | --- | --- |
| Protocol evidence | `AGENTS.md` | No code change is allowed before approved spec and approved implementation plan. |
| Invariant evidence | `docs/invariants/core_invariants.md` | Codegen input must include the exact proto file set and root message. Generated code must be deterministic. |
| Code-read evidence | `crates/codegen/src/config.rs` | `CodegenConfig` and `SchemaRequest` currently contain proto roots, schema, root, module, surface, adapter, and output, but no dictionary source list. |
| Code-read evidence | `crates/codegen/src/config.rs` | CLI parsing currently rejects unknown arguments and has no `--dictionary-source` flag. |
| Code-read evidence | `crates/codegen/src/descriptor.rs` | `load_schema_model` obtains the target source file and calls `dictionaries_from_file(&file.options(), ...)` only for that file. |
| Code-read evidence | `crates/codegen/src/descriptor.rs` | `run_protoc` already passes `--include_imports`, so imported descriptors can be present in the descriptor pool. |
| Code-read evidence | `crates/codegen/src/descriptor.rs` | `validate_dictionaries` rejects duplicate dictionary names and duplicate values. |
| Code-read evidence | `crates/codegen/src/descriptor.rs` | `normalized_hash` already iterates `model.dictionaries`, so a composed dictionary set will be included in hash normalization if stored in `SchemaModel.dictionaries`. |
| Code-read evidence | `crates/codegen/src/options.rs` | `dictionary_from_value` already parses the `dictionary_values` file option payload. |
| Code-read evidence | `crates/codegen/src/tests/mod.rs` | Existing test helpers write temporary proto roots and root schema files; dictionary tests declare file-local dictionary values. |
| Code-read evidence | `crates/codegen/src/tests/test_cli.rs` | Existing CLI tests verify explicit CLI shape and invalid argument rejection. |
| Consumer spec evidence | `schema_registry_shared_instrument_dictionary_v1_SPEC.md` | `mathilde-mbt-schemas` is blocked until MBT can load dictionaries from target source plus explicit dictionary source files. |

## Hypotheses

1. Adding explicit dictionary source files to the existing descriptor request is
   enough for `DescriptorPool::get_file_by_name` to find those files even when
   the target schema imports them indirectly or directly.
   - This is plausible because `run_protoc` already builds descriptor sets and
     can receive multiple input files, but must be proved by tests.
2. No runtime or generated wire behavior changes are required.
   - This is supported by code-read evidence because the change occurs before
     `SchemaModel` emission and only changes dictionary availability.
3. Existing file-local dictionary behavior will remain unchanged when
   `dictionary_sources` is empty.
   - This must be proved by preserving existing tests.

## Unknowns

1. Exact Rust helper names for the implementation.
2. Whether `protoc` error messages for missing dictionary source files are
   sufficient or need a wrapper error.
3. Whether any existing caller constructs `CodegenConfig` manually outside
   `crates/codegen/src/tests/mod.rs`.

## Risks

1. Hidden transitive import scanning would make dictionary availability depend
   on incidental imports. The selected approach avoids this by requiring an
   explicit source list.
2. Allowing duplicate dictionary names with identical values would create
   ambiguous ownership. The selected approach rejects duplicate names.
3. Failing to include dictionary source files in the descriptor request would
   make explicit sources fail unless they were already imported by the target
   schema.
4. Forgetting to include composed dictionaries in `SchemaModel.dictionaries`
   would make schema hashes and generated helpers inconsistent.
5. Adding a new protobuf option would widen the schema syntax unnecessarily.
   No new option is required.

## Required Decisions Before Spec

The research supports these decisions:

- use CLI/config/request field name `dictionary_sources`;
- use repeatable CLI flag `--dictionary-source`;
- compose root source dictionaries first, then explicit dictionary source
  dictionaries in request order;
- reject duplicate dictionary names after composition;
- preserve existing behavior when the list is empty;
- no new MBT protobuf option;
- no runtime, core, adapter, schema, or generated-file changes.

## Evidence Required Before Coding

Before implementation:

1. the spec must bind exact code paths, tests, commands, and failure behavior;
2. the peer audit must pass;
3. the implementation plan must bind exact file edits and validation commands;
4. user approval must be given.

## Recommended Next Phase

Write:

```text
docs/specs/mbt_explicit_dictionary_source_composition_v1_SPEC.md
```

Then write:

```text
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_peer_audit.md
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_implementation_plan.md
```
