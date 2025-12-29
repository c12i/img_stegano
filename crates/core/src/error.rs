#[derive(Debug, thiserror::Error)]
pub enum ImgSteganoError {
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error("Invalid image format")]
    InvalidImageFormat,
    #[error(
        "Message too large: requires {required} bytes but image can only hold {available} bytes"
    )]
    MessageTooLarge { required: usize, available: usize },
    #[error("Empty message provided")]
    EmptyMessage,
    #[error("Decoded message contains invalid UTF-8")]
    InvalidUtf8,
    #[error("Lossy format detected: {format:?}. Steganography works best with lossless formats like PNG")]
    LossyFormatWarning { format: String },
}
