# `crates/benches` - Inventory

Protocol: code-only inventory. Documentation files are intentionally excluded.

## Source Files

- `crates/benches/src/telemetry_regression.rs`: Telemetry regression fixtures, timing rows, report metadata, report writing helpers.
- `crates/benches/src/compression.rs`: compression benchmark source construction, zstd timing, report writing, environment writing, and summary helpers.
- `crates/benches/src/bin/mbt_telemetry_regression_bench.rs`: executable Telemetry regression benchmark timing MBT, projection, and metamorphose lanes.
- `crates/benches/src/bin/mbt_compression_bench.rs`: executable compression benchmark for full and projected Telemetry MBT bytes.
- `crates/benches/src/bin/mbt_projection_bench.rs`: executable projection benchmark for Telemetry and test-compatibility schemas.
- `crates/benches/src/lib.rs`: benchmark crate entrypoint and module exports.
- `crates/benches/src/projection.rs`: projection benchmark fixtures, measurement rows, and report helpers.
- `crates/benches/src/tests/mod.rs`: benchmark test module registration.
- `crates/benches/src/tests/test_telemetry_regression_bench_output.rs`: Telemetry regression report output tests.
- `crates/benches/src/tests/test_compression_bench_output.rs`: compression benchmark report shape and row-count binding tests.
- `crates/benches/src/tests/test_projection_bench_output.rs`: projection benchmark report output tests.

- `crates/benches/src/bin/mbt_measurement_compression_bench.rs`: Recovered wide measurement benchmark or its correctness and report contract checks.

- `crates/benches/src/bin/mbt_measurement_projection_bench.rs`: Recovered wide measurement benchmark or its correctness and report contract checks.

- `crates/benches/src/bin/mbt_measurement_regression_bench.rs`: Recovered wide measurement benchmark or its correctness and report contract checks.

- `crates/benches/src/bin/support/measurement_owned.rs`: Recovered wide measurement benchmark or its correctness and report contract checks.

- `crates/benches/src/measurement_compression.rs`: Recovered wide measurement benchmark or its correctness and report contract checks.

- `crates/benches/src/measurement_projection.rs`: Recovered wide measurement benchmark or its correctness and report contract checks.

- `crates/benches/src/measurement_regression.rs`: Recovered wide measurement benchmark or its correctness and report contract checks.

- `crates/benches/src/tests/test_measurement_compression_bench_output.rs`: Recovered wide measurement benchmark or its correctness and report contract checks.

- `crates/benches/src/tests/test_measurement_projection_bench_output.rs`: Recovered wide measurement benchmark or its correctness and report contract checks.

- `crates/benches/src/tests/test_measurement_regression_bench_output.rs`: Recovered wide measurement benchmark or its correctness and report contract checks.
