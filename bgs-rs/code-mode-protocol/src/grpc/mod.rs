#![allow(clippy::double_must_use)] // Emitted by async_trait in the generated tonic service.

#[cfg(bgs_bazel)]
pub use code_mode_proto::bgs::code_mode::v1::*;

#[cfg(not(bgs_bazel))]
tonic::include_proto!("bgs.code_mode.v1");

pub const MAX_IDENTIFIER_BYTES: usize = 256;

/// Decimal nanoseconds a completed result was buffered before a wait or terminate retrieved it.
/// Zero means immediate delivery. Hosts omit this metadata if no completed result was retrieved.
pub const RESULT_BUFFERED_FOR_NS_METADATA: &str = "bgs-code-mode-result-buffered-for-ns";
