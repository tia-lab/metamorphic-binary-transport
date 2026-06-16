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

# `crates/schemas/test_compatibility_core` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/schemas/test_compatibility_core/src/lib.rs`: schema crate entrypoint and feature-gated generated adapter module declarations.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`: generated all-fields compatibility core schema, validation, checksums, views, and projections.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs`: generated all-fields compatibility Arrow RecordBatch metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs`: generated all-fields compatibility Arrow IPC metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs`: generated all-fields compatibility CSV metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs`: generated all-fields compatibility JSON metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs`: generated all-fields compatibility Parquet metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs`: generated all-fields compatibility protobuf metamorphose adapter module.
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs`: generated all-fields compatibility row-to-column transponding module for columnar adapters.
- `crates/schemas/test_compatibility_core/tests/test_determinism.rs`: deterministic encoding, inspection, and row-order tests for the all-fields schema.
- `crates/schemas/test_compatibility_core/tests/test_failures.rs`: invalid input and corrupt payload failure tests for the all-fields schema.
- `crates/schemas/test_compatibility_core/tests/test_projection.rs`: all-fields projection correctness and schema identity tests.
- `crates/schemas/test_compatibility_core/tests/test_roundtrip.rs`: all-fields encode/access/inspect roundtrip tests across supported scalar and array field kinds.
