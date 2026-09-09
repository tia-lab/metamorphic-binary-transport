# Proposed CSV string-array correction

Status: approved by owner on 2026-09-09 (“proceed pelase”); implementation authorized.
Parent: `docs/specs/mbt_telemetry_migration_SPEC.md`.

## Proven failure

`cargo test -p mbt_schema_telemetry --all-features` fails the independent CSV
read-back oracle in `test_telemetry_metamorphose.rs`. Raw result is retained in
`docs/evidence/mbt_telemetry_migration/telemetry_tests_retry.txt`.
JSON, protobuf and columnar checks in the same test binary pass.

Code-read diagnosis: `emit_csv_bitmask_helpers` in
`crates/codegen/src/rust_emit.rs` emits `writer.string_cell` inside an already
quoted array cell. `CsvWriter::string_cell` quotes a complete CSV cell; it does
not escape a JSON string nested inside that cell. For two simple values, the
current writer produces `"["indoor","test"]"`; valid CSV for the JSON-array
value is `"[""indoor"",""test""]"`.

## Exact amendment to the parent contracts

Sections 4 and 9: permit one shared CSV generator call-site correction. No
descriptor/schema/hash/projection algorithm changes. Section 19 additionally
binds `crates/adapters/csv/src/lib.rs`, its `src/tests/mod.rs`, and
`crates/codegen/src/rust_emit.rs`. Existing bound generator tests and both
generated CSV outputs are affected. All other parent restrictions remain.

Add `CsvWriter::array_string_cell(&str) -> Result<()>`, documented for use between
begin_array_cell/end_array_cell. It writes one JSON string with CSV escaping:

- Opening and closing JSON quotes become two CSV quote bytes each.
- An embedded quote becomes backslash followed by two quote bytes.
- An embedded backslash becomes two backslashes.
- ASCII control bytes 0..31 become lowercase `\u00xx` JSON escapes.
- Other UTF-8 bytes are copied unchanged through CheckedBytes.

No intermediary String/Vec, dependency, configuration or writer-state field is
added. Every byte goes through the existing cap checks. Numeric arrays and
ordinary string cells retain their behavior. Change only the emitted call in
emit_csv_bitmask_helpers to `writer.array_string_cell`.

This changes previously malformed CSV output, not MBT bytes or schema identity.
Generated CSV files must come from the existing approved wrapper; never hand-edit.
Parent generated-file size bounds remain in force. No throughput claim.

## Correction implementation plan

1. Snapshot the three newly bound files before edits; add their SHA-256 values to
   the affected-file manifest. Preserve any pre-existing changes.
2. Add the single writer method and correct the generator call site.
3. Add adapter tests for two simple strings, quotes, backslashes, controls,
   Unicode, empty strings and response-cap overflow. Retain existing tests.
4. Add a generator assertion that bitmask CSV uses the new method.
5. Run `cargo test -p mbt_adapter_csv -p mbt_codegen`; preserve failure logs.
6. Run wrapper write twice and check; repeat the exact telemetry CSV reader
   oracle without weakening its expected values. Run remaining parent checks.

Expected result: standard Python csv.reader recovers one JSON-array cell, and
json.loads recovers the original strings. Byte-for-byte checked/trusted CSV
parity remains. Old invalid CSV expectations, if any, are replaced only with
the independently derived valid encoding above.

Rollback is restricted to the three newly bound source files and the two
regenerated CSV files, restoring captured pre-correction contents. No history or
unrelated-file operation.
