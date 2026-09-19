#![allow(dead_code)]
use mbt_schema_telemetry::telemetry_v1::*;
pub const CAP: usize = 1 << 20;
pub type TestResult = Result<(), Box<dyn std::error::Error>>;

pub fn rows() -> Vec<TelemetryRowV1> {
    vec![
        TelemetryRowV1 {
            schema_version: 1,
            device_ordinal: DEVICE_SENSOR_A,
            recorded_at_ms: 0,
            temperature_c: 21.5,
            battery_percent: 0.0,
            status_ordinal: STATUS_ACTIVE,
            tags_mask: 5,
            presence_bits: 0,
        },
        TelemetryRowV1 {
            schema_version: 1,
            device_ordinal: DEVICE_SENSOR_A,
            recorded_at_ms: 1000,
            temperature_c: 0.0,
            battery_percent: 0.0,
            status_ordinal: STATUS_IDLE,
            tags_mask: 6,
            presence_bits: 1,
        },
        TelemetryRowV1 {
            schema_version: 1,
            device_ordinal: DEVICE_SENSOR_B,
            recorded_at_ms: -1000,
            temperature_c: -4.25,
            battery_percent: 50.0,
            status_ordinal: STATUS_OFFLINE,
            tags_mask: 0,
            presence_bits: 1,
        },
    ]
}

#[cfg(any(feature = "json", feature = "csv", feature = "protobuf"))]
pub fn check_row_format(format: &str, projected: bool, bytes: &[u8]) -> TestResult {
    use serde_json::{Value, json};
    let mut expected = vec![
        json!({"schema_version":1,"device":"sensor_a","recorded_at_ms":0,"recorded_at_utc":"1970-01-01T00:00:00Z","temperature_c":21.5,"status":"active","tags":["indoor","test"]}),
        json!({"schema_version":1,"device":"sensor_a","recorded_at_ms":1000,"recorded_at_utc":"1970-01-01T00:00:01Z","temperature_c":0.0,"battery_percent":0.0,"status":"idle","tags":["outdoor","test"]}),
        json!({"schema_version":1,"device":"sensor_b","recorded_at_ms":-1000,"recorded_at_utc":"1969-12-31T23:59:59Z","temperature_c":-4.25,"battery_percent":50.0,"status":"offline","tags":[]}),
    ];
    if projected {
        for row in &mut expected {
            let object = row.as_object_mut().ok_or("expected object")?;
            object.remove("battery_percent");
            object.remove("status");
            object.remove("tags");
        }
    }
    let mut actual: Vec<Value> = match format {
        "json" => {
            let root: Value = serde_json::from_slice(bytes)?;
            assert_eq!(root["schema_version"], 1);
            root["rows"].as_array().ok_or("missing rows")?.clone()
        }
        "csv" => {
            let mut reader = csv::Reader::from_reader(bytes);
            let headers = reader.headers()?.clone();
            let mut rows = Vec::new();
            for record in reader.records() {
                let record = record?;
                let mut row = serde_json::Map::new();
                for (name, value) in headers.iter().zip(record.iter()) {
                    let decoded = match name {
                        "schema_version" | "recorded_at_ms" => json!(value.parse::<i64>()?),
                        "temperature_c" | "battery_percent" => {
                            if name == "battery_percent" && value.is_empty() {
                                continue;
                            }
                            json!(value.parse::<f64>()?)
                        }
                        "tags" => serde_json::from_str(value)?,
                        _ => json!(value),
                    };
                    row.insert(name.to_string(), decoded);
                }
                rows.push(Value::Object(row));
            }
            rows
        }
        "protobuf" => {
            use prost::Message;
            let response = OracleResponse::decode(bytes)?;
            assert_eq!(response.schema_version, 1);
            response.rows.into_iter().map(|row| {
                let mut result = json!({"schema_version":row.schema_version,"device":row.device,"recorded_at_ms":row.recorded_at_ms,"recorded_at_utc":row.recorded_at_utc,"temperature_c":row.temperature_c});
                if let Some(battery) = row.battery_percent { result["battery_percent"] = json!(battery); }
                if !projected || !row.status.is_empty() { result["status"] = json!(row.status); }
                if !projected || !row.tags.is_empty() { result["tags"] = json!(row.tags); }
                result
            }).collect()
        }
        _ => return Err("unknown row format".into()),
    };
    // JSON permits 0 and 0.0 for the same schema double; compare both as f64.
    for row in &mut actual {
        for field in ["temperature_c", "battery_percent"] {
            if let Some(value) = row.get_mut(field) {
                *value = json!(value.as_f64().ok_or("expected schema double")?);
            }
        }
    }
    assert_eq!(actual, expected, "{format}, projected={projected}");

    Ok(())
}

