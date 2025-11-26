use std::path::PathBuf;

use crate::Image;
pub use crate::ImgSteganoError;
use image::{GenericImageView, Pixel};

pub fn decode_from_image(Image(encoded_image): &Image) -> Result<String, ImgSteganoError> {
    let (width, height) = encoded_image.dimensions();
    let mut decoded_bytes = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    'outer: for y in 0..height {
        for x in 0..width {
            let pixel = encoded_image.get_pixel(x, y);
            let rgb = pixel.to_rgb().0;
            for &channel in &rgb {
                let bit = channel & 1;
                current_byte = (current_byte << 1) | bit;
                bit_count += 1;
                if bit_count == 8 {
                    if current_byte == 0 {
                        break 'outer; // End marker
                    }
                    decoded_bytes.push(current_byte);
                    current_byte = 0;
                    bit_count = 0;
                }
            }
        }
    }

    // Return proper error for invalid UTF-8 instead of lossy conversion
    String::from_utf8(decoded_bytes).map_err(|_| ImgSteganoError::InvalidUtf8)
}

pub fn decode_from_u8_array(input_image: &[u8]) -> Result<String, ImgSteganoError> {
    let image = image::load_from_memory(input_image)?;
    decode_from_image(&image.into())
}

pub fn decode_from_path<T: Into<PathBuf>>(image_path: T) -> Result<String, ImgSteganoError> {
    let image = image::open(image_path.into())?;
    decode_from_image(&image.into())
}
