use thiserror::Error;
use xml::reader::XmlEvent;

#[derive(Debug, Error)]
pub enum HwpxError {
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Unexpected event: {0:?}")]
    UnexpectedEvent(XmlEvent),
}
