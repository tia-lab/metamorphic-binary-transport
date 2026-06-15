use super::*;

#[test]
fn projection_surface_emits_direct_writer_not_owned_row_copy() -> Result<()> {
    let source = run_projection_codegen_to_string(valid_alias_and_projection_ignored_proto())?;
    assert!(source.contains("pub fn project_small("));
    assert!(source.contains("pub unsafe fn project_small_trusted_unchecked("));
    assert!(source.contains("fn project_small_archived_direct("));
    assert!(source.contains("SmallProjectionProjectionPayloadRef"));
    assert!(source.contains("SmallProjectionProjectionRowsRef"));
    assert!(source.contains("SmallProjectionProjectionRowRef"));
    assert!(source.contains("TransportHeader::new_with_schema"));
    assert!(source.contains("SmallProjection::header_spec()"));
    assert!(source.contains("max_response_bytes"));
    assert!(source.contains("Self::access_archived(bytes)?"));

    let bodies = direct_projection_helper_bodies(&source, "project_small_archived_direct");
    assert!(!bodies.is_empty(), "missing direct projection helper body");
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
                "direct helper contains owned-row pattern {forbidden}"
            );
        }
    }
    Ok(())
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
