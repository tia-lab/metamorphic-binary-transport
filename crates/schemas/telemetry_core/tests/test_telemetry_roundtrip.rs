mod common;
use common::*;
use mbt_schema_telemetry::telemetry_v1::*;

#[test]
fn every_field_roundtrips_and_replays() -> TestResult {
    let expected = rows();
    let bytes = TelemetryV1::encode(&expected, CAP)?;
    assert_eq!(bytes, TelemetryV1::encode(&expected, CAP)?);
    let view = TelemetryV1::access(&bytes)?;
    assert_eq!(view.len(), expected.len());
    for (actual, row) in view.rows().zip(&expected) {
        assert_eq!(actual.schema_version(), row.schema_version);
        assert_eq!(actual.device_ordinal(), row.device_ordinal);
        assert_eq!(actual.recorded_at_ms(), row.recorded_at_ms);
        assert_eq!(
            actual.temperature_c().to_bits(),
            row.temperature_c.to_bits()
        );
        assert_eq!(
            actual.battery_percent().to_bits(),
            row.battery_percent.to_bits()
        );
        assert_eq!(actual.status_ordinal(), row.status_ordinal);
        assert_eq!(actual.tags_mask(), row.tags_mask);
        assert_eq!(actual.presence_bits(), row.presence_bits);
        assert_eq!(actual.has_battery_percent(), row.presence_bits == 1);
    }
    let inspection = TelemetryV1::inspect(&bytes)?;
    assert_eq!(inspection.semantic_checksum, semantic_checksum(&expected));
    // SAFETY: checked access above validated the same immutable buffer.
    let archived = unsafe { TelemetryV1::access_archived_trusted_unchecked(&bytes)? };
    assert_eq!(archived.rows.len(), expected.len());
    let empty = TelemetryV1::encode(&[], CAP)?;
    assert!(TelemetryV1::access(&empty)?.is_empty());
    Ok(())
}
