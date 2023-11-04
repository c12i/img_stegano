use image::open;
use img_stegano_rs::{decode_text, encode_text};

#[test]
fn it_works() {
    let image = open("dice.png").expect("failed to open image");
    let encoded = encode_text(&image, "foo bar");
    encoded
        .save_with_format("out.png", image::ImageFormat::Png)
        .expect("Failed to save out.png");
    let encoded = open("out.png").expect("Failed to open encoded out.png file");
    let decoded_text = decode_text(&encoded);
    assert_eq!(decoded_text, "foo bar".to_string());
}
