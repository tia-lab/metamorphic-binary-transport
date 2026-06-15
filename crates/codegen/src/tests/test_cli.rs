use super::*;
use crate::config::{Action, Adapter, Surface, parse_args};
use crate::emit::{check, write};

#[test]
fn cli_shape_is_explicit() -> Result<()> {
    let args = vec![
        "--inspect".to_string(),
        "--proto-root".to_string(),
        "proto".to_string(),
        "--schema".to_string(),
        "test.proto".to_string(),
        "--root".to_string(),
        "test.Root".to_string(),
        "--module".to_string(),
        "test_v1".to_string(),
        "--surface".to_string(),
        "core".to_string(),
    ];
    let parsed = parse_args(args)?;
    assert_eq!(parsed.action, Action::Inspect);
    assert_eq!(parsed.surface, Surface::Core);
    assert_eq!(parsed.adapter, None);
    Ok(())
}

#[test]
fn cli_accepts_projection_surface() -> Result<()> {
    let parsed = parse_args(vec![
        "--inspect".to_string(),
        "--proto-root".to_string(),
        "proto".to_string(),
        "--schema".to_string(),
        "test.proto".to_string(),
        "--root".to_string(),
        "test.Root".to_string(),
        "--module".to_string(),
        "test_v1".to_string(),
        "--surface".to_string(),
        "projection".to_string(),
    ])?;
    assert_eq!(parsed.action, Action::Inspect);
    assert_eq!(parsed.surface, Surface::Projection);
    assert_eq!(parsed.adapter, None);
    Ok(())
}

#[test]
fn cli_requires_adapter_for_metamorphose_surface() -> Result<()> {
    let parsed = parse_args(vec![
        "--inspect".to_string(),
        "--proto-root".to_string(),
        "proto".to_string(),
        "--schema".to_string(),
        "test.proto".to_string(),
        "--root".to_string(),
        "test.Root".to_string(),
        "--module".to_string(),
        "test_v1".to_string(),
        "--surface".to_string(),
        "metamorphose".to_string(),
        "--adapter".to_string(),
        "arrow-ipc".to_string(),
    ])?;
    assert_eq!(parsed.action, Action::Inspect);
    assert_eq!(parsed.surface, Surface::Metamorphose);
    assert_eq!(parsed.adapter, Some(Adapter::ArrowIpc));
    Ok(())
}

#[test]
fn cli_rejects_invalid_argument_shapes() {
    assert!(parse_args(Vec::<String>::new()).is_err());
    assert!(parse_args(vec!["--inspect".to_string(), "--write".to_string()]).is_err());
    assert!(
        parse_args(vec![
            "--inspect".to_string(),
            "--proto-root".to_string(),
            "proto".to_string(),
            "--schema".to_string(),
            "test.proto".to_string(),
            "--root".to_string(),
            "test.Root".to_string(),
            "--module".to_string(),
            "Bad".to_string(),
            "--surface".to_string(),
            "core".to_string(),
        ])
        .is_err()
    );
    assert!(
        parse_args(vec![
            "--inspect".to_string(),
            "--proto-root".to_string(),
            "proto".to_string(),
            "--schema".to_string(),
            "test.proto".to_string(),
            "--root".to_string(),
            "test.Root".to_string(),
            "--module".to_string(),
            "test_v1".to_string(),
            "--surface".to_string(),
            "json".to_string(),
        ])
        .is_err()
    );
    assert!(
        parse_args(vec![
            "--inspect".to_string(),
            "--proto-root".to_string(),
            "proto".to_string(),
            "--schema".to_string(),
            "test.proto".to_string(),
            "--root".to_string(),
            "test.Root".to_string(),
            "--module".to_string(),
            "test_v1".to_string(),
            "--surface".to_string(),
            "core".to_string(),
            "--adapter".to_string(),
            "json".to_string(),
        ])
        .is_err()
    );
    assert!(
        parse_args(vec![
            "--inspect".to_string(),
            "--proto-root".to_string(),
            "proto".to_string(),
            "--schema".to_string(),
            "test.proto".to_string(),
            "--root".to_string(),
            "test.Root".to_string(),
            "--module".to_string(),
            "test_v1".to_string(),
            "--surface".to_string(),
            "metamorphose".to_string(),
        ])
        .is_err()
    );
    assert!(parse_args(vec!["--unknown".to_string()]).is_err());
}

#[test]
fn check_detects_generated_diff() -> Result<()> {
    let root = temp_root("cli")?;
    write_options_proto(&root)?;
    write_proto(&root, "test/fixture/v1/test.proto", valid_scalar_proto())?;
    let out = root.join("generated.rs");
    let mut cfg = config(&root, "test/fixture/v1/test.proto", "fixture_v1");
    cfg.action = Action::Write;
    cfg.out = Some(out.clone());
    write(&cfg)?;
    fs::write(&out, b"changed")?;
    cfg.action = Action::Check;
    assert!(check(&cfg).is_err());
    Ok(())
}
