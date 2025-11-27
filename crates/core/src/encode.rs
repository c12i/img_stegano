use std::{io::Cursor, path::PathBuf};

use image::ImageFormat;
use image::{GenericImage, GenericImageView, Pixel, Rgb};

use crate::{calculate_capacity, Image, ImgSteganoError};

pub fn encode_from_image(
    Image(input_image): Image,
    message: &str,
) -> Result<Image, ImgSteganoError> {
    // Validate message is not empty
    if message.is_empty() {
        return Err(ImgSteganoError::EmptyMessage);
    }

    let (width, height) = input_image.dimensions();
    let capacity = calculate_capacity(width, height);
    let message_bytes = message.as_bytes();

    // Validate message fits in image
    if message_bytes.len() > capacity {
        return Err(ImgSteganoError::MessageTooLarge {
            required: message_bytes.len(),
            available: capacity,
        });
    }

    let mut output_image = input_image.clone();
    let mut message_bits = message_bytes
        .iter()
        .flat_map(|byte| (0..8).rev().map(move |i| (byte >> i) & 1))
        .collect::<Vec<u8>>();
    // adding message termination, to mark the end of a message
    message_bits.extend(vec![0; 8]);
    let mut bit_index = 0;

    'outer: for y in 0..height {
        for x in 0..width {
            let pixel = output_image.get_pixel(x, y);
            let mut rgb = pixel.to_rgb().0;
            for channel in &mut rgb {
                if bit_index < message_bits.len() {
                    // clear the last bit with OxFE as the bitmask
                    // set the message_bits[i] at the cleared LSB
                    *channel = (*channel & 0xFE) | message_bits[bit_index];
                    bit_index += 1;
                } else {
                    break 'outer;
                }
            }
            output_image.put_pixel(x, y, Rgb(rgb).to_rgba());
        }
    }
    Ok(output_image.into())
}

pub fn encode_from_u8_array(input_image: &[u8], message: &str) -> Result<Vec<u8>, ImgSteganoError> {
    let image = image::load_from_memory(input_image)?;
    let encoded_image = encode_from_image(image.into(), message)?;
    let Image(encoded_image) = encoded_image;
    let mut encoded: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut encoded);
    encoded_image.write_to(&mut cursor, ImageFormat::Png)?;
    Ok(encoded)
}

pub fn encode_from_path<T: Into<PathBuf>>(
    image_path: T,
    message: &str,
) -> Result<Image, ImgSteganoError> {
    let image = image::open(image_path.into())?;
    let encoded_image = encode_from_image(image.into(), message)?;
    Ok(encoded_image)
}
