mod utils;

use img_stegano_rs::ImgStegano;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode_text(input_image: &[u8], message: &str) -> Vec<u8> {
    ImgStegano::encode_from_u8_array(input_image, message).unwrap()
}

#[wasm_bindgen]
pub fn decode_text(input_image: &[u8]) -> String {
    ImgStegano::decode_from_u8_array(input_image).unwrap()
}
