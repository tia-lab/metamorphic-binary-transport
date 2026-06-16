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

# `crates/schemas/bars_core` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/schemas/bars_core/src/bars_v1.rs`: generated Bars core schema, envelope access, validation, checksums, views, and MBT-to-MBT projections.
- `crates/schemas/bars_core/src/bars_v1_arrow.rs`: generated Bars Arrow RecordBatch metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs`: generated Bars Arrow IPC metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_csv.rs`: generated Bars CSV metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_json.rs`: generated Bars JSON metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_parquet.rs`: generated Bars Parquet metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_protobuf.rs`: generated Bars protobuf metamorphose adapter module.
- `crates/schemas/bars_core/src/bars_v1_transponding.rs`: generated Bars row-to-column transponding module for columnar adapters.
- `crates/schemas/bars_core/src/lib.rs`: schema crate entrypoint and feature-gated generated adapter module declarations.
- `crates/schemas/bars_core/tests/test_bars_metamorphose.rs`: Bars metamorphose tests for derived UTC row outputs and protobuf nesting.
- `crates/schemas/bars_core/tests/test_bars_projection.rs`: Bars projection tests for checked/trusted parity and projected schema identity.
- `crates/schemas/bars_core/tests/test_bars_shape.rs`: Bars generated-shape tests for schema constants and public type behavior.