// Independent test-only protobuf contract, decoded with prost rather than the writer.
#[derive(Clone, PartialEq, prost::Message)]
struct OracleResponse {
    #[prost(uint32, tag = "1")]
    schema_version: u32,
    #[prost(message, repeated, tag = "2")]
    rows: Vec<OracleRow>,
}

#[derive(Clone, PartialEq, prost::Message)]
struct OracleRow {
    #[prost(uint32, tag = "1")]
    schema_version: u32,
    #[prost(string, tag = "2")]
    device: String,
    #[prost(int64, tag = "3")]
    recorded_at_ms: i64,
    #[prost(string, tag = "4")]
    recorded_at_utc: String,
    #[prost(double, tag = "5")]
    temperature_c: f64,
    #[prost(double, optional, tag = "6")]
    battery_percent: Option<f64>,
    #[prost(string, tag = "7")]
    status: String,
    #[prost(string, repeated, tag = "8")]
    tags: Vec<String>,
}

#[cfg(feature = "arrow")]
pub fn check_arrow(batch: &mbt_adapter_arrow::ArrowRecordBatch, projected: bool) -> TestResult {
    let source = rows();
    let mut expected: Vec<(&str, Vec<u8>)> = vec![
        (
            "schema_version",
            source
                .iter()
                .flat_map(|r| r.schema_version.to_ne_bytes())
                .collect(),
        ),
        (
            "device",
            source
                .iter()
                .flat_map(|r| r.device_ordinal.to_ne_bytes())
                .collect(),
        ),
        (
            "recorded_at_ms",
            source
                .iter()
                .flat_map(|r| r.recorded_at_ms.to_ne_bytes())
                .collect(),
        ),
        (
            "temperature_c",
            source
                .iter()
                .flat_map(|r| r.temperature_c.to_ne_bytes())
                .collect(),
        ),
    ];
    if !projected {
        expected.extend([
            (
                "battery_percent",
                source
                    .iter()
                    .flat_map(|r| r.battery_percent.to_ne_bytes())
                    .collect(),
            ),
            (
                "status",
                source
                    .iter()
                    .flat_map(|r| r.status_ordinal.to_ne_bytes())
                    .collect(),
            ),
            (
                "tags",
                source
                    .iter()
                    .flat_map(|r| r.tags_mask.to_ne_bytes())
                    .collect(),
            ),
        ]);
    }
    if batch.num_rows() != source.len() || batch.num_columns() != expected.len() {
        return Err("unexpected columnar shape".into());
    }
    for (idx, (name, values)) in expected.iter().enumerate() {
        if batch.schema().field(idx).name() != name {
            return Err("unexpected column name".into());
        }
        let column = batch.column(idx);
        let data = column.to_data();
        if data.buffers().len() != 1 || data.buffers()[0].as_slice() != values {
            return Err(format!("column values differ: {name}").into());
        }
        for row in 0..source.len() {
            if column.is_null(row) != (*name == "battery_percent" && row == 0) {
                return Err("column nullability differs".into());
            }
        }
    }
    Ok(())
}
