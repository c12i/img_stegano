mod error;

use std::{io::Cursor, path::PathBuf};

pub use error::ImgSteganoError;
pub use image::ImageFormat;
use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgb};

pub fn encode_from_image(Image(input_image): Image, message: &str) -> Image {
    let mut output_image = input_image.clone();
    let mut message_bits = message
        .as_bytes()
        .iter()
        .flat_map(|byte| (0..8).rev().map(move |i| (byte >> i) & 1))
        .collect::<Vec<u8>>();
    // adding message termination, to mark the end of a message
    message_bits.extend(vec![0; 8]);
    let (width, height) = output_image.dimensions();
    let mut bit_index = 0;

    'outer: for y in 0..height {
        for x in 0..width {
            let pixel = output_image.get_pixel(x, y);
            let mut rgb = pixel.to_rgb().0;
            for i in 0..rgb.len() {
                if bit_index < message_bits.len() {
                    // clear the last bit with OxFE as the bitmask
                    // set the message_bits[i] at the cleared LSB
                    rgb[i] = (rgb[i] & 0xFE) | message_bits[bit_index];
                    bit_index += 1;
                } else {
                    break 'outer;
                }
            }
            output_image.put_pixel(x, y, Rgb(rgb).to_rgba());
        }
    }
    output_image.into()
}

pub fn encode_from_u8_array(
    input_image: &[u8],
    image_extension: &str,
    message: &str,
) -> Result<Vec<u8>, ImgSteganoError> {
    let image_format =
        ImageFormat::from_extension(image_extension).ok_or(ImgSteganoError::InvalidImageFormat)?;
    let image = image::load_from_memory(input_image)?;
    let encoded_image = encode_from_image(image.into(), message);
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
    let encoded_image = encode_from_image(image.into(), message);
    Ok(encoded_image)
}

pub fn decode_from_image(Image(encoded_image): &Image) -> String {
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
    String::from_utf8_lossy(&decoded_bytes).to_string()
}

pub fn decode_from_u8_array(input_image: &[u8]) -> Result<String, ImgSteganoError> {
    let image = image::load_from_memory(input_image)?;
    let decoded = decode_from_image(&image.into());
    Ok(decoded)
}

pub fn decode_from_path<T: Into<PathBuf>>(image_path: T) -> Result<String, ImgSteganoError> {
    let image = image::open(image_path.into())?;
    let decoded = decode_from_image(&image.into());
    Ok(decoded)
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
