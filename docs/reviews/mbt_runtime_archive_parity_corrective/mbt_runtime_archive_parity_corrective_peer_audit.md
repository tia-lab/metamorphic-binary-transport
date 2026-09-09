# Peer Audit: MBT Runtime Archive Parity Corrective

Slug: `mbt_runtime_archive_parity_corrective`

Audited spec:

```text
docs/specs/mbt_runtime_archive_parity_corrective_SPEC.md
```

Classification: `PEER_AUDIT_PASSED`

## Audit Evidence

| Evidence                                                                                                  | Finding                                                                                    |
| --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `docs/specs/mbt_runtime_archive_parity_corrective_SPEC.md`                                                | Mandatory spec sections are present in the required order.                                 |
| `/home/tia/_DEV/MATHILDE/experiments/Cargo.toml:24`                                                       | Old MBT used `rkyv` with `unaligned`.                                                      |
| `crates/schemas/bars_core/Cargo.toml:18`                                                                  | New Bars schema crate currently lacks `unaligned`.                                         |
| `crates/schemas/test_compatibility_core/Cargo.toml:18`                                                    | New compatibility schema crate currently lacks `unaligned`.                                |
| `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/codegen/rust_emit.rs:1409-1416` | Old trusted access is header/schema payload extraction plus unchecked rkyv access.         |
| `crates/codegen/src/rust_emit.rs:2527-2542`                                                               | New trusted access currently performs extra header decode and archived payload validation. |

## Falsification Checks

### Pre-audit closure gate

Passed. The spec includes exact code bindings, generated artifact bindings,
command surfaces, compile checks, benchmark methodology, and correctness oracle.

### Measured object clarity

Passed. The spec targets only archive layout and generated trusted access.

### Schema ownership

Passed. Proto schemas are unchanged. Generated files remain owned by codegen.

### Trusted-access safety

Passed with constraint. The spec keeps checked access defensive and restores
old trusted semantics only for bytes already validated for the same schema and
then stored or transported without mutation.

### Dependency containment

Passed. No new dependency is introduced. The only dependency change is enabling
the old `rkyv` `unaligned` feature through workspace dependencies.

### Benchmark isolation

Passed. The spec reuses the existing Bars regression benchmark and does not add
shortcuts or new measured objects.

### Correctness oracle

Passed. The spec explicitly prevents treating old/new semantic checksum fields
as comparable unless the result review proves the algorithms match.

## Residual Risks

- Runtime parity may still fail after these fixes. If so, a new evidence-bound
  corrective pass is required.
- `Cargo.lock` is not expected to change. If it changes, implementation must
  stop and explain the diff.

## Decision

The spec is implementation-plan ready.
