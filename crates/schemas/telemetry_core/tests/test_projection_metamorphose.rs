#![cfg(any(
    feature = "json",
    feature = "csv",
    feature = "protobuf",
    feature = "arrow"
))]
mod common;
use common::*;
use mbt_schema_telemetry::telemetry_v1::*;

#[cfg(feature = "json")]
#[test]
fn json_values_and_trusted_parity() -> TestResult {
    let bytes = TelemetryV1::encode(&rows(), CAP)?;
    let bytes = TelemetryV1::project_temperature_only(&bytes, CAP)?;
    TelemetryV1TemperatureOnly::access(&bytes)?;
    let checked = TelemetryV1TemperatureOnly::metamorphose_json(&bytes, CAP)?;
    // SAFETY: checked access validated these immutable bytes above.
    let trusted =
        unsafe { TelemetryV1TemperatureOnly::metamorphose_json_trusted_unchecked(&bytes, CAP)? };
    assert_eq!(checked, trusted);
    check_row_format("json", true, &checked)
}

#[cfg(feature = "csv")]
#[test]
fn csv_values_and_trusted_parity() -> TestResult {
    let bytes = TelemetryV1::encode(&rows(), CAP)?;
    let bytes = TelemetryV1::project_temperature_only(&bytes, CAP)?;
    TelemetryV1TemperatureOnly::access(&bytes)?;
    let checked = TelemetryV1TemperatureOnly::metamorphose_csv(&bytes, CAP)?;
    // SAFETY: checked access validated these immutable bytes above.
    let trusted =
        unsafe { TelemetryV1TemperatureOnly::metamorphose_csv_trusted_unchecked(&bytes, CAP)? };
    assert_eq!(checked, trusted);
    check_row_format("csv", true, &checked)
}

#[cfg(feature = "protobuf")]
#[test]
fn protobuf_values_and_trusted_parity() -> TestResult {
    let bytes = TelemetryV1::encode(&rows(), CAP)?;
    let bytes = TelemetryV1::project_temperature_only(&bytes, CAP)?;
    TelemetryV1TemperatureOnly::access(&bytes)?;
    let checked = TelemetryV1TemperatureOnly::metamorphose_protobuf(&bytes, CAP)?;
    // SAFETY: checked access validated these immutable bytes above.
    let trusted = unsafe {
        TelemetryV1TemperatureOnly::metamorphose_protobuf_trusted_unchecked(&bytes, CAP)?
    };
    assert_eq!(checked, trusted);
    check_row_format("protobuf", true, &checked)
}

#[cfg(all(feature = "arrow", feature = "arrow_ipc", feature = "parquet"))]
#[test]
fn columnar_values_and_trusted_parity() -> TestResult {
    let bytes = TelemetryV1::encode(&rows(), CAP)?;
    let bytes = TelemetryV1::project_temperature_only(&bytes, CAP)?;
    TelemetryV1TemperatureOnly::access(&bytes)?;
    let arrow = TelemetryV1TemperatureOnly::metamorphose_arrow(&bytes, CAP)?;
    // SAFETY: all trusted adapters below use the same checked immutable bytes.
    let trusted =
        unsafe { TelemetryV1TemperatureOnly::metamorphose_arrow_trusted_unchecked(&bytes, CAP)? };
    assert_eq!(arrow, trusted);
    check_arrow(&arrow, true)?;
    let ipc = TelemetryV1TemperatureOnly::metamorphose_arrow_ipc(&bytes, CAP)?;
    let trusted_ipc = unsafe {
        TelemetryV1TemperatureOnly::metamorphose_arrow_ipc_trusted_unchecked(&bytes, CAP)?
    };
    assert_eq!(ipc, trusted_ipc);
    let decoded = mbt_adapter_arrow_ipc::record_batch_from_ipc_stream(&ipc)?;
    check_arrow(&decoded, true)?;
    let parquet = TelemetryV1TemperatureOnly::metamorphose_parquet(&bytes, CAP)?;
    let trusted_parquet =
        unsafe { TelemetryV1TemperatureOnly::metamorphose_parquet_trusted_unchecked(&bytes, CAP)? };
    assert_eq!(parquet, trusted_parquet);
    assert!(parquet.starts_with(b"PAR1") && parquet.ends_with(b"PAR1"));
    // Retain a local artifact for the independent Parquet reader validation.
    let directory = std::env::temp_dir()
        .join("mbt-telemetry-migration-backup")
        .join("reader-oracle");
    std::fs::create_dir_all(&directory)?;
    std::fs::write(directory.join("projected.parquet"), parquet)?;
    Ok(())
}
