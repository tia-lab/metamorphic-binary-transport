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

# Corrective Result Review: MBT Bars Regression Benchmark

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15
Status: `BLOCKED_BY_PARITY_FAILURE`

## Result

The corrective benchmark implementation ran, but the result is not accepted as
an old-vs-new performance comparison.

The approved gate requires matching rows by `label` and `row_count`, then
verifying semantic checksum equality before computing speed ratios. That gate
failed for every semantic benchmark row in the paired run set.

## Evidence

Old parity evidence:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_1.json
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_2.json
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_3.json
```

New split evidence:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_4.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_5.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_6.json
```

Projection companion evidence:

```text
docs/evidence/mbt_projection_direct_writer/projection_run_6.json
```

Validation commands:

```text
cargo test -p mathilde_binary_transport --features bars-regression-parity-only test_bars_regression_parity
/usr/bin/time -v cargo check -p mathilde_binary_transport --all-targets --features bars-regression-parity-only
rustfmt --check crates/mathilde-binary-transport/src/lib.rs crates/mathilde-binary-transport/src/benches/bars_regression_parity.rs crates/mathilde-binary-transport/src/benches/mod.rs crates/mathilde-binary-transport/src/main.rs crates/mathilde-binary-transport/src/tests/test_bars_regression_parity.rs crates/mathilde-binary-transport/src/tests/mod.rs
rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!" crates/mathilde-binary-transport/src/lib.rs crates/mathilde-binary-transport/src/benches/bars_regression_parity.rs crates/mathilde-binary-transport/src/benches/mod.rs crates/mathilde-binary-transport/src/main.rs crates/mathilde-binary-transport/src/tests/test_bars_regression_parity.rs crates/mathilde-binary-transport/src/tests/mod.rs
rustfmt --check --config skip_children=true crates/mathilde-binary-transport/src/lib.rs crates/mathilde-binary-transport/src/benches/bars_regression_parity.rs crates/mathilde-binary-transport/src/benches/mod.rs crates/mathilde-binary-transport/src/main.rs crates/mathilde-binary-transport/src/tests/test_bars_regression_parity.rs crates/mathilde-binary-transport/src/tests/mod.rs
```

Observed validation:

| Check | Result |
| --- | --- |
| old parity tests | passed, 5 tests |
| old parity all-targets check | passed; first recorded run 1.39 s wall, max RSS 263256 KB; final warm rerun 0.18 s wall, max RSS 65544 KB |
| recursive bound rustfmt check from `lib.rs` | failed on unrelated old codegen/test child modules outside the corrective patch |
| touched-file rustfmt check with `skip_children=true` | passed |
| forbidden-pattern scan | passed, no matches |

## Parity Failure

Paired run comparison:

| Old run | New run | Matched rows | Semantic rows checked | Semantic mismatches | Output-byte mismatches |
| --- | --- | ---: | ---: | ---: | ---: |
| `bars_regression_parity_run_1.json` | `bars_regression_run_4.json` | 60 | 54 | 54 | 54 |
| `bars_regression_parity_run_2.json` | `bars_regression_run_5.json` | 60 | 54 | 54 | 54 |
| `bars_regression_parity_run_3.json` | `bars_regression_run_6.json` | 60 | 54 | 54 | 54 |

The mismatch is therefore structural, not jitter.

## Diagnosed Causes

Code-read evidence:

- Old generated Bars emits derived UTC fields in metamorphose outputs:
  `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`
  contains `open_utc`, `close_utc`, `writer.utc(...)`, and UTC CSV columns.
- New split Bars core generated module does not emit those UTC fields:
  `crates/schemas/bars_core/src/bars_v1.rs` has no UTC emission path.
- Old and new checksum contracts differ:
  - old `semantic_checksum` and `minimal_projection_checksum` use separate
    seeds and old minimal excludes metadata;
  - new core `minimal_projection_checksum(rows)` currently aliases
    `semantic_checksum(rows)`.
- Old and new full MBT byte sizes differ for the same row count. At 100000
  rows, old full MBT output is `32400138` bytes, while new split full MBT
  output is `33600140` bytes.

These differences mean the run set does not prove equal logical output for the
measured lanes.

## Diagnostic 100000-Row Metrics

The following values are diagnostic only. They are not accepted speed ratios
because the semantic parity gate failed.

| Label | Old avg rows/s | Old bytes | New avg rows/s | New bytes |
| --- | ---: | ---: | ---: | ---: |
| `bars_mbt_full_encode_inspect_checked` | 644091.51 | 32400138 | 385894.32 | 33600140 |
| `bars_metamorphose_json_trusted` | 214656.75 | 140088238 | 363391.64 | 127988238 |
| `bars_metamorphose_protobuf_trusted` | 321657.39 | 50450538 | 2050897.74 | 32350538 |
| `bars_metamorphose_csv_trusted` | 326946.02 | 52289248 | 584902.70 | 33289070 |
| `bars_metamorphose_arrow_ipc_trusted` | 2413451.80 | 32159688 | 1901223.13 | 32159432 |
| `bars_metamorphose_parquet_trusted` | 671348.55 | 17407852 | 638692.88 | 17407584 |

## Conclusion

The compile-isolation correction is proved for the old parity-only feature.

The performance comparison is not proved. The next correction must make the
old parity port and new split benchmark use one normalized semantic/output
contract, or explicitly narrow the benchmark to lanes where the output contract
is identical before speed ratios are computed.
