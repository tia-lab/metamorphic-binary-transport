use crate::error::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BinaryInspection {
    pub row_count: usize,
    pub semantic_checksum: u64,
    pub minimal_projection_checksum: u64,
}

pub trait MbtSchema {
    type Row;
    type View<'a>
    where
        Self: 'a;

    fn encode_rows(rows: &[Self::Row], max_response_bytes: usize) -> Result<Vec<u8>>;
    fn encode_owned_rows(rows: Vec<Self::Row>, max_response_bytes: usize) -> Result<Vec<u8>>;
    fn access_view(bytes: &[u8]) -> Result<Self::View<'_>>;
    fn inspect_bytes(bytes: &[u8]) -> Result<BinaryInspection>;
}

pub fn encode<S: MbtSchema>(rows: &[S::Row], max_response_bytes: usize) -> Result<Vec<u8>> {
    S::encode_rows(rows, max_response_bytes)
}

pub fn encode_owned<S: MbtSchema>(rows: Vec<S::Row>, max_response_bytes: usize) -> Result<Vec<u8>> {
    S::encode_owned_rows(rows, max_response_bytes)
}

pub fn access<S: MbtSchema>(bytes: &[u8]) -> Result<S::View<'_>> {
    S::access_view(bytes)
}

pub fn inspect<S: MbtSchema>(bytes: &[u8]) -> Result<BinaryInspection> {
    S::inspect_bytes(bytes)
}
