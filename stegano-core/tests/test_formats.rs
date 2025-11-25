use image::{DynamicImage, RgbImage};
use img_stegano_core::{decode_from_u8_array, encode_from_u8_array, ImageFormat};
use std::io::Cursor;

const TEST_MESSAGE: &str = "Hello World";

fn create_test_image() -> DynamicImage {
    DynamicImage::ImageRgb8(RgbImage::from_fn(100, 100, |x, y| {
        image::Rgb([(x % 256) as u8, (y % 256) as u8, 128])
    }))
}

#[test]
fn test_png_format() {
    let img = create_test_image();
    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
        .unwrap();

    let encoded = encode_from_u8_array(&buffer, "png", TEST_MESSAGE).expect("Failed to encode PNG");
    let decoded = decode_from_u8_array(&encoded).expect("Failed to decode PNG");
    assert_eq!(decoded, TEST_MESSAGE);
}

#[test]
fn test_bmp_format() {
    let img = create_test_image();
    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Bmp)
        .unwrap();

    let encoded = encode_from_u8_array(&buffer, "bmp", TEST_MESSAGE).expect("Failed to encode BMP");
    let decoded = decode_from_u8_array(&encoded).expect("Failed to decode BMP");
    assert_eq!(decoded, TEST_MESSAGE);
}

#[test]
fn test_tiff_format() {
    let img = create_test_image();
    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Tiff)
        .unwrap();

    let encoded =
        encode_from_u8_array(&buffer, "tiff", TEST_MESSAGE).expect("Failed to encode TIFF");
    let decoded = decode_from_u8_array(&encoded).expect("Failed to decode TIFF");
    assert_eq!(decoded, TEST_MESSAGE);
}

#[test]
fn test_webp_format() {
    let img = create_test_image();
    let mut buffer = Vec::new();

    match img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::WebP) {
        Ok(_) => {
            let encoded =
                encode_from_u8_array(&buffer, "webp", TEST_MESSAGE).expect("Failed to encode WebP");
            let decoded = decode_from_u8_array(&encoded).expect("Failed to decode WebP");
            assert_eq!(decoded, TEST_MESSAGE);
        }
        Err(e) => {
            println!("WebP encoding not supported: {}", e);
        }
    }
}
