# Neutral mirror and placeholder schema research brief

Status: research complete; implementation not approved.
Date: 2026-09-09.
Task class: repository review and research.

## Goal and measured object

Identify company references and application-specific schema coupling in the
mirror, and propose a neutral example schema. The measured object is the current
tracked working tree, not the upstream system or published repository history.

## Findings and evidence

| Evidence type | Observation | Source |
| --- | --- | --- |
| Run | Initial `git status --short` returned no changes; `git ls-files` returned 332 paths. | Local checkout |
| Run | A case-insensitive byte scan of tracked files for `mathilde` or `mathidle` found 148 files and 1,868 matching lines. Groups: docs 117, crates 25, proto 1, and five root files. | `git ls-files -z`, Python byte-line regular-expression scan |
| Schema read | Options use the company namespace; both schema packages, imports and transport names reference it. Bars additionally contains pipeline metadata, repair reasons and source coverage fields. | `proto/mathilde/options.proto`; both schema crates' proto files |
| Code read | Option lookup hardcodes fully qualified company extension names. | `crates/codegen/src/options.rs:31` |
| Code read | Core build identity contains the company name. | `crates/core/src/envelope.rs:10` |
| Code read | Generated bars types and transport names retain references. Generated headers record bars hash 6061383958499356843 and compatibility hash 3233278346470496550. These values were read, not independently regenerated. | Both schema crates' generated core modules |
| Code read | Hash input includes transport name, row type, dictionary values and field paths. A replacement must recompute schema identity; unchanged identity cannot be assumed. | `crates/codegen/src/descriptor.rs:1393` |
| Code read | Regression benchmark uses an absolute internal source-evidence path and the bars row type. | `crates/benches/src/bars_regression.rs:12` |
| Run | A codegen fixture under a nested `target` directory is tracked, including an options proto with the old namespace. Root ignore rule is `/target`. | `git ls-files 'crates/codegen/target/**'`; `.gitignore` |
| Code read | Root metadata contains a company email and package name; package description references another internal project. | `Cargo.toml:23`; `package.json` |

The tracked scan includes ignored-but-tracked files. A separate `rg` working-tree
scan found 207 matching files; its ignore/untracked scope differs, so that number
is not the tracked-file count. Neither scan establishes absence of internal
information under other names or in binary artifacts.

Concurrent workspace changes appeared during review: the final `git diff --stat`
reported 138 modified files, including notice removal and inventory changes.
Those edits were not made by this review. Counts above are point-in-time scan
results, not a final inventory of the changing checkout. Re-scan before migration.

## Candidate approach and MBT binding surface

1. Move MBT options to `proto/mbt/options.proto`, namespace `mbt`, and update
   descriptor lookup and source test fixtures together.
2. Replace the application bars schema with a new synthetic telemetry example,
   provisionally `mbt.example.telemetry.v1`, with response and sample messages.
   Candidate fields: device dictionary key, timestamp key, numeric measurement,
   optional quality, synthetic tag bitmask and derived UTC text. Include a
   projection to demonstrate MBT-to-MBT conversion. Do not reproduce the internal
   bars pipeline under renamed fields.
3. Retain the all-fields correctness fixture in a neutral test namespace with
   fictional dictionary values. It covers types a small example does not.
4. Regenerate every owned output through approved codegen. Migrate example
   imports, adapters, fixtures, snapshots, benches and documented commands.
5. Replace company-specific prose, metadata and paths. Historical evidence needs
   an explicit export/removal policy: renamed old measurements are not evidence
   for a new schema. Review notices and the additional internal project name.
6. Remove the tracked build fixture through the approved plan and cover nested
   build output in ignore rules. Add a tracked-content/path check to prevent
   reintroduction, without storing the forbidden literal in that public check.

MBT owns options, core envelope and codegen. The proposed synthetic schema belongs
to the mirror's example surface; the internal application schema remains outside
it. No dependency addition is proposed. Core wire/archive and checked/trusted
access behavior should remain unchanged apart from explicitly specified identity
changes. This is a proposed constraint, not validated compatibility.

## Sources read and prior contracts

- `AGENTS.md`, lifecycle, research and review/documentation protocols, and core invariants.
- Relevant ownership sections of `mbt_workspace_architecture_SPEC.md`.
- Source-schema sections of `mbt_schema_core_generation_SPEC.md` and its result review.
- Both source schemas, options proto, generated core headers, codegen option
  lookup and normalized hash, benchmark source, manifests and CI workflow.

The existing schema-generation spec explicitly preserves the old namespace,
identity and field names. A new spec must explicitly supersede those bindings.
There is no neutral-mirror spec or implementation plan in this review.

## Unknowns, risks and evidence required before coding

- Exact replacement schema, schema ID, version, output paths and test migration
  need specification and peer audit. The candidate above is not an approved schema.
- Old consumers and stored bytes were not inspected. Compatibility must not be
  claimed; new hash values and wrong-schema rejection need run evidence.
- Correctness oracle: deterministic synthetic input with field-for-field adapter
  roundtrips, projection equivalence, corrupt-byte rejection, checked/trusted
  parity under the existing safety contract, and repeat-generation byte equality.
- Generated check commands must bind both neutral schemas and every output
  surface; run codegen `--check` after regeneration and the migrated test suites.
- Compile-surface budget and exact measurement commands remain to be specified.
  Run the existing all-target/all-feature CI checks after implementation.
- A synthetic benchmark needs a new dataset identity and reproducible local
  baseline. No speed or compile-time claim follows from this review.
- Git history, remotes, releases, external mirror automation and ignored local
  artifacts were not audited. Working-tree cleanup does not establish their cleanup.
