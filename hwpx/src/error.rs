use thiserror::Error;

#[derive(Debug, Error)]
pub enum HwpxError {
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    Zip(#[from] zip::result::ZipError),
}
