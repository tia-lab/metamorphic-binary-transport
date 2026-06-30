use crate::codec::response_checksum;
use crate::error::{Result, TransportError};
use crate::runtime::{BinaryInspection, MbtSchema, access, encode, encode_owned, inspect};

struct TestSchema;

struct Row(u8);

struct View<'a> {
    bytes: &'a [u8],
}

impl MbtSchema for TestSchema {
    type Row = Row;
    type View<'a> = View<'a>;

    fn encode_rows(rows: &[Self::Row], max_response_bytes: usize) -> Result<Vec<u8>> {
        let mut bytes = Vec::with_capacity(rows.len());
        for row in rows {
            bytes.push(row.0);
        }
        if bytes.len() > max_response_bytes {
            return Err(TransportError::ResponseTooLarge {
                observed: bytes.len(),
                cap: max_response_bytes,
            });
        }
        Ok(bytes)
    }

    fn encode_owned_rows(rows: Vec<Self::Row>, max_response_bytes: usize) -> Result<Vec<u8>> {
        Self::encode_rows(&rows, max_response_bytes)
    }

    fn access_view(bytes: &[u8]) -> Result<Self::View<'_>> {
        Ok(View { bytes })
    }

    fn inspect_bytes(bytes: &[u8]) -> Result<BinaryInspection> {
        let checksum = response_checksum(bytes);
        Ok(BinaryInspection {
            row_count: bytes.len(),
            semantic_checksum: checksum,
            minimal_projection_checksum: checksum,
        })
    }
}

#[test]
fn runtime_dispatches_to_schema_implementation() -> Result<()> {
    let rows = [Row(1), Row(2), Row(3)];

    let encoded = encode::<TestSchema>(&rows, 8)?;
    assert_eq!(encoded, vec![1, 2, 3]);

    let owned = encode_owned::<TestSchema>(vec![Row(4), Row(5)], 8)?;
    assert_eq!(owned, vec![4, 5]);

    let view = access::<TestSchema>(&encoded)?;
    assert_eq!(view.bytes, encoded.as_slice());

    let inspected = inspect::<TestSchema>(&encoded)?;
    assert_eq!(
        inspected,
        BinaryInspection {
            row_count: 3,
            semantic_checksum: response_checksum(&encoded),
            minimal_projection_checksum: response_checksum(&encoded),
        }
    );

    Ok(())
}

#[test]
fn runtime_dispatch_propagates_schema_errors() {
    let rows = [Row(1), Row(2), Row(3)];

    assert_eq!(
        encode::<TestSchema>(&rows, 2).err(),
        Some(TransportError::ResponseTooLarge {
            observed: 3,
            cap: 2,
        })
    );
}
