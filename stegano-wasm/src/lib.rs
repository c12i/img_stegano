mod utils;

use img_stegano::ImgStegano;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode_text(input_image: &[u8], image_extension: &str, message: &str) -> Vec<u8> {
    utils::set_panic_hook();
    ImgStegano::encode_from_u8_array(input_image, image_extension, message).unwrap()
}

#[wasm_bindgen]
pub fn decode_text(input_image: &[u8]) -> String {
    ImgStegano::decode_from_u8_array(input_image).unwrap()
}
