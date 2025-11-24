mod utils;

use img_stegano::{decode_from_u8_array, encode_from_u8_array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode_text(input_image: &[u8], image_extension: &str, message: &str) -> Option<Vec<u8>> {
    utils::set_panic_hook();
    if input_image.is_empty() || message.is_empty() {
        return None;
    }
    encode_from_u8_array(input_image, image_extension, message).ok()
}

#[wasm_bindgen]
pub fn decode_text(input_image: &[u8]) -> Option<String> {
    utils::set_panic_hook();
    decode_from_u8_array(input_image).ok()
}
