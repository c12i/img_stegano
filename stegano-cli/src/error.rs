use img_stegano::ImgSteganoError;

#[derive(Debug, thiserror::Error)]
#[allow(unused)]
pub enum StaganoCliError {
    #[error("The output image format is invalid")]
    InvalidFormat,
    #[error("Failed to encode text to image: {0}")]
    EncodeError(#[from] ImgSteganoError),
    #[error("Failed to decode message from image: {0}")]
    DecodeError(#[source] ImgSteganoError),
    #[error("{0}")]
    Generic(String),
}
