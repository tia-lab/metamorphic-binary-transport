# Corrective Result Review: MBT Bars Regression Benchmark

Slug: `mbt_bars_regression_benchmark`

Status: `CORRECTIVE_RERUN_COMPLETE_CORE_PARITY_RESTORED_ADAPTER_GAPS_OPEN`

## Scope

This review records the result of the runtime/archive corrective pass:

- `rkyv` `unaligned` restored through workspace dependency binding;
- generated trusted access restored to the trusted payload plus
  `rkyv::access_unchecked` shape;
- generated checksum helpers restored to seeded field-byte folding without
  re-hashing the current checksum bytes before every field;
- generated checked inspect restored to archived row validation plus separate
  semantic and minimal checksum paths;
- projection-surface schema files regenerated from codegen;
- all-fields unaligned archived numeric array compilation fixed in codegen;
- Bars regression benchmark rerun three times.

## Codegen And Static Proof

Generated files:

```text
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Both files were regenerated with:

```text
--surface projection
--proto-root proto
```

Reproducibility checks passed for both generated files with the same arguments
and `--check`.

Trusted-access scoped negative proof:

```text
for file in crates/schemas/bars_core/src/bars_v1.rs crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs; do
  awk '/pub unsafe fn access_archived_trusted_unchecked/{inside=1} inside{print} inside && /^    }$/{inside=0}' "$file" \
    | rg -n "validate_archived_payload|let header = decode_header" && exit 1 || true
done
```

Observed result:

```text
no matches
```

Trusted-access scoped positive proof found `trusted_payload_for_schema` and
`rkyv::access_unchecked` in the generated trusted functions for Bars full,
Bars projections, test-compatibility full, and test-compatibility projections.

Dependency proof:

```text
cargo tree -p metamorphic_binary_transport_schema_bars --no-default-features -e features | rg "rkyv|unaligned"
cargo tree -p metamorphic_binary_transport_schema_test_compatibility --no-default-features -e features | rg "rkyv|unaligned"
```

Observed result:

```text
rkyv v0.8.16 includes feature "unaligned" for both schema crates
```

## Validation Commands

All commands completed with exit status `0`:

```text
cargo fmt --check
cargo test -p metamorphic_binary_transport_codegen -- --nocapture
cargo check -p metamorphic_binary_transport_schema_bars --no-default-features
cargo check -p metamorphic_binary_transport_schema_test_compatibility --no-default-features
cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
cargo test -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv -- --nocapture
cargo test -p metamorphic_binary_transport_schema_test_compatibility --features json,protobuf,csv -- --nocapture
```

Observed test totals:

- codegen: `29 passed`;
- Bars schema feature tests: `7 passed`;
- test-compatibility schema feature tests: `17 passed`.

## Corrective Implementation Note

The first `test_compatibility_core` no-feature check exposed a real unaligned
archive issue in projection-surface array wrappers. The generated wrapper used
`ArchivedVec::serialize_from_slice` with already archived primitive values.
Under `rkyv` `unaligned`, those values are `Archived*` primitive aliases backed
by unaligned `rend` types and cannot serialize as native values.

The codegen fix streams archived primitive values into the projected archive as
native primitives through `ArchivedVec::serialize_from_unknown_length_iter`.
This avoids a temporary `Vec` and keeps the projection output write bounded to
the required projected MBT bytes.

The follow-up forensic pass exposed a generated checksum and checked-inspect
regression. The new emitter had generated `update_fixed` so it hashed the
current checksum bytes before every field, which changed checksum semantics and
added repeated work. It also generated checked inspect by hashing full archived
rows twice instead of using a separate minimal path. The corrected emitter now:

- calls `checksum_row(checksum, row, include_metadata)`;
- calls `checksum_archived_row(checksum, archived_row, include_metadata)`;
- emits `minimal_projection_archived_row`;
- validates archived rows during checked inspect through `row_from_archived`;
- keeps trusted access unchanged as trusted payload plus unchecked archive
  access.

## Benchmark Evidence

Command, run three times:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

Run evidence:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_11.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_12.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_13.json
```

The old evidence files used for comparison are:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_1.json
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_2.json
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_3.json
```

## 100k Row Snapshot

Rows/sec values are arithmetic means across three runs.

| lane                         | old rows/sec | new rows/sec | new/old | old bytes | new bytes |
| ---------------------------- | -----------: | -----------: | ------: | --------: | --------: |
| MBT encode + inspect checked |   644091.509 |   635666.617 |   0.987 |  32400138 |  32400138 |
| JSON checked                 |   197932.199 |   207980.727 |   1.051 | 140088238 | 165788238 |
| JSON trusted                 |   214656.748 |   246297.061 |   1.147 | 140088238 | 165788238 |
| Protobuf trusted             |   321657.392 |   316561.854 |   0.984 |  50450538 |  50450538 |
| CSV trusted                  |   326946.017 |   347760.087 |   1.064 |  52289248 |  51689248 |
| Arrow IPC trusted            |  2413451.797 |  2012328.780 |   0.834 |  32159688 |  32159432 |
| Parquet trusted              |   671348.549 |   628384.798 |   0.936 |  17407852 |  17407584 |
| serde JSON baseline          |   520330.169 |   489194.992 |   0.940 | 108598210 | 108598210 |

## Evidence Assessment

Proved:

- schema crates compile with `rkyv` `unaligned`;
- generated trusted access no longer performs full archived payload validation
  inside `access_archived_trusted_unchecked`;
- projection-surface generation is reproducible;
- Bars and all-fields schema correctness tests pass after regeneration;
- MBT full checked encode plus inspect has matching 100k output bytes and is
  within about `1.3%` of the old evidence mean;
- protobuf trusted output has matching 100k output bytes and is within about
  `1.6%` of the old evidence mean;
- JSON and CSV are faster in rows/sec in the current evidence, but not an
  apples-to-apples byte comparison because output sizes differ from the old
  evidence.

Not proved:

- old-vs-new semantic checksum parity for the benchmark evidence files;
- JSON/CSV old-vs-new parity, because the current new evidence includes the
  derived UTC output surface while the available old evidence has different
  output sizes.
- Arrow IPC old-vs-new parity;
- Parquet old-vs-new parity.

The available old benchmark evidence is therefore not a complete apples-to-
apples reference for the current generated output surface. The old source tree
currently contains derived UTC output fields, but the old evidence files listed
above were produced earlier and have different JSON/CSV output sizes.

## Open Findings

The previous MBT encode + inspect regression is no longer present in the
current three-run evidence:

```text
old mean: 644091.509 rows/sec
new mean: 635666.617 rows/sec
ratio: 0.987
output bytes: identical at 32400138
```

Remaining adapter gaps at 100k rows:

```text
Arrow IPC trusted ratio: 0.834
Parquet trusted ratio: 0.936
```

No Arrow IPC or Parquet performance parity claim is accepted from this
corrective pass. Core MBT encode plus checked inspect parity is accepted for
the recorded 100k Bars benchmark lane under the run evidence listed above.
