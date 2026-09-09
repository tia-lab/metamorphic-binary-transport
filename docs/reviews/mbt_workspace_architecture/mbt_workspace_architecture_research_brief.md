# MBT Workspace Architecture Research Brief

Status: complete for initial spec draft

Slug: `mbt_workspace_architecture`

Repository: `/home/tia/_DEV/MATHILDE/metamorphic-binary-transport`

## Task Class

Research and spec preparation only.

No Rust code, crate creation, dependency change, generated artifact, benchmark,
or implementation is authorized by this brief.

## Required Reads

Read evidence:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/research_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/architecture/repository_structure.md`
- repository root `Cargo.toml`
- repository root `README.md`
- repository `src/main.rs`

## Measured Object

The measured object for future implementation is the MBT workspace boundary:

```text
one repository workspace
  -> small MBT core crate
  -> separate codegen crate
  -> separate generated schema crates or app-local generated modules
  -> separate boundary adapter crates
  -> separate benchmark/evidence crate
```

This research brief does not measure runtime speed or compile time. It prepares
the architecture spec that will define how those measurements must be taken
after implementation.

## Candidate Approach

Use crate boundaries, not a single feature-heavy crate, as the primary
compile-surface isolation mechanism.

Target crate families:

```text
metamorphic_binary_transport_core
metamorphic_binary_transport_codegen
metamorphic_binary_transport_schema_*
metamorphic_binary_transport_json
metamorphic_binary_transport_protobuf
metamorphic_binary_transport_csv
metamorphic_binary_transport_arrow
metamorphic_binary_transport_parquet
metamorphic_binary_transport_benches
```

Application schemas may also generate bindings inside the application crate
instead of a shared schema crate.

## Expected MBT Binding Surface

The workspace architecture must preserve these boundaries:

- `.proto + MBT options` is the schema source of truth.
- MBT core owns envelope, schema metadata, errors, checked access, trusted
  access contract, and minimal runtime traits.
- Codegen owns proto descriptor parsing, option validation, deterministic
  generation, and generated artifact checks.
- Schema crates own generated schema bindings for a selected schema only.
- Adapter crates own format-specific conversion from validated MBT bytes or
  archived views.
- Benchmark crates own evidence generation and must not be a production
  dependency.

## Evidence Table

| Evidence type      | Source                                                                    | Observation                                                                                                                                                                                                               |
| ------------------ | ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Code-read evidence | `AGENTS.md`                                                               | The repository requires no code before approved spec and implementation plan, speed claims require run evidence, compile-time claims require build evidence, and MBT core must remain small.                              |
| Code-read evidence | `docs/invariants/core_invariants.md`                                      | Core invariants require schemas, adapters, codegen, and benchmarks to be separate surfaces; features are not the primary isolation mechanism.                                                                             |
| Code-read evidence | `docs/architecture/repository_structure.md`                               | The current architecture guardrail names target crate families and forbids a monolithic crate compiling every schema and adapter.                                                                                         |
| Code-read evidence | root `Cargo.toml`                                                         | The repository is currently a single skeleton package named `metamorphic-binary-transport`; no workspace members exist yet.                                                                                               |
| Code-read evidence | `README.md`                                                               | The repository currently documents governance first and says production implementation is not present yet.                                                                                                                |
| Hypothesis         | Experiment history summarized in protocol docs and architecture guardrail | The monolithic experiment crate caused compile-surface pressure by compiling schemas, adapters, generated protobuf DTOs, and benches together. This must be re-proved if used as a quantitative claim in this repository. |

## Unknowns

1. Exact crate names may be adjusted by peer audit before implementation.
2. Whether shared schema crates are needed immediately or app-local generated
   modules are sufficient for the first implementation remains open.
3. Exact wire/archive implementation dependency choices are not bound by this
   architecture brief.
4. Exact adapter APIs are not defined here.
5. Exact schema option names and option package path need a separate options
   spec or a later workspace implementation spec section.
6. Compile-time budgets must be converted into concrete command thresholds in
   implementation plans.

## Risks

1. A workspace split alone does not reduce generated function size inside a
   single wide schema crate.
2. Adapter crates can still become compile-heavy if they emit one large writer
   per wide schema without budget checks.
3. Features can reintroduce cross-crate compile coupling if used as the primary
   architecture boundary.
4. Codegen can accidentally depend on runtime crates and recreate the
   experiment compile issue.
5. Schema crates can accidentally compile unrelated generated schemas if they
   are grouped too broadly.
6. SDK convenience packages can accidentally depend on all adapters and defeat
   opt-in behavior.

## Required Decisions Before Implementation

1. Confirm crate-boundary names and ownership.
2. Confirm that MBT core has no adapter dependencies.
3. Confirm codegen is a separate crate or tool surface and does not compile the
   full runtime/schema graph to run.
4. Confirm whether the initial implementation creates empty crate skeletons only
   or also moves the current root package into a workspace member.
5. Confirm that benchmark/evidence code is never part of production library
   crates.
6. Confirm exact validation commands for the skeleton phase.

## Recommended Next Phase

Write `docs/specs/mbt_workspace_architecture_SPEC.md`, then run a separate peer
audit before any implementation plan.

The spec should bind only workspace architecture and crate boundaries. It
should not attempt to implement MBT runtime, codegen, schemas, or adapters in
the same step.
