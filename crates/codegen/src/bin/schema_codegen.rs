//! Generate every committed example surface using the Rust codegen library.
use std::path::PathBuf;

use mbt_codegen::{
    config::{Action, parse_args},
    emit,
};

const SCHEMAS: [(&str, &str, &str, &str); 3] = [
    (
        "measurement_core",
        "mbt/example/measurement/v1/measurement.proto",
        "mbt.example.measurement.v1.MeasurementResponseV1",
        "measurement_v1",
    ),
    (
        "telemetry_core",
        "mbt/example/telemetry/v1/telemetry.proto",
        "mbt.example.telemetry.v1.TelemetryResponseV1",
        "telemetry_v1",
    ),
    (
        "test_compatibility_core",
        "mbt/test_compatibility/v1/all_fields.proto",
        "mbt.test_compatibility.v1.TestCompatibilityResponseV1",
        "test_compatibility_v1",
    ),
];
const ADAPTERS: [&str; 7] = [
    "json",
    "protobuf",
    "csv",
    "transponding",
    "arrow",
    "arrow-ipc",
    "parquet",
];

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let action = match args.as_slice() {
        [action] if matches!(action.as_str(), "--write" | "--check" | "--inspect") => action,
        _ => return Err("usage: schema_codegen <--write|--check|--inspect>".into()),
    };
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (directory, schema, message, module) in SCHEMAS {
        let base = root.join("crates/schemas").join(directory);
        for adapter in std::iter::once(None).chain(ADAPTERS.into_iter().map(Some)) {
            let mut command = vec![
                action.clone(),
                "--proto-root".into(),
                base.join("proto").display().to_string(),
                "--proto-root".into(),
                root.join("proto").display().to_string(),
                "--schema".into(),
                schema.into(),
                "--root".into(),
                message.into(),
                "--module".into(),
                module.into(),
                "--surface".into(),
                if adapter.is_some() {
                    "metamorphose"
                } else {
                    "projection"
                }
                .into(),
            ];
            let suffix = if let Some(adapter) = adapter {
                command.extend(["--adapter".into(), adapter.into()]);
                format!("_{}", adapter.replace('-', "_"))
            } else {
                String::new()
            };
            if action != "--inspect" {
                command.extend([
                    "--out".into(),
                    base.join("src")
                        .join(format!("{module}{suffix}.rs"))
                        .display()
                        .to_string(),
                ]);
            }
            let config = parse_args(command)?;
            match config.action {
                Action::Write => emit::write(&config)?,
                Action::Check => emit::check(&config)?,
                Action::Inspect => print!("{}", emit::inspect(&config)?),
            }
        }
    }
    Ok(())
}
