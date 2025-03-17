pub mod error;
pub mod file;

mod constant;
mod parse;
mod stream;
mod text;

pub(crate) type Result<T> = std::result::Result<T, error::HwpxError>;
