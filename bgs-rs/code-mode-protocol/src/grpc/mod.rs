#[cfg(bgs_bazel)]
pub use code_mode_proto::bgs::code_mode::v1::*;

#[cfg(not(bgs_bazel))]
tonic::include_proto!("bgs.code_mode.v1");

pub const MAX_IDENTIFIER_BYTES: usize = 256;
