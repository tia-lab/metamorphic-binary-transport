#![forbid(unsafe_code)]
//! Uncompressed Parquet helpers for generated MBT metamorphose adapters.

use std::io::{self, Write};

use arrow_array::RecordBatch;
use metamorphic_binary_transport_core::codec::response_checksum;
use metamorphic_binary_transport_core::error::{Result, TransportError};
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::errors::ParquetError;
use parquet::file::properties::WriterProperties;

mod arrow_bridge;

pub use arrow_array::RecordBatch as ArrowRecordBatch;
pub use arrow_bridge::*;

pub fn write_uncompressed_parquet(
    batch: &RecordBatch,
    max_response_bytes: usize,
) -> Result<Vec<u8>> {
    // Parquet output is intentionally uncompressed; compression is an outer policy.
    let mut sink = CheckedParquetWriter::new(max_response_bytes);
    let properties = WriterProperties::builder()
        .set_compression(Compression::UNCOMPRESSED)
        .build();
    let mut writer = match ArrowWriter::try_new(&mut sink, batch.schema(), Some(properties)) {
        Ok(writer) => writer,
        Err(err) => return Err(parquet_error(err)),
    };
    if let Err(err) = writer.write(batch) {
        drop(writer);
        return Err(parquet_or_overflow_error(&sink, err));
    }
    if let Err(err) = writer.close() {
        return Err(parquet_or_overflow_error(&sink, err));
    }
    sink.finish()
}

pub fn parquet_bytes_checksum(bytes: &[u8]) -> u64 {
    response_checksum(bytes)
}

// The checked writer keeps Parquet output within the caller response cap.
struct CheckedParquetWriter {
    bytes: Vec<u8>,
    cap: usize,
    overflow: Option<usize>,
}

impl CheckedParquetWriter {
    fn new(cap: usize) -> Self {
        Self {
            bytes: Vec::new(),
            cap,
            overflow: None,
        }
    }

    fn finish(self) -> Result<Vec<u8>> {
        if let Some(observed) = self.overflow {
            return Err(TransportError::ResponseTooLarge {
                observed,
                cap: self.cap,
            });
        }
        Ok(self.bytes)
    }
}

impl Write for CheckedParquetWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let observed = match self.bytes.len().checked_add(buf.len()) {
            Some(observed) => observed,
            None => {
                self.overflow = Some(usize::MAX);
                return Err(io::Error::new(
                    io::ErrorKind::OutOfMemory,
                    "parquet response size overflow",
                ));
            }
        };
        if observed > self.cap {
            self.overflow = Some(observed);
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "parquet response exceeds cap",
            ));
        }
        self.bytes.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn parquet_error(err: ParquetError) -> TransportError {
    TransportError::MalformedArchive(err.to_string())
}

fn parquet_or_overflow_error(sink: &CheckedParquetWriter, err: ParquetError) -> TransportError {
    // Preserve cap failures when Parquet reports the sink write as a Parquet error.
    match sink.overflow {
        Some(observed) => TransportError::ResponseTooLarge {
            observed,
            cap: sink.cap,
        },
        None => parquet_error(err),
    }
}

#[cfg(test)]
mod tests;
