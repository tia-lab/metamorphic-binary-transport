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

# `crates/compression` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/compression/src/lib.rs`: export-only compression crate entrypoint.
- `crates/compression/src/runtime.rs`: opt-in zstd compression and decompression helpers for completed MBT bytes.
- `crates/compression/src/tests/mod.rs`: compression test module registration.
- `crates/compression/src/tests/test_runtime.rs`: compression roundtrip, cap, corrupt-input, and caller-buffer tests.
