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

# `crates/benches` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/benches/src/bars_regression.rs`: Bars regression fixtures, timing rows, report metadata, old-evidence parsing, and report writing helpers.
- `crates/benches/src/bin/mbt_bars_regression_bench.rs`: executable Bars regression benchmark timing MBT, projection, and metamorphose lanes.
- `crates/benches/src/bin/mbt_projection_bench.rs`: executable projection benchmark for Bars and test-compatibility schemas.
- `crates/benches/src/lib.rs`: benchmark crate entrypoint and module exports.
- `crates/benches/src/projection.rs`: projection benchmark fixtures, baseline parsing, measurement rows, and report helpers.
- `crates/benches/src/tests/mod.rs`: benchmark test module registration.
- `crates/benches/src/tests/test_bars_regression_bench_output.rs`: Bars regression report output tests.
- `crates/benches/src/tests/test_projection_bench_output.rs`: projection benchmark report output tests.
