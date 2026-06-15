pub mod bars_v1;

#[cfg(feature = "arrow")]
mod bars_v1_arrow;
#[cfg(feature = "arrow_ipc")]
mod bars_v1_arrow_ipc;
#[cfg(feature = "csv")]
mod bars_v1_csv;
#[cfg(feature = "json")]
mod bars_v1_json;
#[cfg(feature = "parquet")]
mod bars_v1_parquet;
#[cfg(feature = "protobuf")]
mod bars_v1_protobuf;
#[cfg(any(feature = "arrow", feature = "arrow_ipc", feature = "parquet"))]
mod bars_v1_transponding;
