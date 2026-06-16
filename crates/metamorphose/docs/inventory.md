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

# `crates/metamorphose` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/metamorphose/src/lib.rs`: crate entrypoint and export-only surface for metamorphose runtime traits and helpers.
- `crates/metamorphose/src/runtime.rs`: public metamorphose format enum, output enum, schema traits, safe dispatch, and trusted-access dispatch helpers.
- `crates/metamorphose/src/tests/mod.rs`: metamorphose test module registration.
- `crates/metamorphose/src/tests/test_runtime.rs`: runtime dispatch and trusted-token behavior tests.
