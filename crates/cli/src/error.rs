use img_stegano::ImgSteganoError;

#[derive(Debug, thiserror::Error)]
#[allow(unused)]
pub enum StaganoCliError {
    #[error("The output image format is invalid")]
    InvalidFormat,
    #[error("Failed to encode text to image")]
    EncodeError(#[from] ImgSteganoError),
    #[error("Failed to decode message from image")]
    DecodeError(#[source] ImgSteganoError),
    #[error("{0}")]
    Generic(String),
}
