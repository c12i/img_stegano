// Tests for PNG format support
// This library only supports PNG for reliable LSB steganography

use image::{DynamicImage, GrayImage, ImageFormat, RgbImage};
use img_stegano::{decode_from_u8_array, encode_from_u8_array};
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

    let encoded = encode_from_u8_array(&buffer, TEST_MESSAGE).expect("Failed to encode PNG");
    let decoded = decode_from_u8_array(&encoded).expect("Failed to decode PNG");
    assert_eq!(decoded, TEST_MESSAGE);
}

#[test]
fn test_png_with_different_input_formats() {
    // Test that we can read various input formats but always output PNG
    let img = create_test_image();

    // Test with BMP input
    let mut bmp_buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut bmp_buffer), ImageFormat::Bmp)
        .unwrap();

    let encoded =
        encode_from_u8_array(&bmp_buffer, TEST_MESSAGE).expect("Failed to encode from BMP");
    let decoded = decode_from_u8_array(&encoded).expect("Failed to decode");
    assert_eq!(decoded, TEST_MESSAGE);

    // Test with JPEG input (lossy, but we can still read it)
    let mut jpeg_buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut jpeg_buffer), ImageFormat::Jpeg)
        .unwrap();

    let encoded =
        encode_from_u8_array(&jpeg_buffer, TEST_MESSAGE).expect("Failed to encode from JPEG");
    let decoded = decode_from_u8_array(&encoded).expect("Failed to decode");
    assert_eq!(decoded, TEST_MESSAGE);
}

#[test]
fn test_png_with_extreme_aspect_ratios() {
    for (width, height) in [(1200, 8), (8, 1200), (257, 13), (13, 257)] {
        let img = DynamicImage::ImageRgb8(RgbImage::from_fn(width, height, |x, y| {
            image::Rgb([(x % 256) as u8, (y % 256) as u8, 128])
        }));
        let mut buffer = Vec::new();
        img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
            .unwrap();

        let encoded = encode_from_u8_array(&buffer, TEST_MESSAGE)
            .unwrap_or_else(|_| panic!("Failed to encode {width}x{height} PNG"));
        let decoded = decode_from_u8_array(&encoded)
            .unwrap_or_else(|_| panic!("Failed to decode {width}x{height} PNG"));
        assert_eq!(decoded, TEST_MESSAGE, "Failed for {width}x{height} PNG");
    }
}

#[test]
fn test_grayscale_png_with_extreme_aspect_ratios() {
    for (width, height) in [(1200, 8), (8, 1200)] {
        let img = DynamicImage::ImageLuma8(GrayImage::from_fn(width, height, |x, y| {
            image::Luma([((x + y) % 256) as u8])
        }));
        let mut buffer = Vec::new();
        img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
            .unwrap();

        let encoded = encode_from_u8_array(&buffer, TEST_MESSAGE)
            .unwrap_or_else(|_| panic!("Failed to encode {width}x{height} grayscale PNG"));
        let decoded = decode_from_u8_array(&encoded)
            .unwrap_or_else(|_| panic!("Failed to decode {width}x{height} grayscale PNG"));
        assert_eq!(
            decoded, TEST_MESSAGE,
            "Failed for {width}x{height} grayscale PNG"
        );
    }
}
