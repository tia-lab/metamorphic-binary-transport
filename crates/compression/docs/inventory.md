# `crates/compression` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/compression/src/lib.rs`: export-only compression crate entrypoint.
- `crates/compression/src/runtime.rs`: opt-in zstd compression and decompression helpers for completed MBT bytes.
- `crates/compression/src/tests/mod.rs`: compression test module registration.
- `crates/compression/src/tests/test_runtime.rs`: compression roundtrip, cap, corrupt-input, and caller-buffer tests.
