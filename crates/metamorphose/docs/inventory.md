# `crates/metamorphose` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/metamorphose/src/lib.rs`: crate entrypoint and export-only surface for metamorphose runtime traits and helpers.
- `crates/metamorphose/src/runtime.rs`: public metamorphose format enum, output enum, schema traits, safe dispatch, and trusted-access dispatch helpers.
- `crates/metamorphose/src/tests/mod.rs`: metamorphose test module registration.
- `crates/metamorphose/src/tests/test_runtime.rs`: runtime dispatch and trusted-token behavior tests.
