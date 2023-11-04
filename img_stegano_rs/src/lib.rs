mod error;

use std::{io::Cursor, path::PathBuf};

use error::ImgSteganoError;
use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat, Pixel, Rgba};

pub struct ImgStegano;

impl ImgStegano {
    pub fn encode_from_u8_array(
        input_image: &[u8],
        image_extension: &str,
        message: &str,
    ) -> Result<Vec<u8>, ImgSteganoError> {
        let image = image::load_from_memory_with_format(input_image, image::ImageFormat::Png)?;
        let encoded_image = encode_text(&image, message);
        let mut encoded: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut encoded);
        let image_format = ImageFormat::from_extension(image_extension)
            .ok_or(ImgSteganoError::InvalidImageFormat)?;
        encoded_image.write_to(&mut cursor, image_format)?;
        Ok(encoded)
    }

    pub fn encode_from_path<T: Into<PathBuf>>(
        image_path: T,
        message: &str,
    ) -> Result<DynamicImage, ImgSteganoError> {
        let image = image::open(image_path.into())?;
        let encoded_image = encode_text(&image, message);
        Ok(encoded_image)
    }

    pub fn decode_from_u8_array(input_image: &[u8]) -> Result<String, ImgSteganoError> {
        let image = image::load_from_memory_with_format(input_image, image::ImageFormat::Png)?;
        let decoded = decode_text(&image);
        Ok(decoded)
    }

    pub fn decode_from_path<T: Into<PathBuf>>(image_path: T) -> Result<String, ImgSteganoError> {
        let image = image::open(image_path.into())?;
        let decoded = decode_text(&image);
        Ok(decoded)
    }
}

pub fn encode_text(input_image: &DynamicImage, message: &str) -> DynamicImage {
    let mut output_image = input_image.clone();
    let message = message
        .as_bytes()
        .iter()
        .map(|v| format!("{:08b}", v))
        .collect::<String>();
    let (width, height) = output_image.dimensions();
    let mut message = message.chars();

    for y in 0..height {
        for x in 0..width {
            let pixel = output_image.get_pixel(x, y);
            let mut rgba = pixel.to_rgba();
            let message_bit = message.next();
            if let Some(bit) = message_bit {
                if let Some(lsb) = bit.to_digit(2) {
                    let lsb = lsb as u8;
                    rgba.0[0] = (rgba.0[0] & 0xFE) | lsb;
                    output_image.put_pixel(x, y, Rgba(rgba.0));
                }
            } else {
                break; // All message bits have been encoded
            }
        }
    }
    output_image
}

pub fn decode_text(encoded_image: &DynamicImage) -> String {
    let (width, height) = encoded_image.dimensions();
    let mut binary_message = String::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    for y in 0..height {
        for x in 0..width {
            let pixel = encoded_image.get_pixel(x, y);
            let rgba = pixel.to_rgba();
            let lsb = rgba.0[0] & 1;
            current_byte = (current_byte << 1) | lsb;
            bit_count += 1;
            if bit_count == 8 {
                if current_byte == 0 {
                    break; // End of message
                }
                binary_message.push(current_byte as char);
                current_byte = 0u8;
                bit_count = 0;
            }
        }
        if current_byte == 0 {
            break; // End of message
        }
    }
    binary_message
}
