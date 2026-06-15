use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::config::{CodegenConfig, Surface};
use crate::descriptor::load_schema_model;
use crate::error::{CodegenError, Result};
use crate::model::SchemaModel;
use crate::rust_emit::{
    generated_metamorphose_adapter_schema, generated_projection_schema, generated_schema,
};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn inspect(config: &CodegenConfig) -> Result<String> {
    let request = config.schema_request();
    let model = load_schema_model(&request)?;
    let generated = generated_unformatted(config, &model)?;
    Ok(inspect_text(
        &model,
        generated.lines().count(),
        config.surface == Surface::Projection,
    ))
}

pub fn write(config: &CodegenConfig) -> Result<()> {
    let output = config
        .out
        .as_ref()
        .ok_or_else(|| CodegenError::UnsupportedArgument("--out is required".to_string()))?;
    let source = generated_formatted(config)?;
    write_if_changed(output, source.as_bytes())
}

pub fn check(config: &CodegenConfig) -> Result<()> {
    let output = config
        .out
        .as_ref()
        .ok_or_else(|| CodegenError::UnsupportedArgument("--out is required".to_string()))?;
    let source = generated_formatted(config)?;
    let tmp = temp_dir("check")?;
    fs::create_dir_all(&tmp)?;
    let generated_path =
        tmp.join(output.file_name().ok_or_else(|| {
            CodegenError::InvalidSchema("output path has no file name".to_string())
        })?);
    write_if_changed(&generated_path, source.as_bytes())?;
    let actual = fs::read(output)?;
    let expected = fs::read(&generated_path)?;
    let _ = fs::remove_dir_all(&tmp);
    if actual == expected {
        Ok(())
    } else {
        Err(CodegenError::GeneratedDiff(output.clone()))
    }
}

pub fn smoke_crate(root: &Path, generated_source: &str) -> Result<PathBuf> {
    let smoke = root.join("smoke");
    let src = smoke.join("src");
    fs::create_dir_all(&src)?;
    let core_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../core");
    let core_path = core_path
        .to_str()
        .ok_or_else(|| CodegenError::InvalidSchema("non-utf8 core path".to_string()))?;
    let manifest = format!(
        r#"[package]
name = "mbt_codegen_smoke"
version = "0.0.0"
edition = "2024"
publish = false

[workspace]

[dependencies]
metamorphic_binary_transport_core = {{ path = {core_path:?} }}
rkyv = "=0.8.16"
"#
    );
    write_if_changed(&smoke.join("Cargo.toml"), manifest.as_bytes())?;
    write_if_changed(
        &src.join("lib.rs"),
        b"#![allow(dead_code)]\npub mod generated_fixture;\n",
    )?;
    write_if_changed(
        &src.join("generated_fixture.rs"),
        generated_source.as_bytes(),
    )?;
    Ok(smoke)
}

pub fn format_rust(source: &str) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let Some(stdin) = child.stdin.as_mut() else {
        return Err(CodegenError::Rustfmt(
            "rustfmt stdin was not available".to_string(),
        ));
    };
    stdin.write_all(source.as_bytes())?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(CodegenError::Rustfmt(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }
    String::from_utf8(output.stdout).map_err(|err| CodegenError::Rustfmt(err.to_string()))
}

pub fn write_if_changed(path: &Path, bytes: &[u8]) -> Result<()> {
    if path.exists() {
        let current = fs::read(path)?;
        if current == bytes {
            return Ok(());
        }
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)?;
    Ok(())
}

pub fn temp_dir(label: &str) -> Result<PathBuf> {
    let mut path = std::env::current_dir()?
        .join("target")
        .join("mbt-codegen-check");
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    path.push(format!("{label}-{}-{counter}", std::process::id()));
    Ok(path)
}

fn generated_formatted(config: &CodegenConfig) -> Result<String> {
    let request = config.schema_request();
    let model = load_schema_model(&request)?;
    let source = generated_unformatted(config, &model)?;
    format_rust(&source)
}

fn generated_unformatted(config: &CodegenConfig, model: &SchemaModel) -> Result<String> {
    match config.surface {
        Surface::Core => generated_schema(model),
        Surface::Projection => generated_projection_schema(model),
        Surface::Metamorphose => generated_metamorphose_adapter_schema(
            model,
            config.adapter.ok_or_else(|| {
                CodegenError::UnsupportedArgument(
                    "--adapter is required with --surface metamorphose".to_string(),
                )
            })?,
        ),
    }
}

fn inspect_text(model: &SchemaModel, generated_lines: usize, include_projections: bool) -> String {
    let mut out = String::new();
    out.push_str(&format!("module={}\n", model.module));
    out.push_str(&format!("proto={}\n", model.proto.display()));
    out.push_str(&format!("root={}\n", model.root));
    out.push_str(&format!("schema_id={}\n", model.schema_id));
    out.push_str(&format!("schema_version={}\n", model.schema_version));
    out.push_str(&format!("transport_name={}\n", model.transport_name));
    out.push_str(&format!("payload_root={}\n", model.payload_root));
    out.push_str(&format!("schema_hash={}\n", model.normalized_schema_hash));
    out.push_str(&format!("payload_type={}\n", model.payload_type));
    out.push_str(&format!("row_type={}\n", model.row_type));
    out.push_str(&format!("generated_lines={generated_lines}\n"));
    if include_projections {
        for projection in &model.projections {
            out.push_str(&format!(
                "projection={} marker={}\n",
                projection.definition.name, projection.marker_type
            ));
        }
    }
    for dictionary in &model.dictionaries {
        out.push_str(&format!(
            "dictionary={} values={}\n",
            dictionary.name,
            dictionary.values.join(",")
        ));
    }
    for field in &model.fields {
        out.push_str(&format!(
            "field={} rust_name={} kind={:?} presence_bit={:?} key_order={:?}\n",
            field.proto_path, field.rust_name, field.kind, field.presence_bit, field.key_order
        ));
    }
    for key in &model.key_parts {
        out.push_str(&format!(
            "key_part={} rust_name={}\n",
            key.order, key.rust_name
        ));
    }
    out
}
