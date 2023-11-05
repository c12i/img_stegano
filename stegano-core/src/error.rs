#[derive(Debug, thiserror::Error)]
pub enum ImgSteganoError {
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error("Invalid image format")]
    InvalidImageFormat,
}
