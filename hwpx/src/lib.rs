pub mod error;
pub mod file;
pub mod parse;

mod constant;
mod stream;
mod text;

pub(crate) type Result<T> = std::result::Result<T, error::HwpxError>;
