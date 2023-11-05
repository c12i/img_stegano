use std::{fs::File, io::Read};

use img_stegano_core::{Image, ImageFormat, ImgStegano};

#[test]
fn test_encode_and_decode_from_image() {
    let image = Image::open("dice.png").expect("failed to open image");
    let encoded = ImgStegano::encode_from_image(image, "foo bar");
    encoded
        .save("out.png", ImageFormat::Png)
        .expect("Failed to save out.png");
    let encoded = Image::open("out.png").expect("Failed to open image");
    let decoded_text = ImgStegano::decode_from_image(&encoded);
    assert_eq!(decoded_text, "foo bar".to_string());
}

#[test]
fn test_encode_and_decode_from_u8_array() {
    let mut file = File::open("dice.png").expect("failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let encoded = ImgStegano::encode_from_u8_array(&buffer, "png", "foo bar")
        .expect("Failed to encode message to image");
    let encoded = Image::open_from_u8_array(&encoded, ImageFormat::Png)
        .expect("Failed to open image from buffer");
    encoded
        .save("out2.png", ImageFormat::Png)
        .expect("Failed to save out2.png");
    let mut decoded = File::open("out2.png").expect("Failed to open input file");
    let mut decoded_buffer = Vec::new();
    decoded.read_to_end(&mut decoded_buffer).unwrap();
    let decoded =
        ImgStegano::decode_from_u8_array(&decoded_buffer).expect("Failed to decode image");
    assert_eq!(decoded, "foo bar".to_string());
}

#[test]
fn test_encode_and_decode_from_path() {
    let encoded = ImgStegano::encode_from_path("dice.png", "foo bar")
        .expect("Failed to encode text to image");
    encoded
        .save("out3.png", ImageFormat::Png)
        .expect("Failed to save out3.png");
    let decoded_text =
        ImgStegano::decode_from_path("out3.png").expect("Failed to decode text from image");
    assert_eq!(decoded_text, "foo bar".to_string());
}
