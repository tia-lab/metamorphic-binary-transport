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

# `crates/transponding` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/transponding/src/lib.rs`: crate entrypoint and export-only surface for shared column buffer contracts.
- `crates/transponding/src/runtime.rs`: transponded column structs, validity bitmap, size checks, and deterministic checksum helpers.
- `crates/transponding/src/tests/mod.rs`: transponding test module registration.
- `crates/transponding/src/tests/test_runtime.rs`: validity bitmap, column sizing, and checksum behavior tests.
