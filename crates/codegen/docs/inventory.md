# `crates/codegen` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/codegen/src/config.rs`: explicit CLI argument parser and validated codegen configuration model.
- `crates/codegen/src/descriptor.rs`: protobuf descriptor loading and MBT schema-model construction from custom options.
- `crates/codegen/src/emit.rs`: codegen action dispatcher, rustfmt integration, write/check handling, and inspect output.
- `crates/codegen/src/error.rs`: typed codegen error surface and shared `Result` alias.
- `crates/codegen/src/lib.rs`: crate entrypoint and module ownership boundary for descriptor parsing and Rust emission.
- `crates/codegen/src/main.rs`: `mbt_codegen` binary entrypoint and process-boundary error handling.
- `crates/codegen/src/model.rs`: resolved schema, field, dictionary, projection, and output-tree model types consumed by emitters.
- `crates/codegen/src/options.rs`: MBT custom option descriptor resolution and typed option extraction helpers.
- `crates/codegen/src/rust_emit.rs`: Rust code emitters for core schemas, projections, metamorphose adapters, and transponding surfaces.
- `crates/codegen/src/tests/mod.rs`: shared codegen fixtures, temporary output helpers, and test module registration.
- `crates/codegen/src/tests/test_cli.rs`: CLI parser validation tests for action, surface, adapter, and output argument rules.
- `crates/codegen/src/tests/test_descriptor.rs`: descriptor-to-model tests for supported schemas and rejected invalid schema shapes.
- `crates/codegen/src/tests/test_model.rs`: schema model helper tests for naming and deterministic model behavior.
- `crates/codegen/src/tests/test_options.rs`: MBT custom option parsing tests.
- `crates/codegen/src/tests/test_rust_emit_core.rs`: generated core schema smoke and check tests.
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`: generated metamorphose adapter smoke and check tests.
- `crates/codegen/src/tests/test_rust_emit_projection_direct_writer.rs`: generated direct projection writer smoke and check tests.
