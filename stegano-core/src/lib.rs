mod error;

use std::{io::Cursor, path::PathBuf};

pub use error::ImgSteganoError;
pub use image::ImageFormat;
use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};

pub struct ImgStegano;

impl ImgStegano {
    pub fn encode_from_image(Image(input_image): Image, message: &str) -> Image {
        let mut output_image = input_image.clone();
        let message = message.as_bytes();
        let (width, height) = output_image.dimensions();
        let mut message_idx = 0;

        for y in 0..height {
            for x in 0..width {
                let pixel = output_image.get_pixel(x, y);
                let mut rgba = pixel.to_rgba();

                for i in 0..3 {
                    let channel_bit = (rgba.0[i]) as u8;
                    let message_bit = (message[message_idx - 1] >> i) & 1;
                    if channel_bit != message_bit {
                        // flip the LSB to match the message bit
                        rgba.0[i] ^= 1;
                    }
                    if message_idx < message.len() {
                        message_idx += 1;
                    } else {
                        break; // all message bits encoded
                    }
                }
                output_image.put_pixel(x, y, Rgba(rgba.0));
                if message_idx >= message.len() {
                    break; // all message bits encoded
                }
            }
        }
        output_image.into()
    }

    pub fn encode_from_u8_array(
        input_image: &[u8],
        image_extension: &str,
        message: &str,
    ) -> Result<Vec<u8>, ImgSteganoError> {
        let image_format = ImageFormat::from_extension(image_extension)
            .ok_or(ImgSteganoError::InvalidImageFormat)?;
        let image = image::load_from_memory(input_image)?;
        let encoded_image = Self::encode_from_image(image.into(), message);
        let Image(encoded_image) = encoded_image;
        let mut encoded: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut encoded);
        encoded_image.write_to(&mut cursor, image_format)?;
        Ok(encoded)
    }

    pub fn encode_from_path<T: Into<PathBuf>>(
        image_path: T,
        message: &str,
    ) -> Result<Image, ImgSteganoError> {
        let image = image::open(image_path.into())?;
        let encoded_image = Self::encode_from_image(image.into(), message);
        Ok(encoded_image)
    }

    pub fn decode_from_image(Image(encoded_image): &Image) -> String {
        let (width, height) = encoded_image.dimensions();
        let mut message = Vec::new();
        let mut current_byte = 0u8;
        let mut bit_count = 0;

        for y in 0..height {
            for x in 0..width {
                let pixel = encoded_image.get_pixel(x, y);
                let rgba = pixel.to_rgba();
                for i in 0..3 {
                    let channel_bit = rgba.0[i] & 1;
                    current_byte = (current_byte << 1) | channel_bit;
                    bit_count += 1;

                    if bit_count == 8 {
                        if current_byte == 0 {
                            return String::from_utf8(message).expect("Invalid string");
                        }
                        message.push(current_byte);
                        current_byte = 0u8;
                        bit_count = 0;
                    }
                }
            }
            if current_byte == 0 {
                return String::from_utf8(message).expect("Invalid string");
            }
        }
        String::from_utf8(message).expect("Invalid string")
    }

    pub fn decode_from_u8_array(input_image: &[u8]) -> Result<String, ImgSteganoError> {
        let image = image::load_from_memory(input_image)?;
        let decoded = Self::decode_from_image(&image.into());
        Ok(decoded)
    }

    pub fn decode_from_path<T: Into<PathBuf>>(image_path: T) -> Result<String, ImgSteganoError> {
        let image = image::open(image_path.into())?;
        let decoded = Self::decode_from_image(&image.into());
        Ok(decoded)
    }
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
}

impl From<DynamicImage> for Image {
    fn from(value: DynamicImage) -> Self {
        Image(value)
    }
}
