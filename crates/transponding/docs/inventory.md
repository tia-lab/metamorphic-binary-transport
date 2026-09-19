# `crates/transponding` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/transponding/src/lib.rs`: crate entrypoint and export-only surface for shared column buffer contracts.
- `crates/transponding/src/runtime.rs`: transponded column structs, validity bitmap, size checks, and deterministic checksum helpers.
- `crates/transponding/src/tests/mod.rs`: transponding test module registration.
- `crates/transponding/src/tests/test_runtime.rs`: validity bitmap, column sizing, and checksum behavior tests.
