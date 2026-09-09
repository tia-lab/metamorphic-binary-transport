# Peer Audit: MBT Explicit Dictionary Source Composition V1

## Classification

PEER_AUDIT_PASSED

## Audited Artifact

```text
docs/specs/mbt_explicit_dictionary_source_composition_v1_SPEC.md
```

Research brief:

```text
docs/reviews/mbt_explicit_dictionary_source_composition_v1/mbt_explicit_dictionary_source_composition_v1_research_brief.md
```

## Audit Scope

This audit tried to falsify:

- pre-audit closure;
- measured object clarity;
- dictionary source ownership;
- CLI/config/request binding;
- descriptor set construction;
- dictionary composition order;
- duplicate-name failure behavior;
- missing-dictionary failure behavior;
- schema hash behavior;
- preservation of source-file-local dictionaries;
- crate boundary and dependency containment;
- compile-surface budget;
- test plan;
- generated artifact boundary.

No code, generated files, dependencies, build, test, codegen, or benchmark
commands were changed or run by this audit.

## Findings

### 1. Pre-Audit Closure

Result: pass.

The spec follows the required section order and includes a specific
pre-audit closure checklist. It binds the CLI surface, code paths, tests,
compile-surface commands, and generated artifact boundary.

### 2. Measured Object

Result: pass.

The measured object is precisely codegen descriptor/model construction:

```text
CodegenConfig / SchemaRequest
  -> protoc descriptor set
  -> DescriptorPool
  -> composed dictionary list
  -> SchemaModel
  -> normalized schema hash
```

This avoids scope creep into runtime, MBT-PG, MBT-Cache, or schema-registry
source changes.

### 3. CLI And Config Contract

Result: pass.

The spec binds:

```text
--dictionary-source <proto-file>
```

as a repeatable flag accepted by all actions and surfaces. Empty values are
rejected. Absence preserves current behavior.

The bound Rust fields are exact:

```text
CodegenConfig.dictionary_sources
SchemaRequest.dictionary_sources
```

### 4. Descriptor Set Construction

Result: pass.

The spec requires `run_protoc` to pass both the target schema and explicit
dictionary source files as descriptor inputs. This closes the failure mode
where a dictionary source is explicit but not present in the descriptor pool.

The existing `--include_imports` behavior is preserved.

### 5. Dictionary Composition Semantics

Result: pass.

The spec binds deterministic order:

1. target source file dictionaries;
2. explicit dictionary source files in request order.

Duplicate names fail. This is stricter than merging identical values, but it
is safer because it preserves single ownership.

### 6. Existing Behavior Preservation

Result: pass.

The empty-list behavior is explicitly source-file-local, matching current
behavior. The test plan requires existing local dictionary fixtures to remain
valid.

### 7. Schema Hash Behavior

Result: pass.

The spec requires composed dictionaries to be stored in
`SchemaModel.dictionaries`. Current `normalized_hash` already includes
`model.dictionaries`, so the code path is sufficient if implemented as bound.

The test plan requires hash change proof when an explicit dictionary value
changes.

### 8. Crate Boundary And Dependencies

Result: pass.

Only `crates/codegen` is in scope. No new dependencies are approved. Runtime,
adapter, schema, generated, and proto files are out of scope.

### 9. Failure Contract

Result: pass.

The spec binds all relevant failure modes:

- missing flag value;
- empty dictionary-source value;
- missing dictionary source file;
- missing source descriptor;
- duplicate dictionary names;
- duplicate dictionary values;
- field references absent dictionary.

No raw-string fallback is approved.

### 10. Test Plan

Result: pass.

The required tests cover the new behavior, failure behavior, hash behavior,
CLI behavior, and existing behavior. Test paths are exact.

### 11. Generated Artifact Boundary

Result: pass.

The spec states that no generated artifacts are approved. That is correct for
this MBT prerequisite. Application generated schemas are downstream work after
MBT is released.

## Non-Blocking Risks

1. Exact helper names remain an implementation detail. The spec allows that
   while binding behavior and file boundaries.
2. `protoc` missing-file diagnostics may be raw descriptor errors. This is
   acceptable unless implementation proves a clearer typed wrapper is needed
   without changing behavior.

## Audit Conclusion

The spec is implementation-ready after user approval of the implementation
plan. No blocker remains.

Classification:

PEER_AUDIT_PASSED