- Removing names alone cannot prove removal of internal design information.

## Concrete placeholder schema proposal

Status: schema candidate for discussion, not an approved migration spec. This
section defines the example payload; it does not authorize source changes.

Use synthetic sensor readings. Each row describes one device at one instant.
There are no financial instruments, venues, processing stages, repair reasons,
internal ingestion timestamps or application finality fields.

Proposed source path:
`crates/schemas/telemetry_core/proto/mbt/example/telemetry/v1/telemetry.proto`.
Proposed crate: `mbt_schema_telemetry`; module: `telemetry_v1`.
The mirror owns this example schema. All fixture data is generated locally.

| Tag | Row field | Logical type | Meaning |
| --- | --- | --- | --- |
| 1 | schema_version | uint32, constant 1 | Existing MBT version annotation |
| 2 | device | dictionary string | Fictional device identifier; first key part |
| 3 | recorded_at_ms | int64 | Unix milliseconds; second key part |
| 4 | recorded_at_utc | derived string | UTC representation of recorded_at_ms; no archived string |
| 5 | temperature_c | double | Synthetic temperature in degrees Celsius |
| 6 | battery_percent | optional double | Missing differs from a present zero; presence bit 0 |
| 7 | status | dictionary string | Fictional device operating state |
| 8 | tags | repeated dictionary string | Unordered set represented by a bitmask |

Proposed full source, to be installed only through the approved migration:

```proto
syntax = "proto3";

package mbt.example.telemetry.v1;

import "mbt/options.proto";

option (mbt.dictionary_values) = {
  name: "device"
  value: "sensor_a"
  value: "sensor_b"
  value: "sensor_c"
};

option (mbt.dictionary_values) = {
  name: "status"
  value: "active"
  value: "idle"
  value: "offline"
};

option (mbt.dictionary_values) = {
  name: "tag"
  value: "indoor"
  value: "outdoor"
  value: "test"
};

message TelemetryResponseV1 {
  option (mbt.schema_id) = 50001;
  option (mbt.schema_version) = 1;
  option (mbt.transport_name) = "mbt.example.telemetry.v1";
  option (mbt.payload_root) = true;
  option (mbt.projection) = {
    name: "temperature_only"
    rust_marker: "TelemetryV1TemperatureOnly"
    include_field: "temperature_c"
  };

  uint32 schema_version = 1 [(mbt.const_u16) = 1];
  repeated TelemetryRowV1 rows = 2 [(mbt.repeated_payload) = true];
}

message TelemetryRowV1 {
  uint32 schema_version = 1 [(mbt.const_u16) = 1];
  string device = 2 [
    (mbt.dictionary) = "device",
    (mbt.key_part) = true,
    (mbt.key_order) = 1
  ];
  int64 recorded_at_ms = 3 [
    (mbt.key_part) = true,
    (mbt.key_order) = 2
  ];
  string recorded_at_utc = 4 [
    (mbt.ignored) = true,
    (mbt.derived_utc_from) = "recorded_at_ms"
  ];
  double temperature_c = 5;
  optional double battery_percent = 6 [(mbt.presence_bit) = 0];
  string status = 7 [(mbt.dictionary) = "status"];
  repeated string tags = 8 [(mbt.bitmask_dictionary) = "tag"];
}
```

The numeric schema ID is a proposed local allocation, distinct from the two
checked-in schema IDs observed in the source scan (1 and 40001). No global
uniqueness is claimed. It is independent of protobuf extension tag numbering.
Do not reuse the bars identity or claim compatibility with bars payloads. Let
codegen compute the new hash; no hash value is assigned in this proposal.

Dictionary declaration order is fixed. Synthetic batches use ascending device
order, then ascending recorded_at_ms, with unique keys. Tag order is not input
semantics; canonical rendering should follow dictionary order. The example
fixture producer uses finite temperatures and battery values within 0..100.
These are fixture constraints, not new runtime range-validation promises.

An example logical row is device `sensor_a`, recorded_at_ms `0`, temperature_c
`21.5`, battery_percent absent, status `active`, tags `indoor` and `test`.
The derived timestamp denotes `1970-01-01T00:00:00.000Z`. This is a logical
example, not a claim about exact adapter JSON formatting.

The temperature projection retains the version and both keys automatically,
plus temperature_c. Derived-field output continues to follow existing adapter
rules; the projection does not store a UTC string. The separate all-fields
fixture remains responsible for arrays, bytes, raw text, other scalar widths
and additional presence combinations. This compact schema cannot replace that
coverage or preserve the old bars benchmark dataset.

Code-read support: `descriptor.rs` validates contiguous presence bits from zero
and key orders from one (`validate_physical_fields`); resolves derived UTC to an
i64 source (`resolve_derived_utc_fields`); and retains constant/version and key
fields in projections (`is_mandatory_projection_field`). These mechanisms
support the candidate approach without a proposed generation-algorithm change.
Actual acceptance by codegen and all generated surfaces remains untested.

The neutral options import does not exist yet. The associated namespace migration
must precede generation. A future migration spec must bind every generated
surface, prior-spec supersession, displaced bars test and benchmark, command,
compile budget and evidence artifact before peer audit. This research proposal
deliberately makes no implementation-readiness or performance claim.

## Recommendation

Continue to spec authoring for a neutral mirror and synthetic schema migration.
Resolve historical-evidence handling and compatibility policy in that spec.
Complete peer audit and an approved implementation plan before code changes, as
required by repository rule 16. No source, schema or generated file was changed
by this review; no tests, builds or benchmarks were run.
