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

# Result Review: MBT Bars Regression Benchmark

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15
Status: `RESULT_REVIEW_SUPERSEDED_BY_CORRECTIVE_PARITY_PORT_SPEC`

## Supersession

This result review is no longer authoritative for old-vs-new parity claims.

The prior implementation compared the new split benchmark against historical
rows from:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md
```

That historical markdown file is retained as context only. It is not the
corrective baseline for this pass.

## Current Corrective Baseline

The accepted baseline is now an old-MBT parity-port benchmark that runs the old
implementation under the current new benchmark semantics:

```text
old MBT implementation + new benchmark semantics
vs
new split MBT implementation + same benchmark semantics
```

The corrective evidence path for the old implementation is:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_*.json
```

The corrective evidence path for the new split implementation remains:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_*.json
```

## Existing Artifacts

Existing artifacts:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_1.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_2.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_3.json
```

are evidence of the first benchmark implementation behavior only. They must
not be used to claim old-vs-new parity until paired with matching old parity
port reports produced by the corrective plan.

## Required Follow-Up

After validation and benchmark runs, write:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

That corrective result review must load at least three old parity reports and
three new split reports, match rows by label and row count, verify full MBT
semantic checksum equality before speed ratios, and report instability rather
than hiding it.
