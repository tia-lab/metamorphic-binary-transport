use mbt_core::error::{Result, TransportError};

use crate::{
    CompressionConfig, DEFAULT_ZSTD_LEVEL, compress, compress_into, decompress, decompress_into,
};

const SAMPLE_BYTES: &[u8] = b"mathilde-binary-transport-compression-sample";

#[test]
fn default_config_uses_level_three() -> Result<()> {
    assert_eq!(CompressionConfig::default().level, DEFAULT_ZSTD_LEVEL);
    assert_eq!(CompressionConfig::new(5).level, 5);
    Ok(())
}

#[test]
fn compress_and_decompress_preserve_bytes() -> Result<()> {
    let compressed = compress(SAMPLE_BYTES, 1_024)?;
    let decompressed = decompress(&compressed, SAMPLE_BYTES.len())?;
    assert_eq!(decompressed, SAMPLE_BYTES);
    Ok(())
}

#[test]
fn compress_into_and_decompress_into_preserve_bytes() -> Result<()> {
    let mut compressed = Vec::with_capacity(1_024);
    let mut decompressed = Vec::with_capacity(SAMPLE_BYTES.len());
    compress_into(
        SAMPLE_BYTES,
        &mut compressed,
        1_024,
        CompressionConfig::default(),
    )?;
    decompress_into(&compressed, &mut decompressed, SAMPLE_BYTES.len())?;
    assert_eq!(decompressed, SAMPLE_BYTES);
    Ok(())
}

#[test]
fn compress_into_reuses_preallocated_buffer_on_success() -> Result<()> {
    let mut compressed = Vec::with_capacity(1_024);
    let capacity = compressed.capacity();
    compressed.extend_from_slice(b"old");
    compress_into(
        SAMPLE_BYTES,
        &mut compressed,
        1_024,
        CompressionConfig::default(),
    )?;
    assert_eq!(compressed.capacity(), capacity);
    assert_ne!(compressed, b"old");
    Ok(())
}

#[test]
fn decompress_into_reuses_preallocated_buffer_on_success() -> Result<()> {
    let compressed = compress(SAMPLE_BYTES, 1_024)?;
    let mut decompressed = Vec::with_capacity(SAMPLE_BYTES.len());
    let capacity = decompressed.capacity();
    decompressed.extend_from_slice(b"old");
    decompress_into(&compressed, &mut decompressed, SAMPLE_BYTES.len())?;
    assert_eq!(decompressed.capacity(), capacity);
    assert_eq!(decompressed, SAMPLE_BYTES);
    Ok(())
}

#[test]
fn compress_into_rejects_compressed_cap_overflow() -> Result<()> {
    let mut compressed = Vec::new();
    let result = compress_into(
        SAMPLE_BYTES,
        &mut compressed,
        1,
        CompressionConfig::default(),
    );
    match result {
        Err(TransportError::ResponseTooLarge { observed, cap }) => {
            assert!(observed > cap);
            assert_eq!(cap, 1);
            Ok(())
        }
        Err(err) => Err(err),
        Ok(()) => Err(TransportError::MalformedArchive(
            "expected compressed cap overflow".to_string(),
        )),
    }
}

#[test]
fn decompress_into_rejects_decompressed_cap_overflow() -> Result<()> {
    let compressed = compress(SAMPLE_BYTES, 1_024)?;
    let mut decompressed = Vec::new();
    let result = decompress_into(&compressed, &mut decompressed, SAMPLE_BYTES.len() - 1);
    match result {
        Err(TransportError::ResponseTooLarge { observed, cap }) => {
            assert!(observed > cap);
            assert_eq!(cap, SAMPLE_BYTES.len() - 1);
            Ok(())
        }
        Err(err) => Err(err),
        Ok(()) => Err(TransportError::MalformedArchive(
            "expected decompressed cap overflow".to_string(),
        )),
    }
}

#[test]
fn decompress_rejects_invalid_zstd_bytes() -> Result<()> {
    let result = decompress(b"not-a-zstd-frame", 1_024);
    match result {
        Err(TransportError::MalformedArchive(_)) => Ok(()),
        Err(err) => Err(err),
        Ok(_) => Err(TransportError::MalformedArchive(
            "expected invalid zstd frame rejection".to_string(),
        )),
    }
}

#[test]
fn decompress_rejects_truncated_zstd_bytes() -> Result<()> {
    let compressed = compress(SAMPLE_BYTES, 1_024)?;
    let truncated_len = compressed.len() / 2;
    let result = decompress(&compressed[..truncated_len], 1_024);
    match result {
        Err(TransportError::MalformedArchive(_)) => Ok(()),
        Err(err) => Err(err),
        Ok(_) => Err(TransportError::MalformedArchive(
            "expected truncated zstd frame rejection".to_string(),
        )),
    }
}
