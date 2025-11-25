mod utils;

use img_stegano::{decode_from_u8_array, encode_from_u8_array, Image};
use wasm_bindgen::prelude::*;

/// Encode a text message into an image using steganography
///
/// # Arguments
/// * `input_image` - The image data as a byte array
/// * `image_extension` - The image format extension (e.g., "png", "jpg")
/// * `message` - The text message to encode
///
/// # Returns
/// * `Ok(Vec<u8>)` - The encoded image data
/// * `Err(String)` - Error message if encoding fails
#[wasm_bindgen]
pub fn encode_text(
    input_image: &[u8],
    image_extension: &str,
    message: &str,
) -> Result<Vec<u8>, String> {
    utils::set_panic_hook();

    if input_image.is_empty() {
        return Err("Input image is empty".to_string());
    }

    if message.is_empty() {
        return Err("Message is empty".to_string());
    }

    encode_from_u8_array(input_image, image_extension, message)
        .map_err(|e| format!("Encoding failed: {}", e))
}

/// Decode a text message from an image using steganography
///
/// # Arguments
/// * `input_image` - The encoded image data as a byte array
///
/// # Returns
/// * `Ok(String)` - The decoded text message
/// * `Err(String)` - Error message if decoding fails
#[wasm_bindgen]
pub fn decode_text(input_image: &[u8]) -> Result<String, String> {
    utils::set_panic_hook();

    if input_image.is_empty() {
        return Err("Input image is empty".to_string());
    }

    decode_from_u8_array(input_image).map_err(|e| format!("Decoding failed: {}", e))
}

/// Get the maximum message capacity (in bytes) for an image
///
/// # Arguments
/// * `input_image` - The image data as a byte array
///
/// # Returns
/// * `Ok(usize)` - The maximum capacity in bytes
/// * `Err(String)` - Error message if the image cannot be loaded
#[wasm_bindgen]
pub fn get_image_capacity(input_image: &[u8]) -> Result<usize, String> {
    utils::set_panic_hook();

    if input_image.is_empty() {
        return Err("Input image is empty".to_string());
    }

    let image =
        image::load_from_memory(input_image).map_err(|e| format!("Failed to load image: {}", e))?;

    let img: Image = image.into();
    Ok(img.capacity())
}
