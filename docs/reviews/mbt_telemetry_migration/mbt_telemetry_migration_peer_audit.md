# Telemetry migration spec audit

Classification: `PEER_AUDIT_PASSED`.
Date: 2026-09-09.
Scope: design and file/command bindings, not implementation or release readiness.
Method: single-agent falsification review in a separate artifact. No independent
human or second-agent review is claimed.

## Findings

No blocking contradiction found in the bounded functional spec. Approval remains
required. This classification does not approve implementation and does not prove
that the candidate proto or generated Rust compiles.

Historical cleanup is explicitly excluded. The broader request to remove all
internal references remains incomplete after this phase and must not be reported
as fulfilled. The historical-document preference question remains separate.

## Evidence and challenges

| Challenge | Observed basis | Disposition |
| --- | --- | --- |
| Namespace-only generator change is insufficient if algorithms depend on bars | Descriptor model derives row names, fields and projections; options.rs owns qualified extension lookup | Preserve algorithms; migrate option/test source names only |
| Two generators might overwrite the same core module | emit.rs dispatches core and projection to distinct emitters; projection surface includes base plus projections | Spec assigns the main module exclusively to projection |
| Placeholder could silently change old wire acceptance | envelope.rs validates explicit schema identity; normalized_hash uses names, field paths and dictionaries | New telemetry ID, regenerated hashes and old-identity rejection tests bound |
| Build identifier rename could be presented as a cosmetic-only change | envelope.rs hashes BUILD_ID_INPUT and writes it at offset 32 | Explicitly declared byte change; magic/layout preserved |
| Two-part key or optional value could require generator changes | descriptor.rs requires contiguous key orders and presence bits; rust_emit.rs emits tuple-order and absent-value validation | Proposed schema follows those rules; actual generation is still a validation gate |
| Projection could drop mandatory keys | descriptor.rs retains constant/version and key fields | Temperature projection's expected physical fields explicitly defined |
| Schema replacement could remove nested-message and array coverage | codegen tests include nested derived UTC fixture; compatibility tests cover nullable arrays and scalar kinds | Those tests are preserved; application-specific assertions are mapped to telemetry |
| Benchmarks might read internal files or reuse old results | projection.rs and bars_regression.rs contain old baseline sources; compression.rs consumes bars fixtures | Remove old loaders/comparison contracts and migrate all three benchmark programs |
| A smaller fixture might inherit old performance claims | Prior benchmarks describe a different dataset and lane count | New dataset identity, directories and lane expectations; no comparative claim |
| Compilation might pull unrelated adapters into core | Existing schema manifests gate adapters by feature; core source change is one identity constant | Preserve manifest structure and dependencies; default/all-feature tree checks bound |
| Rollback could overwrite concurrent edits | Working tree is dirty before implementation | File-hash manifest and affected-file snapshot required; no git reset |

## Pre-audit closure review

The spec has the 24 required sections in order. Prior-spec supersession is
restricted to executable namespace/schema/benchmark bindings. Each generated
artifact has one command owner through a finite, explicit matrix. Code bindings
include benchmark dispatch, library modules, codegen lookup and test fixtures.
The file manifest records 60 existing paths. Test migration accounts for every
bars test file and preservation of compatibility and generator tests. Validation
commands, new artifact directories and compile-size gates are stated.

## Limitations

No codegen, tests, builds or benchmarks were run in this audit. Dependency
installation and local platform behavior remain runtime checks. The telemetry
output-size bound must be measured, not assumed. An implementation that requires
changes to generator algorithms, new dependencies or unspecified failure
semantics must return to spec authoring. Historical artifact handling is outside
this audit's scope.

Next: implementation plan and explicit approval of the concrete spec and plan.
