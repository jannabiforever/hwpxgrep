pub mod error;
pub mod simplified;
pub mod unzip;

mod constant;

pub(crate) type Result<T> = std::result::Result<T, error::HwpxError>;
