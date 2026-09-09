mod common;
use common::*;
use mbt_core::error::TransportError;
use mbt_schema_telemetry::telemetry_v1::*;

#[test]
fn invalid_values_fail_before_encoding() -> TestResult {
    let mutations: &[fn(&mut TelemetryRowV1)] = &[
        |r| r.schema_version = 2,
        |r| r.device_ordinal = 99,
        |r| r.status_ordinal = 99,
        |r| r.tags_mask = 8,
        |r| r.presence_bits = 2,
        |r| r.battery_percent = 1.,
        |r| r.temperature_c = f64::NAN,
        |r| r.temperature_c = f64::INFINITY,
        |r| r.battery_percent = f64::NEG_INFINITY,
    ];
    for mutate in mutations {
        let mut values = rows();
        mutate(&mut values[0]);
        assert!(TelemetryV1::encode(&values, CAP).is_err());
    }
    let mut values = rows();
    values.swap(0, 1);
    assert!(matches!(
        TelemetryV1::encode(&values, CAP),
        Err(TransportError::InvalidTimeGrid(_))
    ));
    assert!(matches!(
        TelemetryV1::encode(&rows(), 1),
        Err(TransportError::ResponseTooLarge { .. })
    ));
    Ok(())
}

#[test]
fn corrupt_partial_and_old_identity_fail() -> TestResult {
    let bytes = TelemetryV1::encode(&rows(), CAP)?;
    for length in [0, 8, 127, bytes.len() - 1] {
        assert!(TelemetryV1::access(&bytes[..length]).is_err());
    }
    let mut changed = bytes.clone();
    changed[0] ^= 1;
    assert!(matches!(
        TelemetryV1::access(&changed),
        Err(TransportError::CorruptMagic)
    ));
    let mut changed = bytes.clone();
    changed[16..20].copy_from_slice(&1_u32.to_le_bytes());
    assert!(matches!(
        TelemetryV1::access(&changed),
        Err(TransportError::UnknownSchemaId(1))
    ));
    let mut changed = bytes.clone();
    changed[20..22].copy_from_slice(&2_u16.to_le_bytes());
    assert!(matches!(
        TelemetryV1::access(&changed),
        Err(TransportError::SchemaVersionMismatch { .. })
    ));
    let mut changed = bytes.clone();
    changed[24..32].copy_from_slice(&6061383958499356843_u64.to_le_bytes());
    assert!(matches!(
        TelemetryV1::access(&changed),
        Err(TransportError::SchemaHashMismatch { .. })
    ));
    let mut changed = bytes;
    changed[128] ^= 1;
    assert!(matches!(
        TelemetryV1::access(&changed),
        Err(TransportError::PayloadChecksumMismatch { .. })
    ));
    Ok(())
}
