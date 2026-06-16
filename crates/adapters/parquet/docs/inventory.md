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

# `crates/adapters/parquet` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/adapters/parquet/src/arrow_bridge.rs`: Arrow array and record-batch bridge helpers reused before Parquet writing.
- `crates/adapters/parquet/src/lib.rs`: uncompressed Parquet writer and adapter exports for generated metamorphose modules.
- `crates/adapters/parquet/src/tests/mod.rs`: Parquet adapter tests for record-batch conversion and Parquet byte writing.
