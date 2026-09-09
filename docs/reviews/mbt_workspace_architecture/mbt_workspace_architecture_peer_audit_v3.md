# MBT Workspace Architecture Peer Audit V3

Classification: `BLOCKED`

Slug: `mbt_workspace_architecture`

Audited spec:

```text
docs/specs/mbt_workspace_architecture_SPEC.md
```

Supersedes:

```text
docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_peer_audit_v2.md
```

This audit reviews the amended workspace architecture that replaces the
previous three-crate verbose skeleton with the short-folder full workspace
shape.

## Required Reads

Completed for this audit:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/specs/mbt_workspace_architecture_SPEC.md`
- `docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_peer_audit_v2.md`

## Blocking Findings

### 1. Adapter And Metamorphose Ownership Is Still Ambiguous

Evidence:

- `docs/specs/mbt_workspace_architecture_SPEC.md` section 8 creates both:
  - `crates/metamorphose/src/json.rs`
  - `crates/metamorphose/src/protobuf.rs`
  - `crates/metamorphose/src/csv.rs`
  - `crates/adapters/json/src/lib.rs`
  - `crates/adapters/protobuf/src/lib.rs`
  - `crates/adapters/csv/src/lib.rs`
- Section 10 says format-specific implementation belongs "either" in
  `crates/metamorphose` when dependency-free or in adapter crates when
  dependencies are format-specific.

Why this blocks:

The spec does not define one exact ownership model. This can recreate duplicate
format surfaces: dependency-free writers in `metamorphose` plus adapter crates
for the same formats. That violates the no-drift and zero-duplication
invariants.

Required amendment:

Define exactly one of these shapes:

1. `crates/metamorphose` owns only public traits/enums and shared runtime
   helpers, while all concrete format writers live in `crates/adapters/*`; or
2. `crates/metamorphose` owns dependency-free JSON/protobuf/CSV writers and the
   adapter crates are not created for those formats in the skeleton.

The audit does not choose the design. The spec must choose it.

### 2. Protobuf Options Ownership Is Claimed But No File Structure Is Bound

Evidence:

- Section 7 says the MBT repository owns MBT option definitions.
- Section 8 canonical tree does not include any `proto/` or options file.
- Section 25 defers exact protobuf option ownership and package layout.

Why this blocks:

The spec simultaneously claims ownership and defers the file/package layout.
This leaves a core schema/codegen surface undefined. The user requirement is
that the entire folder and file structure is defined up front to prevent drift.

Required amendment:

Either:

- bind the exact options file path and package ownership now, for example a
  `proto/.../options.proto` path; or
- remove options ownership from this skeleton and state that no options files
  are owned or created in this phase.

The current mixed state is not precise enough.

### 3. Skeleton Source And Manifest Contents Are Not Exact

Evidence:

- Section 8 lists the complete file tree.
- Section 9 binds the root workspace members.
- Section 20 lists allowed code-binding paths.
- Section 23 requires the implementation plan to bind exact skeleton code.

Why this blocks:

The previous v2 audit required exact skeleton decisions at the spec level
before implementation planning. The amended spec now creates many more crates
and files, but it does not bind exact `Cargo.toml` contents, exact `lib.rs`
contents, exact placeholder module text, or exact test-module text for the new
crates.

This leaves too much room for implementation-plan drift, especially because
adapter, projection, metamorphose, and transponding crates are now created in
the skeleton.

Required amendment:

For every crate created by the skeleton, the spec must bind:

- package name;
- publish policy;
- workspace package fields;
- dependency list;
- exact module declarations;
- exact placeholder source text;
- exact test module content if a `tests/mod.rs` file is created.

If the spec intentionally delegates exact source text to the implementation
plan, it must reverse the v2 strictness and justify why that is acceptable.

### 4. Generated Output Surface Is Broader Than The Skeleton Can Validate

Evidence:

- Section 12 defines required output surfaces for core schema output,
  projection output, JSON, protobuf, CSV, columnar/transponding, Arrow, Arrow
  IPC, Parquet, and benchmark/check output.
- Section 21 forbids generated artifacts in this phase.
- Section 19 test plan includes checks against generated output imports even
  though no generated output exists.

Why this blocks:

The spec mixes final architecture output surfaces with skeleton validation.
That is not necessarily wrong, but the validation plan cannot prove generated
output behavior in a phase that forbids generated output. The spec must
separate "reserved future output surfaces" from "current skeleton correctness
oracle".

Required amendment:

Split section 12 into:

- reserved future generated output contracts; and
- current skeleton checks that can actually be run now.

Then remove generated-output import checks from the current skeleton test plan,
or label them explicitly as later-codegen-spec checks.

### 5. Existing Verbose-Path Code Is Blocked But Cleanup Authority Is Not Fully Bound

Evidence:

- Section 2 says existing code or artifacts targeting verbose folders are
  stale.
- Section 20 says the implementation plan must remove the rejected verbose
  paths if they exist.
- Current repository inspection shows the rejected folders exist now:
  - `crates/metamorphic_binary_transport_core`
  - `crates/metamorphic_binary_transport_codegen`
  - `crates/metamorphic_binary_transport_benches`

Why this blocks:

The spec correctly identifies the rejected paths, but it does not bind whether
the corrected architecture implementation may delete `Cargo.lock` entries or
target artifacts generated by the stale skeleton. It lists `Cargo.lock` in the
canonical tree and code bindings, but does not state the allowed lockfile
behavior after deleting the old crates.

Required amendment:

Define whether `Cargo.lock` is:

- preserved as Cargo-generated state;
- regenerated by `cargo check --workspace`;
- deleted and regenerated;
- or untouched.

Also state that `target/` is build output and not part of source cleanup.

## Non-Blocking Observations

### Short Folder Names Are Correctly Bound

Evidence:

Section 3 and section 8 define short workspace folders:

```text
crates/core
crates/codegen
crates/projection
crates/metamorphose
crates/transponding
crates/adapters/*
crates/benches
```

This resolves the user-level naming concern.

### Downstream Runtime Spec Is Correctly Blocked

Evidence:

`docs/specs/mbt_core_runtime_migration_SPEC.md` now has status:

```text
BLOCKED_BY_MBT_WORKSPACE_ARCHITECTURE_AMENDMENT
```

This prevents stale verbose-path runtime migration from authorizing code.

### Root Workspace Intent Is Clear

Evidence:

Section 9 requires a virtual workspace and no root binary/library after the
skeleton implementation.

This is consistent with the compile-surface invariant.

## Required Amendment Summary

Before implementation planning, amend the workspace architecture spec to:

1. choose the exact metamorphose/adapters ownership model;
2. bind or explicitly defer protobuf options file/package ownership without
   contradiction;
3. bind exact skeleton manifest and source text for every created crate/file;
4. separate future generated-output contracts from current skeleton validation;
5. define `Cargo.lock` behavior and build-output cleanup boundaries.

## Final Classification

`BLOCKED`

The spec is directionally aligned with the requested full workspace structure,
but it is not implementation-plan ready. The blocking items must be amended and
audited again before any implementation plan is accepted.
