mod decode;
mod encode;
mod error;

use std::path::PathBuf;

pub use decode::*;
pub use encode::*;
pub use error::ImgSteganoError;
pub use image::ImageFormat;
use image::{DynamicImage, GenericImageView};

/// Calculate the maximum message capacity (in bytes) for an image
pub fn calculate_capacity(width: u32, height: u32) -> usize {
    // Each pixel has 3 RGB channels, each can store 1 bit
    // Divide by 8 to get bytes, subtract 1 for null terminator
    ((width as usize * height as usize * 3) / 8).saturating_sub(1)
}

#[derive(Debug, Clone)]
pub struct Image(DynamicImage);

impl Image {
    pub fn open<P: Into<PathBuf>>(path: P) -> Result<Self, ImgSteganoError> {
        let image = image::open(path.into())?;
        Ok(Image(image))
    }

    pub fn open_from_u8_array(buf: &[u8], format: ImageFormat) -> Result<Self, ImgSteganoError> {
        let image = image::load_from_memory_with_format(buf, format)?;
        Ok(Image(image))
    }

    pub fn save<P: Into<PathBuf>>(
        &self,
        path: P,
        format: ImageFormat,
    ) -> Result<(), ImgSteganoError> {
        self.0.save_with_format(path.into(), format)?;
        Ok(())
    }

    /// Get the maximum message capacity (in bytes) for this image
    pub fn capacity(&self) -> usize {
        let (width, height) = self.0.dimensions();
        calculate_capacity(width, height)
    }

    /// Get the dimensions of the image
    pub fn dimensions(&self) -> (u32, u32) {
        self.0.dimensions()
    }
}

impl From<DynamicImage> for Image {
    fn from(value: DynamicImage) -> Self {
        Image(value)
    }
}
