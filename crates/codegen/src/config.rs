use std::path::PathBuf;

use crate::error::{CodegenError, Result};
use crate::model::validate_module_name;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Inspect,
    Write,
    Check,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    Core,
    Projection,
    Metamorphose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adapter {
    Json,
    Protobuf,
    Csv,
    Transponding,
    Arrow,
    ArrowIpc,
    Parquet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenConfig {
    pub action: Action,
    pub proto_roots: Vec<PathBuf>,
    pub schema: PathBuf,
    pub root: String,
    pub module: String,
    pub surface: Surface,
    pub adapter: Option<Adapter>,
    pub out: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaRequest {
    pub proto_roots: Vec<PathBuf>,
    pub schema: PathBuf,
    pub root: String,
    pub module: String,
}

impl CodegenConfig {
    pub fn schema_request(&self) -> SchemaRequest {
        SchemaRequest {
            proto_roots: self.proto_roots.clone(),
            schema: self.schema.clone(),
            root: self.root.clone(),
            module: self.module.clone(),
        }
    }
}

pub fn parse_args<I>(args: I) -> Result<CodegenConfig>
where
    I: IntoIterator<Item = String>,
{
    let args: Vec<String> = args.into_iter().collect();
    let mut action = None;
    let mut proto_roots = Vec::new();
    let mut schema = None;
    let mut root = None;
    let mut module = None;
    let mut surface = None;
    let mut adapter = None;
    let mut out = None;
    let mut idx = 0;

    while idx < args.len() {
        match args[idx].as_str() {
            "--inspect" => {
                set_action(&mut action, Action::Inspect)?;
                idx += 1;
            }
            "--write" => {
                set_action(&mut action, Action::Write)?;
                idx += 1;
            }
            "--check" => {
                set_action(&mut action, Action::Check)?;
                idx += 1;
            }
            "--proto-root" => {
                let value = next_arg(&args, idx, "--proto-root")?;
                if value.is_empty() {
                    return Err(CodegenError::UnsupportedArgument(
                        "--proto-root cannot be empty".to_string(),
                    ));
                }
                proto_roots.push(PathBuf::from(value));
                idx += 2;
            }
            "--schema" => {
                schema = Some(PathBuf::from(next_arg(&args, idx, "--schema")?));
                idx += 2;
            }
            "--root" => {
                root = Some(next_arg(&args, idx, "--root")?.to_string());
                idx += 2;
            }
            "--module" => {
                module = Some(next_arg(&args, idx, "--module")?.to_string());
                idx += 2;
            }
            "--surface" => {
                let value = next_arg(&args, idx, "--surface")?;
                surface = Some(match value {
                    "core" => Surface::Core,
                    "projection" => Surface::Projection,
                    "metamorphose" => Surface::Metamorphose,
                    _ => {
                        return Err(CodegenError::UnsupportedArgument(format!(
                            "unsupported --surface {value}"
                        )));
                    }
                });
                idx += 2;
            }
            "--adapter" => {
                let value = next_arg(&args, idx, "--adapter")?;
                adapter = Some(match value {
                    "json" => Adapter::Json,
                    "protobuf" => Adapter::Protobuf,
                    "csv" => Adapter::Csv,
                    "transponding" => Adapter::Transponding,
                    "arrow" => Adapter::Arrow,
                    "arrow-ipc" => Adapter::ArrowIpc,
                    "parquet" => Adapter::Parquet,
                    _ => {
                        return Err(CodegenError::UnsupportedArgument(format!(
                            "unsupported --adapter {value}"
                        )));
                    }
                });
                idx += 2;
            }
            "--out" => {
                out = Some(PathBuf::from(next_arg(&args, idx, "--out")?));
                idx += 2;
            }
            other => return Err(CodegenError::UnsupportedArgument(other.to_string())),
        }
    }

    let action = action.ok_or_else(|| {
        CodegenError::UnsupportedArgument("missing --inspect, --write, or --check".to_string())
    })?;
    if proto_roots.is_empty() {
        return Err(CodegenError::UnsupportedArgument(
            "at least one --proto-root is required".to_string(),
        ));
    }
    let schema =
        schema.ok_or_else(|| CodegenError::UnsupportedArgument("missing --schema".to_string()))?;
    let root =
        root.ok_or_else(|| CodegenError::UnsupportedArgument("missing --root".to_string()))?;
    let module =
        module.ok_or_else(|| CodegenError::UnsupportedArgument("missing --module".to_string()))?;
    validate_module_name(&module)?;
    let surface = surface.ok_or_else(|| {
        CodegenError::UnsupportedArgument(
            "missing --surface core|projection|metamorphose".to_string(),
        )
    })?;
    match (surface, adapter) {
        (Surface::Metamorphose, None) => {
            return Err(CodegenError::UnsupportedArgument(
                "--adapter is required with --surface metamorphose".to_string(),
            ));
        }
        (Surface::Core | Surface::Projection, Some(_)) => {
            return Err(CodegenError::UnsupportedArgument(
                "--adapter is forbidden unless --surface metamorphose".to_string(),
            ));
        }
        _ => {}
    }

    match (action, out.as_ref()) {
        (Action::Inspect, Some(_)) => {
            return Err(CodegenError::UnsupportedArgument(
                "--out is forbidden with --inspect".to_string(),
            ));
        }
        (Action::Write | Action::Check, None) => {
            return Err(CodegenError::UnsupportedArgument(
                "--out is required with --write and --check".to_string(),
            ));
        }
        _ => {}
    }

    Ok(CodegenConfig {
        action,
        proto_roots,
        schema,
        root,
        module,
        surface,
        adapter,
        out,
    })
}

fn set_action(action: &mut Option<Action>, next: Action) -> Result<()> {
    if action.is_some() {
        return Err(CodegenError::UnsupportedArgument(
            "multiple action flags".to_string(),
        ));
    }
    *action = Some(next);
    Ok(())
}

fn next_arg<'a>(args: &'a [String], idx: usize, flag: &str) -> Result<&'a str> {
    if idx + 1 >= args.len() {
        return Err(CodegenError::UnsupportedArgument(format!(
            "missing value for {flag}"
        )));
    }
    Ok(args[idx + 1].as_str())
}
