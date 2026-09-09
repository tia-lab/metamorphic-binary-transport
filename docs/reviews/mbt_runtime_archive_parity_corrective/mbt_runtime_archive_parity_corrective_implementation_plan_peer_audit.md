# Implementation Plan Peer Audit: MBT Runtime Archive Parity Corrective

Slug: `mbt_runtime_archive_parity_corrective`

Audited plan:

```text
docs/reviews/mbt_runtime_archive_parity_corrective/mbt_runtime_archive_parity_corrective_implementation_plan.md
```

Classification: `PEER_AUDIT_PASSED`

## Audit Evidence

| Evidence                                                                                                          | Finding                                                                                                           |
| ----------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `docs/specs/mbt_runtime_archive_parity_corrective_SPEC.md`                                                        | Spec binds the same dependency, codegen, generated-file, validation, and benchmark surfaces as the plan.          |
| `docs/reviews/mbt_runtime_archive_parity_corrective/mbt_runtime_archive_parity_corrective_implementation_plan.md` | Plan lists exact files to edit and exact generated files.                                                         |
| `crates/codegen/src/config.rs`                                                                                    | Existing codegen CLI requires `--surface`; adapter flags are not needed for core regeneration.                    |
| `crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto`                                          | Bars root message is `mathilde.binary_transport.v1.MathildeTransportResponseV1`.                                  |
| `crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto`   | Test compatibility root message is `mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1`. |

## Falsification Checks

### File bindings

Passed. The plan does not authorize unbounded edits.

### Generated artifacts

Passed. Generated core schema files are regenerated through codegen only.

### Dependency changes

Passed. The plan adds no new dependency and centralizes the existing `rkyv`
version plus old `unaligned` feature.

### Validation commands

Passed. The plan includes format, codegen tests, schema checks, schema tests,
dependency proof, trusted access grep proof, and benchmark reruns.

### Benchmark claims

Passed. The plan requires a result review and does not claim speed parity before
run evidence exists.

### Rollback boundary

Passed. The rollback boundary is explicit and excludes destructive git commands.

## Residual Risks

- The benchmark may still be slower after these two fixes. The plan correctly
  requires a result review rather than a claimed outcome.

## Decision

The implementation plan is ready for explicit approval.

Next command:

```text
Approved: implement docs/reviews/mbt_runtime_archive_parity_corrective/mbt_runtime_archive_parity_corrective_implementation_plan.md
```
