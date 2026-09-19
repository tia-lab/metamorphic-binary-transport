# `crates/core` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/core/src/codec.rs`: deterministic response-checksum helper for tests, reports, and benchmark evidence.
- `crates/core/src/envelope.rs`: MBT envelope constants, header encode/decode, checksum, checked schema validation, and trusted payload slicing.
- `crates/core/src/error.rs`: typed transport error surface and shared `Result` alias.
- `crates/core/src/lib.rs`: crate entrypoint and public re-export surface for schema-agnostic runtime helpers.
- `crates/core/src/output.rs`: cap-aware byte buffer and encoding helpers shared by generated boundary writers.
- `crates/core/src/runtime.rs`: schema marker trait plus generic encode, access, and inspection dispatch helpers.
- `crates/core/src/tests/mod.rs`: core test module registration.
- `crates/core/src/tests/test_codec.rs`: checksum behavior tests for the core codec helper.
- `crates/core/src/tests/test_envelope.rs`: envelope validation, corruption, schema mismatch, and trusted-payload tests.
- `crates/core/src/tests/test_output.rs`: checked output buffer and response-cap behavior tests.
- `crates/core/src/tests/test_runtime.rs`: generic runtime trait dispatch smoke tests.
