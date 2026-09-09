mod common;
use common::*;
use mbt_schema_telemetry::telemetry_v1::*;

#[test]
fn generated_dictionary_and_key_shape_matches_fresh_inspection() -> TestResult {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "-p",
            "mbt_codegen",
            "--bin",
            "mbt_codegen",
            "--",
            "--inspect",
            "--proto-root",
            "crates/schemas/telemetry_core/proto",
            "--proto-root",
            "proto",
            "--schema",
            "mbt/example/telemetry/v1/telemetry.proto",
            "--root",
            "mbt.example.telemetry.v1.TelemetryResponseV1",
            "--module",
            "telemetry_v1",
            "--surface",
            "projection",
        ])
        .current_dir(root)
        .output()?;
    if !output.status.success() {
        return Err(String::from_utf8(output.stderr)?.into());
    }
    let text = String::from_utf8(output.stdout)?;
    assert_eq!(
        text,
        include_str!("expected_telemetry_projection_inspect.txt")
    );
    for expected in [
        "schema_id=50001",
        "transport_name=mbt.example.telemetry.v1",
        "key_part=1 rust_name=device_ordinal",
        "key_part=2 rust_name=recorded_at_ms",
        "projection=temperature_only marker=TelemetryV1TemperatureOnly",
    ] {
        assert!(text.contains(expected), "missing {expected}");
    }
    assert_eq!(
        [DEVICE_SENSOR_A, DEVICE_SENSOR_B, DEVICE_SENSOR_C],
        [0, 1, 2]
    );
    assert_eq!([STATUS_ACTIVE, STATUS_IDLE, STATUS_OFFLINE], [1, 2, 3]);
    assert_eq!(VALID_TAG_MASK, 7);
    Ok(())
}
