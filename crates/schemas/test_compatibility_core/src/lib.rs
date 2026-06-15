pub mod test_compatibility_v1;

#[cfg(feature = "arrow")]
mod test_compatibility_v1_arrow;
#[cfg(feature = "arrow_ipc")]
mod test_compatibility_v1_arrow_ipc;
#[cfg(feature = "csv")]
mod test_compatibility_v1_csv;
#[cfg(feature = "json")]
mod test_compatibility_v1_json;
#[cfg(feature = "parquet")]
mod test_compatibility_v1_parquet;
#[cfg(feature = "protobuf")]
mod test_compatibility_v1_protobuf;
#[cfg(any(feature = "arrow", feature = "arrow_ipc", feature = "parquet"))]
mod test_compatibility_v1_transponding;
