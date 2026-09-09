mod common;
use common::*;
use mbt_schema_telemetry::telemetry_v1::*;

#[test]
fn projection_preserves_values_and_checked_trusted_parity() -> TestResult {
    let expected = rows();
    let source = TelemetryV1::encode(&expected, CAP)?;
    TelemetryV1::access(&source)?;
    let checked = TelemetryV1::project_temperature_only(&source, CAP)?;
    // SAFETY: source was checked above and remains immutable.
    let trusted = unsafe { TelemetryV1::project_temperature_only_trusted_unchecked(&source, CAP)? };
    assert_eq!(checked, trusted);
    let view = TelemetryV1TemperatureOnly::access(&checked)?;
    assert_eq!(view.len(), expected.len());
    for (actual, row) in view.rows().zip(expected) {
        assert_eq!(actual.schema_version(), 1);
        assert_eq!(actual.device_ordinal(), row.device_ordinal);
        assert_eq!(actual.recorded_at_ms(), row.recorded_at_ms);
        assert_eq!(
            actual.temperature_c().to_bits(),
            row.temperature_c.to_bits()
        );
    }
    assert!(TelemetryV1::access(&checked).is_err());
    assert!(TelemetryV1TemperatureOnly::access(&source).is_err());
    Ok(())
}

#[test]
fn direct_projection_helpers_do_not_copy_owned_rows() {
    let source = include_str!("../src/telemetry_v1.rs");
    for name in ["project_temperature_only_archived_direct"] {
        let bodies = direct_projection_helper_bodies(source, name);
        assert!(!bodies.is_empty(), "missing helper {name}");
        for body in bodies {
            for forbidden in [
                "Vec::with_capacity(archived.",
                ".to_string()",
                ".to_vec()",
                ".collect()",
                "rows.push(",
                "encode_owned(rows",
            ] {
                assert!(
                    !body.contains(forbidden),
                    "direct helper {name} contains {forbidden}"
                );
            }
        }
    }
}

fn direct_projection_helper_bodies<'a>(source: &'a str, name: &str) -> Vec<&'a str> {
    let needle = format!("fn {name}");
    let mut bodies = Vec::new();
    let mut cursor = source;
    while let Some(relative_start) = cursor.find(&needle) {
        let function_start = source.len() - cursor.len() + relative_start;
        let Some(open_relative) = source[function_start..].find('{') else {
            break;
        };
        let open = function_start + open_relative;
        let Some(close) = matching_close_brace(source, open) else {
            break;
        };
        bodies.push(&source[open + 1..close]);
        cursor = &source[close + 1..];
    }
    bodies
}

fn matching_close_brace(source: &str, open: usize) -> Option<usize> {
    let mut depth = 0_u32;
    for (offset, byte) in source.as_bytes()[open..].iter().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}
