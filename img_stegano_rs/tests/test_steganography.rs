use std::{fs::File, io::Read};

use image::open;
use img_stegano_rs::{decode_text, encode_text, ImgStegano};

#[test]
fn test_encode_and_decode_text() {
    let image = open("dice.png").expect("failed to open image");
    let encoded = encode_text(&image, "foo bar");
    encoded
        .save_with_format("out.png", image::ImageFormat::Png)
        .expect("Failed to save out.png");
    let encoded = open("out.png").expect("Failed to open encoded out.png file");
    let decoded_text = decode_text(&encoded);
    assert_eq!(decoded_text, "foo bar".to_string());
}

#[test]
fn test_encode_and_decode_from_u8_array() {
    let mut file = File::open("dice.png").expect("failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let encoded = ImgStegano::encode_from_u8_array(&buffer, "png", "foo bar")
        .expect("Failed to encode message to image");
    let encoded = image::load_from_memory_with_format(&encoded, image::ImageFormat::Png)
        .expect("Failed to load image");
    encoded
        .save_with_format("out2.png", image::ImageFormat::Png)
        .expect("Failed to save out2.png");
    let mut decoded = File::open("out2.png").expect("Failed to open input file");
    let mut decoded_buffer = Vec::new();
    decoded.read_to_end(&mut decoded_buffer).unwrap();
    let decoded =
        ImgStegano::decode_from_u8_array(&decoded_buffer).expect("Failed to decode image");
    assert_eq!(decoded, "foo bar".to_string());
}
