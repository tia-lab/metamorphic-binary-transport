use std::io::{self, Cursor, Write};

use mbt_core::error::{Result, TransportError};

pub const DEFAULT_ZSTD_LEVEL: i32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressionConfig {
    pub level: i32,
}

impl CompressionConfig {
    pub const fn new(level: i32) -> Self {
        Self { level }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            level: DEFAULT_ZSTD_LEVEL,
        }
    }
}

pub fn compress(bytes: &[u8], max_compressed_bytes: usize) -> Result<Vec<u8>> {
    compress_with_config(bytes, max_compressed_bytes, CompressionConfig::default())
}

pub fn compress_with_config(
    bytes: &[u8],
    max_compressed_bytes: usize,
    config: CompressionConfig,
) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    compress_into(bytes, &mut out, max_compressed_bytes, config)?;
    Ok(out)
}

pub fn compress_into(
    bytes: &[u8],
    out: &mut Vec<u8>,
    max_compressed_bytes: usize,
    config: CompressionConfig,
) -> Result<()> {
    out.clear();
    let source = Cursor::new(bytes);
    let mut writer = CappedVecWriter::new(out, max_compressed_bytes);
    let result = zstd::stream::copy_encode(source, &mut writer, config.level);
    map_stream_result(result, writer.overflow, max_compressed_bytes)
}

pub fn decompress(compressed: &[u8], max_decompressed_bytes: usize) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    decompress_into(compressed, &mut out, max_decompressed_bytes)?;
    Ok(out)
}

pub fn decompress_into(
    compressed: &[u8],
    out: &mut Vec<u8>,
    max_decompressed_bytes: usize,
) -> Result<()> {
    out.clear();
    let source = Cursor::new(compressed);
    let mut writer = CappedVecWriter::new(out, max_decompressed_bytes);
    let result = zstd::stream::copy_decode(source, &mut writer);
    map_stream_result(result, writer.overflow, max_decompressed_bytes)
}

struct CappedVecWriter<'a> {
    out: &'a mut Vec<u8>,
    cap: usize,
    overflow: Option<usize>,
}

impl<'a> CappedVecWriter<'a> {
    fn new(out: &'a mut Vec<u8>, cap: usize) -> Self {
        Self {
            out,
            cap,
            overflow: None,
        }
    }
}

impl Write for CappedVecWriter<'_> {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        let observed = match self.out.len().checked_add(input.len()) {
            Some(value) => value,
            None => usize::MAX,
        };
        if observed > self.cap {
            self.overflow = Some(observed);
            return Err(io::Error::other("MBT compression output cap exceeded"));
        }
        self.out.extend_from_slice(input);
        Ok(input.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn map_stream_result(result: io::Result<()>, overflow: Option<usize>, cap: usize) -> Result<()> {
    match overflow {
        Some(observed) => Err(TransportError::ResponseTooLarge { observed, cap }),
        None => match result {
            Ok(_) => Ok(()),
            Err(err) => Err(TransportError::MalformedArchive(err.to_string())),
        },
    }
}
