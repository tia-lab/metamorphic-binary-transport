# Telemetry migration implementation plan

Approved amendment (2026-09-09): [CSV string-array correction](csv_correction_amendment.md) extends the file bindings and validation plan.

Status: approved by the repository owner in conversation on 2026-09-09.
Date: 2026-09-09.
Spec: `docs/specs/mbt_telemetry_migration_SPEC.md`.
Audit: `mbt_telemetry_migration_peer_audit.md`, classification
`PEER_AUDIT_PASSED` with the stated single-agent review limitation.

## Outcome and scope

Replace bars with the telemetry example across the workspace, generated adapters,
tests and benchmarks. Move options to the neutral namespace without changing
generation algorithms. This functional phase does not complete historical
document cleanup or establish release readiness.

## Exact file and dependency boundary

The 60 existing paths in `file_bindings.md` and spec sections 6, 9, 18–21 are
the complete read/edit/create/remove bindings. New schema output is the finite
16-file matrix in spec section 9, not a handwritten copy of existing output.
The new wrapper `scripts/schema_codegen.py` only orchestrates approved codegen.
No new third-party dependency or dependency version change is permitted.

The old bars directory is removed only after the new crate, consumer migration,
tests and generated outputs are complete. Only the recorded tracked build fixture
is removed from nested target output; unrelated local build files are preserved.

## Execution order

1. Reread the approved spec, plan and mandatory locked-refresh protocols. Compare
   affected files with the observed hash manifest. Inspect any intervening diff;
   do not overwrite new user edits. Snapshot their current contents and initial
   path existence under `/tmp/mbt-telemetry-migration-backup`, refusing to overwrite
   an existing backup. Record initial dirty state and environment in the new
   evidence directory. Inventory the eight current bars generated outputs before
   any removal, including line and byte counts.
2. Prepare the new options path/package and update qualified lookups plus source
   fixtures. Run narrow generator tests while the existing workspace members
   still have valid source files. No descriptor/emission algorithm edit is allowed.
3. Install the exact proposed telemetry proto and move the compatibility proto,
   applying only the identity/dictionary changes in the spec. Implement the
   finite codegen wrapper. Generate all outputs before switching workspace module
   declarations. Install telemetry Cargo.toml/lib.rs with the existing optional
   feature structure, replace workspace membership and bench dependency, and
   regenerate compatibility outputs through the same wrapper.
4. Add the independent telemetry test oracle and migrate all four bars test
   files plus inspection snapshot. Add roundtrip/failure cases. Preserve the
   codegen nested UTC tests and all five compatibility test suites; update their
   fictional bitmask constants and values. Gate optional-adapter tests by feature.
5. Replace the single benchmark fixture producer in projection support with
   telemetry_rows. Migrate regression naming and the projection/compression
   consumers. Remove old baseline loaders, data paths, ratio fields and the serde
   DTO lane. Keep the timing boundaries and exact new lane counts in the spec.
   Add required machine/dataset metadata and fail rather than invent a rate for
   a nonpositive sample duration. Retain report overwrite protection.
6. Change the core build identity string and neutral compression sample. Remove
   obsolete bars source/generated/test files, old options/schema paths and the
   tracked build fixture. Update nested target ignore coverage. Update README,
   architecture and affected inventories to current commands and telemetry APIs.
   Add codegen check and all-feature test steps to CI with protoc availability.
   Preserve all unrelated dirty content and historical evidence.
7. Perform a pre-test source audit against the spec. Confirm codegen algorithms,
   core layout and dependencies are unchanged, all output owners are unique,
   trusted calls cite prior validation, and no unbound behavior was introduced.
8. Execute validation below in order. Diagnose failures before changing code or
   expectations. A required algorithm/dependency/schema change requires a spec
   amendment; no widening the plan during implementation.
9. Write the result review with observed identities, command outcomes, generated
   sizes, compile evidence, benchmark limitations and the remaining historical
   cleanup scope. Review the complete diff without committing or publishing it.

## Validation and expected outputs

| Check | Expected result |
| --- | --- |
| `cargo test -p mbt_codegen` | Existing valid/invalid fixtures, reproducibility and smoke compilation pass |
| Wrapper write, second write, check | Sixteen files generated; two SHA-256 manifests identical; check exits zero |
| Wrapper inspect and telemetry snapshot comparison | New identities and exact expected keys/dictionaries/projection; snapshot equals fresh stdout |
| Telemetry default-feature tests | Core behavior and deterministic replay pass without unrelated adapter compilation |
| Telemetry all-feature tests | Exact row/projection/adapter oracles, presence and UTC behavior pass |
| Compatibility all-feature tests | Existing all-fields, nullable arrays and failures remain covered |
| Core/compression/benches tests | Existing transport/compression contracts pass; new report schema/lanes are exact |
| Workspace all-feature tests and all-target/all-feature check | No remaining executable dependency on removed bars paths/types |
| All-feature build and formatting/diff checks | Exit zero without unapproved source changes |
| Generated line/byte inventory | Telemetry stays within each old bars output and aggregate bound |
| Timed telemetry checks/release build and dependency trees | Recorded observations; no new dependency or unsupported compile-time claim |
| Three benchmark commands from spec section 17 | New reports; nine regression lanes, nine projection lanes per row count, two compression lanes; no external baseline reads |

Exact test commands and feature selections are spec section 18. Additional
compile evidence commands are:

```text
/usr/bin/time -p cargo check -p mbt_schema_telemetry
/usr/bin/time -p cargo check -p mbt_schema_telemetry --all-features
/usr/bin/time -p cargo build --release -p mbt_schema_telemetry --all-features
cargo tree -p mbt_schema_telemetry
cargo tree -p mbt_schema_telemetry --all-features
```

Record stdout/stderr and status in `docs/evidence/mbt_telemetry_migration` using
the filenames bound in the spec. After tests, record benchmark invocations and
outputs there. No old benchmark number is copied into a new report.

## Rollback boundary

Rollback means restoring only migration-owned edits to their captured
pre-migration working-tree contents and removing only newly created migration
files. Compare current files first to avoid losing intervening edits. Do not use
`git reset`, `git clean`, a blanket checkout, or a history rewrite. Do not restore
the workspace to HEAD, because that would discard pre-existing changes.

## Known risks and stop conditions

- Neutral schema names and the build identifier change emitted bytes. Old
  payload compatibility is explicitly not promised.
- A small schema covers fewer shapes; removing all-fields or generator fixture
  coverage would violate the plan.
- The old bars nested-message assertion is retired only because existing
  generator nested-message coverage remains, not because telemetry tests prove it.
- Benchmarks describe a new dataset and fewer lanes. New results cannot be
  compared to old bars measurements as an equivalent workload.
- The new schema's acceptance, generated sizes, adapter semantics and build costs
  remain unproved until the specified commands run.
- Network/toolchain restrictions may require command approval. Preserve failures
  and request the necessary sandbox escalation when encountered.
- Historical documentation and root identity metadata still need a separately
  specified cleanup. Do not label this phase a fully sanitized mirror.

## Approval

Requested approval covers this functional migration, its explicit removals,
benchmark lane changes, validation and rollback boundary. It does not authorize
publishing, Git-history changes, removal of historical records or edits to the
internal upstream repository.

Metadata portability correction within step 5: replace GNU-only date parsing
in the already-bound compression.rs with standard-library epoch milliseconds;
retain error propagation. Validate with benches tests and the exact compression
smoke command. Rollback remains the captured compression.rs.
