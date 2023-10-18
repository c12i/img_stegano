use image::open;
use steganography_rust::{decode_text, encode_text};

#[test]
fn it_works() {
    let mut image = open("dice.png").expect("failed to open image");
    let encoded = encode_text(&mut image, "foo bar").expect("Failed to encode text");
    encoded
        .save_with_format("out.png", image::ImageFormat::Png)
        .expect("Failed to save out.png");
    let encoded = open("out.png").expect("Failed to open encoded out.png file");
    let decoded_text = decode_text(&encoded);
    println!("{:?}", decoded_text);
}
